use crate::{
    convert::{scan_code_to_code, virtual_key_code_to_code, virtual_key_code_to_key},
    window::Window,
};
#[cfg(not(target_arch = "wasm32"))]
use accesskit::{Action, NodeBuilder, TreeUpdate};
#[cfg(not(target_arch = "wasm32"))]
use accesskit_winit;
use std::cell::RefCell;
use vizia_core::backend::*;
#[cfg(not(target_arch = "wasm32"))]
use vizia_core::context::EventProxy;
use vizia_core::prelude::*;
use vizia_id::GenerationalId;
use vizia_window::Position;
use winit::event_loop::EventLoopBuilder;
#[cfg(all(
    feature = "clipboard",
    feature = "wayland",
    any(
        target_os = "linux",
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "netbsd",
        target_os = "openbsd"
    )
))]
use winit::platform::wayland::WindowExtWayland;
use winit::{
    event::VirtualKeyCode,
    event_loop::{ControlFlow, EventLoop},
};

#[cfg(not(target_arch = "wasm32"))]
use winit::event_loop::EventLoopProxy;

#[derive(Debug)]
pub enum UserEvent {
    Event(Event),
    #[cfg(not(target_arch = "wasm32"))]
    AccessKitActionRequest(accesskit_winit::ActionRequestEvent),
}

#[cfg(not(target_arch = "wasm32"))]
impl From<accesskit_winit::ActionRequestEvent> for UserEvent {
    fn from(action_request_event: accesskit_winit::ActionRequestEvent) -> Self {
        UserEvent::AccessKitActionRequest(action_request_event)
    }
}

impl From<vizia_core::events::Event> for UserEvent {
    fn from(event: vizia_core::events::Event) -> Self {
        UserEvent::Event(event)
    }
}

type AppBuilder = Option<Box<dyn FnOnce(&mut Context)>>;
type IdleCallback = Option<Box<dyn Fn(&mut Context)>>;

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
    context: Context,
    event_loop: EventLoop<UserEvent>,
    builder: AppBuilder,
    on_idle: IdleCallback,
    window_description: WindowDescription,
    should_poll: bool,
}

#[cfg(not(target_arch = "wasm32"))]
pub struct WinitEventProxy(EventLoopProxy<UserEvent>);

#[cfg(not(target_arch = "wasm32"))]
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
        // wasm + debug: send panics to console
        #[cfg(all(debug_assertions, target_arch = "wasm32"))]
        console_error_panic_hook::set_once();

        // TODO: User scale factors and window resizing has not been implement for winit
        // TODO: Changing the scale factor doesn't work for winit anyways since winit doesn't let
        //       you resize the window, so there's no mutator for that at he moment
        #[allow(unused_mut)]
        let mut context = Context::new(WindowSize::new(1, 1), 1.0);

        let event_loop = EventLoopBuilder::with_user_event().build();
        #[cfg(not(target_arch = "wasm32"))]
        {
            let mut cx = BackendContext::new(&mut context);
            let event_proxy_obj = event_loop.create_proxy();
            cx.set_event_proxy(Box::new(WinitEventProxy(event_proxy_obj)));
        }

        Self {
            context,
            event_loop,
            builder: Some(Box::new(content)),
            on_idle: None,
            window_description: WindowDescription::new(),
            should_poll: false,
        }
    }

    /// Sets the default built-in theming to be ignored.
    pub fn ignore_default_theme(mut self) -> Self {
        self.context.ignore_default_theme = true;
        self
    }

    pub fn set_text_config(mut self, text_config: TextConfig) -> Self {
        BackendContext::new(&mut self.context).set_text_config(text_config);
        self
    }

    pub fn should_poll(mut self) -> Self {
        self.should_poll = true;

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
        self.context.get_proxy()
    }

    /// Starts the application and enters the main event loop.
    pub fn run(mut self) {
        let mut context = self.context;

        let event_loop = self.event_loop;

        let (window, canvas) = Window::new(&event_loop, &self.window_description);

        // On windows cloak (hide) the window initially, we later reveal it after the first draw.
        // This is a workaround to hide the "white flash" that occurs during application startup.
        #[cfg(target_os = "windows")]
        let mut is_initially_cloaked = window.set_cloak(true);

        #[cfg(not(target_arch = "wasm32"))]
        let event_loop_proxy = event_loop.create_proxy();

        let mut cx = BackendContext::new(&mut context);

        // update the sys theme if any
        if let Some(theme) = window.window().theme() {
            let theme = match theme {
                winit::window::Theme::Light => ThemeMode::LightMode,
                winit::window::Theme::Dark => ThemeMode::DarkMode,
            };
            cx.emit_origin(WindowEvent::ThemeChanged(theme));
        }

        #[cfg(not(target_arch = "wasm32"))]
        let root_node = NodeBuilder::new(Role::Window).build(cx.accesskit_node_classes());
        #[cfg(not(target_arch = "wasm32"))]
        let accesskit = accesskit_winit::Adapter::new(
            window.window(),
            move || {
                // TODO: set a flag to signify that a screen reader has been attached
                use accesskit::Tree;

                let root_id = Entity::root().accesskit_id();

                TreeUpdate {
                    nodes: vec![(root_id, root_node)],
                    tree: Some(Tree::new(root_id)),
                    focus: Some(Entity::root().accesskit_id()),
                }
            },
            event_loop_proxy,
        );

        // Accesskit requires that the window starts invisible until accesskit has been initialised.
        // At this point we can set the visibility based on the desired visibility from the window description.
        window.window().set_visible(self.window_description.visible);

        #[cfg(all(
            feature = "clipboard",
            feature = "wayland",
            any(
                target_os = "linux",
                target_os = "dragonfly",
                target_os = "freebsd",
                target_os = "netbsd",
                target_os = "openbsd"
            )
        ))]
        unsafe {
            if let Some(display) = window.window().wayland_display() {
                let (_, clipboard) =
                    copypasta::wayland_clipboard::create_clipboards_from_external(display);
                cx.set_clipboard_provider(Box::new(clipboard));
            }
        }

        let scale_factor = window.window().scale_factor() as f32;
        cx.add_main_window(&self.window_description, canvas, scale_factor);
        cx.add_window(window);

        cx.0.remove_user_themes();
        cx.renegotiate_language();
        if let Some(builder) = self.builder.take() {
            (builder)(cx.0);
        }

        let on_idle = self.on_idle.take();

        let event_loop_proxy = event_loop.create_proxy();

        let default_should_poll = self.should_poll;
        let stored_control_flow = RefCell::new(ControlFlow::Poll);

        #[cfg(not(target_arch = "wasm32"))]
        cx.process_tree_updates(|tree_updates| {
            for update in tree_updates.iter() {
                accesskit.update(update.clone());
            }
        });

        let mut cursor_moved = false;
        let mut cursor = (0.0f32, 0.0f32);
        #[cfg(target_os = "windows")]
        let mut inside_window = false;

        // cx.process_events();

        cx.process_data_updates();
        cx.process_style_updates();
        cx.process_visual_updates();

        let mut main_events = false;
        event_loop.run(move |event, _, control_flow| {
            let mut cx = BackendContext::new_with_event_manager(&mut context);

            match event {
                winit::event::Event::NewEvents(_) => {
                    cx.process_timers();
                    cx.emit_scheduled_events();
                }

                winit::event::Event::UserEvent(user_event) => match user_event {
                    UserEvent::Event(event) => {
                        cx.send_event(event);
                    }

                    #[cfg(not(target_arch = "wasm32"))]
                    UserEvent::AccessKitActionRequest(action_request_event) => {
                        let node_id = action_request_event.request.target;

                        if action_request_event.request.action != Action::ScrollIntoView {
                            let entity = Entity::new(node_id.0.get() as u64 - 1, 0);

                            // Handle focus action from screen reader
                            if action_request_event.request.action == Action::Focus {
                                cx.0.with_current(entity, |cx| {
                                    cx.focus();
                                });
                            }

                            cx.send_event(
                                Event::new(WindowEvent::ActionRequest(
                                    action_request_event.request,
                                ))
                                .direct(entity),
                            );
                        }
                    }
                },

                winit::event::Event::MainEventsCleared => {
                    main_events = true;

                    *stored_control_flow.borrow_mut() =
                        if default_should_poll { ControlFlow::Poll } else { ControlFlow::Wait };

                    if cursor_moved {
                        cx.emit_origin(WindowEvent::MouseMove(cursor.0, cursor.1));
                        cursor_moved = false;
                    }

                    cx.process_events();

                    cx.process_data_updates();

                    cx.process_style_updates();

                    if cx.process_animations() {
                        *stored_control_flow.borrow_mut() = ControlFlow::Poll;

                        event_loop_proxy
                            .send_event(UserEvent::Event(Event::new(WindowEvent::Redraw)))
                            .expect("Failed to send redraw event");

                        cx.mutate_window(|_, window: &Window| {
                            window.window().request_redraw();
                        });
                    }

                    cx.process_visual_updates();

                    #[cfg(not(target_arch = "wasm32"))]
                    cx.process_tree_updates(|tree_updates| {
                        for update in tree_updates.iter() {
                            accesskit.update(update.clone());
                        }
                    });

                    cx.mutate_window(|cx, window: &Window| {
                        cx.style().should_redraw(|| {
                            window.window().request_redraw();
                        });
                    });

                    if let Some(idle_callback) = &on_idle {
                        cx.set_current(Entity::root());
                        (idle_callback)(cx.context());
                    }

                    if cx.has_queued_events() {
                        *stored_control_flow.borrow_mut() = ControlFlow::Poll;
                        event_loop_proxy
                            .send_event(UserEvent::Event(Event::new(())))
                            .expect("Failed to send event");
                    }

                    cx.mutate_window(|_, window: &Window| {
                        if window.should_close {
                            *stored_control_flow.borrow_mut() = ControlFlow::Exit;
                        }
                    });
                }

                winit::event::Event::RedrawRequested(_) => {
                    if main_events {
                        // Redraw
                        cx.draw();
                        cx.mutate_window(|_, window: &Window| {
                            window.swap_buffers();
                        });

                        // Un-cloak
                        #[cfg(target_os = "windows")]
                        if is_initially_cloaked {
                            is_initially_cloaked = false;
                            cx.draw();
                            cx.mutate_window(|_, window: &Window| {
                                window.swap_buffers();
                                window.set_cloak(false);
                            });
                        }
                    }
                }

                winit::event::Event::WindowEvent { window_id: _, event } => {
                    match event {
                        winit::event::WindowEvent::CloseRequested => {
                            cx.emit_origin(WindowEvent::WindowClose);
                        }

                        winit::event::WindowEvent::Focused(is_focused) => {
                            cx.0.window_has_focus = is_focused;
                            #[cfg(not(target_arch = "wasm32"))]
                            accesskit.update_if_active(|| TreeUpdate {
                                nodes: vec![],
                                tree: None,
                                focus: is_focused.then_some(cx.focused().accesskit_id()),
                            });
                        }

                        winit::event::WindowEvent::ScaleFactorChanged {
                            scale_factor,
                            new_inner_size,
                        } => {
                            cx.set_scale_factor(scale_factor);
                            cx.set_window_size(
                                new_inner_size.width as f32,
                                new_inner_size.height as f32,
                            );
                            cx.needs_refresh();
                        }

                        winit::event::WindowEvent::DroppedFile(path) => {
                            cx.emit_origin(WindowEvent::Drop(DropData::File(path)));
                        }

                        #[allow(deprecated)]
                        winit::event::WindowEvent::CursorMoved {
                            device_id: _,
                            position,
                            modifiers: _,
                        } => {
                            // To avoid calling the hover system multiple times in one frame when multiple cursor moved
                            // events are received, instead we set a flag here and emit the MouseMove event during MainEventsCleared.
                            if !cursor_moved {
                                cursor_moved = true;
                                cursor.0 = position.x as f32;
                                cursor.1 = position.y as f32;
                            }

                            // Temporary fix for windows platform until winit merge #3154
                            #[cfg(target_os = "windows")]
                            {
                                let (width, height) = {
                                    let scale_factor = cx.scale_factor();
                                    let size = cx.window_size();
                                    (
                                        (size.width as f32 * scale_factor).round() as u32,
                                        (size.height as f32 * scale_factor).round() as u32,
                                    )
                                };

                                let x = position.x.is_positive()
                                    && (0..width).contains(&(position.x as u32));
                                let y = position.y.is_positive()
                                    && (0..height).contains(&(position.y as u32));

                                if !inside_window && x && y {
                                    inside_window = true;
                                    cx.emit_origin(WindowEvent::MouseEnter);
                                } else if inside_window && !(x && y) {
                                    inside_window = false;
                                    cx.emit_origin(WindowEvent::MouseLeave);
                                }
                            }
                        }

                        #[allow(deprecated)]
                        winit::event::WindowEvent::MouseInput {
                            device_id: _,
                            button,
                            state,
                            modifiers: _,
                        } => {
                            let button = match button {
                                winit::event::MouseButton::Left => MouseButton::Left,
                                winit::event::MouseButton::Right => MouseButton::Right,
                                winit::event::MouseButton::Middle => MouseButton::Middle,
                                winit::event::MouseButton::Other(val) => MouseButton::Other(val),
                            };

                            let event = match state {
                                winit::event::ElementState::Pressed => {
                                    WindowEvent::MouseDown(button)
                                }
                                winit::event::ElementState::Released => {
                                    WindowEvent::MouseUp(button)
                                }
                            };

                            cx.emit_origin(event);
                        }

                        winit::event::WindowEvent::MouseWheel { delta, phase: _, .. } => {
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

                            cx.emit_origin(out_event);
                        }

                        winit::event::WindowEvent::KeyboardInput {
                            device_id: _,
                            input,
                            is_synthetic: _,
                        } => {
                            // Prefer virtual keycodes to scancodes, as scancodes aren't uniform between platforms
                            let code = if let Some(vkey) = input.virtual_keycode {
                                virtual_key_code_to_code(vkey)
                            } else {
                                scan_code_to_code(input.scancode)
                            };

                            let key = virtual_key_code_to_key(
                                input.virtual_keycode.unwrap_or(VirtualKeyCode::NoConvert),
                            );

                            let event = match input.state {
                                winit::event::ElementState::Pressed => {
                                    WindowEvent::KeyDown(code, key)
                                }
                                winit::event::ElementState::Released => {
                                    WindowEvent::KeyUp(code, key)
                                }
                            };

                            cx.emit_origin(event);
                        }

                        winit::event::WindowEvent::ReceivedCharacter(character) => {
                            cx.emit_origin(WindowEvent::CharInput(character));
                        }

                        winit::event::WindowEvent::Resized(physical_size) => {
                            cx.mutate_window(|_, window: &Window| {
                                window.resize(physical_size);
                            });

                            cx.set_window_size(
                                physical_size.width as f32,
                                physical_size.height as f32,
                            );

                            cx.needs_refresh();
                        }

                        winit::event::WindowEvent::ThemeChanged(theme) => {
                            let theme = match theme {
                                winit::window::Theme::Light => ThemeMode::LightMode,
                                winit::window::Theme::Dark => ThemeMode::DarkMode,
                            };
                            cx.emit_origin(WindowEvent::ThemeChanged(theme));
                        }

                        winit::event::WindowEvent::ModifiersChanged(modifiers_state) => {
                            cx.modifiers().set(Modifiers::SHIFT, modifiers_state.shift());
                            cx.modifiers().set(Modifiers::ALT, modifiers_state.alt());
                            cx.modifiers().set(Modifiers::CTRL, modifiers_state.ctrl());
                            cx.modifiers().set(Modifiers::LOGO, modifiers_state.logo());
                        }

                        winit::event::WindowEvent::CursorEntered { device_id: _ } => {
                            #[cfg(target_os = "windows")]
                            {
                                inside_window = true;
                            }
                            cx.emit_origin(WindowEvent::MouseEnter);
                        }

                        winit::event::WindowEvent::CursorLeft { device_id: _ } => {
                            #[cfg(target_os = "windows")]
                            {
                                inside_window = false;
                            }
                            cx.emit_origin(WindowEvent::MouseLeave);
                        }

                        _ => {}
                    }
                }

                _ => {}
            }

            if *stored_control_flow.borrow() == ControlFlow::Exit {
                *control_flow = ControlFlow::Exit;
            } else if let Some(timer_time) = cx.get_next_timer_time() {
                *control_flow = ControlFlow::WaitUntil(timer_time);
            } else {
                *control_flow = *stored_control_flow.borrow();
            }
        });
    }
}

impl WindowModifiers for Application {
    fn title<T: ToString>(mut self, title: T) -> Self {
        self.window_description.title = title.to_string();

        self
    }

    fn inner_size<S: Into<WindowSize>>(mut self, size: S) -> Self {
        self.window_description.inner_size = size.into();

        self
    }

    fn min_inner_size<S: Into<WindowSize>>(mut self, size: Option<S>) -> Self {
        self.window_description.min_inner_size = size.map(|size| size.into());

        self
    }

    fn max_inner_size<S: Into<WindowSize>>(mut self, size: Option<S>) -> Self {
        self.window_description.max_inner_size = size.map(|size| size.into());

        self
    }

    fn position<P: Into<Position>>(mut self, position: P) -> Self {
        self.window_description.position = Some(position.into());

        self
    }

    fn resizable(mut self, flag: bool) -> Self {
        self.window_description.resizable = flag;

        self
    }

    fn minimized(mut self, flag: bool) -> Self {
        self.window_description.minimized = flag;

        self
    }

    fn maximized(mut self, flag: bool) -> Self {
        self.window_description.maximized = flag;

        self
    }

    fn visible(mut self, flag: bool) -> Self {
        self.window_description.visible = flag;

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

    #[cfg(target_arch = "wasm32")]
    fn canvas(mut self, canvas: &str) -> Self {
        self.window_description.target_canvas = Some(canvas.to_owned());

        self
    }
}
