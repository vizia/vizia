use glutin::context::GlProfile;
use glutin::surface::SwapInterval;

use std::error::Error;
use std::num::NonZeroU32;
use std::{ffi::CString, sync::Arc};
use winit::raw_window_handle::HasWindowHandle;

use gl::types::*;
use glutin::{
    config::{Config, ConfigTemplateBuilder},
    context::{ContextApi, ContextAttributesBuilder},
    display::GetGlDisplay,
    prelude::*,
    surface::{SurfaceAttributesBuilder, WindowSurface},
};
use glutin_winit::DisplayBuilder;

use skia_safe::{
    gpu::{
        self, backend_render_targets, ganesh::context_options, gl::FramebufferInfo, ContextOptions,
        DirectContext, SurfaceOrigin,
    },
    ColorSpace, ColorType, PixelGeometry, Surface, SurfaceProps, SurfacePropsFlags,
};

use winit::{
    dpi::PhysicalSize,
    event_loop::ActiveEventLoop,
    window::{Window, WindowAttributes},
};

use vizia_core::prelude::*;
use vizia_window::{GraphicsBackend, WindowDescription};

use crate::draw_surface::DrawSurface;

#[allow(unused)] // TODO
pub(super) struct WinState {
    entity: Entity,
    window: Arc<Window>,

    vsync: bool,
    is_initially_cloaked: bool,

    gl_config: Config,
    gl_context: glutin::context::PossiblyCurrentContext,
    gl_surface: glutin::surface::Surface<glutin::surface::WindowSurface>,

    gr_context: DirectContext,
    surface: skia_safe::Surface,
    dirty_surface: skia_safe::Surface,
    should_close: bool,
}

impl Drop for WinState {
    fn drop(&mut self) {
        self.gl_context.make_current(&self.gl_surface).unwrap();
    }
}

impl WinState {
    pub(crate) fn new(
        entity: Entity,
        window_attributes: &WindowAttributes,
        window_description: &WindowDescription,
        event_loop: &ActiveEventLoop,
    ) -> Result<Self, Box<dyn Error>> {
        let (window, gl_config) = build_window(event_loop, window_attributes.clone());

        let window = Arc::new(window.expect("failed to create window"));

        let raw_window_handle = window.window_handle().unwrap().as_raw();

        let gl_display = gl_config.display();

        let not_current_gl_context = [
            ContextApi::OpenGl(None), // default
            ContextApi::Gles(None),   // fallback
        ]
        .into_iter()
        .find_map(|context_api| unsafe {
            gl_display
                .create_context(
                    &gl_config,
                    &ContextAttributesBuilder::new()
                        .with_profile(GlProfile::Core)
                        .with_context_api(context_api)
                        .build(Some(raw_window_handle)),
                )
                .ok()
        })
        .expect("failed to create context");

        let PhysicalSize { width, height } = window.inner_size();
        let width = NonZeroU32::new(width.max(1)).unwrap();
        let height = NonZeroU32::new(height.max(1)).unwrap();

        let attrs = SurfaceAttributesBuilder::<WindowSurface>::new() //
            .with_srgb(Some(true))
            .build(raw_window_handle, width, height);

        let gl_surface =
            unsafe { gl_config.display().create_window_surface(&gl_config, &attrs).unwrap() };

        let gl_context = not_current_gl_context.make_current(&gl_surface).unwrap();

        if !window_description.vsync {
            gl_surface
                .set_swap_interval(&gl_context, SwapInterval::DontWait)
                .expect("Failed to set vsync");
        }

        // Build skia renderer
        gl::load_with(|s| {
            gl_config.display().get_proc_address(CString::new(s).unwrap().as_c_str())
        });

        let interface = skia_safe::gpu::gl::Interface::new_load_with(|name| {
            if name == "eglGetCurrentDisplay" {
                return std::ptr::null();
            }
            gl_config.display().get_proc_address(CString::new(name).unwrap().as_c_str())
        })
        .expect("Could not create interface");

        // https://github.com/rust-skia/rust-skia/issues/476
        let mut context_options = ContextOptions::new();
        context_options.skip_gl_error_checks = context_options::Enable::Yes;

        let mut gr_context = skia_safe::gpu::direct_contexts::make_gl(interface, &context_options)
            .expect("Could not create direct context");

        let fb_info = {
            let mut fboid: GLint = 0;
            unsafe { gl::GetIntegerv(gl::FRAMEBUFFER_BINDING, &mut fboid) };

            FramebufferInfo {
                fboid: fboid.try_into().unwrap(),
                format: skia_safe::gpu::gl::Format::RGBA8.into(),
                ..Default::default()
            }
        };

        let inner_size: (i32, i32) = window.inner_size().into();

        let mut surface = create_surface(
            &window,
            fb_info,
            &mut gr_context,
            gl_config.num_samples() as usize,
            gl_config.stencil_size() as usize,
        );
        let dirty_surface = surface.new_surface_with_dimensions(inner_size).unwrap();

        Ok(Self {
            entity,
            window,

            vsync: window_description.vsync,
            is_initially_cloaked: cfg!(target_os = "windows"),

            gl_config,
            gl_context,
            gl_surface,

            gr_context,

            surface,
            dirty_surface,

            should_close: false,
        })
    }
}

impl DrawSurface for WinState {
    fn backend(&self) -> GraphicsBackend {
        GraphicsBackend::Gl
    }

    #[inline]
    fn entity(&self) -> Entity {
        self.entity
    }

    #[inline]
    fn window(&self) -> Arc<Window> {
        self.window.clone()
    }

    fn resize(&mut self, size: PhysicalSize<u32>) -> bool {
        let (width, height): (u32, u32) = size.into();

        if width == 0 || height == 0 {
            return false;
        }

        let fb_info = {
            let mut fboid: GLint = 0;
            unsafe { gl::GetIntegerv(gl::FRAMEBUFFER_BINDING, &mut fboid) };

            FramebufferInfo {
                fboid: fboid.try_into().unwrap(),
                format: skia_safe::gpu::gl::Format::RGBA8.into(),
                ..Default::default()
            }
        };

        self.gl_context.make_current(&self.gl_surface).unwrap();

        self.surface = create_surface(
            &self.window,
            fb_info,
            &mut self.gr_context,
            self.gl_config.num_samples() as usize,
            self.gl_config.stencil_size() as usize,
        );

        self.dirty_surface = self
            .surface
            .new_surface_with_dimensions((width.max(1) as i32, height.max(1) as i32))
            .unwrap();

        self.gl_surface.resize(
            &self.gl_context,
            NonZeroU32::new(width.max(1)).unwrap(),
            NonZeroU32::new(height.max(1)).unwrap(),
        );

        true
    }

    fn make_current(&mut self) {
        self.gl_context.make_current(&self.gl_surface).unwrap();
    }

    fn surfaces_mut(&mut self) -> Option<(&mut Surface, &mut Surface)> {
        Some((&mut self.surface, &mut self.dirty_surface))
    }

    fn swap_buffers(&mut self, _dirty_rect: BoundingBox) {
        self.gr_context.flush_and_submit();
        self.gl_surface.swap_buffers(&self.gl_context).expect("Failed to swap buffers");
    }

    fn is_initially_cloaked(&mut self) -> &mut bool {
        &mut self.is_initially_cloaked
    }
}

impl From<WinState> for Box<dyn DrawSurface> {
    #[inline]
    fn from(value: WinState) -> Self {
        Box::new(value) as Self
    }
}

fn build_window(
    event_loop: &ActiveEventLoop,
    window_attributes: WindowAttributes,
) -> (Option<winit::window::Window>, Config) {
    let template = ConfigTemplateBuilder::new().with_alpha_size(8).with_transparency(true);
    let display_builder = DisplayBuilder::new().with_window_attributes(Some(window_attributes));

    display_builder
        .build(event_loop, template, |configs| {
            // Find the config with the maximum number of samples, so our triangle will
            // be smooth.
            configs
                .reduce(|accum, config| {
                    let transparency_check = config.supports_transparency().unwrap_or(false)
                        & !accum.supports_transparency().unwrap_or(false);

                    if transparency_check || config.num_samples() < accum.num_samples() {
                        config
                    } else {
                        accum
                    }
                })
                .unwrap()
        })
        .unwrap()
}

pub fn create_surface(
    window: &Window,
    fb_info: FramebufferInfo,
    gr_context: &mut DirectContext,
    num_samples: usize,
    stencil_size: usize,
) -> Surface {
    let size = window.inner_size();
    let size = (
        size.width.try_into().expect("Could not convert width"),
        size.height.try_into().expect("Could not convert height"),
    );

    let backend_render_target =
        backend_render_targets::make_gl(size, num_samples, stencil_size, fb_info);

    let surface_props = SurfaceProps::new_with_text_properties(
        SurfacePropsFlags::default(),
        PixelGeometry::default(),
        0.5,
        0.0,
    );

    gpu::surfaces::wrap_backend_render_target(
        gr_context,
        &backend_render_target,
        SurfaceOrigin::BottomLeft,
        ColorType::RGBA8888,
        ColorSpace::new_srgb(),
        Some(surface_props).as_ref(),
        // None,
    )
    .expect("Could not create skia surface")
}
