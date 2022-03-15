use winit::{
    event::VirtualKeyCode,
    event_loop::{ControlFlow, EventLoop, EventLoopProxy},
};

use vizia_core::*;

use crate::keyboard::{scan_to_code, vcode_to_code, vk_to_key};
use crate::window::Window;

pub struct Application {
    context: Context,
    event_loop: EventLoop<Event>,
    builder: Option<Box<dyn Fn(&mut Context)>>,
    on_idle: Option<Box<dyn Fn(&mut Context)>>,
    window_description: WindowDescription,
    should_poll: bool,
}

// TODO uhhhhhhhhhhhhhhhhhhhhhh I think it's a winit bug that EventLoopProxy isn't Send on web
#[cfg(not(target_arch = "wasm32"))]
pub struct WinitEventProxy(EventLoopProxy<Event>);

#[cfg(not(target_arch = "wasm32"))]
impl EventProxy for WinitEventProxy {
    fn send(&self, event: Event) -> Result<(), ()> {
        self.0.send_event(event).map_err(|_| ())
    }

    fn make_clone(&self) -> Box<dyn EventProxy> {
        Box::new(WinitEventProxy(self.0.clone()))
    }
}

impl Application {
    pub fn new<F>(window_description: WindowDescription, builder: F) -> Self
    where
        F: 'static + Fn(&mut Context),
    {
        // wasm + debug: send panics to console
        #[cfg(all(debug_assertions, target_arch = "wasm32"))]
        console_error_panic_hook::set_once();

        let mut context = Context::new();

        let event_loop = EventLoop::with_user_event();
        let event_proxy_obj = event_loop.create_proxy();
        #[cfg(not(target_arch = "wasm32"))]
        {
            context.event_proxy = Some(Box::new(WinitEventProxy(event_proxy_obj)));
        }

        Self {
            context,
            event_loop,
            builder: Some(Box::new(builder)),
            on_idle: None,
            window_description,
            should_poll: false,
        }
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
    /// ```no_run
    /// # use vizia_core::*;
    /// # use vizia_winit::application::Application;
    /// Application::new(WindowDescription::new(), |cx|{
    ///     // Build application here
    /// })
    /// .on_idle(|cx|{
    ///     // Code here runs at the end of every event loop after OS and tuix events have been handled
    /// })
    /// .run();
    /// ```
    pub fn on_idle<F: 'static + Fn(&mut Context)>(mut self, callback: F) -> Self {
        self.on_idle = Some(Box::new(callback));

        self
    }

    // TODO - Rename this
    pub fn get_proxy(&self) -> EventLoopProxy<Event> {
        self.event_loop.create_proxy()
    }

    pub fn background_color(mut self, color: Color) -> Self {
        self.context.style.background_color.insert(Entity::root(), color);

        self
    }

    pub fn locale(mut self, id: &str) -> Self {
        self.context.enviroment.set_locale(id);

        self
    }

    pub fn run(mut self) {
        let mut context = self.context;

        let event_loop = self.event_loop;

        // let handle = ContextBuilder::new()
        //     .with_vsync(true)
        //     .build_windowed(WindowBuilder::new(), &event_loop)
        //     .expect("Failed to build windowed context");

        // let handle = unsafe { handle.make_current().unwrap() };

        // let renderer = OpenGl::new(|s| handle.context().get_proc_address(s) as *const _)
        //     .expect("Cannot create renderer");
        // let mut canvas = Canvas::new(renderer).expect("Cannot create canvas");

        let mut window = Window::new(&event_loop, &self.window_description);

        // let font = canvas.add_font_mem(FONT).expect("Failed to load font");

        // context.fonts = vec![font];

        let regular_font = include_bytes!("../../fonts/Roboto-Regular.ttf");
        let bold_font = include_bytes!("../../fonts/Roboto-Bold.ttf");
        let icon_font = include_bytes!("../../fonts/entypo.ttf");
        let emoji_font = include_bytes!("../../fonts/OpenSansEmoji.ttf");
        let arabic_font = include_bytes!("../../fonts/amiri-regular.ttf");
        let material_font = include_bytes!("../../fonts/MaterialIcons-Regular.ttf");

        context.add_font_mem("roboto", regular_font);
        context.add_font_mem("roboto-bold", bold_font);
        context.add_font_mem("icons", icon_font);
        context.add_font_mem("emoji", emoji_font);
        context.add_font_mem("arabic", arabic_font);
        context.add_font_mem("material", material_font);

        context.style.default_font = "roboto".to_string();

        // Load resources
        for (name, font) in context.resource_manager.fonts.iter_mut() {
            match font {
                FontOrId::Font(data) => {
                    let id1 = window
                        .canvas
                        .add_font_mem(&data.clone())
                        .expect(&format!("Failed to load font file for: {}", name));
                    let id2 = context.text_context.add_font_mem(&data.clone()).expect("failed");
                    if id1 != id2 {
                        panic!(
                            "Fonts in canvas must have the same id as fonts in the text context"
                        );
                    }
                    *font = FontOrId::Id(id1);
                }

                _ => {}
            }
        }

        let dpi_factor = window.window().scale_factor();
        let size = window.window().inner_size();

        let clear_color =
            context.style.background_color.get(Entity::root()).cloned().unwrap_or_default();

        window.canvas.set_size(size.width as u32, size.height as u32, dpi_factor as f32);
        window.canvas.clear_rect(0, 0, size.width as u32, size.height as u32, clear_color.into());

        context.views.insert(Entity::root(), Box::new(window));

        context.cache.set_width(Entity::root(), self.window_description.inner_size.width as f32);
        context.cache.set_height(Entity::root(), self.window_description.inner_size.height as f32);

        context
            .style
            .width
            .insert(Entity::root(), Units::Pixels(self.window_description.inner_size.width as f32));
        context.style.height.insert(
            Entity::root(),
            Units::Pixels(self.window_description.inner_size.height as f32),
        );

        context.style.pseudo_classes.insert(Entity::root(), PseudoClass::default()).unwrap();
        context.style.disabled.insert(Entity::root(), false);

        let mut bounding_box = BoundingBox::default();
        bounding_box.w = size.width as f32;
        bounding_box.h = size.height as f32;

        context.cache.set_clip_region(Entity::root(), bounding_box);

        let mut event_manager = EventManager::new();

        // if let Some(builder) = self.builder.take() {
        //     (builder)(&mut context);

        //     self.builder = Some(builder);
        // }

        let builder = self.builder.take();

        let on_idle = self.on_idle.take();

        let event_loop_proxy = event_loop.create_proxy();

        let should_poll = self.should_poll;

        event_loop.run(move |event, _, control_flow|{

            if should_poll {
                *control_flow = ControlFlow::Poll;
            } else {
                *control_flow = ControlFlow::Wait;
            }

            match event {

                winit::event::Event::UserEvent(event) => {
                    context.event_queue.push_back(event);
                }

                winit::event::Event::MainEventsCleared => {

                    // Rebuild application if required
                    if context.enviroment.needs_rebuild {
                        context.current = Entity::root();
                        context.count = 0;
                        if let Some(builder) = &builder {
                            (builder)(&mut context);
                        }
                        context.enviroment.needs_rebuild = false;
                    }

                    if let Some(mut window_view) = context.views.remove(&Entity::root()) {
                        if let Some(window) = window_view.downcast_mut::<Window>() {

                            // Load resources
                            for (name, font) in context.resource_manager.fonts.iter_mut() {
                                match font {
                                    FontOrId::Font(data) => {
                                        let id1 = window.canvas.add_font_mem(&data.clone()).expect(&format!("Failed to load font file for: {}", name));
                                        let id2 = context.text_context.add_font_mem(&data.clone()).expect("failed");
                                        if id1 != id2 {
                                            panic!("Fonts in canvas must have the same id as fonts in the text context");
                                        }
                                        *font = FontOrId::Id(id1);
                                    }

                                    _=> {}
                                }
                            }

                        }

                        context.views.insert(Entity::root(), window_view);
                    }

                    // Events
                    while !context.event_queue.is_empty() {
                        event_manager.flush_events(&mut context);
                    }

                    context.process_data_updates();
                    context.process_style_updates();

                    if context.has_animations() {

                        *control_flow = ControlFlow::Poll;

                        //context.insert_event(Event::new(WindowEvent::Relayout).target(Entity::root()));
                        event_loop_proxy.send_event(Event::new(WindowEvent::Redraw)).unwrap();
                        //window.handle.window().request_redraw();
                        if let Some(window_event_handler) = context.views.remove(&Entity::root()) {
                            if let Some(window) = window_event_handler.downcast_ref::<Window>() {
                                window.window().request_redraw();
                            }

                            context.views.insert(Entity::root(), window_event_handler);
                        }
                    } else {
                        if should_poll {
                            *control_flow = ControlFlow::Poll;
                        } else {
                            *control_flow = ControlFlow::Wait;
                        }
                    }

                    context.apply_animations();

                    context.process_visual_updates();

                    if let Some(window_view) = context.views.get(&Entity::root()) {
                        if let Some(window) = window_view.downcast_ref::<Window>() {
                            if context.style.needs_redraw {
                                window.window().request_redraw();
                                context.style.needs_redraw = false;
                            }
                        }
                    }

                    if let Some(idle_callback) = &on_idle {
                        context.current = Entity::root();
                        context.count = 0;
                        (idle_callback)(&mut context);
                    }

                    if !context.event_queue.is_empty() {
                        event_loop_proxy.send_event(Event::new(())).expect("Failed to send event");
                    }
                }

                winit::event::Event::RedrawRequested(_) => {
                    // Redraw here
                    context_draw(&mut context);
                }

                winit::event::Event::WindowEvent {
                    window_id: _,
                    event,
                } => {
                    match event {
                        winit::event::WindowEvent::CloseRequested => {
                            *control_flow = ControlFlow::Exit;
                        }

                        #[allow(deprecated)]
                        winit::event::WindowEvent::CursorMoved {
                            device_id: _,
                            position,
                            modifiers: _
                        } => {
                            context.dispatch_system_event(
                                WindowEvent::MouseMove(position.x as f32, position.y as f32)
                            );

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
                                winit::event::ElementState::Pressed => WindowEvent::MouseDown(button),
                                winit::event::ElementState::Released => WindowEvent::MouseUp(button),
                            };

                            context.dispatch_system_event(event);
                        }

                        winit::event::WindowEvent::MouseWheel {
                            delta, phase: _, ..
                        } => {
                            let out_event = match delta {
                                winit::event::MouseScrollDelta::LineDelta(x, y) => {
                                    WindowEvent::MouseScroll(x, y)
                                }
                                winit::event::MouseScrollDelta::PixelDelta(pos) => {
                                    WindowEvent::MouseScroll(pos.x as f32, pos.y as f32)
                                }
                            };

                            context.dispatch_system_event(out_event);
                        }

                        winit::event::WindowEvent::KeyboardInput {
                            device_id: _,
                            input,
                            is_synthetic: _,
                        } => {
                            // Prefer virtual keycodes to scancodes, as scancodes aren't uniform between platforms
                            let code = if let Some(vkey) = input.virtual_keycode {
                                vcode_to_code(vkey)
                            } else {
                                scan_to_code(input.scancode)
                            };

                            let key = vk_to_key(
                                input.virtual_keycode.unwrap_or(VirtualKeyCode::NoConvert),
                            );
                            let event = match input.state {
                                winit::event::ElementState::Pressed => WindowEvent::KeyDown(code, key),
                                winit::event::ElementState::Released => WindowEvent::KeyUp(code, key),
                            };

                            context.dispatch_system_event(event);
                        }

                        winit::event::WindowEvent::ReceivedCharacter(character) => {
                            context.dispatch_system_event(WindowEvent::CharInput(character));
                        }

                        winit::event::WindowEvent::Resized(size) => {
                            //println!("Resized: {:?}", size);

                            if let Some(mut window_view) = context.views.remove(&Entity::root()) {
                                if let Some(window) = window_view.downcast_mut::<Window>() {
                                    window.resize(size);
                                }

                                context.views.insert(Entity::root(), window_view);
                            }

                            context
                                .style
                                .width
                                .insert(Entity::root(), Units::Pixels(size.width as f32));

                            context
                                .style
                                .height
                                .insert(Entity::root(), Units::Pixels(size.height as f32));

                            context
                                .cache
                                .set_width(Entity::root(), size.width as f32);
                            context
                                .cache
                                .set_height(Entity::root(), size.height as f32);

                            let mut bounding_box = BoundingBox::default();
                            bounding_box.w = size.width as f32;
                            bounding_box.h = size.height as f32;

                            context.cache.set_clip_region(Entity::root(), bounding_box);

                            context.style.needs_restyle = true;
                            context.style.needs_relayout = true;
                            context.style.needs_redraw = true;

                            // let mut bounding_box = BoundingBox::default();
                            // bounding_box.w = size.width as f32;
                            // bounding_box.h = size.height as f32;

                            // context.cache.set_clip_region(Entity::root(), bounding_box);
                        }

                        winit::event::WindowEvent::ModifiersChanged(modifiers_state) => {
                            context.modifiers.set(Modifiers::SHIFT, modifiers_state.shift());
                            context.modifiers.set(Modifiers::ALT, modifiers_state.alt());
                            context.modifiers.set(Modifiers::CTRL, modifiers_state.ctrl());
                            context.modifiers.set(Modifiers::LOGO, modifiers_state.logo());
                        }

                        _=> {}
                    }
                }

                _=> {}
            }
        });
    }
}

impl Env for Application {
    fn ignore_default_styles(mut self) -> Self {
        if self.context.enviroment.include_default_theme {
            self.context.enviroment.include_default_theme = false;
            self.context.enviroment.needs_rebuild = true;
            self.context.reload_styles().expect("Failed to reload styles");
        }

        self
    }
}

// fn debug(cx: &mut Context, entity: Entity) -> String {
//     if let Some(view) = cx.views.get(&entity) {
//         view.debug(entity)
//     } else {
//         "None".to_string()
//     }
// }

fn context_draw(cx: &mut Context) {
    if let Some(mut window_view) = cx.views.remove(&Entity::root()) {
        if let Some(window) = window_view.downcast_mut::<Window>() {
            let dpi_factor = window.window().scale_factor();
            cx.draw(&mut window.canvas, dpi_factor as f32);
            window.swap_buffers();
        }

        cx.views.insert(Entity::root(), window_view);
    }
}
