//use crate::event_manager::EventManager;
use crate::window::TuixWindow;
use crate::Renderer;
use baseview::{Window, WindowScalePolicy};
use femtovg::Canvas;
use raw_window_handle::{HasRawWindowHandle, RawWindowHandle};
use tuix_core::TreeExt;
use tuix_core::{MouseButton, MouseButtonState};
use tuix_core::WindowWidget;
use tuix_core::{
    Event, Propagation,
    WindowDescription,
    BoundingBox
};
use tuix_core::{
    Entity, EventManager, Tree, PropSet, WindowSize, State, Units, Visibility,
    WindowEvent,
};

pub struct Application<F>
where
    F: FnOnce(&mut State, Entity),
    F: 'static + Send,
{
    app: F,
    window_description: WindowDescription,
    on_idle: Option<Box<dyn Fn(&mut State) + Send>>,
}

impl<F> Application<F>
where
    F: FnOnce(&mut Context),
    F: 'static + Send,
{
    pub fn new(window_description: WindowDescription, app: F) -> Self {
        Self {
            app,
            window_description,
            on_idle: None,
        }
    }

    /// Open a new window that blocks the current thread until the window is destroyed.
    ///
    /// Do **not** use this in the context of audio plugins, unless it is compiled as a
    /// standalone application.
    ///
    /// * `app` - The Tuix application builder.
    pub fn run(self) {
        TuixWindow::open_blocking(self.window_description, self.app, self.on_idle)
    }

    /// Open a new child window.
    ///
    /// This function does **not** block the current thread. This is only to be
    /// used in the context of audio plugins.
    ///
    /// * `parent` - The parent window.
    /// * `app` - The Tuix application builder.
    pub fn open_parented<P: HasRawWindowHandle>(self, parent: &P) {
        TuixWindow::open_parented(parent, self.window_description, self.app, self.on_idle)
    }

    /// Open a new window as if it had a parent window.
    ///
    /// This function does **not** block the current thread. This is only to be
    /// used in the context of audio plugins.
    ///
    /// * `app` - The Tuix application builder.
    pub fn open_as_if_parented(self) -> RawWindowHandle {
        TuixWindow::open_as_if_parented(self.window_description, self.app, self.on_idle)
    }


    /// Takes a closure which will be called at the end of every loop of the application.
    /// 
    /// The callback provides a place to run 'idle' processing and happens at the end of each loop but before drawing.
    /// If the callback pushes events into the queue in state then the event loop will re-run. Care must be taken not to
    /// push events into the queue every time the callback runs unless this is intended.
    ///
    /// # Example
    /// ```
    /// Application::new(WindowDescription::new(), |state, window|{
    ///     // Build application here
    /// })
    /// .on_idle(|state|{
    ///     // Code here runs at the end of every event loop after OS and tuix events have been handled 
    /// })
    /// .run();
    /// ```
    pub fn on_idle<I: 'static + Fn(&mut State) + Send>(mut self, callback: I) -> Self {
        self.on_idle = Some(Box::new(callback));

        self
    } 


}

pub(crate) struct ApplicationRunner {
    state: State,
    event_manager: EventManager,
    canvas: Canvas<Renderer>,
    tree: Tree,
    pos: (f32, f32),
    should_redraw: bool,
    scale_policy: WindowScalePolicy,
    scale_factor: f64,
}

impl ApplicationRunner {
    pub fn new(mut state: State, win_desc: WindowDescription, renderer: Renderer) -> Self {
        let event_manager = EventManager::new();

        let mut canvas = Canvas::new(renderer).expect("Cannot create canvas");

        // TODO: Get scale policy from `win_desc`.
        let scale_policy = WindowScalePolicy::SystemScaleFactor;

        // Assume scale for now until there is an event with a new one.
        let scale = match scale_policy {
            WindowScalePolicy::ScaleFactor(scale) => scale,
            WindowScalePolicy::SystemScaleFactor => 1.0,
        };

        let logical_size = win_desc.inner_size;
        let physical_size = WindowSize {
            width: (logical_size.width as f64 * scale).round() as u32,
            height: (logical_size.height as f64 * scale).round() as u32,
        };

        canvas.set_size(physical_size.width, physical_size.height, 1.0);

        let regular_font = include_bytes!("../../resources/FiraCode-Regular.ttf");
        let bold_font = include_bytes!("../../resources/Roboto-Bold.ttf");
        let icon_font = include_bytes!("../../resources/entypo.ttf");
        let emoji_font = include_bytes!("../../resources/OpenSansEmoji.ttf");
        let arabic_font = include_bytes!("../../resources/amiri-regular.ttf");

        state.add_font_mem("roboto", regular_font);
        state.add_font_mem("roboto-bold", bold_font);
        state.add_font_mem("icon", icon_font);
        state.add_font_mem("emoji", emoji_font);
        state.add_font_mem("arabic", arabic_font);

        canvas.scale(scale as f32, scale as f32);

        state
            .style
            .width
            .insert(Entity::root(), Units::Pixels(logical_size.width as f32));
        state
            .style
            .height
            .insert(Entity::root(), Units::Pixels(logical_size.height as f32));

        state
            .data
            .set_width(Entity::root(), physical_size.width as f32);
        state
            .data
            .set_height(Entity::root(), physical_size.height as f32);
        state.data.set_opacity(Entity::root(), 1.0);

        let mut bounding_box = BoundingBox::default();
        bounding_box.w = logical_size.width as f32;
        bounding_box.h = logical_size.height as f32;

        state.data.set_clip_region(Entity::root(), bounding_box);

        WindowWidget::new().build_window(&mut state);

        let root = Entity::root();

        root.restyle(&mut state);
        root.relayout(&mut state);

        let tree = state.tree.clone();

        //tuix_core::systems::apply_styles(&mut state, &tree);

        ApplicationRunner {
            event_manager,
            state,
            canvas,
            tree,
            pos: (0.0, 0.0),
            should_redraw: true,
            scale_policy,
            scale_factor: scale,
        }
    }

    /*
    pub fn get_window(&self) -> Entity {
        self.Entity::root()
    }

    pub fn get_state(&mut self) -> &mut State {
        &mut self.state
    }

    pub fn get_event_manager(&mut self) -> &mut EventManager {
        &mut self.event_manager
    }
    */

    pub fn on_frame_update(&mut self) {

        
        if self.state.apply_animations() {
            Entity::root().restyle(&mut self.state);
            Entity::root().relayout(&mut self.state);
            Entity::root().redraw(&mut self.state);
        }

        while !self.state.event_queue.is_empty() {
            self.event_manager.flush_events(&mut self.state);
        } 

        if self.state.needs_redraw {
        //     // TODO - Move this to EventManager
            self.should_redraw = true;
            self.state.needs_redraw = false;
        }

    }

    pub fn render(&mut self) {
        let tree = self.state.tree.clone();
        tuix_core::apply_clipping(&mut self.state, &tree);
        self.event_manager.draw(&mut self.state, &mut self.canvas);
        self.should_redraw = false;
    }

    pub fn handle_event(&mut self, event: baseview::Event, should_quit: &mut bool) {
        if requests_exit(&event) {
            self.state
                .insert_event(Event::new(WindowEvent::WindowClose));
            *should_quit = true;
        }

        match event {
            baseview::Event::Mouse(event) => match event {
                baseview::MouseEvent::CursorMoved { position } => {
                    let cursorx = (position.x) as f32;
                    let cursory = (position.y) as f32;

                    self.state.mouse.cursorx = cursorx;
                    self.state.mouse.cursory = cursory;

                    tuix_core::apply_hover(&mut self.state);

                    if self.state.captured != Entity::null() {
                        self.state.insert_event(
                            Event::new(WindowEvent::MouseMove(cursorx, cursory))
                                .target(self.state.captured)
                                .propagate(Propagation::Direct),
                        );
                    } else if self.state.hovered != Entity::root() {
                        self.state.insert_event(
                            Event::new(WindowEvent::MouseMove(cursorx, cursory))
                                .target(self.state.hovered),
                        );
                    }
                }
                baseview::MouseEvent::ButtonPressed(button) => {
                    let b = match button {
                        baseview::MouseButton::Left => MouseButton::Left,
                        baseview::MouseButton::Right => MouseButton::Right,
                        baseview::MouseButton::Middle => MouseButton::Middle,
                        baseview::MouseButton::Other(id) => MouseButton::Other(id as u16),
                        baseview::MouseButton::Back => MouseButton::Other(4),
                        baseview::MouseButton::Forward => MouseButton::Other(5),
                    };

                    match b {
                        MouseButton::Left => {
                            self.state.mouse.left.state = MouseButtonState::Pressed;
                        }
                        MouseButton::Right => {
                            self.state.mouse.right.state = MouseButtonState::Pressed;
                        }
                        MouseButton::Middle => {
                            self.state.mouse.middle.state = MouseButtonState::Pressed;
                        }
                        _ => {}
                    };

                    // if self.state.hovered != Entity::null()
                    //     && self.state.active != self.state.hovered
                    // {
                    //     self.state.active = self.state.hovered;
                    //     self.state.insert_event(Event::new(WindowEvent::Restyle).target(Entity::root()));
                    //     self.state.needs_restyle = true;
                    // }

                    let target = if self.state.captured != Entity::null() {
                        self.state.insert_event(
                            Event::new(WindowEvent::MouseDown(b))
                                .target(self.state.captured)
                                .propagate(Propagation::Direct),
                        );
                        self.state.captured
                    } else {
                        self.state.insert_event(
                            Event::new(WindowEvent::MouseDown(b))
                                .target(self.state.hovered),
                        );
                        self.state.hovered
                    };

                    // if let Some(event_handler) = self.event_manager.event_handlers.get_mut(&target) {
                    //     if let Some(callback) = self.event_manager.callbacks.get_mut(&target) {
                    //         (callback)(event_handler, &mut self.state, target);
                    //     }
                    // }

                    match b {
                        MouseButton::Left => {
                            self.state.mouse.left.pos_down =
                                (self.state.mouse.cursorx, self.state.mouse.cursory);
                            self.state.mouse.left.pressed = self.state.hovered;
                        }

                        MouseButton::Middle => {
                            self.state.mouse.middle.pos_down =
                                (self.state.mouse.cursorx, self.state.mouse.cursory);
                            self.state.mouse.left.pressed = self.state.hovered;
                        }

                        MouseButton::Right => {
                            self.state.mouse.right.pos_down =
                                (self.state.mouse.cursorx, self.state.mouse.cursory);
                            self.state.mouse.left.pressed = self.state.hovered;
                        }

                        _ => {}
                    }
                }
                baseview::MouseEvent::ButtonReleased(button) => {
                    let b = match button {
                        baseview::MouseButton::Left => MouseButton::Left,
                        baseview::MouseButton::Right => MouseButton::Right,
                        baseview::MouseButton::Middle => MouseButton::Middle,
                        baseview::MouseButton::Other(id) => MouseButton::Other(id as u16),
                        baseview::MouseButton::Back => MouseButton::Other(4),
                        baseview::MouseButton::Forward => MouseButton::Other(5),
                    };

                    match b {
                        MouseButton::Left => {
                            self.state.mouse.left.state = MouseButtonState::Released;
                        }
                        MouseButton::Right => {
                            self.state.mouse.right.state = MouseButtonState::Released;
                        }
                        MouseButton::Middle => {
                            self.state.mouse.middle.state = MouseButtonState::Released;
                        }
                        _ => {}
                    };

                    // self.state.active = Entity::null();
                    // self.state.insert_event(Event::new(WindowEvent::Restyle).target(Entity::root()));
                    // self.state.needs_restyle = true;

                    if self.state.captured != Entity::null() {
                        self.state.insert_event(
                            Event::new(WindowEvent::MouseUp(b))
                                .target(self.state.captured)
                                .propagate(Propagation::Direct),
                        );
                    } else {
                        self.state.insert_event(
                            Event::new(WindowEvent::MouseUp(b)).target(self.state.hovered),
                        );
                    }

                    match b {
                        MouseButton::Left => {
                            self.state.mouse.left.pos_up =
                                (self.state.mouse.cursorx, self.state.mouse.cursory);
                            self.state.mouse.left.released = self.state.hovered;
                        }

                        MouseButton::Middle => {
                            self.state.mouse.middle.pos_up =
                                (self.state.mouse.cursorx, self.state.mouse.cursory);
                            self.state.mouse.left.released = self.state.hovered;
                        }

                        MouseButton::Right => {
                            self.state.mouse.right.pos_up =
                                (self.state.mouse.cursorx, self.state.mouse.cursory);
                            self.state.mouse.left.released = self.state.hovered;
                        }

                        _ => {}
                    }
                }
                baseview::MouseEvent::WheelScrolled(scroll_delta) => {
                    let (lines_x, lines_y) = match scroll_delta {
                        baseview::ScrollDelta::Lines { x, y } => (x, y),
                        baseview::ScrollDelta::Pixels { x, y } => (
                            if x < 0.0 {
                                -1.0
                            } else if x > 1.0 {
                                1.0
                            } else {
                                0.0
                            },
                            if y < 0.0 {
                                -1.0
                            } else if y > 1.0 {
                                1.0
                            } else {
                                0.0
                            },
                        ),
                    };

                    if self.state.captured != Entity::null() {
                        self.state.insert_event(
                            Event::new(WindowEvent::MouseScroll(lines_x, lines_y))
                                .target(self.state.captured)
                                .propagate(Propagation::Direct),
                        );
                    } else {
                        self.state.insert_event(
                            Event::new(WindowEvent::MouseScroll(lines_x, lines_y))
                                .target(self.state.hovered),
                        );
                    }
                }
                _ => {}
            },
            baseview::Event::Keyboard(event) => {
                use keyboard_types::Code;

                let (s, pressed) = match event.state {
                    keyboard_types::KeyState::Down => (MouseButtonState::Pressed, true),
                    keyboard_types::KeyState::Up => (MouseButtonState::Released, false),
                };

                match event.code {
                    Code::ShiftLeft | Code::ShiftRight => self.state.modifiers.shift = pressed,
                    Code::ControlLeft | Code::ControlRight => self.state.modifiers.ctrl = pressed,
                    Code::AltLeft | Code::AltRight => self.state.modifiers.alt = pressed,
                    Code::MetaLeft | Code::MetaRight => self.state.modifiers.logo = pressed,
                    _ => (),
                }

                if event.code == Code::F5 && s == MouseButtonState::Pressed {
                    self.state.reload_styles().unwrap();
                }

                if event.code == Code::Tab && s == MouseButtonState::Pressed {
                    let next_focus = self
                        .state
                        .style
                        .focus_order
                        .get(self.state.focused)
                        .cloned()
                        .unwrap_or_default()
                        .next;
                    let prev_focus = self
                        .state
                        .style
                        .focus_order
                        .get(self.state.focused)
                        .cloned()
                        .unwrap_or_default()
                        .prev;

                    if self.state.modifiers.shift {
                        if prev_focus != Entity::null() {
                            self.state.focused.set_focus(&mut self.state, false);
                            self.state.focused = prev_focus;
                            self.state.focused.set_focus(&mut self.state, true);
                        } else {
                            // TODO impliment reverse iterator for tree
                            // state.focused = match state.focused.into_iter(&state.tree).next() {
                            //     Some(val) => val,
                            //     None => Entity::root(),
                            // };
                        }
                    } else {
                        if next_focus != Entity::null() {
                            self.state.focused.set_focus(&mut self.state, false);
                            self.state.focused = next_focus;
                            self.state.focused.set_focus(&mut self.state, true);
                        } else {
                            self.state.focused.set_focus(&mut self.state, false);
                            self.state.focused =
                                match self.state.focused.tree_iter(&self.tree).next() {
                                    Some(val) => val,
                                    None => Entity::root(),
                                };
                            self.state.focused.set_focus(&mut self.state, true);
                        }
                    }

                    Entity::root().restyle(&mut self.state);
                }

                match s {
                    MouseButtonState::Pressed => {
                        if self.state.focused != Entity::null() {
                            self.state.insert_event(
                                Event::new(WindowEvent::KeyDown(
                                    event.code,
                                    Some(event.key.clone()),
                                ))
                                .target(self.state.focused)
                                .propagate(Propagation::DownUp),
                            );
                        } else {
                            self.state.insert_event(
                                Event::new(WindowEvent::KeyDown(
                                    event.code,
                                    Some(event.key.clone()),
                                ))
                                .target(self.state.hovered)
                                .propagate(Propagation::DownUp),
                            );
                        }

                        if let keyboard_types::Key::Character(written) = &event.key {
                            for chr in written.chars() {
                                self.state.insert_event(
                                    Event::new(WindowEvent::CharInput(chr))
                                        .target(self.state.focused)
                                        .propagate(Propagation::Down),
                                );
                            }
                        }
                    }

                    MouseButtonState::Released => {
                        if self.state.focused != Entity::null() {
                            self.state.insert_event(
                                Event::new(WindowEvent::KeyUp(event.code, Some(event.key)))
                                    .target(self.state.focused)
                                    .propagate(Propagation::DownUp),
                            );
                        } else {
                            self.state.insert_event(
                                Event::new(WindowEvent::KeyUp(event.code, Some(event.key)))
                                    .target(self.state.hovered)
                                    .propagate(Propagation::DownUp),
                            );
                        }
                    }
                }
            }
            baseview::Event::Window(event) => match event {
                baseview::WindowEvent::Focused => {
                    Entity::root().restyle(&mut self.state);
                    Entity::root().relayout(&mut self.state);
                    Entity::root().redraw(&mut self.state);
                }
                baseview::WindowEvent::Resized(window_info) => {
                    self.scale_factor = match self.scale_policy {
                        WindowScalePolicy::ScaleFactor(scale) => scale,
                        WindowScalePolicy::SystemScaleFactor => window_info.scale(),
                    };

                    let logical_size = (
                        (window_info.physical_size().width as f64 / self.scale_factor),
                        (window_info.physical_size().height as f64 / self.scale_factor),
                    );

                    let physical_size = (
                        window_info.physical_size().width,
                        window_info.physical_size().height,
                    );

                    self.state
                        .style
                        .width
                        .insert(Entity::root(), Units::Pixels(logical_size.0 as f32));
                    self.state
                        .style
                        .height
                        .insert(Entity::root(), Units::Pixels(logical_size.1 as f32));

                    self.state
                        .data
                        .set_width(Entity::root(), physical_size.0 as f32);
                    self.state
                        .data
                        .set_height(Entity::root(), physical_size.1 as f32);

                    let mut bounding_box = BoundingBox::default();
                    bounding_box.w = physical_size.0 as f32;
                    bounding_box.h = physical_size.1 as f32;

                    self.state.data.set_clip_region(Entity::root(), bounding_box);

                    Entity::root().restyle(&mut self.state);
                    Entity::root().relayout(&mut self.state);
                    Entity::root().redraw(&mut self.state);

                }
                baseview::WindowEvent::WillClose => {
                    self.state
                        .insert_event(Event::new(WindowEvent::WindowClose));
                }
                _ => {}
            },
        }
    }

    pub fn handle_idle(&mut self, on_idle: &Option<Box<dyn Fn(&mut State) + Send>>) {
        if let Some(idle_callback) = on_idle {
            (idle_callback)(&mut self.state);
        }
    }
}

/// Returns true if the provided event should cause an [`Application`] to
/// exit.
pub fn requests_exit(event: &baseview::Event) -> bool {
    match event {
        baseview::Event::Window(baseview::WindowEvent::WillClose) => true,
        #[cfg(target_os = "macos")]
        baseview::Event::Keyboard(event) => {
            if event.code == keyboard_types::Code::KeyQ {
                if event.modifiers == keyboard_types::Modifiers::META {
                    if event.state == keyboard_types::KeyState::Down {
                        return true;
                    }
                }
            }

            false
        }
        _ => false,
    }
}
