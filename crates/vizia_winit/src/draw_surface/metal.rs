use std::{error::Error, sync::Arc};

use skia_safe::{
    ColorSpace, ColorType, Surface,
    gpu::{
        DirectContext, SurfaceOrigin, backend_render_targets, direct_contexts,
        mtl::{BackendContext, TextureInfo},
        surfaces,
    },
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
    presents_with_transaction: bool,
    is_initially_cloaked: bool,
    current_size: PhysicalSize<u32>,

    surface: Option<(Surface, Retained<Drawable>)>,
    dirty_surface: Option<Surface>,

    direct_context: DirectContext,

    metal_layer: Retained<CAMetalLayer>,
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
            self.surface = Some(self.create_surface());
        }
        if self.dirty_surface.is_none() {
            self.dirty_surface = Some(self.create_dirty_surface());
        }

        let (surface, _) = self.surface.as_mut()?;
        let dirty_surface = self.dirty_surface.as_mut()?;

        Some((surface, dirty_surface))
    }

    fn swap_buffers(&mut self, _dirty_rect: BoundingBox) {
        let (surface, drawable) = self.surface.as_mut().unwrap();
        self.direct_context.flush_and_submit_surface(surface, None);

        let command_buffer = self.queue.commandBuffer().unwrap();
        command_buffer.presentDrawable(drawable);
        command_buffer.commit();
        if self.presents_with_transaction {
            command_buffer.waitUntilScheduled();
        }

        self.surface = None;
    }

    fn set_presents_with_transaction(&mut self, enabled: bool) {
        self.presents_with_transaction = enabled;
        self.metal_layer.setPresentsWithTransaction(enabled);
    }

    fn resize(&mut self, size: PhysicalSize<u32>) -> bool {
        if size.width == 0 || size.height == 0 {
            return false;
        }
        if size == self.current_size {
            return false;
        }

        self.current_size = size;
        self.surface = None;
        self.dirty_surface = None;

        let (width, height) = size.into();
        self.metal_layer.setDrawableSize(NSSize { width, height });

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
        let current_size = window.inner_size();

        let RawWindowHandle::AppKit(handle) = window.window_handle().unwrap().as_raw() else {
            unreachable!();
        };

        let view: Retained<NSView> = unsafe {
            Retained::retain(handle.ns_view.as_ptr().cast())
                .expect("Failed to get NSView from Window.")
        };

        let device = MTLCreateSystemDefaultDevice().expect("Failed to get default system device.");
        let queue = device.newCommandQueue().expect("Failed to create a command queue.");
        let metal_layer = create_metal_layer(&device, vsync, window.inner_size());

        view.setWantsLayer(true);
        view.setLayer(Some(&metal_layer));

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
            presents_with_transaction: false,
            is_initially_cloaked: false,
            current_size,

            surface: None,
            dirty_surface: None,

            direct_context,

            metal_layer,

            queue,
            device,
            view,
        };

        Ok(this)
    }

    pub fn drawable_size(&self) -> [u32; 2] {
        let size = self.metal_layer.drawableSize();
        [size.width as u32, size.height as u32]
    }

    fn create_surface(&mut self) -> (Surface, Retained<Drawable>) {
        let drawable = self.metal_layer.nextDrawable().unwrap();
        let texture = Retained::as_ptr(&drawable.texture());
        let texture_info = unsafe { TextureInfo::new(texture.cast()) };

        let [w, h] = self.drawable_size();
        let backend_render_target =
            backend_render_targets::make_mtl((w as i32, h as i32), &texture_info);

        let surface = surfaces::wrap_backend_render_target(
            &mut self.direct_context,
            &backend_render_target,
            SurfaceOrigin::TopLeft,
            ColorType::BGRA8888,
            ColorSpace::new_srgb(),
            None,
        )
        .unwrap();

        let drawable = ProtocolObject::from_retained(drawable);

        (surface, drawable)
    }

    fn create_dirty_surface(&mut self) -> Surface {
        let [w, h] = self.drawable_size();
        let (surface, _) = self.surface.as_mut().unwrap();
        surface.new_surface_with_dimensions((w as i32, h as i32)).unwrap()
    }
}

fn create_metal_layer(
    device: &Device,
    vsync: bool,
    size: PhysicalSize<u32>,
) -> Retained<CAMetalLayer> //
{
    let mtm = MainThreadMarker::new().unwrap();

    let (width, height) = size.into();
    let size = NSSize { width, height };

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
}
