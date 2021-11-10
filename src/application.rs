use std::{cell::RefCell, collections::{HashMap, VecDeque}, rc::Rc};

use femtovg::{Canvas, renderer::OpenGl};
use glutin::{ContextBuilder, event::{ElementState, VirtualKeyCode}, event_loop::{ControlFlow, EventLoop, EventLoopProxy}, window::WindowBuilder};
use morphorm::Units;

use crate::{CachedData, Color, Context, Data, Entity, Enviroment, Event, EventManager, FontOrId, IdManager, MouseButton, MouseButtonState, MouseState, Propagation, ResourceManager, Style, Tree, TreeExt, WindowEvent, apply_hover, apply_styles, scan_to_code, vcode_to_code, vk_to_key, Modifiers};

static DEFAULT_THEME: &str = include_str!("default_theme.css");

pub struct Application {
    context: Context,
    event_loop: EventLoop<()>,
    builder: Option<Box<dyn Fn(&mut Context)>>,
    on_idle: Option<Box<dyn Fn(&mut Context)>>,
}

impl Application {
    pub fn new<F>(builder: F) -> Self
    where F: 'static + Fn(&mut Context)
    {

        let mut cache = CachedData::default();
        cache.add(Entity::root()).expect("Failed to add entity to cache");

        let mut context = Context {
            entity_manager: IdManager::new(),
            tree: Tree::new(),
            current: Entity::root(),
            count: 0,
            views: HashMap::new(),
            state: HashMap::new(),  
            data: Data::new(),
            style: Rc::new(RefCell::new(Style::default())),
            cache,
            enviroment: Enviroment::new(),
            event_queue: VecDeque::new(),
            mouse: MouseState::default(),
            modifiers: Modifiers::empty(),
            captured: Entity::null(),
            hovered: Entity::root(),
            focused: Entity::root(),
            state_count: 0,
            resource_manager: ResourceManager::new(),
            fonts: Vec::new(),
        };

        context.entity_manager.create();

        context.add_theme(DEFAULT_THEME);

        Self {
            context,
            event_loop: EventLoop::new(),
            builder: Some(Box::new(builder)),
            on_idle: None,
        }
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
    pub fn on_idle<F: 'static + Fn(&mut Context)>(mut self, callback: F) -> Self {
        self.on_idle = Some(Box::new(callback));

        self
    } 

    // TODO - Rename this
    pub fn get_proxy(&self) -> EventLoopProxy<()> {
        self.event_loop.create_proxy()
    }

    pub fn background_color(self, color: Color) -> Self {
        self.context.style.borrow_mut().background_color.insert(Entity::root(), color);

        self
    }

    pub fn locale(mut self, id: &str) -> Self {
        self.context.enviroment.set_locale(id);


        self
    }

    pub fn run(mut self) {

        let mut context = self.context;
        
        let event_loop = self.event_loop;
        
        let handle = ContextBuilder::new()
            .build_windowed(WindowBuilder::new(), &event_loop)
            .expect("Failed to build windowed context");

        let handle = unsafe { handle.make_current().unwrap() };

        let renderer = OpenGl::new(|s| handle.context().get_proc_address(s) as *const _)
            .expect("Cannot create renderer");
        let mut canvas = Canvas::new(renderer).expect("Cannot create canvas");

        // let font = canvas.add_font_mem(FONT).expect("Failed to load font");

        // context.fonts = vec![font];

        let regular_font = include_bytes!("../fonts/Roboto-Regular.ttf");
        let bold_font = include_bytes!("../fonts/Roboto-Bold.ttf");
        let icon_font = include_bytes!("../fonts/entypo.ttf");
        let emoji_font = include_bytes!("../fonts/OpenSansEmoji.ttf");
        let arabic_font = include_bytes!("../fonts/amiri-regular.ttf");

        context.add_font_mem("roboto", regular_font);
        context.add_font_mem("roboto-bold", bold_font);
        context.add_font_mem("icon", icon_font);
        context.add_font_mem("emoji", emoji_font);
        context.add_font_mem("arabic", arabic_font);

        context.style.borrow_mut().default_font = "roboto".to_string();

        let dpi_factor = handle.window().scale_factor();
        let size = handle.window().inner_size();

        let clear_color = context.style.borrow_mut().background_color.get(Entity::root()).cloned().unwrap_or_default();

        canvas.set_size(size.width as u32, size.height as u32, dpi_factor as f32);
        canvas.clear_rect(
            0,
            0,
            size.width as u32,
            size.height as u32,
            clear_color.into(),
        );

        context
            .cache
            .set_width(Entity::root(), 800.0);
        context
            .cache
            .set_height(Entity::root(), 600.0);

        context.style.borrow_mut().width.insert(Entity::root(), Units::Pixels(800.0));
        context.style.borrow_mut().height.insert(Entity::root(), Units::Pixels(600.0));

        let mut event_manager = EventManager::new();

        // if let Some(builder) = self.builder.take() {
        //     (builder)(&mut context);

        //     self.builder = Some(builder);
        // }

        let builder = self.builder.take();

        let on_idle = self.on_idle.take();

        let event_loop_proxy = event_loop.create_proxy();

        event_loop.run(move |event, _, control_flow|{
            *control_flow = ControlFlow::Wait;

            match event {
                glutin::event::Event::MainEventsCleared => {

                    if context.enviroment.needs_rebuild {
                        context.current = Entity::root();
                        context.count = 0;
                        if let Some(builder) = &builder {
                            (builder)(&mut context);
                        }
                        context.enviroment.needs_rebuild = false;
                    }

                    // Load resources
                    for (name, font) in context.resource_manager.fonts.iter_mut() {
            
                        match font {
                            FontOrId::Font(data) => {
                                let id1 = canvas.add_font_mem(&data.clone()).expect(&format!("Failed to load font file for: {}", name));
                                //let id2 = context.text_context.add_font_mem(&data.clone()).expect("failed");
                                // if id1 != id2 {
                                //     panic!("Fonts in canvas must have the same id as fonts in the text context");
                                // }
                                *font = FontOrId::Id(id1);
                            }
            
                            _=> {}
                        }
                    }

                    // Events
                    while !context.event_queue.is_empty() {
                        event_manager.flush_events(&mut context);
                    }

                    // Updates
                    for entity in context.tree.clone().into_iter() {
                        let mut observers = Vec::new();
                     
                        if let Some(model_list) = context.data.model_data.get(entity) {
                            for (_, model) in model_list.iter() {
                                //observers = model.update();
                                if model.is_dirty() {
                                    observers.extend(model.update().iter());
                                }
                            }
                        }

                        for observer in observers.iter() {
                            if let Some(mut view) = context.views.remove(observer) {
                                let prev = context.current;
                                context.current = *observer;
                                let prev_count = context.count;
                                context.count = 0;
                                view.body(&mut context);
                                context.current = prev;
                                context.count = prev_count;
                    
                
                                context.views.insert(*observer, view);
                            }
                        }

                        if let Some(model_list) = context.data.model_data.get_mut(entity) {
                            for (_, model) in model_list.iter_mut() {
                                model.reset();
                            }
                        }
                        
                    }

                    // Not ideal
                    let tree = context.tree.clone();

                    // Styling (TODO)
                    apply_styles(&mut context, &tree);

                    // Layout
                    morphorm::layout(&mut context.cache, &context.tree, &context.style.borrow());

                    apply_hover(&mut context);

                    handle.window().request_redraw();

                    if let Some(idle_callback) = &on_idle {
                        context.current = Entity::root();
                        context.count = 0;
                        (idle_callback)(&mut context);

                        if !context.event_queue.is_empty() {
                            event_loop_proxy.send_event(()).unwrap();
                        }
                    }
                }

                glutin::event::Event::RedrawRequested(_) => {
                    // Redraw here
                    // println!("Redraw");

                    let window_width = context.cache.get_width(Entity::root());
                    let window_height = context.cache.get_height(Entity::root());

                    canvas.set_size(window_width as u32, window_height as u32, dpi_factor as f32);
                    let clear_color = context.style.borrow_mut().background_color.get(Entity::root()).cloned().unwrap_or(Color::white());
                    canvas.clear_rect(
                        0,
                        0,
                        window_width as u32,
                        window_height as u32,
                        clear_color.into(),
                    );
                    for entity in context.tree.clone().into_iter() {
                        if let Some(view) = context.views.remove(&entity) {

                            context.current = entity;
                            view.draw(&context, &mut canvas);
                            
                            context.views.insert(entity, view);
                        }
                    }

                    canvas.flush();
                    handle.swap_buffers().expect("Failed to swap buffers");
                }

                glutin::event::Event::WindowEvent {
                    window_id: _,
                    event,
                } => {
                    match event {
                        glutin::event::WindowEvent::CloseRequested => {
                            *control_flow = ControlFlow::Exit;
                        }

                        glutin::event::WindowEvent::CursorMoved {
                            device_id: _,
                            position,
                            modifiers: _
                        } => {

                            context.mouse.cursorx = position.x as f32;
                            context.mouse.cursory = position.y as f32;

                            apply_hover(&mut context);

                            if context.captured != Entity::null() {
                                context.event_queue.push_back(
                                    Event::new(WindowEvent::MouseMove(context.mouse.cursorx, context.mouse.cursory))
                                        .target(context.captured)
                                        .propagate(Propagation::Direct),
                                );
                            } else if context.hovered != Entity::root() {
                                context.event_queue.push_back(
                                    Event::new(WindowEvent::MouseMove(context.mouse.cursorx, context.mouse.cursory))
                                        .target(context.hovered),
                                );
                            }
                        }

                        glutin::event::WindowEvent::MouseInput {
                            device_id: _,
                            button,
                            state,
                            modifiers: _,
                        } => {
                            let button = match button {
                                glutin::event::MouseButton::Left => MouseButton::Left,
                                glutin::event::MouseButton::Right => MouseButton::Right,
                                glutin::event::MouseButton::Middle => MouseButton::Middle,
                                glutin::event::MouseButton::Other(val) => MouseButton::Other(val),
                            };

                            let state = match state {
                                glutin::event::ElementState::Pressed => MouseButtonState::Pressed,
                                glutin::event::ElementState::Released => MouseButtonState::Released,
                            };

                            match state {
                                MouseButtonState::Pressed => {
                                    //context.event_queue.push_back(Event::new(WindowEvent::MouseDown(button)).target(context.hovered).propagate(Propagation::Up));
                                
                                    if context.captured != Entity::null() {
                                        context.event_queue.push_back(
                                            Event::new(WindowEvent::MouseDown(button))
                                                .target(context.captured)
                                                .propagate(Propagation::Direct),
                                        );
                                    } else {
                                        context.event_queue.push_back(
                                            Event::new(WindowEvent::MouseDown(button))
                                                .target(context.hovered),
                                        );
                                    }
                                }

                                MouseButtonState::Released => {
                                    //context.event_queue.push_back(Event::new(WindowEvent::MouseUp(button)).target(context.hovered).propagate(Propagation::Up));
                                
                                    if context.captured != Entity::null() {
                                        context.event_queue.push_back(
                                            Event::new(WindowEvent::MouseUp(button))
                                                .target(context.captured)
                                                .propagate(Propagation::Direct),
                                        );
                                    } else {
                                        context.event_queue.push_back(
                                            Event::new(WindowEvent::MouseUp(button))
                                                .target(context.hovered),
                                        );
                                    }
                                }
                            }
                        }

                        glutin::event::WindowEvent::KeyboardInput {
                            device_id: _,
                            input,
                            is_synthetic: _,
                        } => {
                            if input.virtual_keycode == Some(VirtualKeyCode::H) && input.state == ElementState::Pressed {
                                println!("Tree");
                                for entity in context.tree.into_iter() {
                                    println!("Entity: {} Parent: {:?} posx: {} posy: {} width: {} height: {}", entity, entity.parent(&context.tree), context.cache.get_posx(entity), context.cache.get_posy(entity), context.cache.get_width(entity), context.cache.get_height(entity));
                                }
                            }

                            
                            if input.virtual_keycode == Some(VirtualKeyCode::F5) && input.state == ElementState::Pressed {
                                context.reload_styles().unwrap();
                            }

                            let s = match input.state {
                                glutin::event::ElementState::Pressed => MouseButtonState::Pressed,
                                glutin::event::ElementState::Released => MouseButtonState::Released,
                            };

	                        // Prefer virtual keycodes to scancodes, as scancodes aren't uniform between platforms
	                        let code = if let Some(vkey) = input.virtual_keycode {
		                        vcode_to_code(vkey)
	                        } else {
		                        scan_to_code(input.scancode)
	                        };

                            let key = vk_to_key(
                                input.virtual_keycode.unwrap_or(VirtualKeyCode::NoConvert),
                            );

                            match s {
                                MouseButtonState::Pressed => {
                                    if context.focused != Entity::null() {
                                        context.event_queue.push_back(
                                            Event::new(WindowEvent::KeyDown(code, key))
                                                .target(context.focused)
                                                .propagate(Propagation::Up),
                                        );
                                    } else {
                                        context.event_queue.push_back(
                                            Event::new(WindowEvent::KeyDown(code, key))
                                                .target(context.hovered)
                                                .propagate(Propagation::Up),
                                        );
                                    }
                                }

                                MouseButtonState::Released => {
                                    if context.focused != Entity::null() {
                                        context.event_queue.push_back(
                                            Event::new(WindowEvent::KeyUp(code, key))
                                                .target(context.focused)
                                                .propagate(Propagation::Up),
                                        );
                                    } else {
                                        context.event_queue.push_back(
                                            Event::new(WindowEvent::KeyUp(code, key))
                                                .target(context.hovered)
                                                .propagate(Propagation::Up),
                                        );
                                    }
                                }
                            }
                        }

                        glutin::event::WindowEvent::ReceivedCharacter(character) => {
                            context.event_queue.push_back(
                                Event::new(WindowEvent::CharInput(character))
                                    .target(context.focused)
                                    .propagate(Propagation::Up),
                            );
                        }

                        glutin::event::WindowEvent::Resized(size) => {
                            //println!("Resized: {:?}", size);
                            handle.resize(size);

                            context
                                .style
                                .borrow_mut()
                                .width
                                .insert(Entity::root(), Units::Pixels(size.width as f32));

                            context
                                .style
                                .borrow_mut()
                                .height
                                .insert(Entity::root(), Units::Pixels(size.height as f32));

                            context
                                .cache
                                .set_width(Entity::root(), size.width as f32);
                            context
                                .cache
                                .set_height(Entity::root(), size.height as f32);

                            // let mut bounding_box = BoundingBox::default();
                            // bounding_box.w = size.width as f32;
                            // bounding_box.h = size.height as f32;

                            // context.cache.set_clip_region(Entity::root(), bounding_box);
                        }

                        glutin::event::WindowEvent::ModifiersChanged(modifiers_state) => {
                            
                            
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

// fn debug(cx: &mut Context, entity: Entity) -> String {
//     if let Some(view) = cx.views.get(&entity) {
//         view.debug(entity)
//     } else {
//         "None".to_string()
//     }
// }