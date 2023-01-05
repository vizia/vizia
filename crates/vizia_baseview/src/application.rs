use crate::window::ViziaWindow;
use baseview::{Window, WindowHandle, WindowScalePolicy};
use raw_window_handle::HasRawWindowHandle;

use crate::proxy::queue_get;
use vizia_core::cache::BoundingBox;
use vizia_core::context::backend::*;
use vizia_core::events::EventManager;
use vizia_core::prelude::*;
use vizia_id::GenerationalId;

pub struct Application<F>
where
    F: Fn(&mut Context) + Send + 'static,
{
    app: F,
    window_description: WindowDescription,
    window_scale_policy: WindowScalePolicy,
    on_idle: Option<Box<dyn Fn(&mut Context) + Send>>,
    ignore_default_theme: bool,
}

impl<F> Application<F>
where
    F: Fn(&mut Context),
    F: 'static + Send,
{
    pub fn new(app: F) -> Self {
        Self {
            app,
            window_description: WindowDescription::new(),
            window_scale_policy: WindowScalePolicy::SystemScaleFactor,
            on_idle: None,
            ignore_default_theme: false,
        }
    }

    pub fn ignore_default_theme(mut self) -> Self {
        self.ignore_default_theme = true;
        self
    }

    /// Change the window's scale policy. Not part of [`new()`][Self::new] to keep the same
    /// signature as the winit backend. This should only be used for HiDPI scaling, use
    /// [`WindowDescription::scale_factor`] to set a separate arbitrary scale factor.
    pub fn with_scale_policy(mut self, scale_policy: WindowScalePolicy) -> Self {
        self.window_scale_policy = scale_policy;
        self
    }

    pub fn title(mut self, title: &str) -> Self {
        self.window_description.title = title.to_owned();

        self
    }

    pub fn inner_size(mut self, size: impl Into<WindowSize>) -> Self {
        self.window_description.inner_size = size.into();

        self
    }

    /// A scale factor applied on top of any DPI scaling, defaults to 1.0.
    pub fn user_scale_factor(mut self, factor: f64) -> Self {
        self.window_description.user_scale_factor = factor;

        self
    }

    /// Open a new window that blocks the current thread until the window is destroyed.
    ///
    /// Do **not** use this in the context of audio plugins, unless it is compiled as a
    /// standalone application.
    ///
    /// * `app` - The Vizia application builder.
    pub fn run(self) {
        ViziaWindow::open_blocking(
            self.window_description,
            self.window_scale_policy,
            self.app,
            self.on_idle,
            self.ignore_default_theme,
        )
    }

    /// Open a new child window.
    ///
    /// This function does **not** block the current thread. This is only to be
    /// used in the context of audio plugins.
    ///
    /// * `parent` - The parent window.
    /// * `app` - The Vizia application builder.
    pub fn open_parented<P: HasRawWindowHandle>(self, parent: &P) -> WindowHandle {
        ViziaWindow::open_parented(
            parent,
            self.window_description,
            self.window_scale_policy,
            self.app,
            self.on_idle,
            self.ignore_default_theme,
        )
    }

    /// Open a new window as if it had a parent window.
    ///
    /// This function does **not** block the current thread. This is only to be
    /// used in the context of audio plugins.
    ///
    /// * `app` - The Vizia application builder.
    pub fn open_as_if_parented(self) -> WindowHandle {
        ViziaWindow::open_as_if_parented(
            self.window_description,
            self.window_scale_policy,
            self.app,
            self.on_idle,
            self.ignore_default_theme,
        )
    }

    /// Takes a closure which will be called at the end of every loop of the application.
    ///
    /// The callback provides a place to run 'idle' processing and happens at the end of each loop but before drawing.
    /// If the callback pushes events into the queue in context then the event loop will re-run. Care must be taken not to
    /// push events into the queue every time the callback runs unless this is intended.
    ///
    /// # Example
    /// ```no_run
    /// # use vizia_core::prelude::*;
    /// # use vizia_baseview::Application;
    /// Application::new(|cx|{
    ///     // Build application here
    /// })
    /// .on_idle(|cx|{
    ///     // Code here runs at the end of every event loop after OS and vizia events have been handled
    /// })
    /// .run();
    /// ```
    pub fn on_idle<I: 'static + Fn(&mut Context) + Send>(mut self, callback: I) -> Self {
        self.on_idle = Some(Box::new(callback));

        self
    }
}

pub(crate) struct ApplicationRunner {
    context: Context,
    event_manager: EventManager,
    should_redraw: bool,

    /// If this is set to `true`, then `window_scale_factor` will be updated during
    /// [`baseview::WindowEvent::Resized`] events in accordance to the system's reported DPI. This
    /// can change at runtime when the window is dragged between displays. Otherwise
    /// `window_scale_factor` will not change.
    use_system_scaling: bool,
    /// The scale factor for the window itself. This is either determined by either the operating
    /// system or explicitly overridden by the creator of the window. In some cases window resize
    /// events may change this scaling policy. This value is only used when translating logical
    /// mouse coordinates to physical window coordinates. For any other use within VIZIA itself this
    /// always needs to be multplied by `user_scale_factor`.
    window_scale_factor: f64,
    /// The scale factor applied on top of the `window_scale` to convert the window's logical size
    /// to a physical size. If this is different from `*cx.user_scale_factor` after handling the
    /// events then the window will be resized.
    current_user_scale_factor: f64,
    /// The window's current logical size, before `user_scale_factor` has been applied. Needed to
    /// resize the window when changing the scale factor.
    current_window_size: WindowSize,
}

impl ApplicationRunner {
    pub fn new(mut context: Context, use_system_scaling: bool, window_scale_factor: f64) -> Self {
        let mut cx = BackendContext::new(&mut context);
        let event_manager = EventManager::new();

        ApplicationRunner {
            event_manager,
            should_redraw: true,

            use_system_scaling,
            window_scale_factor,
            current_user_scale_factor: *cx.user_scale_factor(),
            current_window_size: *cx.window_size(),

            context,
        }
    }

    /// Handle all reactivity within a frame. The window instance is used to resize the window when
    /// needed.
    pub fn on_frame_update(&mut self, window: &mut Window) {
        let mut cx = BackendContext::new(&mut self.context);

        while let Some(event) = queue_get() {
            cx.send_event(event);
        }

        // Events
        while self.event_manager.flush_events(&mut cx.context()) {}

        if *cx.window_size() != self.current_window_size
            || *cx.user_scale_factor() != self.current_user_scale_factor
        {
            self.current_window_size = *cx.window_size();
            self.current_user_scale_factor = *cx.user_scale_factor();

            // The user scale factor is not part of the HiDPI scaling, so baseview should treat it
            // as part of our logical size
            window.resize(baseview::Size {
                width: self.current_window_size.width as f64 * self.current_user_scale_factor,
                height: self.current_window_size.height as f64 * self.current_user_scale_factor,
            });

            // TODO: These calculations are now repeated in three places, should probably be moved
            //       to a function
            cx.style().dpi_factor = self.window_scale_factor * self.current_user_scale_factor;
            cx.style()
                .width
                .insert(Entity::root(), Units::Pixels(self.current_window_size.width as f32));
            cx.style()
                .height
                .insert(Entity::root(), Units::Pixels(self.current_window_size.height as f32));

            let new_physical_width =
                self.current_window_size.width as f32 * cx.style().dpi_factor as f32;
            let new_physical_height =
                self.current_window_size.height as f32 * cx.style().dpi_factor as f32;
            cx.cache().set_width(Entity::root(), new_physical_width);
            cx.cache().set_height(Entity::root(), new_physical_height);

            cx.cache().set_clip_region(
                Entity::root(),
                BoundingBox {
                    w: new_physical_width,
                    h: new_physical_height,
                    ..BoundingBox::default()
                },
            );

            cx.0.need_restyle();
            cx.0.need_relayout();
            cx.0.need_redraw();

            // After the window is resized, we should let every view know about this so they can act
            // accordingly
            cx.0.emit_custom(
                Event::new(WindowEvent::WindowResize)
                    .target(Entity::root())
                    .origin(Entity::root())
                    .propagate(Propagation::Subtree),
            );
            self.event_manager.flush_events(cx.context());
        }

        cx.load_images();

        // Force restyle on every frame for baseview backend to avoid style inheritance issues
        cx.style().needs_restyle = true;
        cx.process_data_updates();
        cx.process_style_updates();

        cx.process_visual_updates();

        if cx.style().needs_redraw {
            // TODO - Move this to EventManager
            self.should_redraw = true;
            cx.style().needs_redraw = false;
        }
    }

    pub fn render(&mut self) {
        let mut cx = BackendContext::new(&mut self.context);
        cx.draw();
        self.should_redraw = false;
    }

    pub fn handle_event(&mut self, event: baseview::Event, should_quit: &mut bool) {
        let mut cx = BackendContext::new(&mut self.context);

        if requests_exit(&event) {
            cx.send_event(Event::new(WindowEvent::WindowClose));
            *should_quit = true;
        }

        let mut update_modifiers = |modifiers: vizia_input::KeyboardModifiers| {
            cx.modifiers()
                .set(Modifiers::SHIFT, modifiers.contains(vizia_input::KeyboardModifiers::SHIFT));
            cx.modifiers()
                .set(Modifiers::CTRL, modifiers.contains(vizia_input::KeyboardModifiers::CONTROL));
            cx.modifiers()
                .set(Modifiers::LOGO, modifiers.contains(vizia_input::KeyboardModifiers::META));
            cx.modifiers()
                .set(Modifiers::ALT, modifiers.contains(vizia_input::KeyboardModifiers::ALT));
        };

        match event {
            baseview::Event::Mouse(event) => match event {
                baseview::MouseEvent::CursorMoved { position, modifiers } => {
                    update_modifiers(modifiers);

                    // NOTE: We multiply by `self.window_scale_factor` and not by
                    //       `self.context.style.dpi_factor`. Since the additional scaling by
                    //       internally do additional scaling by `self.context.user_scale_factor` is
                    //       done internally to be able to separate actual HiDPI scaling from
                    //       arbitrary uniform scaling baseview only knows about its own scale
                    //       factor.
                    let physical_posx = position.x * self.window_scale_factor;
                    let physical_posy = position.y * self.window_scale_factor;
                    let cursorx = (physical_posx) as f32;
                    let cursory = (physical_posy) as f32;
                    cx.emit_origin(WindowEvent::MouseMove(cursorx, cursory));
                }
                baseview::MouseEvent::ButtonPressed { button, modifiers } => {
                    update_modifiers(modifiers);

                    let b = translate_mouse_button(button);
                    cx.emit_origin(WindowEvent::MouseDown(b));
                }
                baseview::MouseEvent::ButtonReleased { button, modifiers } => {
                    update_modifiers(modifiers);

                    let b = translate_mouse_button(button);
                    cx.emit_origin(WindowEvent::MouseUp(b));
                }
                baseview::MouseEvent::WheelScrolled { delta, modifiers } => {
                    update_modifiers(modifiers);

                    let (lines_x, lines_y) = match delta {
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

                    cx.emit_origin(WindowEvent::MouseScroll(lines_x, lines_y));
                }
                _ => {}
            },
            baseview::Event::Keyboard(event) => {
                let (s, pressed) = match event.state {
                    vizia_input::KeyState::Down => (MouseButtonState::Pressed, true),
                    vizia_input::KeyState::Up => (MouseButtonState::Released, false),
                };

                match event.code {
                    Code::ShiftLeft | Code::ShiftRight => {
                        cx.modifiers().set(Modifiers::SHIFT, pressed)
                    }
                    Code::ControlLeft | Code::ControlRight => {
                        cx.modifiers().set(Modifiers::CTRL, pressed)
                    }
                    Code::AltLeft | Code::AltRight => cx.modifiers().set(Modifiers::ALT, pressed),
                    Code::MetaLeft | Code::MetaRight => {
                        cx.modifiers().set(Modifiers::LOGO, pressed)
                    }
                    _ => (),
                }

                match s {
                    MouseButtonState::Pressed => {
                        cx.emit_origin(WindowEvent::KeyDown(event.code, Some(event.key.clone())));

                        if let vizia_input::Key::Character(written) = &event.key {
                            for chr in written.chars() {
                                cx.emit_origin(WindowEvent::CharInput(chr));
                            }
                        }
                    }

                    MouseButtonState::Released => {
                        cx.emit_origin(WindowEvent::KeyUp(event.code, Some(event.key.clone())));
                    }
                }
            }
            baseview::Event::Window(event) => match event {
                baseview::WindowEvent::Focused => {
                    cx.0.need_restyle();
                    cx.0.need_relayout();
                    cx.0.need_redraw();
                }
                baseview::WindowEvent::Resized(window_info) => {
                    // We keep track of the current size before applying the user scale factor while
                    // baseview's logical size includes that factor so we need to compensate for it
                    self.current_window_size = *cx.window_size();
                    self.current_window_size.width =
                        (window_info.logical_size().width / *cx.user_scale_factor()).round() as u32;
                    self.current_window_size.height = (window_info.logical_size().height
                        / *cx.user_scale_factor())
                    .round() as u32;
                    *cx.window_size() = self.current_window_size;

                    // Only use new DPI settings when `WindowScalePolicy::SystemScaleFactor` was
                    // used
                    if self.use_system_scaling {
                        self.window_scale_factor = window_info.scale();
                    }

                    cx.style().dpi_factor = self.window_scale_factor * *cx.user_scale_factor();

                    // Since we apply a user scale factor, our logical size may not match baseview's
                    // logical size
                    let logical_size = (
                        (window_info.physical_size().width as f64 / cx.style().dpi_factor),
                        (window_info.physical_size().height as f64 / cx.style().dpi_factor),
                    );

                    let physical_size =
                        (window_info.physical_size().width, window_info.physical_size().height);

                    cx.style().width.insert(Entity::root(), Units::Pixels(logical_size.0 as f32));
                    cx.style().height.insert(Entity::root(), Units::Pixels(logical_size.1 as f32));

                    cx.cache().set_width(Entity::root(), physical_size.0 as f32);
                    cx.cache().set_height(Entity::root(), physical_size.1 as f32);

                    let mut bounding_box = BoundingBox::default();
                    bounding_box.w = physical_size.0 as f32;
                    bounding_box.h = physical_size.1 as f32;

                    cx.cache().set_clip_region(Entity::root(), bounding_box);

                    cx.0.need_restyle();
                    cx.0.need_relayout();
                    cx.0.need_redraw();

                    // TODO: We send an event when changes to `*cx.window_size` or
                    //       `*cx.user_scale_factor` would trigger a resize, but not when when the
                    //       window gets resized externally. If we do end up doing this, then these
                    //       events should be separate from the current `WindowResize` because they
                    //       may need to be handled differently (and they should not get stuck in a
                    //       feedback loop).
                }
                baseview::WindowEvent::WillClose => {
                    cx.send_event(Event::new(WindowEvent::WindowClose));
                }
                _ => {}
            },
        }
    }

    pub fn handle_idle(&mut self, on_idle: &Option<Box<dyn Fn(&mut Context) + Send>>) {
        let mut cx = BackendContext::new(&mut self.context);
        if let Some(idle_callback) = on_idle {
            cx.set_current(Entity::root());
            (idle_callback)(&mut self.context);
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
            if event.code == vizia_input::Code::KeyQ {
                if event.modifiers == vizia_input::KeyboardModifiers::META {
                    if event.state == vizia_input::KeyState::Down {
                        return true;
                    }
                }
            }

            false
        }
        _ => false,
    }
}

fn translate_mouse_button(button: baseview::MouseButton) -> MouseButton {
    match button {
        baseview::MouseButton::Left => MouseButton::Left,
        baseview::MouseButton::Right => MouseButton::Right,
        baseview::MouseButton::Middle => MouseButton::Middle,
        baseview::MouseButton::Other(id) => MouseButton::Other(id as u16),
        baseview::MouseButton::Back => MouseButton::Other(4),
        baseview::MouseButton::Forward => MouseButton::Other(5),
    }
}
