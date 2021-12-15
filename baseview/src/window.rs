use std::{cell::RefCell, collections::{HashMap, VecDeque}, rc::Rc};
use crate::{application::ApplicationRunner, Renderer};
use baseview::{Event, EventStatus, Window, WindowHandle, WindowHandler, WindowOpenOptions, WindowScalePolicy};
use raw_window_handle::{HasRawWindowHandle, RawWindowHandle};
use vizia_core::{AppData, BoundingBox, CachedData, Units, Color, Context, Data, Display, Entity, Enviroment, EventManager, FontOrId, IdManager, ModelData, Modifiers, MouseButton, MouseButtonState, MouseState, Propagation, ResourceManager, Style, Tree, TreeExt, Visibility, WindowDescription, WindowEvent};

static DEFAULT_THEME: &str = include_str!("../../core/src/default_theme.css");

/// Handles an vizia_baseview application
pub(crate) struct ViziaWindow {
    application: ApplicationRunner,
    context: raw_gl_context::GlContext,
    builder: Option<Box<dyn Fn(&mut Context) + Send>>,
    on_idle: Option<Box<dyn Fn(&mut Context) + Send>>,
}

impl ViziaWindow {
    fn new(cx: Context, win_desc: WindowDescription, window: &mut baseview::Window, builder: Option<Box<dyn Fn(&mut Context) + Send>>, on_idle: Option<Box<dyn Fn(&mut Context) + Send>>) -> ViziaWindow {
        let (renderer, context) = load_renderer(window);

        context.make_current();
        let application = ApplicationRunner::new(cx, win_desc, renderer);
        context.make_not_current();

        ViziaWindow {
            application,
            context,
            builder,
            on_idle,
        }
    }

    /// Open a new child window.
    ///
    /// * `parent` - The parent window.
    /// * `app` - The Tuix application builder.
    pub fn open_parented<P, F>(parent: &P, win_desc: WindowDescription, mut app: F, on_idle: Option<Box<dyn Fn(&mut Context) + Send>>) -> WindowHandle
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
            scale: WindowScalePolicy::ScaleFactor(1.0),
            //scale: WindowScalePolicy::SystemScaleFactor,
        };

        Window::open_parented(
            parent,
            window_settings,
            move |window: &mut baseview::Window<'_>| -> ViziaWindow {
                let mut cache = CachedData::default();
                cache.add(Entity::root()).expect("Failed to add entity to cache");
        
                let mut context = Context {
                    entity_manager: IdManager::new(),
                    tree: Tree::new(),
                    current: Entity::root(),
                    count: 0,
                    views: HashMap::new(),
                    lenses: HashMap::new(),
                    //state: HashMap::new(),  
                    data: AppData::new(),
                    style: Rc::new(RefCell::new(Style::default())),
                    cache,
                    enviroment: Enviroment::new(),
                    event_queue: VecDeque::new(),
                    mouse: MouseState::default(),
                    modifiers: Modifiers::empty(),
                    captured: Entity::null(),
                    hovered: Entity::root(),
                    focused: Entity::root(),
                    //state_count: 0,
                    resource_manager: ResourceManager::new(),
                    fonts: Vec::new(),
                    text_context: femtovg::TextContext::default(),
                };
        
                context.entity_manager.create();
        
                context.add_theme(DEFAULT_THEME);

                let root = Entity::root();
                //state.tree.add(Entity::root(), None);

                //(app)(&mut context);

                ViziaWindow::new(context, win_desc, window, Some(Box::new(app)), on_idle)
            },
        )
    }

    /// Open a new window as if it had a parent window.
    ///
    /// * `app` - The Tuix application builder.
    pub fn open_as_if_parented<F>(win_desc: WindowDescription, mut app: F, on_idle: Option<Box<dyn Fn(&mut Context) + Send>>) -> WindowHandle
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
            scale: WindowScalePolicy::ScaleFactor(1.0),
            //scale: WindowScalePolicy::SystemScaleFactor,
        };

        Window::open_as_if_parented(
            window_settings,
            move |window: &mut baseview::Window<'_>| -> ViziaWindow {
                let mut cache = CachedData::default();
                cache.add(Entity::root()).expect("Failed to add entity to cache");
        
                let mut context = Context {
                    entity_manager: IdManager::new(),
                    tree: Tree::new(),
                    current: Entity::root(),
                    count: 0,
                    views: HashMap::new(),
                    lenses: HashMap::new(),
                    //state: HashMap::new(),  
                    data: AppData::new(),
                    style: Rc::new(RefCell::new(Style::default())),
                    cache,
                    enviroment: Enviroment::new(),
                    event_queue: VecDeque::new(),
                    mouse: MouseState::default(),
                    modifiers: Modifiers::empty(),
                    captured: Entity::null(),
                    hovered: Entity::root(),
                    focused: Entity::root(),
                    //state_count: 0,
                    resource_manager: ResourceManager::new(),
                    fonts: Vec::new(),
                    text_context: femtovg::TextContext::default(),
                };
        
                context.entity_manager.create();
        
                context.add_theme(DEFAULT_THEME);

                let root = Entity::root();
                //state.tree.add(Entity::root(), None);

                //(app)(&mut context);

                ViziaWindow::new(context, win_desc, window, Some(Box::new(app)), on_idle)
            },
        )
    }

    /// Open a new window that blocks the current thread until the window is destroyed.
    ///
    /// * `app` - The Tuix application builder.
    pub fn open_blocking<F>(win_desc: WindowDescription, mut app: F, on_idle: Option<Box<dyn Fn(&mut Context) + Send>>)
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
            scale: WindowScalePolicy::ScaleFactor(1.0),
            //scale: WindowScalePolicy::SystemScaleFactor,
        };

        Window::open_blocking(
            window_settings,
            move |window: &mut baseview::Window<'_>| -> ViziaWindow {
                let mut cache = CachedData::default();
                cache.add(Entity::root()).expect("Failed to add entity to cache");
        
                let mut context = Context {
                    entity_manager: IdManager::new(),
                    tree: Tree::new(),
                    current: Entity::root(),
                    count: 0,
                    views: HashMap::new(),
                    lenses: HashMap::new(),
                    //state: HashMap::new(),  
                    data: AppData::new(),
                    style: Rc::new(RefCell::new(Style::default())),
                    cache,
                    enviroment: Enviroment::new(),
                    event_queue: VecDeque::new(),
                    mouse: MouseState::default(),
                    modifiers: Modifiers::empty(),
                    captured: Entity::null(),
                    hovered: Entity::root(),
                    focused: Entity::root(),
                    //state_count: 0,
                    resource_manager: ResourceManager::new(),
                    fonts: Vec::new(),
                    text_context: femtovg::TextContext::default(),
                };
        
                context.entity_manager.create();
        
                context.add_theme(DEFAULT_THEME);

                let root = Entity::root();
                //state.tree.add(Entity::root(), None);

                //let win_desc = WindowDescription::new();
                //(app)(&mut context);

                ViziaWindow::new(context, win_desc, window, Some(Box::new(app)), on_idle)
            },
        )
    }
}

impl WindowHandler for ViziaWindow {
    fn on_frame(&mut self, _window: &mut Window) {

        self.application.rebuild(&self.builder);

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
