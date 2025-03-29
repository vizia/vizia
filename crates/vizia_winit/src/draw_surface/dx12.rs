use std::{error::Error, sync::Arc};

use skia_safe::{
    gpu::{
        d3d::{BackendContext, TextureResourceInfo},
        surfaces, BackendRenderTarget, DirectContext, Protected, SurfaceOrigin,
    },
    ColorSpace, ColorType, Surface, SurfaceProps,
};

use windows::{
    core::Interface,
    Win32::{
        Foundation::*,
        Graphics::{
            Direct3D::*,
            Direct3D12::*,
            Dxgi::{Common::*, *},
        },
        System::Threading::*,
    },
};

use winit::{
    dpi::PhysicalSize,
    event_loop::ActiveEventLoop,
    window::{Window, WindowAttributes},
};

use vizia_core::prelude::{BoundingBox, Entity};
use vizia_window::WindowDescription;

use super::{DrawSurface, GraphicsBackend};

const BUFFER_COUNT: u32 = 2;

#[allow(unused)]
pub(super) struct WinState {
    entity: Entity,
    window: Arc<Window>,

    vsync: bool,
    is_initially_cloaked: bool,

    surfaces: Vec<(Surface, BackendRenderTarget)>,

    direct_context: DirectContext,
    backend_context: BackendContext,

    swap_chain: IDXGISwapChain3,
    swap_chain_waitable: HANDLE,

    sync_interval: u32,
    present_flags: DXGI_PRESENT,

    inner_size: PhysicalSize<u32>,
    buffer_size: PhysicalSize<u32>,
}

impl WinState {
    pub fn new(
        entity: Entity,
        window_attributes: &WindowAttributes,
        window_description: &WindowDescription,
        event_loop: &ActiveEventLoop,
    ) -> Result<Self, Box<dyn Error>> {
        let vsync = window_description.vsync;

        let window = Arc::new(event_loop.create_window(window_attributes.clone())?);

        let inner_size = window.inner_size();
        let buffer_size = window.current_monitor().map_or(inner_size, |monitor| monitor.size());

        let (factory, adapter, device) =
            get_hardware_adapter_and_device() //
                .expect("Failed to get hardware adapter and device.");

        let queue = create_command_queue(&device) //
            .expect("Failed to create command queue.");

        let (sync_interval, present_flags) = get_present_args(&factory, vsync) //
            .expect("Failed to get present args.");

        let (swap_chain, swap_chain_waitable) = create_swap_chain(
            &factory,
            &queue,
            &window,
            inner_size,
            buffer_size,
            sync_interval,
            present_flags,
        )
        .expect("Failed to create swap chain.");

        let (direct_context, backend_context) =
            create_skia_contexts(adapter, device, queue) //
                .expect("Failed to create Skia contexts.");

        let mut this = Self {
            entity,
            window,

            surfaces: vec![],

            direct_context,
            backend_context,

            swap_chain,
            swap_chain_waitable,

            sync_interval,
            present_flags,

            inner_size,
            buffer_size,

            vsync,
            is_initially_cloaked: true,
        };

        this.create_surfaces();

        Ok(this)
    }

    pub fn current_surface_index(&self) -> usize {
        unsafe { self.swap_chain.GetCurrentBackBufferIndex() as usize }
    }

    pub fn create_surfaces(&mut self) {
        let size = self.inner_size.into();

        self.surfaces.clear();
        self.surfaces.extend((0..BUFFER_COUNT).map(|i| {
            let resource = unsafe { self.swap_chain.GetBuffer(i).unwrap() };

            let mut info = TextureResourceInfo::from_resource(resource);
            info.format = DXGI_FORMAT_R8G8B8A8_UNORM;

            let backend_render_target = BackendRenderTarget::new_d3d(size, &info);

            let surface_props = SurfaceProps::new_with_text_properties(
                Default::default(),
                Default::default(),
                0.5,
                0.0,
            );

            let surface = surfaces::wrap_backend_render_target(
                &mut self.direct_context,
                &backend_render_target,
                SurfaceOrigin::TopLeft,
                ColorType::RGBA8888,
                ColorSpace::new_srgb(),
                Some(&surface_props),
            )
            .unwrap();

            (surface, backend_render_target)
        }));
    }

    fn image_bounds(&self) -> BoundingBox {
        let (w, h) = self.inner_size.into();
        BoundingBox { x: 0.0, y: 0.0, w, h }
    }
}

impl DrawSurface for WinState {
    fn backend(&self) -> GraphicsBackend {
        GraphicsBackend::Dx12
    }

    fn entity(&self) -> Entity {
        self.entity
    }

    fn window(&self) -> Arc<Window> {
        self.window.clone()
    }

    fn surfaces_mut(&mut self) -> Option<(&mut Surface, &mut Surface)> {
        let index = self.current_surface_index();

        let [(s0, _), (s1, _)] = self.surfaces.as_mut_slice() else { unreachable!() };

        Some(if index == 0 { (s0, s1) } else { (s1, s0) })
    }

    fn swap_buffers(&mut self, dirty_rect: BoundingBox) {
        let dirty_rect = self.image_bounds().intersection(&dirty_rect);
        if dirty_rect.is_empty() {
            return;
        }

        let mut rects = [RECT {
            left: dirty_rect.left() as i32,
            top: dirty_rect.top() as i32,
            right: dirty_rect.right() as i32,
            bottom: dirty_rect.bottom() as i32,
        }];

        let index = self.current_surface_index();
        let (surface, _) = &mut self.surfaces[index];

        unsafe {
            WaitForSingleObject(self.swap_chain_waitable, 1000);

            self.direct_context.flush_and_submit_surface(surface, None);

            self.swap_chain
                .Present1(
                    self.sync_interval,
                    self.present_flags,
                    &DXGI_PRESENT_PARAMETERS {
                        DirtyRectsCount: 1,
                        pDirtyRects: rects.as_mut_ptr(),
                        pScrollRect: std::ptr::null_mut(),
                        pScrollOffset: std::ptr::null_mut(),
                    },
                )
                .unwrap();
        }
    }

    fn draw_surface(&mut self, draw: &mut dyn FnMut(&mut Surface) -> BoundingBox) {
        let Some((surface, _)) = self.surfaces_mut() else {
            return;
        };

        let dirty_rect = draw(surface);

        self.swap_buffers(dirty_rect);
    }

    fn resize(&mut self, size: PhysicalSize<u32>) -> bool {
        let (w, h) = self.inner_size.into();

        if size.width == 0 || size.height == 0 {
            return false;
        }
        if size.width == w && size.height == h {
            return false;
        }

        self.inner_size = size;

        // We only need to resize the buffers if the inner size is larger than the buffer size.

        if self.inner_size.width > self.buffer_size.width
            || self.inner_size.height > self.buffer_size.height
        {
            self.buffer_size =
                self.window.current_monitor().map_or(self.inner_size, |monitor| monitor.size());

            // All references to our buffers must be released before calling `ResizeBuffers`.

            self.direct_context.free_gpu_resources();
            self.direct_context.reset(None);
            self.surfaces.clear();

            unsafe {
                self.swap_chain
                    .ResizeBuffers(
                        BUFFER_COUNT,
                        self.buffer_size.width,
                        self.buffer_size.height,
                        DXGI_FORMAT_R8G8B8A8_UNORM,
                        Default::default(),
                    )
                    .unwrap();
            }
        }

        self.create_surfaces();

        true
    }

    fn is_initially_cloaked(&mut self) -> &mut bool {
        &mut self.is_initially_cloaked
    }
}

/// Get the first "high performance" hardware adapter that supports Direct3D 12.
///
fn get_hardware_adapter_and_device(
) -> windows::core::Result<(IDXGIFactory6, IDXGIAdapter1, ID3D12Device)> {
    let factory: IDXGIFactory6 = unsafe { CreateDXGIFactory2(Default::default())? };

    for i in 0.. {
        let adapter: IDXGIAdapter1 =
            unsafe { factory.EnumAdapterByGpuPreference(i, DXGI_GPU_PREFERENCE_HIGH_PERFORMANCE)? };

        let adapter_desc = unsafe { adapter.GetDesc1()? };

        // Don't select the "Microsoft Basic Render Driver" adapter.
        let flags = DXGI_ADAPTER_FLAG(adapter_desc.Flags as _);
        if (flags & DXGI_ADAPTER_FLAG_SOFTWARE) != DXGI_ADAPTER_FLAG_NONE {
            continue;
        }

        let mut device = None;
        let result = unsafe { D3D12CreateDevice(&adapter, D3D_FEATURE_LEVEL_11_0, &mut device) };

        if result.is_ok() {
            return Ok((factory, adapter, device.unwrap()));
        }
    }

    unreachable!()
}

fn create_command_queue(device: &ID3D12Device) -> windows::core::Result<ID3D12CommandQueue> {
    unsafe {
        device.CreateCommandQueue(&D3D12_COMMAND_QUEUE_DESC {
            Type: D3D12_COMMAND_LIST_TYPE_DIRECT,
            Priority: 0,
            Flags: D3D12_COMMAND_QUEUE_FLAG_NONE,
            NodeMask: 0,
        })
    }
}

fn get_present_args(
    factory: &IDXGIFactory6,
    vsync: bool,
) -> windows::core::Result<(u32, DXGI_PRESENT)> {
    let mut sync_interval = 1;
    let mut present_flags = DXGI_PRESENT(0);

    if vsync == false {
        sync_interval = 0;

        // Support variable refresh rate displays. (AMD FreeSync, NVIDIA G-Sync, etc)
        let mut allow_tearing = FALSE;

        let result = unsafe {
            factory.CheckFeatureSupport(
                DXGI_FEATURE_PRESENT_ALLOW_TEARING,
                std::ptr::from_mut(&mut allow_tearing) as _,
                std::mem::size_of::<BOOL>() as _,
            )
        };

        if result.is_ok() && (allow_tearing == TRUE) {
            present_flags |= DXGI_PRESENT_ALLOW_TEARING;
        }
    }

    Ok((sync_interval, present_flags))
}

fn create_swap_chain(
    factory: &IDXGIFactory6,
    queue: &ID3D12CommandQueue,
    window: &Window,
    _inner_size: PhysicalSize<u32>,
    buffer_size: PhysicalSize<u32>,
    sync_interval: u32,
    present_flags: DXGI_PRESENT,
) -> windows::core::Result<(IDXGISwapChain3, HANDLE)> {
    let mut flags = DXGI_SWAP_CHAIN_FLAG_FRAME_LATENCY_WAITABLE_OBJECT;

    if (present_flags & DXGI_PRESENT_ALLOW_TEARING) != DXGI_PRESENT(0) {
        flags |= DXGI_SWAP_CHAIN_FLAG_ALLOW_TEARING;
    }

    let desc = DXGI_SWAP_CHAIN_DESC1 {
        Width: buffer_size.width,
        Height: buffer_size.height,
        Format: DXGI_FORMAT_R8G8B8A8_UNORM,
        Stereo: FALSE,
        SampleDesc: DXGI_SAMPLE_DESC { Count: 1, Quality: 0 },
        BufferUsage: DXGI_USAGE_RENDER_TARGET_OUTPUT,
        BufferCount: BUFFER_COUNT,
        Scaling: DXGI_SCALING_NONE,
        SwapEffect: DXGI_SWAP_EFFECT_FLIP_SEQUENTIAL,
        AlphaMode: DXGI_ALPHA_MODE_UNSPECIFIED,
        Flags: flags.0 as _,
    };

    let hwnd = HWND(u64::from(window.id()) as _);

    unsafe {
        let swap_chain = factory
            .CreateSwapChainForHwnd(queue, hwnd, &desc, None, None)?
            .cast::<IDXGISwapChain3>()?;

        let waitable = swap_chain.GetFrameLatencyWaitableObject();

        WaitForSingleObject(waitable, 1000);

        swap_chain.Present(sync_interval, present_flags).unwrap();

        Ok((swap_chain, waitable))
    }
}

fn create_skia_contexts(
    adapter: IDXGIAdapter1,
    device: ID3D12Device,
    queue: ID3D12CommandQueue,
) -> windows::core::Result<(DirectContext, BackendContext)> {
    let backend_context = BackendContext {
        adapter,
        device,
        queue,
        memory_allocator: None,
        protected_context: Protected::No,
    };
    let direct_context = unsafe { DirectContext::new_d3d(&backend_context, None).unwrap() };

    Ok((direct_context, backend_context))
}
