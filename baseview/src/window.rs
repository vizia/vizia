use crate::{application::ApplicationRunner, Renderer};
use baseview::{Event, EventStatus, Window, WindowHandler, WindowOpenOptions, WindowScalePolicy};
use raw_window_handle::{HasRawWindowHandle, RawWindowHandle};
use tuix_core::{Entity, State, WindowDescription};

/// Handles an tuix_baseview application
pub(crate) struct TuixWindow {
    application: ApplicationRunner,
    context: raw_gl_context::GlContext,
    on_idle: Option<Box<dyn Fn(&mut State) + Send>>,
}

impl TuixWindow {
    fn new(state: State, win_desc: WindowDescription, window: &mut baseview::Window, on_idle: Option<Box<dyn Fn(&mut State) + Send>>) -> TuixWindow {
        let (renderer, context) = load_renderer(window);

        context.make_current();
        let application = ApplicationRunner::new(state, win_desc, renderer);
        context.make_not_current();

        TuixWindow {
            application,
            context,
            on_idle,
        }
    }

    /// Open a new child window.
    ///
    /// * `parent` - The parent window.
    /// * `app` - The Tuix application builder.
    pub fn open_parented<P, F>(parent: &P, win_desc: WindowDescription, mut app: F, on_idle: Option<Box<dyn Fn(&mut State) + Send>>)
    where
        P: HasRawWindowHandle,
        F: FnOnce(&mut State, Entity),
        F: 'static + Send,
    {
        let window_settings = WindowOpenOptions {
            title: win_desc.title.clone(),
            size: baseview::Size::new(
                win_desc.inner_size.width as f64,
                win_desc.inner_size.height as f64,
            ),
            scale: WindowScalePolicy::SystemScaleFactor,
        };

        Window::open_parented(
            parent,
            window_settings,
            move |window: &mut baseview::Window<'_>| -> TuixWindow {
                let mut state = State::new();

                let root = Entity::root();
                //state.tree.add(Entity::root(), None);

                (app)(&mut state, root);

                TuixWindow::new(state, win_desc, window, on_idle)
            },
        )
    }

    /// Open a new window as if it had a parent window.
    ///
    /// * `app` - The Tuix application builder.
    pub fn open_as_if_parented<F>(win_desc: WindowDescription, mut app: F, on_idle: Option<Box<dyn Fn(&mut State) + Send>>) -> RawWindowHandle
    where
        F: FnOnce(&mut State, Entity),
        F: 'static + Send,
    {
        let window_settings = WindowOpenOptions {
            title: win_desc.title.clone(),
            size: baseview::Size::new(
                win_desc.inner_size.width as f64,
                win_desc.inner_size.height as f64,
            ),
            scale: WindowScalePolicy::SystemScaleFactor,
        };

        Window::open_as_if_parented(
            window_settings,
            move |window: &mut baseview::Window<'_>| -> TuixWindow {
                let mut state = State::new();

                let root = Entity::root();
                //state.tree.add(Entity::root(), None);

                (app)(&mut state, root);

                TuixWindow::new(state, win_desc, window, on_idle)
            },
        )
    }

    /// Open a new window that blocks the current thread until the window is destroyed.
    ///
    /// * `app` - The Tuix application builder.
    pub fn open_blocking<F>(win_desc: WindowDescription, mut app: F, on_idle: Option<Box<dyn Fn(&mut State) + Send>>)
    where
        F: FnOnce(&mut State, Entity),
        F: 'static + Send,
    {
        let window_settings = WindowOpenOptions {
            title: win_desc.title.clone(),
            size: baseview::Size::new(
                win_desc.inner_size.width as f64,
                win_desc.inner_size.height as f64,
            ),
            scale: WindowScalePolicy::SystemScaleFactor,
        };

        Window::open_blocking(
            window_settings,
            move |window: &mut baseview::Window<'_>| -> TuixWindow {
                let mut state = State::new();

                let root = Entity::root();
                //state.tree.add(Entity::root(), None);

                let win_desc = WindowDescription::new();
                (app)(&mut state, root);

                TuixWindow::new(state, win_desc, window, on_idle)
            },
        )
    }
}

impl WindowHandler for TuixWindow {
    fn on_frame(&mut self, _window: &mut Window) {
        self.application.on_frame_update();

        self.context.make_current();

        self.application.render();
        self.context.swap_buffers();
        

        self.context.make_not_current();
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

fn load_renderer(window: &Window) -> (Renderer, raw_gl_context::GlContext) {
    let mut config = raw_gl_context::GlConfig::default();
    config.vsync = false;

    let context = raw_gl_context::GlContext::create(window, config).unwrap();

    context.make_current();

    let renderer = femtovg::renderer::OpenGl::new(|s| context.get_proc_address(s) as *const _)
        .expect("Cannot create renderer");

    context.make_not_current();

    (renderer, context)
}
