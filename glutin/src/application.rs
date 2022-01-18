use glutin::{
    event::{ElementState, VirtualKeyCode},
    event_loop::{ControlFlow, EventLoop, EventLoopProxy},
};

use vizia_core::*;

use crate::keyboard::{scan_to_code, vcode_to_code, vk_to_key};
use crate::window::Window;

static DEFAULT_THEME: &str = include_str!("../../core/src/default_theme.css");

pub struct Application {
    context: Context,
    event_loop: EventLoop<Event>,
    builder: Option<Box<dyn Fn(&mut Context)>>,
    on_idle: Option<Box<dyn Fn(&mut Context)>>,
    window_description: WindowDescription,
    should_poll: bool,
}

impl Application {
    pub fn new<F>(window_description: WindowDescription, builder: F) -> Self
    where
        F: 'static + Fn(&mut Context),
    {
        let mut context = Context::new();

        context.entity_manager.create();

        context.add_theme(DEFAULT_THEME);

        Self {
            context,
            event_loop: EventLoop::with_user_event(),
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
    /// # use vizia_glutin::application::Application;
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

        context.add_font_mem("roboto", regular_font);
        context.add_font_mem("roboto-bold", bold_font);
        context.add_font_mem("icon", icon_font);
        context.add_font_mem("emoji", emoji_font);
        context.add_font_mem("arabic", arabic_font);

        context.style.default_font = "roboto".to_string();

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

        let dpi_factor = window.handle.window().scale_factor();
        let size = window.handle.window().inner_size();

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

        let mut click_time = std::time::Instant::now();
        let double_click_interval = std::time::Duration::from_millis(500);
        let mut double_click = false;
        let mut click_pos = (0.0, 0.0);

        event_loop.run(move |event, _, control_flow|{

            if should_poll {
                *control_flow = ControlFlow::Poll;
            } else {
                *control_flow = ControlFlow::Wait;
            }

            match event {

                glutin::event::Event::UserEvent(event) => {
                    context.event_queue.push_back(event);
                }

                glutin::event::Event::MainEventsCleared => {

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

                    // Data Updates
                    let mut observers: Vec<Entity> = Vec::new();

                    for model_store in context.data.dense.iter_mut().map(|entry| &mut entry.value) {
                        for (_, lens) in model_store.lenses.iter_mut() {
                            for (_, model) in model_store.data.iter() {
                                if lens.update(model) {
                                    observers.extend(lens.observers().iter());
                                }
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

                    // Not ideal
                    let tree = context.tree.clone();

                    apply_inline_inheritance(&mut context, &tree);

                    apply_styles(&mut context, &tree);

                    apply_shared_inheritance(&mut context, &tree);

                    apply_z_ordering(&mut context, &tree);

                    apply_visibility(&mut context, &tree);

                    apply_text_constraints(&mut context, &tree);

                    // Layout
                    if context.style.needs_relayout {
                        vizia_core::apply_layout(&mut context.cache, &context.tree, &context.style);
                        context.style.needs_relayout = false;
                    }

                    // Emit any geometry changed events
                    geometry_changed(&mut context, &tree);

                    if !context.event_queue.is_empty() {
                        event_loop_proxy.send_event(Event::new(())).expect("Failed to send event");
                    }

                    apply_transform(&mut context, &tree);

                    apply_hover(&mut context);

                    apply_clipping(&mut context, &tree);

                    if let Some(window_view) = context.views.get(&Entity::root()) {
                        if let Some(window) = window_view.downcast_ref::<Window>() {
                            if context.style.needs_redraw {
                                window.handle.window().request_redraw();
                                context.style.needs_redraw = false;
                            }
                        }
                    }



                    if let Some(idle_callback) = &on_idle {
                        context.current = Entity::root();
                        context.count = 0;
                        (idle_callback)(&mut context);

                        if !context.event_queue.is_empty() {
                            event_loop_proxy.send_event(Event::new(())).unwrap();
                        }
                    }
                }

                glutin::event::Event::RedrawRequested(_) => {
                    // Redraw here
                    //println!("Redraw");

                    if let Some(mut window_view) = context.views.remove(&Entity::root()) {
                        if let Some(window) = window_view.downcast_mut::<Window>() {

                            let window_width = context.cache.get_width(Entity::root());
                            let window_height = context.cache.get_height(Entity::root());

                            window.canvas.set_size(window_width as u32, window_height as u32, dpi_factor as f32);
                            let clear_color = context.style.background_color.get(Entity::root()).cloned().unwrap_or(Color::white());
                            window.canvas.clear_rect(
                                0,
                                0,
                                window_width as u32,
                                window_height as u32,
                                clear_color.into(),
                            );

                            // Sort the tree by z order
                            let mut draw_tree: Vec<Entity> = context.tree.into_iter().collect();
                            draw_tree.sort_by_cached_key(|entity| context.cache.get_z_index(*entity));

                            for entity in draw_tree.into_iter() {


                                // Skip window
                                if entity == Entity::root() {
                                    continue;
                                }

                                // Skip invisible widgets
                                if context.cache.get_visibility(entity) == Visibility::Invisible {
                                    continue;
                                }

                                if context.cache.get_display(entity) == Display::None {
                                    continue;
                                }

                                // Skip widgets that have 0 opacity
                                if context.cache.get_opacity(entity) == 0.0 {
                                    continue;
                                }

                                // Apply clipping
                                let clip_region = context.cache.get_clip_region(entity);
                                window.canvas.scissor(
                                    clip_region.x,
                                    clip_region.y,
                                    clip_region.w,
                                    clip_region.h,
                                );

                                // Apply transform
                                let transform = context.cache.get_transform(entity);
                                window.canvas.save();
                                window.canvas.set_transform(transform[0], transform[1], transform[2], transform[3], transform[4], transform[5]);

                                if let Some(view) = context.views.remove(&entity) {

                                    context.current = entity;
                                    view.draw(&mut context, &mut window.canvas);

                                    context.views.insert(entity, view);
                                }

                                window.canvas.restore();
                            }

                            window.canvas.flush();
                            window.handle.swap_buffers().expect("Failed to swap buffers");
                        }

                        context.views.insert(Entity::root(), window_view);
                    }
                }

                glutin::event::Event::WindowEvent {
                    window_id: _,
                    event,
                } => {
                    match event {
                        glutin::event::WindowEvent::CloseRequested => {
                            *control_flow = ControlFlow::Exit;
                        }

                        #[allow(deprecated)]
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

                        #[allow(deprecated)]
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

                            match button {
                                MouseButton::Left => {
                                    context.mouse.left.state = state;
                                }
                                MouseButton::Right => {
                                    context.mouse.right.state = state;
                                }
                                MouseButton::Middle => {
                                    context.mouse.middle.state = state;
                                }
                                _=> {}
                            }

                            match state {
                                MouseButtonState::Pressed => {
                                    //context.event_queue.push_back(Event::new(WindowEvent::MouseDown(button)).target(context.hovered).propagate(Propagation::Up));

                                    let new_click_time = std::time::Instant::now();
                                    let click_duration = new_click_time - click_time;
                                    let new_click_pos = (context.mouse.cursorx, context.mouse.cursory);

                                    if click_duration <= double_click_interval && new_click_pos == click_pos{
                                        if !double_click {
                                            let _target = if context.captured != Entity::null() {
                                                context.event_queue.push_back(
                                                    Event::new(WindowEvent::MouseDoubleClick(button))
                                                        .target(context.captured)
                                                        .propagate(Propagation::Direct),
                                                );
                                                context.captured
                                            } else {
                                                context.event_queue.push_back(
                                                    Event::new(WindowEvent::MouseDoubleClick(button))
                                                        .target(context.hovered),
                                                );
                                                context.hovered
                                            };
                                            double_click = true;
                                        }
                                    } else {
                                        double_click = false;
                                    }

                                    click_time = new_click_time;
                                    click_pos = new_click_pos;

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

                                    match button {
                                        MouseButton::Left => {
                                            context.mouse.left.pos_down =
                                                (context.mouse.cursorx, context.mouse.cursory);
                                            context.mouse.left.pressed = context.hovered;
                                        }

                                        MouseButton::Right => {
                                            context.mouse.right.pos_down =
                                                (context.mouse.cursorx, context.mouse.cursory);
                                                context.mouse.right.pressed = context.hovered;
                                        }

                                        MouseButton::Middle => {
                                            context.mouse.middle.pos_down =
                                                (context.mouse.cursorx, context.mouse.cursory);
                                                context.mouse.middle.pressed = context.hovered;
                                        }

                                        _=> {}
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

                        glutin::event::WindowEvent::MouseWheel {
                            delta, phase: _, ..
                        } => {
                            let out_event = match delta {
                                glutin::event::MouseScrollDelta::LineDelta(x, y) => {
                                    WindowEvent::MouseScroll(x, y)
                                }
                                glutin::event::MouseScrollDelta::PixelDelta(pos) => {
                                    WindowEvent::MouseScroll(pos.x as f32, pos.y as f32)
                                }
                            };
                            context.event_queue.push_back(
                                Event::new(out_event)
                                    .target(context.hovered)
                                    .propagate(Propagation::Up),
                            );
                        }

                        glutin::event::WindowEvent::KeyboardInput {
                            device_id: _,
                            input,
                            is_synthetic: _,
                        } => {
                            if input.virtual_keycode == Some(VirtualKeyCode::H) && input.state == ElementState::Pressed {
                                println!("Tree");
                                for entity in context.tree.into_iter() {
                                    println!("Entity: {} Parent: {:?} posx: {} posy: {} width: {} height: {} scissor: {:?}", entity, entity.parent(&context.tree), context.cache.get_posx(entity), context.cache.get_posy(entity), context.cache.get_width(entity), context.cache.get_height(entity), context.cache.get_clip_region(entity));
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

                            if let Some(mut window_view) = context.views.remove(&Entity::root()) {
                                if let Some(window) = window_view.downcast_mut::<Window>() {
                                    window.handle.resize(size);
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
