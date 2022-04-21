use glutin::{dpi::Position, window::WindowId};
use std::{cell::RefCell, collections::HashMap};
use winit::{
    dpi::LogicalSize,
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

        #[allow(unused_mut)]
        let mut context = Context::new();

        let event_loop = EventLoop::with_user_event();
        #[cfg(not(target_arch = "wasm32"))]
        {
            let event_proxy_obj = event_loop.create_proxy();
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
    ///     // Code here runs at the end of every event loop after OS and vizia events have been handled
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

    pub fn run(mut self) {
        let mut context = self.context;

        let event_loop = self.event_loop;

        let mut window = Window::new(&event_loop, &self.window_description);
        let root_window_id = window.id.unwrap();
        context.sub_windows.insert(Entity::root(), self.window_description.clone());

        let regular_font = fonts::ROBOTO_REGULAR;
        let bold_font = fonts::ROBOTO_BOLD;
        let icon_font = fonts::ENTYPO;
        let emoji_font = fonts::OPEN_SANS_EMOJI;
        let arabic_font = fonts::AMIRI_REGULAR;
        let material_font = fonts::MATERIAL_ICONS_REGULAR;

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
                        .as_mut()
                        .unwrap()
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

        let dpi_factor = window.window().unwrap().scale_factor();
        let size = window.window().unwrap().inner_size();

        let clear_color =
            context.style.background_color.get(Entity::root()).cloned().unwrap_or_default();

        window.canvas.as_mut().unwrap().set_size(
            size.width as u32,
            size.height as u32,
            dpi_factor as f32,
        );
        window.canvas.as_mut().unwrap().clear_rect(
            0,
            0,
            size.width as u32,
            size.height as u32,
            clear_color.into(),
        );

        context.style.dpi_factor = window.window().unwrap().scale_factor();

        context.views.insert(Entity::root(), Box::new(window));

        let logical_size: LogicalSize<f32> = physical_size.to_logical(dpi_factor);

        context.cache.set_width(Entity::root(), physical_size.width as f32);
        context.cache.set_height(Entity::root(), physical_size.height as f32);

        context.style.width.insert(Entity::root(), Units::Pixels(logical_size.width));
        context.style.height.insert(Entity::root(), Units::Pixels(logical_size.height));

        context.style.pseudo_classes.insert(Entity::root(), PseudoClass::default()).unwrap();
        context.style.disabled.insert(Entity::root(), false);

        let mut bounding_box = BoundingBox::default();
        bounding_box.w = physical_size.width as f32;
        bounding_box.h = physical_size.height as f32;

        context.cache.set_clip_region(Entity::root(), bounding_box);

        let mut event_manager = EventManager::new();

        // if let Some(builder) = self.builder.take() {
        //     (builder)(&mut context);

        //     self.builder = Some(builder);
        // }

        let builder = self.builder.take();

        let on_idle = self.on_idle.take();

        let event_loop_proxy = event_loop.create_proxy();

        let default_should_poll = self.should_poll;
        let stored_control_flow = RefCell::new(ControlFlow::Poll);

        // Multiwindow
        let mut windows: HashMap<WindowId, Entity> = HashMap::new();
        windows.insert(root_window_id, Entity::root());

        event_loop.run(move |event, event_loop_window_target, control_flow|{
            match event {

                winit::event::Event::UserEvent(event) => {
                    context.event_queue.push_back(event);
                }

                winit::event::Event::MainEventsCleared => {
                    *stored_control_flow.borrow_mut() = if default_should_poll {
                        ControlFlow::Poll
                    } else {
                        ControlFlow::Wait
                    };

                    // Rebuild application if required
                    if context.enviroment.needs_rebuild {
                        context.current = Entity::root();
                        if let Some(builder) = &builder {
                            (builder)(&mut context);
                        }
                        context.enviroment.needs_rebuild = false;
                    }

                    // Build any windows which were added
                    if context.sub_windows.len() != windows.len() {

                        for (win_entity, window_description) in context.sub_windows.iter_mut() {

                            if *win_entity == Entity::root() {
                                continue;
                            }

                            if let Some(win) = context.views.get_mut(&win_entity).and_then(|win_view| win_view.downcast_mut::<Window>()) {
                                let mut window = Window::new(event_loop_window_target, &window_description);
                                let window_id = window.id.unwrap();

                                context.tree.set_window(*win_entity, true);

                                context.style.position_type.insert(*win_entity, PositionType::SelfDirected);

                                let size = window.window().unwrap().inner_size();

                                let clear_color =
                                    context.style.background_color.get(*win_entity).cloned().unwrap_or_default();

                                window.canvas.as_mut().unwrap().set_size(size.width as u32, size.height as u32, dpi_factor as f32);
                                window.canvas.as_mut().unwrap().clear_rect(0, 0, size.width as u32, size.height as u32, clear_color.into());

                                context.cache.set_width(*win_entity, self.window_description.inner_size.width as f32);
                                context.cache.set_height(*win_entity, self.window_description.inner_size.height as f32);

                                context
                                    .style
                                    .width
                                    .insert(*win_entity, Units::Pixels(self.window_description.inner_size.width as f32));
                                context.style.height.insert(
                                    *win_entity,
                                    Units::Pixels(self.window_description.inner_size.height as f32),
                                );

                                context.style.pseudo_classes.insert(*win_entity, PseudoClass::default()).unwrap();
                                context.style.disabled.insert(*win_entity, false);

                                let mut bounding_box = BoundingBox::default();
                                bounding_box.w = size.width as f32;
                                bounding_box.h = size.height as f32;

                                context.cache.set_clip_region(*win_entity, bounding_box);


                                //context.views.insert(*win_entity, Box::new(window));
                                windows.insert(window_id, *win_entity);

                                *win = window;
                            }
                        }
                    }

                    for (_, window_entity) in windows.iter() {
                        if let Some(mut window_view) = context.views.remove(window_entity) {
                            if let Some(window) = window_view.downcast_mut::<Window>() {
                                // Load resources
                                for (name, font) in context.resource_manager.fonts.iter_mut() {
                                    match font {
                                        FontOrId::Font(data) => {
                                            let id1 = window.canvas.as_mut().unwrap().add_font_mem(&data.clone()).expect(&format!("Failed to load font file for: {}", name));
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
                            context.views.insert(*window_entity, window_view);
                        }
                    }


                    // Events
                    while !context.event_queue.is_empty() {
                        event_manager.flush_events(&mut context);
                    }

                    context.process_data_updates();
                    context.process_style_updates();

                    if context.has_animations() {

                        *stored_control_flow.borrow_mut() = ControlFlow::Poll;

                        //context.insert_event(Event::new(WindowEvent::Relayout).target(Entity::root()));
                        event_loop_proxy.send_event(Event::new(WindowEvent::Redraw)).unwrap();
                        //window.handle.window().request_redraw();
                        for (_, window_entity) in windows.iter() {
                            if let Some(window_event_handler) = context.views.remove(window_entity) {
                                if let Some(window) = window_event_handler.downcast_ref::<Window>() {
                                    window.window().unwrap().request_redraw();
                                }

                                context.views.insert(*window_entity, window_event_handler);
                            }
                        }
                    }

                    context.apply_animations();

                    context.process_visual_updates();

                    for (_, window_entity) in windows.iter() {
                        if let Some(window_view) = context.views.get(window_entity) {
                            if let Some(window) = window_view.downcast_ref::<Window>() {
                                if context.style.needs_redraw {
                                    window.window().unwrap().request_redraw();
                                    context.style.needs_redraw = false;
                                }
                            }
                        }
                    }

                    if let Some(idle_callback) = &on_idle {
                        context.current = Entity::root();
                        (idle_callback)(&mut context);
                    }

                    if !context.event_queue.is_empty() {
                        *stored_control_flow.borrow_mut() = ControlFlow::Poll;
                        event_loop_proxy.send_event(Event::new(())).expect("Failed to send event");
                    }
                }

                winit::event::Event::RedrawRequested(window_id) => {
                    if let Some(window_entity) = windows.get(&window_id) {

                        // Redraw here
                        context_draw(&mut context, *window_entity);
                    }
                }

                winit::event::Event::WindowEvent {
                    window_id,
                    event,
                } => {
                    match event {
                        winit::event::WindowEvent::CloseRequested => {
                            if let Some(window_entity) = windows.get(&window_id) {
                                if *window_entity == Entity::root() {
                                    *stored_control_flow.borrow_mut() = ControlFlow::Exit;
                                } else {

                                    if let Some(window) = context.views.get_mut(window_entity).and_then(|window_view| window_view.downcast_mut::<Window>()) {
                                        window.make_current();
                                    }

                                    context.remove(*window_entity);
                                    context.sub_windows.remove(window_entity);
                                    windows.remove(&window_id);
                                }
                            }

                            if windows.is_empty() {
                                *stored_control_flow.borrow_mut() = ControlFlow::Exit;
                            }
                        }

                        winit::event::WindowEvent::ScaleFactorChanged {
                            scale_factor,
                            new_inner_size,
                        } => {
                            context.style.dpi_factor = scale_factor;
                            context.cache.set_width(Entity::root(), new_inner_size.width as f32);
                            context.cache.set_height(Entity::root(), new_inner_size.height as f32);

                            let logical_size: LogicalSize<f32> = new_inner_size.to_logical(context.style.dpi_factor);

                            context
                                .style
                                .width
                                .insert(Entity::root(), Units::Pixels(logical_size.width as f32));

                            context
                                .style
                                .height
                                .insert(Entity::root(), Units::Pixels(logical_size.height as f32));
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
                                    WindowEvent::MouseScroll(pos.x as f32 / 20.0, pos.y as f32 / 114.0)
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

                        winit::event::WindowEvent::Resized(physical_size) => {
                            //println!("Resized: {:?}", size);

                            if let Some(mut window_view) = context.views.remove(&Entity::root()) {
                                if let Some(window) = window_view.downcast_mut::<Window>() {
                                    window.resize(physical_size);
                                }

                                context.views.insert(Entity::root(), window_view);
                            }

                            let logical_size: LogicalSize<f32> = physical_size.to_logical(context.style.dpi_factor);

                            context
                                .style
                                .width
                                .insert(Entity::root(), Units::Pixels(logical_size.width as f32));

                            context
                                .style
                                .height
                                .insert(Entity::root(), Units::Pixels(logical_size.height as f32));

                            context
                                .cache
                                .set_width(Entity::root(), physical_size.width as f32);
                            context
                                .cache
                                .set_height(Entity::root(), physical_size.height as f32);

                            let mut bounding_box = BoundingBox::default();
                            bounding_box.w = physical_size.width as f32;
                            bounding_box.h = physical_size.height as f32;

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

            *control_flow = *stored_control_flow.borrow();
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

fn context_draw(cx: &mut Context, window_entity: Entity) {
    if let Some(mut window_view) = cx.views.remove(&window_entity) {
        if let Some(window) = window_view.downcast_mut::<Window>() {
            let dpi_factor = window.window().unwrap().scale_factor();
            window.make_current();
            cx.draw(&mut window.canvas.as_mut().unwrap(), dpi_factor as f32, window_entity);
            window.swap_buffers();
        }

        cx.views.insert(window_entity, window_view);
    }
}
