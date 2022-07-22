use crate::{application::ApplicationRunner, Renderer};
use baseview::gl::GlConfig;
use baseview::{
    Event, EventStatus, Window, WindowHandle, WindowHandler, WindowOpenOptions, WindowScalePolicy,
};
use raw_window_handle::HasRawWindowHandle;

use crate::proxy::BaseviewProxy;
use vizia_core::prelude::*;

static DEFAULT_THEME: &str = include_str!("../../core/resources/themes/default_theme.css");
static DEFAULT_LAYOUT: &str = include_str!("../../core/resources/themes/default_layout.css");

/// Handles a vizia_baseview application
pub(crate) struct ViziaWindow {
    application: ApplicationRunner,
    on_idle: Option<Box<dyn Fn(&mut Context) + Send>>,
}

impl ViziaWindow {
    fn new(
        mut cx: Context,
        win_desc: WindowDescription,
        scale_policy: WindowScalePolicy,
        window: &mut baseview::Window,
        builder: Option<Box<dyn FnOnce(&mut Context) + Send>>,
        on_idle: Option<Box<dyn Fn(&mut Context) + Send>>,
    ) -> ViziaWindow {
        if let Some(builder) = builder {
            (builder)(&mut cx);
        }

        let context = window.gl_context().expect("Window was created without OpenGL support");
        let renderer = load_renderer(window);

        unsafe { context.make_current() };
        let application = ApplicationRunner::new(cx, win_desc, scale_policy, renderer);
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
                win_desc.inner_size.width as f64,
                win_desc.inner_size.height as f64,
            ),
            scale: scale_policy,
            gl_config: Some(GlConfig { vsync: false, ..GlConfig::default() }),
        };

        Window::open_parented(
            parent,
            window_settings,
            move |window: &mut baseview::Window<'_>| -> ViziaWindow {
                let mut context = Context::new();

                context.ignore_default_theme = ignore_default_theme;

                context.add_theme(DEFAULT_LAYOUT);

                if !ignore_default_theme {
                    context.add_theme(DEFAULT_THEME);
                }

                context.set_event_proxy(Box::new(BaseviewProxy()));
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

    /// Open a new window as if it had a parent window.
    ///
    /// * `app` - The Vizia application builder.
    pub fn open_as_if_parented<F>(
        win_desc: WindowDescription,
        scale_policy: WindowScalePolicy,
        app: F,
        on_idle: Option<Box<dyn Fn(&mut Context) + Send>>,
        ignore_default_theme: bool,
    ) -> WindowHandle
    where
        F: Fn(&mut Context),
        F: 'static + Send,
    {
        let window_settings = WindowOpenOptions {
            title: win_desc.title.clone(),
            size: baseview::Size::new(
                win_desc.inner_size.width as f64,
                win_desc.inner_size.height as f64,
            ),
            scale: scale_policy,
            gl_config: Some(GlConfig { vsync: false, ..GlConfig::default() }),
        };

        Window::open_as_if_parented(
            window_settings,
            move |window: &mut baseview::Window<'_>| -> ViziaWindow {
                let mut context = Context::new();

                context.ignore_default_theme = ignore_default_theme;

                context.add_theme(DEFAULT_LAYOUT);

                if !ignore_default_theme {
                    context.add_theme(DEFAULT_THEME);
                }

                context.set_event_proxy(Box::new(BaseviewProxy()));
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
    ) where
        F: Fn(&mut Context),
        F: 'static + Send,
    {
        let window_settings = WindowOpenOptions {
            title: win_desc.title.clone(),
            size: baseview::Size::new(
                win_desc.inner_size.width as f64,
                win_desc.inner_size.height as f64,
            ),
            scale: scale_policy,
            gl_config: Some(GlConfig { vsync: false, ..GlConfig::default() }),
        };

        Window::open_blocking(
            window_settings,
            move |window: &mut baseview::Window<'_>| -> ViziaWindow {
                let mut context = Context::new();

                context.ignore_default_theme = ignore_default_theme;

                context.add_theme(DEFAULT_LAYOUT);

                if !ignore_default_theme {
                    context.add_theme(DEFAULT_THEME);
                }

                context.set_event_proxy(Box::new(BaseviewProxy()));
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
        let context = window.gl_context().expect("Window was created without OpenGL support");

        self.application.on_frame_update();

        unsafe { context.make_current() };

        self.application.render();
        context.swap_buffers();

        unsafe { context.make_not_current() };
    }

    fn on_event(&mut self, _window: &mut Window<'_>, event: Event) -> EventStatus {
        let mut should_quit = false;
        self.application.handle_event(event, &mut should_quit);

        //self.application.update_data();

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
