use crate::window::create_surface;
use crate::window::ViziaWindow;
use baseview::{Window, WindowHandle, WindowScalePolicy};
use gl_rs as gl;
use gl_rs::types::GLint;
use raw_window_handle::HasRawWindowHandle;
use skia_safe::gpu::gl::FramebufferInfo;
use vizia_core::events::EventManager;

use crate::proxy::queue_get;
use vizia_core::backend::*;
use vizia_core::prelude::*;

#[derive(Debug)]
pub enum ApplicationError {}

///Creating a new application creates a root `Window` and a `Context`. Views declared within the closure passed to `Application::new()` are added to the context and rendered into the root window.
///
/// # Example
/// ```no_run
/// # use vizia_core::prelude::*;
/// # use vizia_baseview::Application;
///
/// Application::new(|cx|{
///    // Content goes here
/// })
/// .run();
///```
/// Calling `run()` on the `Application` causes the program to enter the event loop and for the main window to display.
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

    /// Sets the default built-in theming to be ignored.
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
    pub fn run(self) -> Result<(), ApplicationError> {
        ViziaWindow::open_blocking(
            self.window_description,
            self.window_scale_policy,
            self.app,
            self.on_idle,
            self.ignore_default_theme,
        );

        Ok(())
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
    cx: BackendContext,
    event_manager: EventManager,
    pub gr_context: skia_safe::gpu::DirectContext,
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
    /// always needs to be multiplied by `user_scale_factor`.
    window_scale_factor: f64,
    // /// The scale factor applied on top of the `window_scale` to convert the window's logical size
    // /// to a physical size. If this is different from `*cx.user_scale_factor` after handling the
    // /// events then the window will be resized.
    // current_user_scale_factor: f64,
    // /// The window's current logical size, before `user_scale_factor` has been applied. Needed to
    // /// resize the window when changing the scale factor.
    // current_window_size: WindowSize,
    pub surface: skia_safe::Surface,
    pub dirty_surface: skia_safe::Surface,
    window_description: WindowDescription,
    is_initialized: bool,
}

impl ApplicationRunner {
    pub fn new(
        cx: BackendContext,
        gr_context: skia_safe::gpu::DirectContext,
        use_system_scaling: bool,
        window_scale_factor: f64,
        surface: skia_safe::Surface,
        dirty_surface: skia_safe::Surface,
        window_description: WindowDescription,
    ) -> Self {
        ApplicationRunner {
            should_redraw: true,
            gr_context,
            event_manager: EventManager::new(),
            use_system_scaling,
            window_scale_factor,
            //current_user_scale_factor: cx.user_scale_factor(),
            //current_window_size: *cx.window_size(),
            cx,
            surface,
            dirty_surface,
            window_description,
            is_initialized: false,
        }
    }

    /// Handle all reactivity within a frame. The window instance is used to resize the window when
    /// needed.
    pub fn on_frame_update(&mut self, window: &mut Window) {
        while let Some(event) = queue_get() {
            self.cx.send_event(event);
        }

        // Events
        self.event_manager.flush_events(self.cx.context(), |window_event| match window_event {
            // For some reason calling window.close() crashes baseview on macos
            // WindowEvent::WindowClose => *should_close = true,
            WindowEvent::FocusIn => {
                #[cfg(not(target_os = "linux"))] // not implemented for linux yet
                if !window.has_focus() {
                    window.focus();
                }
            }
            _ => {}
        });

        // We need to resize the window to make sure that the new size is applied. This is a workaround
        // for the fact that baseview does not resize the window when the scale factor changes.
        if !self.is_initialized {
            // Resizing the window doesn't apply unless the size has actually changed.
            // So we resize the window slightly larger and then back again to force a resize event.
            window.resize(baseview::Size {
                width: self.window_description.inner_size.width as f64 + 1.0,
                height: self.window_description.inner_size.height as f64 + 1.0,
            });

            window.resize(baseview::Size {
                width: self.window_description.inner_size.width as f64,
                height: self.window_description.inner_size.height as f64,
            });
            self.is_initialized = true;
        }

        // if *cx.window_size() != self.current_window_size
        //     || cx.user_scale_factor() != self.current_user_scale_factor
        // {
        //     self.current_window_size = *cx.window_size();
        //     self.current_user_scale_factor = cx.user_scale_factor();

        //     // The user scale factor is not part of the HiDPI scaling, so baseview should treat it
        //     // as part of our logical size
        //     window.resize(baseview::Size {
        //         width: self.current_window_size.width as f64 * self.current_user_scale_factor,
        //         height: self.current_window_size.height as f64 * self.current_user_scale_factor,
        //     });

        //     // TODO: These calculations are now repeated in three places, should probably be moved
        //     //       to a function
        //     cx.set_scale_factor(self.window_scale_factor * self.current_user_scale_factor);
        //     let new_physical_width =
        //         self.current_window_size.width as f32 * cx.style().scale_factor();
        //     let new_physical_height =
        //         self.current_window_size.height as f32 * cx.style().scale_factor();

        //     cx.set_window_size(new_physical_width, new_physical_height);

        //     if let Some(surface) = cx.get_surface_mut(Entity::root()) {
        //         if new_physical_width != 0.0 || new_physical_height != 0.0 {
        //             let fb_info = {
        //                 let mut fboid: GLint = 0;
        //                 unsafe { gl::GetIntegerv(gl::FRAMEBUFFER_BINDING, &mut fboid) };

        //                 FramebufferInfo {
        //                     fboid: fboid.try_into().unwrap(),
        //                     format: skia_safe::gpu::gl::Format::RGBA8.into(),
        //                     ..Default::default()
        //                 }
        //             };

        //             let backend_render_target = backend_render_targets::make_gl(
        //                 (new_physical_width as i32, new_physical_height as i32),
        //                 None,
        //                 8,
        //                 fb_info,
        //             );

        //             surface.0 = gpu::surfaces::wrap_backend_render_target(
        //                 &mut self.gr_context,
        //                 &backend_render_target,
        //                 SurfaceOrigin::BottomLeft,
        //                 ColorType::RGBA8888,
        //                 None,
        //                 None,
        //             )
        //             .expect("Could not create skia surface");

        //             surface.1 = surface
        //                 .0
        //                 .new_surface_with_dimensions((
        //                     new_physical_width.max(1.0) as i32,
        //                     new_physical_height.max(1.0) as i32,
        //                 ))
        //                 .unwrap();
        //         }
        //     }

        //     cx.needs_refresh();

        //     // hmmm why are we flushing events again?
        //     // self.event_manager.flush_events(cx.context());
        // }

        let context = window.gl_context().expect("Window was created without OpenGL support");
        unsafe { context.make_current() };
        self.cx.process_style_updates();
        unsafe { context.make_not_current() };

        self.cx.process_animations();

        self.cx.process_visual_updates();

        if self.cx.0.windows.iter().any(|(_, window_state)| !window_state.redraw_list.is_empty()) {
            self.should_redraw = true;
        }
    }

    pub fn render(&mut self, window: &mut Window) {
        if self.should_redraw {
            let context = window.gl_context().expect("Window was created without OpenGL support");
            unsafe { context.make_current() };
            self.cx.draw(Entity::root(), &mut self.surface, &mut self.dirty_surface);
            self.gr_context.flush_and_submit();
            self.should_redraw = false;
            context.swap_buffers();
            unsafe { context.make_not_current() };
        }
    }

    pub fn handle_event(&mut self, event: baseview::Event, should_quit: &mut bool) {
        if requests_exit(&event) {
            self.cx.send_event(Event::new(WindowEvent::WindowClose));
            *should_quit = true;
        }

        let mut update_modifiers = |modifiers: vizia_input::KeyboardModifiers| {
            self.cx
                .modifiers()
                .set(Modifiers::SHIFT, modifiers.contains(vizia_input::KeyboardModifiers::SHIFT));
            self.cx
                .modifiers()
                .set(Modifiers::CTRL, modifiers.contains(vizia_input::KeyboardModifiers::CONTROL));
            self.cx
                .modifiers()
                .set(Modifiers::SUPER, modifiers.contains(vizia_input::KeyboardModifiers::META));
            self.cx
                .modifiers()
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
                    let cursor_x = (physical_posx) as f32;
                    let cursor_y = (physical_posy) as f32;
                    self.cx.emit_origin(WindowEvent::MouseMove(cursor_x, cursor_y));
                }
                baseview::MouseEvent::ButtonPressed { button, modifiers } => {
                    update_modifiers(modifiers);

                    let b = translate_mouse_button(button);
                    self.cx.emit_origin(WindowEvent::MouseDown(b));
                }
                baseview::MouseEvent::ButtonReleased { button, modifiers } => {
                    update_modifiers(modifiers);

                    let b = translate_mouse_button(button);
                    self.cx.emit_origin(WindowEvent::MouseUp(b));
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

                    self.cx.emit_origin(WindowEvent::MouseScroll(lines_x, lines_y));
                }

                baseview::MouseEvent::CursorEntered => {
                    self.cx.emit_origin(WindowEvent::MouseEnter);
                }

                baseview::MouseEvent::CursorLeft => {
                    self.cx.emit_origin(WindowEvent::MouseLeave);
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
                        self.cx.modifiers().set(Modifiers::SHIFT, pressed)
                    }
                    Code::ControlLeft | Code::ControlRight => {
                        self.cx.modifiers().set(Modifiers::CTRL, pressed)
                    }
                    Code::AltLeft | Code::AltRight => {
                        self.cx.modifiers().set(Modifiers::ALT, pressed)
                    }
                    Code::MetaLeft | Code::MetaRight => {
                        self.cx.modifiers().set(Modifiers::SUPER, pressed)
                    }
                    _ => (),
                }

                match s {
                    MouseButtonState::Pressed => {
                        if let vizia_input::Key::Character(written) = &event.key {
                            for chr in written.chars() {
                                self.cx.emit_origin(WindowEvent::CharInput(chr));
                            }
                        }

                        self.cx.emit_origin(WindowEvent::KeyDown(event.code, Some(event.key)));
                    }

                    MouseButtonState::Released => {
                        self.cx.emit_origin(WindowEvent::KeyUp(event.code, Some(event.key)));
                    }
                }
            }
            baseview::Event::Window(event) => match event {
                baseview::WindowEvent::Focused => self.cx.needs_refresh(Entity::root()),
                baseview::WindowEvent::Resized(window_info) => {
                    let fb_info = {
                        let mut fboid: GLint = 0;
                        unsafe { gl::GetIntegerv(gl::FRAMEBUFFER_BINDING, &mut fboid) };

                        FramebufferInfo {
                            fboid: fboid.try_into().unwrap(),
                            format: skia_safe::gpu::gl::Format::RGBA8.into(),
                            ..Default::default()
                        }
                    };

                    self.surface = create_surface(
                        (
                            window_info.physical_size().width as i32,
                            window_info.physical_size().height as i32,
                        ),
                        fb_info,
                        &mut self.gr_context,
                    );

                    self.dirty_surface = self
                        .surface
                        .new_surface_with_dimensions((
                            window_info.physical_size().width as i32,
                            window_info.physical_size().height as i32,
                        ))
                        .unwrap();

                    // // We keep track of the current size before applying the user scale factor while
                    // // baseview's logical size includes that factor so we need to compensate for it
                    // self.current_window_size = *self.cx.window_size();
                    // self.current_window_size.width = (window_info.logical_size().width
                    //     / self.cx.user_scale_factor())
                    // .round() as u32;
                    // self.current_window_size.height = (window_info.logical_size().height
                    //     / self.cx.user_scale_factor())
                    // .round() as u32;
                    // *self.cx.window_size() = self.current_window_size;

                    // Only use new DPI settings when `WindowScalePolicy::SystemScaleFactor` was
                    // used
                    if self.use_system_scaling {
                        self.window_scale_factor = window_info.scale();
                    }

                    //let user_scale_factor = self.cx.user_scale_factor();

                    //self.cx.set_scale_factor(self.window_scale_factor * user_scale_factor);

                    let physical_size =
                        (window_info.physical_size().width, window_info.physical_size().height);

                    self.cx.set_window_size(
                        Entity::root(),
                        physical_size.0 as f32,
                        physical_size.1 as f32,
                    );

                    self.cx.needs_refresh(Entity::root());
                }
                baseview::WindowEvent::WillClose => {
                    self.cx.send_event(Event::new(WindowEvent::WindowClose));
                }
                _ => {}
            },
        }
    }

    pub fn handle_idle(&mut self, on_idle: &Option<Box<dyn Fn(&mut Context) + Send>>) {
        if let Some(idle_callback) = on_idle {
            self.cx.set_current(Entity::root());
            (idle_callback)(self.cx.context());
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
            if event.code == vizia_input::Code::KeyQ
                && event.modifiers == vizia_input::KeyboardModifiers::META
                && event.state == vizia_input::KeyState::Down
            {
                return true;
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
