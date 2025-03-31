use crate::application::ApplicationRunner;
use baseview::gl::GlConfig;
use baseview::{
    Event, EventStatus, Window, WindowHandle, WindowHandler, WindowOpenOptions, WindowScalePolicy,
};
use gl::types::GLint;
use gl_rs as gl;
use raw_window_handle::HasRawWindowHandle;
use skia_safe::gpu::gl::FramebufferInfo;
use skia_safe::gpu::{
    self, backend_render_targets, ganesh::context_options, ContextOptions, SurfaceOrigin,
};
use skia_safe::{ColorSpace, ColorType, PixelGeometry, Surface, SurfaceProps, SurfacePropsFlags};

use crate::proxy::BaseviewProxy;
use vizia_core::backend::*;
use vizia_core::prelude::*;

/// Handles a vizia_baseview application
pub(crate) struct ViziaWindow {
    application: ApplicationRunner,
    #[allow(clippy::type_complexity)]
    on_idle: Option<Box<dyn Fn(&mut Context) + Send>>,
}

impl ViziaWindow {
    fn new(
        mut cx: BackendContext,
        win_desc: WindowDescription,
        window_scale_policy: WindowScalePolicy,
        window: &mut baseview::Window,
        builder: Option<Box<dyn FnOnce(&mut Context) + Send>>,
        on_idle: Option<Box<dyn Fn(&mut Context) + Send>>,
    ) -> ViziaWindow {
        let context = window.gl_context().expect("Window was created without OpenGL support");

        unsafe { context.make_current() };

        // Build skia renderer
        gl::load_with(|s| context.get_proc_address(s));
        let interface = skia_safe::gpu::gl::Interface::new_load_with(|name| {
            if name == "eglGetCurrentDisplay" {
                return std::ptr::null();
            }
            context.get_proc_address(name)
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

        let mut surface = create_surface(
            (win_desc.inner_size.width as i32, win_desc.inner_size.height as i32),
            fb_info,
            &mut gr_context,
        );

        let dirty_surface = surface
            .new_surface_with_dimensions((
                win_desc.inner_size.width as i32,
                win_desc.inner_size.height as i32,
            ))
            .unwrap();

        // Assume scale for now until there is an event with a new one.
        // Scaling is a combination of the window's scale factor (which is usually determined by the
        // operating system, or explicitly overridden by a hosting application) and a custom user
        // scale factor.
        let (use_system_scaling, window_scale_factor) = match window_scale_policy {
            WindowScalePolicy::ScaleFactor(scale) => (false, scale),
            // NOTE: This is not correct, but we should get a `Resized` event to correct this
            //       immediately after the window is created
            WindowScalePolicy::SystemScaleFactor => (true, 1.25),
        };
        let dpi_factor = window_scale_factor * win_desc.user_scale_factor;

        cx.add_main_window(Entity::root(), &win_desc, dpi_factor as f32);
        cx.add_window(WindowView {});

        cx.0.windows.insert(
            Entity::root(),
            WindowState { window_description: win_desc.clone(), ..Default::default() },
        );

        cx.context().remove_user_themes();
        if let Some(builder) = builder {
            (builder)(cx.context());
        }

        let application = ApplicationRunner::new(
            cx,
            gr_context,
            use_system_scaling,
            window_scale_factor,
            surface,
            dirty_surface,
            win_desc,
        );
        unsafe { context.make_not_current() };

        ViziaWindow { application, on_idle }
    }

    /// Open a new child window.
    ///
    /// * `parent` - The parent window.
    /// * `app` - The Vizia application builder.
    pub fn open_parented<P, F>(
        parent: &P,
        win_desc: WindowDescription,
        scale_policy: WindowScalePolicy,
        app: F,
        on_idle: Option<Box<dyn Fn(&mut Context) + Send>>,
        ignore_default_theme: bool,
    ) -> WindowHandle
    where
        P: HasRawWindowHandle,
        F: Fn(&mut Context),
        F: 'static + Send,
    {
        let window_settings = WindowOpenOptions {
            title: win_desc.title.clone(),
            size: baseview::Size::new(
                // We have our own uniform non-DPI scaling factor that gets applied in addition to
                // the DPI scaling since both can change independently at runtime
                win_desc.inner_size.width as f64 * win_desc.user_scale_factor,
                win_desc.inner_size.height as f64 * win_desc.user_scale_factor,
            ),
            scale: scale_policy,
            gl_config: Some(GlConfig { vsync: false, ..GlConfig::default() }),
        };

        Window::open_parented(
            parent,
            window_settings,
            move |window: &mut baseview::Window<'_>| -> ViziaWindow {
                let mut cx = Context::new();

                cx.ignore_default_theme = ignore_default_theme;
                cx.remove_user_themes();

                let mut cx = BackendContext::new(cx);

                cx.set_event_proxy(Box::new(BaseviewProxy));
                ViziaWindow::new(cx, win_desc, scale_policy, window, Some(Box::new(app)), on_idle)
            },
        )
    }

    /// Open a new window that blocks the current thread until the window is destroyed.
    ///
    /// * `app` - The Vizia application builder.
    pub fn open_blocking<F>(
        win_desc: WindowDescription,
        scale_policy: WindowScalePolicy,
        app: F,
        on_idle: Option<Box<dyn Fn(&mut Context) + Send>>,
        ignore_default_theme: bool,
    ) where
        F: Fn(&mut Context),
        F: 'static + Send,
    {
        let window_settings = WindowOpenOptions {
            title: win_desc.title.clone(),
            size: baseview::Size::new(
                win_desc.inner_size.width as f64 * win_desc.user_scale_factor,
                win_desc.inner_size.height as f64 * win_desc.user_scale_factor,
            ),
            scale: scale_policy,
            gl_config: Some(GlConfig { vsync: false, ..GlConfig::default() }),
        };

        Window::open_blocking(
            window_settings,
            move |window: &mut baseview::Window<'_>| -> ViziaWindow {
                let mut cx = Context::new();

                cx.ignore_default_theme = ignore_default_theme;
                cx.remove_user_themes();

                let mut cx = BackendContext::new(cx);

                cx.set_event_proxy(Box::new(BaseviewProxy));
                ViziaWindow::new(cx, win_desc, scale_policy, window, Some(Box::new(app)), on_idle)
            },
        )
    }
}

impl WindowHandler for ViziaWindow {
    fn on_frame(&mut self, window: &mut Window) {
        self.application.on_frame_update(window);

        self.application.render(window);
    }

    fn on_event(&mut self, window: &mut Window<'_>, event: Event) -> EventStatus {
        let mut should_quit = false;

        self.application.handle_event(event, &mut should_quit);

        self.application.handle_idle(&self.on_idle);

        if should_quit {
            window.close();
        }

        EventStatus::Ignored
    }
}

pub struct WindowView {}

impl View for WindowView {}

pub fn create_surface(
    size: (i32, i32),
    fb_info: FramebufferInfo,
    gr_context: &mut skia_safe::gpu::DirectContext,
) -> Surface {
    let backend_render_target = backend_render_targets::make_gl(size, None, 8, fb_info);

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
