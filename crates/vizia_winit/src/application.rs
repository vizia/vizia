#[cfg(target_os = "windows")]
use crate::window::set_cloak;
use crate::{
    convert::{winit_key_code_to_code, winit_key_to_key},
    window::{WinState, Window},
    window_modifiers::WindowModifiers,
};
#[cfg(feature = "accesskit")]
use accesskit_winit::Adapter;
use hashbrown::HashMap;
use std::{error::Error, fmt::Display, sync::Arc};

// #[cfg(feature = "accesskit")]
// use accesskit::{Action, NodeBuilder, NodeId, TreeUpdate};
// #[cfg(feature = "accesskit")]
// use accesskit_winit;
// use std::cell::RefCell;
use vizia_core::context::EventProxy;
use vizia_core::prelude::*;
use vizia_core::{backend::*, events::EventManager};
use winit::{
    application::ApplicationHandler,
    dpi::{LogicalPosition, LogicalSize, PhysicalPosition, PhysicalSize},
    error::EventLoopError,
    event::ElementState,
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop, EventLoopProxy},
    keyboard::{NativeKeyCode, PhysicalKey},
    window::{CursorIcon, CustomCursor, WindowAttributes, WindowId, WindowLevel},
};

// #[cfg(all(
//     feature = "clipboard",
//     feature = "wayland",
//     any(
//         target_os = "linux",
//         target_os = "dragonfly",
//         target_os = "freebsd",
//         target_os = "netbsd",
//         target_os = "openbsd"
//     )
// ))]
// use raw_window_handle::{HasRawDisplayHandle, RawDisplayHandle};
use vizia_window::{Anchor, AnchorTarget, WindowPosition};

#[derive(Debug)]
pub enum UserEvent {
    Event(Event),
    #[cfg(feature = "accesskit")]
    AccessKitEvent(accesskit_winit::Event),
}

#[cfg(feature = "accesskit")]
impl From<accesskit_winit::Event> for UserEvent {
    fn from(action_request_event: accesskit_winit::Event) -> Self {
        UserEvent::AccessKitEvent(action_request_event)
    }
}

impl From<vizia_core::events::Event> for UserEvent {
    fn from(event: vizia_core::events::Event) -> Self {
        UserEvent::Event(event)
    }
}

type IdleCallback = Option<Box<dyn Fn(&mut Context)>>;

#[derive(Debug)]
pub enum ApplicationError {
    EventLoopError(EventLoopError),
    LogError,
}

impl Display for ApplicationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ApplicationError::EventLoopError(ele) => write!(f, "{}", ele),
            ApplicationError::LogError => write!(f, "log error"),
        }
    }
}

impl std::error::Error for ApplicationError {}

///Creating a new application creates a root `Window` and a `Context`. Views declared within the closure passed to `Application::new()` are added to the context and rendered into the root window.
///
/// # Example
/// ```no_run
/// # use vizia_core::prelude::*;
/// # use vizia_winit::application::Application;
/// Application::new(|cx|{
///    // Content goes here
/// })
/// .run();
///```
/// Calling `run()` on the `Application` causes the program to enter the event loop and for the main window to display.
pub struct Application {
    cx: BackendContext,
    event_manager: EventManager,
    pub(crate) event_loop: Option<EventLoop<UserEvent>>,
    on_idle: IdleCallback,
    window_description: WindowDescription,
    control_flow: ControlFlow,
    event_loop_proxy: EventLoopProxy<UserEvent>,
    windows: HashMap<WindowId, WinState>,
    window_ids: HashMap<Entity, WindowId>,
    #[cfg(feature = "accesskit")]
    accesskit_adapter: Option<accesskit_winit::Adapter>,
    #[cfg(feature = "accesskit")]
    adapter_initialized: bool,
}

pub struct WinitEventProxy(EventLoopProxy<UserEvent>);

impl EventProxy for WinitEventProxy {
    fn send(&self, event: Event) -> Result<(), ()> {
        self.0.send_event(UserEvent::Event(event)).map_err(|_| ())
    }

    fn make_clone(&self) -> Box<dyn EventProxy> {
        Box::new(WinitEventProxy(self.0.clone()))
    }
}

impl Application {
    pub fn new<F>(content: F) -> Self
    where
        F: 'static + FnOnce(&mut Context),
    {
        let context = Context::new();

        let event_loop =
            EventLoop::<UserEvent>::with_user_event().build().expect("Failed to create event loop");

        let mut cx = BackendContext::new(context);
        let event_proxy_obj = event_loop.create_proxy();
        cx.set_event_proxy(Box::new(WinitEventProxy(event_proxy_obj)));

        cx.renegotiate_language();
        cx.0.remove_user_themes();
        (content)(cx.context());

        let proxy = event_loop.create_proxy();

        Self {
            cx,
            event_manager: EventManager::new(),
            event_loop: Some(event_loop),
            on_idle: None,
            window_description: WindowDescription::new(),
            control_flow: ControlFlow::Wait,
            event_loop_proxy: proxy,
            windows: HashMap::new(),
            window_ids: HashMap::new(),
            #[cfg(feature = "accesskit")]
            accesskit_adapter: None,
            #[cfg(feature = "accesskit")]
            adapter_initialized: false,
        }
    }

    fn create_window(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_entity: Entity,
        window_description: &WindowDescription,
        #[allow(unused_variables)] owner: Option<Arc<winit::window::Window>>,
    ) -> Result<Arc<winit::window::Window>, Box<dyn Error>> {
        #[allow(unused_mut)]
        let mut window_attributes = apply_window_description(window_description);

        let window_state = WinState::new(event_loop, window_entity, window_attributes, owner)?;
        let window = window_state.window.clone();

        if let Some(position) = window_description.position {
            window.set_outer_position(LogicalPosition::new(position.x, position.y));
        } else {
            let (anchor, mut parent_anchor) =
                match (window_description.anchor, window_description.parent_anchor) {
                    (Some(a), None) => (Some(a), Some(a)),
                    (None, Some(b)) => (Some(b.opposite()), Some(b)),
                    t => t,
                };

            if let Some(anchor) = anchor {
                let (y, x) = match anchor {
                    Anchor::TopLeft => (0.0, 0.0),
                    Anchor::TopCenter => (0.0, 0.5),
                    Anchor::TopRight => (0.0, 1.0),
                    Anchor::Left => (0.5, 0.0),
                    Anchor::Center => (0.5, 0.5),
                    Anchor::Right => (0.5, 1.0),
                    Anchor::BottomLeft => (1.0, 0.0),
                    Anchor::BottomCenter => (1.0, 0.5),
                    Anchor::BottomRight => (1.0, 1.0),
                };

                let window_size = window.inner_size();

                let anchor_target = window_description.anchor_target.unwrap_or_default();
                let parent = match anchor_target {
                    AnchorTarget::Monitor => window
                        .current_monitor()
                        .map(|monitor| (PhysicalPosition::default(), monitor.size())),
                    AnchorTarget::Window => self
                        .cx
                        .0
                        .tree
                        .get_parent_window(window_entity)
                        .and_then(|parent_window| self.window_ids.get(&parent_window))
                        .and_then(|id| self.windows.get(id))
                        .map(|win_state| {
                            (
                                win_state.window.outer_position().unwrap(),
                                win_state.window.inner_size(),
                            )
                        }),
                    AnchorTarget::Mouse => self
                        .cx
                        .0
                        .tree
                        .get_parent_window(window_entity)
                        .and_then(|parent_window| self.window_ids.get(&parent_window))
                        .and_then(|id| self.windows.get(id))
                        .map(|win_state| {
                            let pos = win_state.window.outer_position().unwrap();
                            (
                                PhysicalPosition::new(
                                    pos.x + self.cx.0.mouse.cursor_x as i32,
                                    pos.y + self.cx.0.mouse.cursor_y as i32,
                                ),
                                PhysicalSize::new(0, 0),
                            )
                        }),
                };

                if let Some((parent_position, parent_size)) = parent {
                    if anchor_target != AnchorTarget::Window {
                        parent_anchor = Some(anchor);
                    }

                    let (py, px) = match parent_anchor.unwrap_or_default() {
                        Anchor::TopLeft => (0.0, 0.0),
                        Anchor::TopCenter => (0.0, 0.5),
                        Anchor::TopRight => (0.0, 1.0),
                        Anchor::Left => (0.5, 0.0),
                        Anchor::Center => (0.5, 0.5),
                        Anchor::Right => (0.5, 1.0),
                        Anchor::BottomLeft => (1.0, 0.0),
                        Anchor::BottomCenter => (1.0, 0.5),
                        Anchor::BottomRight => (1.0, 1.0),
                    };

                    let x = (((parent_size.width as f32 * px) as i32
                        - (window_size.width as f32 * x) as i32)
                        as f32) as i32;
                    let y = (((parent_size.height as f32 * py) as i32
                        - (window_size.height as f32 * y) as i32)
                        as f32) as i32;

                    let offset = window_description.offset.unwrap_or_default();
                    let offset: PhysicalPosition<i32> = PhysicalPosition::from_logical(
                        LogicalPosition::new(offset.x, offset.y),
                        window.scale_factor(),
                    );

                    window.set_outer_position(PhysicalPosition::new(
                        parent_position.x + x as i32 + offset.x,
                        parent_position.y + y as i32 + offset.y,
                    ));
                }
            }
        }

        let window_id = window_state.window.id();
        self.windows.insert(window_id, window_state);
        self.window_ids.insert(window_entity, window_id);
        Ok(window)
    }

    /// Sets the default built-in theming to be ignored.
    pub fn ignore_default_theme(mut self) -> Self {
        self.cx.context().ignore_default_theme = true;
        self
    }

    pub fn should_poll(mut self) -> Self {
        self.control_flow = ControlFlow::Poll;

        self
    }

    /// Takes a closure which will be called at the end of every loop of the application.
    ///
    /// The callback provides a place to run 'idle' processing and happens at the end of each loop but before drawing.
    /// If the callback pushes events into the queue in state then the event loop will re-run. Care must be taken not to
    /// push events into the queue every time the callback runs unless this is intended.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use vizia_core::prelude::*;
    /// # use vizia_winit::application::Application;
    /// #
    /// Application::new(|cx| {
    ///     // Build application here
    /// })
    /// .on_idle(|cx| {
    ///     // Code here runs at the end of every event loop after OS and vizia events have been handled
    /// })
    /// .run();
    /// ```
    pub fn on_idle<F: 'static + Fn(&mut Context)>(mut self, callback: F) -> Self {
        self.on_idle = Some(Box::new(callback));

        self
    }

    /// Returns a `ContextProxy` which can be used to send events from another thread.
    pub fn get_proxy(&self) -> ContextProxy {
        self.cx.0.get_proxy()
    }

    pub fn run(mut self) -> Result<(), ApplicationError> {
        self.event_loop.take().unwrap().run_app(&mut self).map_err(ApplicationError::EventLoopError)
    }
}

impl ApplicationHandler<UserEvent> for Application {
    fn user_event(&mut self, _event_loop: &ActiveEventLoop, user_event: UserEvent) {
        match user_event {
            UserEvent::Event(event) => {
                self.cx.send_event(event);
            }

            #[cfg(feature = "accesskit")]
            UserEvent::AccessKitEvent(access_event) => {
                match access_event.window_event {
                    accesskit_winit::WindowEvent::InitialTreeRequested => {
                        let tree_update = self.cx.init_accessibility_tree();
                        if let Some(adapter) = &mut self.accesskit_adapter {
                            adapter.update_if_active(|| {
                                self.adapter_initialized = true;
                                tree_update
                            });
                        }
                    }
                    accesskit_winit::WindowEvent::ActionRequested(action_request) => {
                        let node_id = action_request.target;

                        if action_request.action != Action::ScrollIntoView {
                            let entity = Entity::new(node_id.0, 0);

                            // Handle focus action from screen reader
                            if action_request.action == Action::Focus {
                                self.cx.0.with_current(entity, |cx| {
                                    cx.focus();
                                });
                            }

                            self.cx.send_event(
                                Event::new(WindowEvent::ActionRequest(action_request))
                                    .direct(entity),
                            );
                        }
                    }
                    accesskit_winit::WindowEvent::AccessibilityDeactivated => todo!(),
                }
            }
        }
    }

    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.windows.is_empty() {
            // Create the main window
            let main_window: Arc<winit::window::Window> = self
                .create_window(event_loop, Entity::root(), &self.window_description.clone(), None)
                .expect("failed to create initial window");
            let custom_cursors = Arc::new(load_default_cursors(event_loop));
            self.cx.add_main_window(
                Entity::root(),
                &self.window_description,
                main_window.scale_factor() as f32,
            );
            self.cx.add_window(Window {
                window: Some(main_window.clone()),
                on_close: None,
                on_create: None,
                should_close: false,
                custom_cursors: custom_cursors.clone(),
            });

            self.cx.0.windows.insert(
                Entity::root(),
                WindowState {
                    window_description: self.window_description.clone(),
                    ..Default::default()
                },
            );

            #[cfg(feature = "accesskit")]
            {
                self.accesskit_adapter = Some(Adapter::with_event_loop_proxy(
                    &main_window,
                    self.event_loop_proxy.clone(),
                ));
            }

            // set current system theme if available
            if let Some(theme) = main_window.theme() {
                let theme = match theme {
                    winit::window::Theme::Light => ThemeMode::LightMode,
                    winit::window::Theme::Dark => ThemeMode::DarkMode,
                };
                self.cx.emit_origin(WindowEvent::ThemeChanged(theme));
            }

            self.cx.0.remove_user_themes();

            // Create any subwindows
            for (window_entity, window_state) in self.cx.0.windows.clone().into_iter() {
                if window_entity == Entity::root() {
                    continue;
                }
                let owner = window_state.owner.and_then(|entity| {
                    self.window_ids
                        .get(&entity)
                        .and_then(|id| self.windows.get(id).map(|ws| ws.window.clone()))
                });

                let window = self
                    .create_window(
                        event_loop,
                        window_entity,
                        &window_state.window_description,
                        owner,
                    )
                    .expect("Failed to create window");

                self.cx.add_main_window(
                    window_entity,
                    &window_state.window_description,
                    window.scale_factor() as f32,
                );

                self.cx.0.with_current(window_entity, |cx| {
                    if let Some(content) = &window_state.content {
                        (content)(cx)
                    }
                });
                self.cx.mutate_window(window_entity, |cx, win: &mut Window| {
                    win.window = Some(window.clone());
                    win.custom_cursors = custom_cursors.clone();
                    if let Some(callback) = &win.on_create {
                        (callback)(&mut EventContext::new_with_current(
                            cx.context(),
                            window_entity,
                        ));
                    }
                });
                self.cx.needs_refresh(window_entity);
            }
        }
    }

    fn window_event(
        &mut self,
        _event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: winit::event::WindowEvent,
    ) {
        let window = match self.windows.get_mut(&window_id) {
            Some(window) => window,
            None => return,
        };

        match event {
            winit::event::WindowEvent::Resized(size) => {
                window.resize(size);
                self.cx.set_window_size(window.entity, size.width as f32, size.height as f32);
                self.cx.needs_refresh(window.entity);
                window.window().request_redraw();

                #[cfg(target_os = "windows")]
                {
                    self.event_manager.flush_events(self.cx.context(), |_| {});

                    self.cx.process_style_updates();

                    if self.cx.process_animations() {
                        window.window().request_redraw();
                    }

                    self.cx.process_visual_updates();

                    // #[cfg(feature = "accesskit")]

                    // self.cx.process_tree_updates(|tree_updates| {
                    //     for update in tree_updates.iter_mut() {
                    //         self.accesskit_adapter
                    //             .unwrap()
                    //             .update_if_active(|| update.take().unwrap());
                    //     }
                    // });

                    // for update in self.cx.0.tree_updates.iter_mut() {
                    //     self.accesskit_adapter
                    //         .as_mut()
                    //         .unwrap()
                    //         .update_if_active(|| update.take().unwrap());
                    // }

                    // self.cx.0.tree_updates.clear();

                    window.window().request_redraw();
                }
            }

            winit::event::WindowEvent::Moved(position) => {
                let window_entity = window.entity;
                self.cx.emit_window_event(
                    window_entity,
                    WindowEvent::WindowMoved(WindowPosition { x: position.x, y: position.y }),
                );

                #[cfg(target_os = "windows")]
                {
                    self.event_manager.flush_events(self.cx.context(), |_| {});

                    self.cx.process_style_updates();

                    if self.cx.process_animations() {
                        window.window().request_redraw();
                    }

                    self.cx.process_visual_updates();

                    // #[cfg(feature = "accesskit")]

                    // self.cx.process_tree_updates(|tree_updates| {
                    //     for update in tree_updates.iter_mut() {
                    //         self.accesskit_adapter
                    //             .unwrap()
                    //             .update_if_active(|| update.take().unwrap());
                    //     }
                    // });

                    // for update in self.cx.0.tree_updates.iter_mut() {
                    //     self.accesskit_adapter
                    //         .as_mut()
                    //         .unwrap()
                    //         .update_if_active(|| update.take().unwrap());
                    // }

                    // self.cx.0.tree_updates.clear();

                    window.window().request_redraw();
                }
            }

            winit::event::WindowEvent::CloseRequested | winit::event::WindowEvent::Destroyed => {
                let window_entity = window.entity;
                self.cx.emit_window_event(window_entity, WindowEvent::WindowClose);
            }
            winit::event::WindowEvent::DroppedFile(path) => {
                self.cx.emit_window_event(window.entity, WindowEvent::Drop(DropData::File(path)));
            }

            winit::event::WindowEvent::HoveredFile(_) => {}
            winit::event::WindowEvent::HoveredFileCancelled => {}
            winit::event::WindowEvent::Focused(is_focused) => {
                self.cx.emit_window_event(window.entity, WindowEvent::WindowFocused(is_focused));

                self.cx.0.window_has_focus = is_focused;
                // #[cfg(feature = "accesskit")]
                // accesskit.update_if_active(|| TreeUpdate {
                //     nodes: vec![],
                //     tree: None,
                //     focus: is_focused.then_some(self.cx.focused().accesskit_id()).unwrap_or(NodeId(0)),
                // });
            }
            winit::event::WindowEvent::KeyboardInput { device_id: _, event, is_synthetic: _ } => {
                let code = match event.physical_key {
                    PhysicalKey::Code(code) => winit_key_code_to_code(code),
                    PhysicalKey::Unidentified(native) => match native {
                        NativeKeyCode::Windows(_scancode) => return,
                        _ => return,
                    },
                };

                let key = match event.logical_key {
                    winit::keyboard::Key::Named(named_key) => winit_key_to_key(named_key),
                    _ => None,
                };

                if let winit::keyboard::Key::Character(character) = event.logical_key {
                    if event.state == ElementState::Pressed {
                        self.cx.emit_window_event(
                            window.entity,
                            WindowEvent::CharInput(character.as_str().chars().next().unwrap()),
                        );
                    }
                }

                let event = match event.state {
                    winit::event::ElementState::Pressed => WindowEvent::KeyDown(code, key),
                    winit::event::ElementState::Released => WindowEvent::KeyUp(code, key),
                };

                self.cx.emit_window_event(window.entity, event);
                window.window().request_redraw();
            }
            winit::event::WindowEvent::ModifiersChanged(modifiers) => {
                self.cx.modifiers().set(Modifiers::SHIFT, modifiers.state().shift_key());

                self.cx.modifiers().set(Modifiers::ALT, modifiers.state().alt_key());

                self.cx.modifiers().set(Modifiers::CTRL, modifiers.state().control_key());

                self.cx.modifiers().set(Modifiers::SUPER, modifiers.state().super_key());

                window.window().request_redraw();
            }
            winit::event::WindowEvent::Ime(_) => {}
            winit::event::WindowEvent::CursorMoved { device_id: _, position } => {
                self.cx.emit_window_event(
                    window.entity,
                    WindowEvent::MouseMove(position.x as f32, position.y as f32),
                );
                window.window().request_redraw();
            }
            winit::event::WindowEvent::CursorEntered { device_id: _ } => {
                self.cx.emit_window_event(window.entity, WindowEvent::MouseEnter);
                window.window().request_redraw();
            }
            winit::event::WindowEvent::CursorLeft { device_id: _ } => {
                self.cx.emit_window_event(window.entity, WindowEvent::MouseLeave);
                window.window().request_redraw();
            }
            winit::event::WindowEvent::MouseWheel { device_id: _, delta, phase: _ } => {
                let out_event = match delta {
                    winit::event::MouseScrollDelta::LineDelta(x, y) => {
                        WindowEvent::MouseScroll(x, y)
                    }
                    winit::event::MouseScrollDelta::PixelDelta(pos) => {
                        WindowEvent::MouseScroll(
                            pos.x as f32 / 20.0,
                            pos.y as f32 / 20.0, // this number calibrated for wayland
                        )
                    }
                };

                self.cx.emit_window_event(window.entity, out_event);
                window.window().request_redraw();
            }
            winit::event::WindowEvent::MouseInput { device_id: _, state, button } => {
                let button = match button {
                    winit::event::MouseButton::Left => MouseButton::Left,
                    winit::event::MouseButton::Right => MouseButton::Right,
                    winit::event::MouseButton::Middle => MouseButton::Middle,
                    winit::event::MouseButton::Other(val) => MouseButton::Other(val),
                    winit::event::MouseButton::Back => MouseButton::Back,
                    winit::event::MouseButton::Forward => MouseButton::Forward,
                };

                let event = match state {
                    winit::event::ElementState::Pressed => WindowEvent::MouseDown(button),
                    winit::event::ElementState::Released => WindowEvent::MouseUp(button),
                };

                self.cx.emit_window_event(window.entity, event);
                window.window().request_redraw();
            }

            winit::event::WindowEvent::ScaleFactorChanged {
                scale_factor,
                inner_size_writer: _,
            } => {
                self.cx.set_scale_factor(scale_factor);
                self.cx.needs_refresh(window.entity);
            }
            winit::event::WindowEvent::ThemeChanged(theme) => {
                let theme = match theme {
                    winit::window::Theme::Light => ThemeMode::LightMode,
                    winit::window::Theme::Dark => ThemeMode::DarkMode,
                };
                self.cx.emit_window_event(window.entity, WindowEvent::ThemeChanged(theme));
            }
            winit::event::WindowEvent::Occluded(_) => {}
            winit::event::WindowEvent::RedrawRequested => {
                for window in self.windows.values_mut() {
                    window.make_current();
                    //self.cx.needs_refresh(window.entity);
                    if self.cx.draw(window.entity, &mut window.surface, &mut window.dirty_surface) {
                        window.swap_buffers();
                    }

                    // Un-cloak
                    #[cfg(target_os = "windows")]
                    if window.is_initially_cloaked {
                        window.is_initially_cloaked = false;
                        set_cloak(window.window(), false);
                    }
                }
            }

            _ => {}
        }
    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        if self.windows.is_empty() {
            event_loop.exit();
            return;
        }

        event_loop.set_control_flow(self.control_flow);

        self.event_manager.flush_events(self.cx.context(), |_| {});

        self.cx.process_style_updates();

        if self.cx.process_animations() {
            for window in self.windows.values() {
                window.window().request_redraw();
            }
        }

        self.cx.process_visual_updates();

        #[cfg(feature = "accesskit")]
        {
            self.cx.process_tree_updates();

            if self.adapter_initialized {
                for update in self.cx.0.tree_updates.iter_mut() {
                    self.accesskit_adapter
                        .as_mut()
                        .unwrap()
                        .update_if_active(|| update.take().unwrap());
                }
            }

            self.cx.0.tree_updates.clear();
        }

        if let Some(idle_callback) = &self.on_idle {
            self.cx.set_current(Entity::root());
            (idle_callback)(self.cx.context());
        }

        if self.cx.has_queued_events() {
            self.event_loop_proxy
                .send_event(UserEvent::Event(Event::new(())))
                .expect("Failed to send event");
        }

        if self.cx.0.windows.iter().any(|(_, window_state)| !window_state.redraw_list.is_empty()) {
            for window in self.windows.values() {
                window.window().request_redraw();
            }
        }

        if self.control_flow != ControlFlow::Poll {
            if let Some(timer_time) = self.cx.get_next_timer_time() {
                event_loop.set_control_flow(ControlFlow::WaitUntil(timer_time));
            } else {
                event_loop.set_control_flow(ControlFlow::Wait);
            }
        }

        let window_entities = self
            .cx
            .0
            .windows
            .iter()
            .filter_map(|(entity, state)| state.should_close.then_some(*entity))
            .collect::<Vec<_>>();

        for window_entity in window_entities {
            self.cx.0.remove(window_entity);
        }

        // Sync window state with context
        self.windows.retain(|_, win| self.cx.0.windows.contains_key(&win.entity));
        self.window_ids.retain(|e, _| self.cx.0.windows.contains_key(e));

        if self.windows.len() != self.cx.0.windows.len() {
            for (window_entity, window_state) in self.cx.0.windows.clone().iter() {
                if !self.window_ids.contains_key(window_entity) {
                    let owner = window_state.owner.and_then(|entity| {
                        self.window_ids
                            .get(&entity)
                            .and_then(|id| self.windows.get(id).map(|ws| ws.window.clone()))
                    });

                    let window = self
                        .create_window(
                            event_loop,
                            *window_entity,
                            &window_state.window_description,
                            owner,
                        )
                        .expect("Failed to create window");

                    self.cx.add_main_window(
                        *window_entity,
                        &window_state.window_description,
                        window.scale_factor() as f32,
                    );

                    self.cx.0.with_current(*window_entity, |cx| {
                        if let Some(content) = &window_state.content {
                            (content)(cx)
                        }
                    });

                    self.cx.mutate_window(*window_entity, |cx, win: &mut Window| {
                        win.window = Some(window.clone());
                        if let Some(callback) = &win.on_create {
                            (callback)(&mut EventContext::new_with_current(
                                cx.context(),
                                *window_entity,
                            ));
                        }
                    });
                }
            }
        }

        if self.windows.is_empty() {
            event_loop.exit();
        }
    }

    fn new_events(&mut self, _event_loop: &ActiveEventLoop, _cause: winit::event::StartCause) {
        self.cx.process_timers();
        self.cx.emit_scheduled_events();
    }

    fn exiting(&mut self, _event_loop: &ActiveEventLoop) {}
}

impl WindowModifiers for Application {
    fn title<T: ToString>(mut self, title: impl Res<T>) -> Self {
        self.window_description.title = title.get(&self.cx.0).to_string();

        title.set_or_bind(&mut self.cx.0, Entity::root(), |cx, title| {
            cx.emit(WindowEvent::SetTitle(title.get(cx).to_string()));
        });

        self
    }

    fn inner_size<S: Into<WindowSize>>(mut self, size: impl Res<S>) -> Self {
        self.window_description.inner_size = size.get(&self.cx.0).into();

        size.set_or_bind(&mut self.cx.0, Entity::root(), |cx, size| {
            cx.emit(WindowEvent::SetSize(size.get(cx).into()));
        });

        self
    }

    fn min_inner_size<S: Into<WindowSize>>(mut self, size: impl Res<Option<S>>) -> Self {
        self.window_description.min_inner_size = size.get(&self.cx.0).map(|s| s.into());

        size.set_or_bind(&mut self.cx.0, Entity::root(), |cx, size| {
            cx.emit(WindowEvent::SetMinSize(size.get(cx).map(|s| s.into())));
        });

        self
    }

    fn max_inner_size<S: Into<WindowSize>>(mut self, size: impl Res<Option<S>>) -> Self {
        self.window_description.max_inner_size = size.get(&self.cx.0).map(|s| s.into());

        size.set_or_bind(&mut self.cx.0, Entity::root(), |cx, size| {
            cx.emit(WindowEvent::SetMaxSize(size.get(cx).map(|s| s.into())));
        });
        self
    }

    fn position<P: Into<WindowPosition>>(mut self, position: impl Res<P>) -> Self {
        self.window_description.position = Some(position.get(&self.cx.0).into());

        position.set_or_bind(&mut self.cx.0, Entity::root(), |cx, size| {
            cx.emit(WindowEvent::SetPosition(size.get(cx).into()));
        });

        self
    }

    fn offset<P: Into<WindowPosition>>(mut self, offset: impl Res<P>) -> Self {
        self.window_description.offset = Some(offset.get(&self.cx.0).into());

        self
    }

    fn anchor<P: Into<Anchor>>(mut self, anchor: impl Res<P>) -> Self {
        self.window_description.anchor = Some(anchor.get(&self.cx.0).into());

        self
    }

    fn anchor_target<P: Into<AnchorTarget>>(mut self, anchor_target: impl Res<P>) -> Self {
        self.window_description.anchor_target = Some(anchor_target.get(&self.cx.0).into());

        self
    }

    fn parent_anchor<P: Into<Anchor>>(mut self, parent_anchor: impl Res<P>) -> Self {
        self.window_description.parent_anchor = Some(parent_anchor.get(&self.cx.0).into());

        self
    }

    fn resizable(mut self, flag: impl Res<bool>) -> Self {
        self.window_description.resizable = flag.get(&self.cx.0);

        flag.set_or_bind(&mut self.cx.0, Entity::root(), |cx, flag| {
            cx.emit(WindowEvent::SetResizable(flag.get(cx)));
        });

        self
    }

    fn minimized(mut self, flag: impl Res<bool>) -> Self {
        self.window_description.minimized = flag.get(&self.cx.0);

        flag.set_or_bind(&mut self.cx.0, Entity::root(), |cx, flag| {
            cx.emit(WindowEvent::SetMinimized(flag.get(cx)));
        });
        self
    }

    fn maximized(mut self, flag: impl Res<bool>) -> Self {
        self.window_description.maximized = flag.get(&self.cx.0);

        flag.set_or_bind(&mut self.cx.0, Entity::root(), |cx, flag| {
            cx.emit(WindowEvent::SetMaximized(flag.get(cx)));
        });

        self
    }

    fn visible(mut self, flag: impl Res<bool>) -> Self {
        self.window_description.visible = flag.get(&self.cx.0);

        flag.set_or_bind(&mut self.cx.0, Entity::root(), |cx, flag| {
            cx.emit(WindowEvent::SetVisible(flag.get(cx)));
        });

        self
    }

    fn transparent(mut self, flag: bool) -> Self {
        self.window_description.transparent = flag;

        self
    }

    fn decorations(mut self, flag: bool) -> Self {
        self.window_description.decorations = flag;

        self
    }

    fn always_on_top(mut self, flag: bool) -> Self {
        self.window_description.always_on_top = flag;
        self
    }

    fn vsync(mut self, flag: bool) -> Self {
        self.window_description.vsync = flag;

        self
    }

    fn icon(mut self, width: u32, height: u32, image: Vec<u8>) -> Self {
        self.window_description.icon = Some(image);
        self.window_description.icon_width = width;
        self.window_description.icon_height = height;

        self
    }

    fn on_close(self, _callback: impl Fn(&mut EventContext)) -> Self {
        self
    }

    fn on_create(self, _callback: impl Fn(&mut EventContext)) -> Self {
        self
    }

    fn enabled_window_buttons(mut self, window_buttons: WindowButtons) -> Self {
        self.window_description.enabled_window_buttons = window_buttons;

        self
    }
}

fn apply_window_description(description: &WindowDescription) -> WindowAttributes {
    let mut window_attributes = winit::window::Window::default_attributes();

    window_attributes = window_attributes.with_title(&description.title).with_inner_size(
        LogicalSize::new(description.inner_size.width, description.inner_size.height),
    );

    if let Some(min_inner_size) = description.min_inner_size {
        window_attributes = window_attributes
            .with_min_inner_size(LogicalSize::new(min_inner_size.width, min_inner_size.height));
    }

    if let Some(max_inner_size) = description.max_inner_size {
        window_attributes = window_attributes
            .with_max_inner_size(LogicalSize::new(max_inner_size.width, max_inner_size.height));
    }

    if let Some(position) = description.position {
        window_attributes =
            window_attributes.with_position(LogicalPosition::new(position.x, position.y));
    }

    window_attributes
        .with_resizable(description.resizable)
        .with_maximized(description.maximized)
        // Accesskit requires that the window start invisible until accesskit is initialized.
        //.with_visible(false)
        .with_window_level(if description.always_on_top {
            WindowLevel::AlwaysOnTop
        } else {
            WindowLevel::Normal
        })
        .with_transparent(description.transparent)
        .with_decorations(description.decorations)
        .with_window_icon(description.icon.as_ref().map(|icon| {
            winit::window::Icon::from_rgba(
                icon.clone(),
                description.icon_width,
                description.icon_height,
            )
            .unwrap()
        }))
        .with_enabled_buttons(
            winit::window::WindowButtons::from_bits(description.enabled_window_buttons.bits())
                .unwrap(),
        )
}

#[allow(unused_variables)]
pub fn load_default_cursors(event_loop: &ActiveEventLoop) -> HashMap<CursorIcon, CustomCursor> {
    #[allow(unused_mut)]
    let mut custom_cursors = HashMap::new();

    #[cfg(target_os = "windows")]
    {
        let mut load_cursor = |cursor, bytes, x, y| {
            custom_cursors.insert(
                cursor,
                event_loop.create_custom_cursor(
                    CustomCursor::from_rgba(bytes, 32, 32, x, y)
                        .expect("Failed to create custom cursor"),
                ),
            );
        };

        load_cursor(
            CursorIcon::Alias, //
            include_bytes!("../resources/cursors/windows/aliasb"),
            0,
            0,
        );
        load_cursor(
            CursorIcon::Cell, //
            include_bytes!("../resources/cursors/windows/cell"),
            7,
            7,
        );
        load_cursor(
            CursorIcon::ColResize,
            include_bytes!("../resources/cursors/windows/col_resize"),
            10,
            8,
        );
        load_cursor(
            CursorIcon::Copy, //
            include_bytes!("../resources/cursors/windows/copy"),
            0,
            0,
        );
        load_cursor(
            CursorIcon::Grab, //
            include_bytes!("../resources/cursors/windows/grab"),
            6,
            0,
        );
        load_cursor(
            CursorIcon::Grabbing, //
            include_bytes!("../resources/cursors/windows/grabbing"),
            6,
            0,
        );
        load_cursor(
            CursorIcon::RowResize, //
            include_bytes!("../resources/cursors/windows/row_resize"),
            9,
            10,
        );
        load_cursor(
            CursorIcon::VerticalText, //
            include_bytes!("../resources/cursors/windows/vertical_text"),
            9,
            3,
        );
        load_cursor(
            CursorIcon::ZoomIn, //
            include_bytes!("../resources/cursors/windows/zoom_in"),
            6,
            6,
        );
        load_cursor(
            CursorIcon::ZoomOut, //
            include_bytes!("../resources/cursors/windows/zoom_out"),
            6,
            6,
        );
    }

    custom_cursors
}
