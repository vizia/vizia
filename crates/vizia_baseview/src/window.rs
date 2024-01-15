use crate::{application::ApplicationRunner, Renderer};
use baseview::gl::GlConfig;
use baseview::{
    Event, EventStatus, Window, WindowHandle, WindowHandler, WindowOpenOptions, WindowScalePolicy,
};
use raw_window_handle::HasRawWindowHandle;

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
        mut cx: Context,
        win_desc: WindowDescription,
        window_scale_policy: WindowScalePolicy,
        window: &mut baseview::Window,
        builder: Option<Box<dyn FnOnce(&mut Context) + Send>>,
        on_idle: Option<Box<dyn Fn(&mut Context) + Send>>,
    ) -> ViziaWindow {
        let context = window.gl_context().expect("Window was created without OpenGL support");
        let renderer = load_renderer(window);

        unsafe { context.make_current() };

        let canvas = Canvas::new(renderer).expect("Cannot create canvas");

        // Assume scale for now until there is an event with a new one.
        // Assume scale for now until there is an event with a new one.
        // Scaling is a combination of the window's scale factor (which is usually determined by the
        // operating system, or explicitly overridden by a hosting application) and a custom user
        // scale factor.
        let (use_system_scaling, window_scale_factor) = match window_scale_policy {
            WindowScalePolicy::ScaleFactor(scale) => (false, scale),
            // NOTE: This is not correct, but we should get a `Resized` event to correct this
            //       immediately after the window is created
            WindowScalePolicy::SystemScaleFactor => (true, 1.0),
        };
        let dpi_factor = window_scale_factor * win_desc.user_scale_factor;

        BackendContext::new(&mut cx).add_main_window(&win_desc, canvas, dpi_factor as f32);

        cx.remove_user_themes();
        if let Some(builder) = builder {
            (builder)(&mut cx);
        }

        let application = ApplicationRunner::new(cx, use_system_scaling, window_scale_factor);
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
        text_config: TextConfig,
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
                let mut context = Context::new(win_desc.inner_size, win_desc.user_scale_factor);

                context.ignore_default_theme = ignore_default_theme;
                context.remove_user_themes();

                let mut cx = BackendContext::new(&mut context);
                cx.set_text_config(text_config);

                cx.set_event_proxy(Box::new(BaseviewProxy()));
                ViziaWindow::new(
                    context,
                    win_desc,
                    scale_policy,
                    window,
                    Some(Box::new(app)),
                    on_idle,
                )
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
        text_config: TextConfig,
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
                let mut context = Context::new(win_desc.inner_size, win_desc.user_scale_factor);

                context.ignore_default_theme = ignore_default_theme;
                context.remove_user_themes();

                let mut cx = BackendContext::new(&mut context);
                cx.set_text_config(text_config);

                cx.set_event_proxy(Box::new(BaseviewProxy()));
                ViziaWindow::new(
                    context,
                    win_desc,
                    scale_policy,
                    window,
                    Some(Box::new(app)),
                    on_idle,
                )
            },
        )
    }
}

impl WindowHandler for ViziaWindow {
    fn on_frame(&mut self, window: &mut Window) {
        self.application.on_frame_update(window);

        let context = window.gl_context().expect("Window was created without OpenGL support");
        unsafe { context.make_current() };

        self.application.render();
        context.swap_buffers();

        unsafe { context.make_not_current() };
    }

    fn on_event(&mut self, _window: &mut Window<'_>, event: Event) -> EventStatus {
        let mut should_quit = false;
        self.application.handle_event(event, &mut should_quit);

        self.application.handle_idle(&self.on_idle);

        if should_quit {
            // TODO: Request close.
        }

        EventStatus::Ignored
    }
}

fn load_renderer(window: &Window) -> Renderer {
    let context = window.gl_context().expect("Window was created without OpenGL support");

    unsafe { context.make_current() };

    let renderer = unsafe {
        femtovg::renderer::OpenGl::new_from_function(|s| context.get_proc_address(s) as *const _)
            .expect("Cannot create renderer")
    };

    unsafe { context.make_not_current() };

    renderer
}
