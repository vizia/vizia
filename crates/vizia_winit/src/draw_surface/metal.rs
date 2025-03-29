use std::{error::Error, sync::Arc};

use skia_safe::{
    gpu::{
        backend_render_targets, direct_contexts,
        mtl::{BackendContext, TextureInfo},
        surfaces, DirectContext, SurfaceOrigin,
    },
    ColorSpace, ColorType, Surface, SurfaceProps,
};

use vizia_core::prelude::{BoundingBox, Entity};
use vizia_window::WindowDescription;

use winit::{
    dpi::PhysicalSize,
    event_loop::ActiveEventLoop,
    raw_window_handle::{HasWindowHandle, RawWindowHandle},
    window::{Window, WindowAttributes},
};

use objc2::{rc::Retained, runtime::ProtocolObject};
use objc2_app_kit::NSView;
use objc2_foundation::{MainThreadMarker, NSSize};
use objc2_metal::{
    MTLCommandBuffer, MTLCommandQueue, MTLCreateSystemDefaultDevice, MTLDevice, MTLDrawable,
    MTLPixelFormat,
};
use objc2_quartz_core::{CAMetalDrawable, CAMetalLayer};

use super::{DrawSurface, GraphicsBackend};

type Device = ProtocolObject<dyn MTLDevice>;
type Drawable = ProtocolObject<dyn MTLDrawable>;
type CommandQueue = ProtocolObject<dyn MTLCommandQueue>;

pub struct WinState {
    entity: Entity,
    window: Arc<Window>,

    vsync: bool,
    is_initially_cloaked: bool,

    surface: Option<(Surface, Retained<Drawable>)>,
    dirty_surface: Option<(Surface, Retained<Drawable>)>,

    direct_context: DirectContext,

    metal_layers: [Retained<CAMetalLayer>; 2],
    queue: Retained<CommandQueue>,
    device: Retained<Device>,
    view: Retained<NSView>,
}

impl DrawSurface for WinState {
    fn entity(&self) -> Entity {
        self.entity
    }

    fn window(&self) -> Arc<Window> {
        self.window.clone()
    }

    fn backend(&self) -> GraphicsBackend {
        GraphicsBackend::Metal
    }

    fn surfaces_mut(&mut self) -> Option<(&mut Surface, &mut Surface)> {
        if self.surface.is_none() {
            self.surface = Some(self.create_surface(0));
        }
        if self.dirty_surface.is_none() {
            self.dirty_surface = Some(self.create_surface(1));
        }

        let (surface, _) = self.surface.as_mut()?;
        let (dirty_surface, _) = self.dirty_surface.as_mut()?;

        Some((surface, dirty_surface))
    }

    fn swap_buffers(&mut self, _dirty_rect: BoundingBox) {
        let layer = &self.metal_layers[0];
        unsafe {
            self.view.setWantsLayer(true);
            self.view.setLayer(Some(layer));
        };

        let (surface, drawable) = self.surface.as_mut().unwrap();
        self.direct_context.flush_and_submit_surface(surface, None);

        let command_buffer = self.queue.commandBuffer().unwrap();
        command_buffer.presentDrawable(drawable);
        command_buffer.commit();

        self.dirty_surface = self.surface.take();
        self.metal_layers.swap(0, 1);
    }

    fn resize(&mut self, size: PhysicalSize<u32>) -> bool {
        let [w, h] = self.drawable_size();

        if size.width == w && size.height == h {
            return false;
        }
        if size.width == 0 || size.height == 0 {
            return false;
        }

        self.surface = None;
        self.dirty_surface = None;

        self.metal_layers = create_metal_layers(&self.device, self.vsync, size);

        true
    }

    fn is_initially_cloaked(&mut self) -> &mut bool {
        &mut self.is_initially_cloaked
    }
}

impl WinState {
    pub fn new(
        entity: Entity,
        window_attributes: &WindowAttributes,
        window_description: &WindowDescription,
        event_loop: &ActiveEventLoop,
    ) -> Result<Self, Box<dyn Error>> {
        let vsync = window_description.vsync;

        let window = event_loop.create_window(window_attributes.clone())?;

        let RawWindowHandle::AppKit(handle) = window.window_handle().unwrap().as_raw() else {
            unreachable!();
        };

        let view = unsafe {
            Retained::retain(handle.ns_view.as_ptr().cast())
                .expect("Failed to get NSView from Window.")
        };

        let device = MTLCreateSystemDefaultDevice().expect("Failed to get default system device.");
        let queue = device.newCommandQueue().expect("Failed to create a command queue.");
        let metal_layers = create_metal_layers(&device, vsync, window.inner_size());

        let backend_context = unsafe {
            BackendContext::new(
                Retained::as_ptr(&device).cast(),
                Retained::as_ptr(&queue).cast(), //
            )
        };
        let direct_context = direct_contexts::make_metal(&backend_context, None).unwrap();

        let this = Self {
            entity,
            window: window.into(),

            vsync,
            is_initially_cloaked: false,

            surface: None,
            dirty_surface: None,

            direct_context,

            metal_layers,

            queue,
            device,
            view,
        };

        Ok(this)
    }

    pub fn drawable_size(&self) -> [u32; 2] {
        let size = unsafe { self.metal_layers[0].drawableSize() };
        [size.width as u32, size.height as u32]
    }

    fn create_surface(&mut self, index: usize) -> (Surface, Retained<Drawable>) {
        let layer = &self.metal_layers[index];

        let drawable = unsafe { layer.nextDrawable().unwrap() };
        let texture = unsafe { Retained::as_ptr(&drawable.texture()) };
        let texture_info = unsafe { TextureInfo::new(texture.cast()) };

        let backend_render_target =
            backend_render_targets::make_mtl(self.window.inner_size().into(), &texture_info);

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
            ColorType::BGRA8888,
            ColorSpace::new_srgb(),
            Some(&surface_props),
        )
        .unwrap();

        let drawable = ProtocolObject::from_retained(drawable);

        (surface, drawable)
    }
}

fn create_metal_layers(
    device: &Device,
    vsync: bool,
    size: PhysicalSize<u32>,
) -> [Retained<CAMetalLayer>; 2] //
{
    let mtm = MainThreadMarker::new().unwrap();

    let (width, height) = size.into();
    let size = NSSize { width, height };

    std::array::from_fn(|_| unsafe {
        let layer = CAMetalLayer::init(mtm.alloc());
        layer.setDevice(Some(device));
        layer.setDisplaySyncEnabled(vsync);
        layer.setPixelFormat(MTLPixelFormat::BGRA8Unorm);
        layer.setPresentsWithTransaction(false);
        layer.setDrawableSize(size);

        // Disabling this option allows Skia's Blend Mode to work.
        // More about: https://developer.apple.com/documentation/quartzcore/cametallayer/1478168-framebufferonly
        layer.setFramebufferOnly(false);

        // layer.setNeedsDisplay();
        layer
    })
}
