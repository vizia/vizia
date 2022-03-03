//use crate::event_manager::EventManager;
use crate::window::ViziaWindow;
use crate::Renderer;
use baseview::{WindowHandle, WindowScalePolicy};
use femtovg::Canvas;
use raw_window_handle::HasRawWindowHandle;
use vizia_core::{apply_inline_inheritance, apply_shared_inheritance, TreeDepthIterator, TreeExt};
use vizia_core::{MouseButton, MouseButtonState};
//use vizia_core::WindowWidget;
use vizia_core::{
    apply_clipping, apply_hover, apply_styles, apply_text_constraints, apply_transform,
    apply_visibility, apply_z_ordering, geometry_changed, Context, Display, Entity, EventManager,
    FontOrId, Modifiers, Units, Visibility, WindowEvent, WindowSize,
};
use vizia_core::{BoundingBox, Event, Propagation, WindowDescription};

pub struct Application<F>
where
    F: Fn(&mut Context),
    F: 'static + Send,
{
    app: F,
    window_description: WindowDescription,
    on_idle: Option<Box<dyn Fn(&mut Context) + Send>>,
}

impl<F> Application<F>
where
    F: Fn(&mut Context),
    F: 'static + Send,
{
    pub fn new(window_description: WindowDescription, app: F) -> Self {
        Self { app, window_description, on_idle: None }
    }

    /// Open a new window that blocks the current thread until the window is destroyed.
    ///
    /// Do **not** use this in the context of audio plugins, unless it is compiled as a
    /// standalone application.
    ///
    /// * `app` - The Tuix application builder.
    pub fn run(self) {
        ViziaWindow::open_blocking(self.window_description, self.app, self.on_idle)
    }

    /// Open a new child window.
    ///
    /// This function does **not** block the current thread. This is only to be
    /// used in the context of audio plugins.
    ///
    /// * `parent` - The parent window.
    /// * `app` - The Tuix application builder.
    pub fn open_parented<P: HasRawWindowHandle>(self, parent: &P) -> WindowHandle {
        ViziaWindow::open_parented(parent, self.window_description, self.app, self.on_idle)
    }

    /// Open a new window as if it had a parent window.
    ///
    /// This function does **not** block the current thread. This is only to be
    /// used in the context of audio plugins.
    ///
    /// * `app` - The Tuix application builder.
    pub fn open_as_if_parented(self) -> WindowHandle {
        ViziaWindow::open_as_if_parented(self.window_description, self.app, self.on_idle)
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
    /// # use vizia_baseview::Application;
    /// Application::new(WindowDescription::new(), |cx|{
    ///     // Build application here
    /// })
    /// .on_idle(|cx|{
    ///     // Code here runs at the end of every event loop after OS and tuix events have been handled
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
    canvas: Canvas<Renderer>,
    should_redraw: bool,
    scale_policy: WindowScalePolicy,
    scale_factor: f64,

    click_time: std::time::Instant,
    double_click_interval: std::time::Duration,
    double_click: bool,
    click_pos: (f32, f32),
}

impl ApplicationRunner {
    pub fn new(mut context: Context, win_desc: WindowDescription, renderer: Renderer) -> Self {
        let event_manager = EventManager::new();

        let mut canvas = Canvas::new(renderer).expect("Cannot create canvas");

        // // TODO: Get scale policy from `win_desc`.
        let scale_policy = WindowScalePolicy::SystemScaleFactor;

        // // Assume scale for now until there is an event with a new one.
        // let scale = match scale_policy {
        //     WindowScalePolicy::ScaleFactor(scale) => scale,
        //     WindowScalePolicy::SystemScaleFactor => 1.0,
        // };

        let scale = 1.0;

        let logical_size = win_desc.inner_size;
        let physical_size = WindowSize {
            width: (logical_size.width as f64 * scale).round() as u32,
            height: (logical_size.height as f64 * scale).round() as u32,
        };

        canvas.set_size(physical_size.width, physical_size.height, 1.0);

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

        //canvas.scale(scale as f32, scale as f32);

        context.style.width.insert(Entity::root(), Units::Pixels(logical_size.width as f32));
        context.style.height.insert(Entity::root(), Units::Pixels(logical_size.height as f32));

        context.style.disabled.insert(Entity::root(), false);

        context.cache.set_width(Entity::root(), physical_size.width as f32);
        context.cache.set_height(Entity::root(), physical_size.height as f32);
        context.cache.set_opacity(Entity::root(), 1.0);

        let mut bounding_box = BoundingBox::default();
        bounding_box.w = logical_size.width as f32;
        bounding_box.h = logical_size.height as f32;

        context.cache.set_clip_region(Entity::root(), bounding_box);

        ApplicationRunner {
            event_manager,
            context,
            canvas,
            should_redraw: true,
            scale_policy,
            scale_factor: scale,

            click_time: std::time::Instant::now(),
            double_click_interval: std::time::Duration::from_millis(500),
            double_click: false,
            click_pos: (0.0, 0.0),
        }
    }

    /*
    pub fn get_window(&self) -> Entity {
        self.Entity::root()
    }

    pub fn get_state(&mut self) -> &mut State {
        &mut self.context
    }

    pub fn get_event_manager(&mut self) -> &mut EventManager {
        &mut self.event_manager
    }
    */

    // pub fn update_data(&mut self) {
    //     // Data Updates
    //     let mut observers: Vec<Entity> = Vec::new();
    //     for model_list in self.context.data.model_data.dense.iter().map(|entry| &entry.value){
    //         for (_, model) in model_list.iter() {
    //             //println!("Lenses: {:?}", context.lenses.len());
    //             for (_, lens) in self.context.lenses.iter_mut() {
    //                 if lens.update(model) {
    //                     observers.extend(lens.observers().iter());
    //                 }
    //             }
    //         }
    //     }

    //     for observer in observers.iter() {
    //         if let Some(mut view) = self.context.views.remove(observer) {
    //             let prev = self.context.current;
    //             self.context.current = *observer;
    //             let prev_count = self.context.count;
    //             self.context.count = 0;
    //             view.body(&mut self.context);
    //             self.context.current = prev;
    //             self.context.count = prev_count;

    //             self.context.style.needs_redraw = true;

    //             self.context.views.insert(*observer, view);
    //         }
    //     }
    // }

    pub fn on_frame_update(&mut self) {
        //if let Some(mut window_view) = context.views.remove(&Entity::root()) {
        //if let Some(window) = window_view.downcast_mut::<Window>() {

        // Load resources
        for (name, font) in self.context.resource_manager.fonts.iter_mut() {
            match font {
                FontOrId::Font(data) => {
                    let id1 = self
                        .canvas
                        .add_font_mem(&data.clone())
                        .expect(&format!("Failed to load font file for: {}", name));
                    let id2 =
                        self.context.text_context.add_font_mem(&data.clone()).expect("failed");
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

        //}

        //context.views.insert(Entity::root(), window_view);
        //}

        // Events
        while !self.context.event_queue.is_empty() {
            self.event_manager.flush_events(&mut self.context);
        }

        // Data Updates
        let mut observers: Vec<Entity> = Vec::new();
        for model_store in self.context.data.dense.iter_mut().map(|entry| &mut entry.value) {
            for (_, model) in model_store.data.iter() {
                for lens in model_store.lenses_dup.iter_mut() {
                    if lens.update(model) {
                        observers.extend(lens.observers().iter());
                    }
                }
                for (_, lens) in model_store.lenses_dedup.iter_mut() {
                    if lens.update(model) {
                        observers.extend(lens.observers().iter());
                    }
                }
            }
        }
        for img in self.context.resource_manager.images.values_mut() {
            if img.dirty {
                observers.extend(img.observers.iter());
                img.dirty = false;
            }
        }

        for observer in observers.iter() {
            if let Some(mut view) = self.context.views.remove(observer) {
                let prev = self.context.current;
                self.context.current = *observer;
                let prev_count = self.context.count;
                self.context.count = 0;
                view.body(&mut self.context);
                self.context.current = prev;
                self.context.count = prev_count;

                self.context.style.needs_redraw = true;

                self.context.views.insert(*observer, view);
            }
        }

        // Not ideal
        let tree = self.context.tree.clone();

        // Styling
        apply_inline_inheritance(&mut self.context, &tree);
        apply_styles(&mut self.context, &tree);
        apply_shared_inheritance(&mut self.context, &tree);

        apply_z_ordering(&mut self.context, &tree);

        apply_visibility(&mut self.context, &tree);

        // Layout
        if self.context.style.needs_relayout {
            apply_text_constraints(&mut self.context, &tree);

            vizia_core::apply_layout(
                &mut self.context.cache,
                &self.context.tree,
                &self.context.style,
            );
            self.context.style.needs_relayout = false;
        }

        // Emit any geometry changed events
        geometry_changed(&mut self.context, &tree);

        apply_transform(&mut self.context, &tree);

        apply_hover(&mut self.context);

        apply_clipping(&mut self.context, &tree);

        if self.context.style.needs_redraw {
            //     // TODO - Move this to EventManager
            self.should_redraw = true;
            self.context.style.needs_redraw = false;
        }
    }

    pub fn render(&mut self) {
        // TODO
        let dpi_factor = 1.0;

        let window_width = self.context.cache.get_width(Entity::root());
        let window_height = self.context.cache.get_height(Entity::root());

        self.canvas.set_size(window_width as u32, window_height as u32, dpi_factor as f32);
        let clear_color = self
            .context
            .style
            .background_color
            .get(Entity::root())
            .cloned()
            .unwrap_or(vizia_core::Color::white());
        self.canvas.clear_rect(0, 0, window_width as u32, window_height as u32, clear_color.into());

        // Sort the tree by z order
        let mut draw_tree: Vec<Entity> = self.context.tree.into_iter().collect();
        draw_tree.sort_by_cached_key(|entity| self.context.cache.get_z_index(*entity));

        self.context.resource_manager.mark_images_unused();

        for entity in draw_tree.into_iter() {
            // Skip window
            if entity == Entity::root() {
                continue;
            }

            // Skip invisible widgets
            if self.context.cache.get_visibility(entity) == Visibility::Invisible {
                continue;
            }

            if self.context.cache.get_display(entity) == Display::None {
                continue;
            }

            // Skip widgets that have 0 opacity
            if self.context.cache.get_opacity(entity) == 0.0 {
                continue;
            }

            // Apply clipping
            let clip_region = self.context.cache.get_clip_region(entity);
            self.canvas.scissor(clip_region.x, clip_region.y, clip_region.w, clip_region.h);

            // Apply transform
            let transform = self.context.cache.get_transform(entity);
            self.canvas.save();
            self.canvas.set_transform(
                transform[0],
                transform[1],
                transform[2],
                transform[3],
                transform[4],
                transform[5],
            );

            if let Some(view) = self.context.views.remove(&entity) {
                self.context.current = entity;
                view.draw(&mut self.context, &mut self.canvas);

                self.context.views.insert(entity, view);
            }

            self.canvas.restore();
        }

        self.canvas.flush();
        self.context.resource_manager.evict_unused_images();

        self.should_redraw = false;
    }

    pub fn handle_event(&mut self, event: baseview::Event, should_quit: &mut bool) {
        if requests_exit(&event) {
            self.context.event_queue.push_back(Event::new(WindowEvent::WindowClose));
            *should_quit = true;
        }

        match event {
            baseview::Event::Mouse(event) => match event {
                baseview::MouseEvent::CursorMoved { position } => {
                    let cursorx = (position.x) as f32;
                    let cursory = (position.y) as f32;

                    self.context.mouse.cursorx = cursorx;
                    self.context.mouse.cursory = cursory;

                    vizia_core::apply_hover(&mut self.context);

                    if self.context.captured != Entity::null() {
                        self.context.event_queue.push_back(
                            Event::new(WindowEvent::MouseMove(cursorx, cursory))
                                .target(self.context.captured)
                                .propagate(Propagation::Direct),
                        );
                    } else if self.context.hovered != Entity::root() {
                        self.context.event_queue.push_back(
                            Event::new(WindowEvent::MouseMove(cursorx, cursory))
                                .target(self.context.hovered),
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
                            self.context.mouse.left.state = MouseButtonState::Pressed;
                        }
                        MouseButton::Right => {
                            self.context.mouse.right.state = MouseButtonState::Pressed;
                        }
                        MouseButton::Middle => {
                            self.context.mouse.middle.state = MouseButtonState::Pressed;
                        }
                        _ => {}
                    };

                    let new_click_time = std::time::Instant::now();
                    let click_duration = new_click_time - self.click_time;
                    let new_click_pos = (self.context.mouse.cursorx, self.context.mouse.cursory);

                    if click_duration <= self.double_click_interval
                        && new_click_pos == self.click_pos
                    {
                        if !self.double_click {
                            let _target = if self.context.captured != Entity::null() {
                                self.context.event_queue.push_back(
                                    Event::new(WindowEvent::MouseDoubleClick(b))
                                        .target(self.context.captured)
                                        .propagate(Propagation::Direct),
                                );
                                self.context.captured
                            } else {
                                self.context.event_queue.push_back(
                                    Event::new(WindowEvent::MouseDoubleClick(b))
                                        .target(self.context.hovered),
                                );
                                self.context.hovered
                            };
                            self.double_click = true;
                        }
                    } else {
                        self.double_click = false;
                    }

                    self.click_time = new_click_time;
                    self.click_pos = new_click_pos;

                    // if self.context.hovered != Entity::null()
                    //     && self.context.active != self.context.hovered
                    // {
                    //     self.context.active = self.context.hovered;
                    //     self.context.event_queue.push_back(Event::new(WindowEvent::Restyle).target(Entity::root()));
                    //     self.context.needs_restyle = true;
                    // }

                    if self.context.captured != Entity::null() {
                        self.context.event_queue.push_back(
                            Event::new(WindowEvent::MouseDown(b))
                                .target(self.context.captured)
                                .propagate(Propagation::Direct),
                        );
                    } else {
                        self.context.event_queue.push_back(
                            Event::new(WindowEvent::MouseDown(b)).target(self.context.hovered),
                        );
                    };

                    // if let Some(event_handler) = self.event_manager.event_handlers.get_mut(&target) {
                    //     if let Some(callback) = self.event_manager.callbacks.get_mut(&target) {
                    //         (callback)(event_handler, &mut self.context, target);
                    //     }
                    // }

                    match b {
                        MouseButton::Left => {
                            self.context.mouse.left.pos_down =
                                (self.context.mouse.cursorx, self.context.mouse.cursory);
                            self.context.mouse.left.pressed = self.context.hovered;
                        }

                        MouseButton::Middle => {
                            self.context.mouse.middle.pos_down =
                                (self.context.mouse.cursorx, self.context.mouse.cursory);
                            self.context.mouse.left.pressed = self.context.hovered;
                        }

                        MouseButton::Right => {
                            self.context.mouse.right.pos_down =
                                (self.context.mouse.cursorx, self.context.mouse.cursory);
                            self.context.mouse.left.pressed = self.context.hovered;
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
                            self.context.mouse.left.state = MouseButtonState::Released;
                        }
                        MouseButton::Right => {
                            self.context.mouse.right.state = MouseButtonState::Released;
                        }
                        MouseButton::Middle => {
                            self.context.mouse.middle.state = MouseButtonState::Released;
                        }
                        _ => {}
                    };

                    // self.context.active = Entity::null();
                    // self.context.event_queue.push_back(Event::new(WindowEvent::Restyle).target(Entity::root()));
                    // self.context.needs_restyle = true;

                    if self.context.captured != Entity::null() {
                        self.context.event_queue.push_back(
                            Event::new(WindowEvent::MouseUp(b))
                                .target(self.context.captured)
                                .propagate(Propagation::Direct),
                        );
                    } else {
                        self.context.event_queue.push_back(
                            Event::new(WindowEvent::MouseUp(b)).target(self.context.hovered),
                        );
                    }

                    match b {
                        MouseButton::Left => {
                            self.context.mouse.left.pos_up =
                                (self.context.mouse.cursorx, self.context.mouse.cursory);
                            self.context.mouse.left.released = self.context.hovered;
                        }

                        MouseButton::Middle => {
                            self.context.mouse.middle.pos_up =
                                (self.context.mouse.cursorx, self.context.mouse.cursory);
                            self.context.mouse.left.released = self.context.hovered;
                        }

                        MouseButton::Right => {
                            self.context.mouse.right.pos_up =
                                (self.context.mouse.cursorx, self.context.mouse.cursory);
                            self.context.mouse.left.released = self.context.hovered;
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

                    if self.context.captured != Entity::null() {
                        self.context.event_queue.push_back(
                            Event::new(WindowEvent::MouseScroll(lines_x, lines_y))
                                .target(self.context.captured)
                                .propagate(Propagation::Direct),
                        );
                    } else {
                        self.context.event_queue.push_back(
                            Event::new(WindowEvent::MouseScroll(lines_x, lines_y))
                                .target(self.context.hovered),
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
                    Code::ShiftLeft | Code::ShiftRight => {
                        self.context.modifiers.set(Modifiers::SHIFT, pressed)
                    }
                    Code::ControlLeft | Code::ControlRight => {
                        self.context.modifiers.set(Modifiers::CTRL, pressed)
                    }
                    Code::AltLeft | Code::AltRight => {
                        self.context.modifiers.set(Modifiers::ALT, pressed)
                    }
                    Code::MetaLeft | Code::MetaRight => {
                        self.context.modifiers.set(Modifiers::LOGO, pressed)
                    }
                    _ => (),
                }

                if event.code == Code::F5 && s == MouseButtonState::Pressed {
                    self.context.reload_styles().unwrap();
                }

                #[cfg(debug_assertions)]
                if event.code == Code::KeyH && s == MouseButtonState::Pressed {
                    println!("Tree");
                    for entity in self.context.tree.into_iter() {
                        println!("Entity: {} Parent: {:?} posx: {} posy: {} width: {} height: {} scissor: {:?}", entity, entity.parent(&self.context.tree), self.context.cache.get_posx(entity), self.context.cache.get_posy(entity), self.context.cache.get_width(entity), self.context.cache.get_height(entity), self.context.cache.get_clip_region(entity));
                    }
                }

                #[cfg(debug_assertions)]
                if event.code == Code::KeyI && s == MouseButtonState::Pressed {
                    let iter = TreeDepthIterator::full(&self.context.tree);
                    for (entity, level) in iter {
                        if let Some(view) = self.context.views.get(&entity) {
                            if let Some(element_name) = view.element() {
                                println!(
                                    "{:indent$} {} {} {:?} {:?} {:?}",
                                    "",
                                    entity,
                                    element_name,
                                    self.context.cache.get_visibility(entity),
                                    self.context.cache.get_display(entity),
                                    self.context.cache.get_bounds(entity),
                                    indent = level
                                );
                            }
                        }
                    }
                }

                if event.code == Code::Tab && s == MouseButtonState::Pressed {
                    // let next_focus = self
                    //     .state
                    //     .style
                    //     .focus_order
                    //     .get(self.context.focused)
                    //     .cloned()
                    //     .unwrap_or_default()
                    //     .next;
                    // let prev_focus = self
                    //     .state
                    //     .style
                    //     .focus_order
                    //     .get(self.context.focused)
                    //     .cloned()
                    //     .unwrap_or_default()
                    //     .prev;

                    // if self.context.modifiers.shift {
                    //     if prev_focus != Entity::null() {
                    //         self.context.focused.set_focus(&mut self.context, false);
                    //         self.context.focused = prev_focus;
                    //         self.context.focused.set_focus(&mut self.context, true);
                    //     } else {
                    //         // TODO impliment reverse iterator for tree
                    //         // state.focused = match state.focused.into_iter(&state.tree).next() {
                    //         //     Some(val) => val,
                    //         //     None => Entity::root(),
                    //         // };
                    //     }
                    // } else {
                    //     if next_focus != Entity::null() {
                    //         self.context.focused.set_focus(&mut self.context, false);
                    //         self.context.focused = next_focus;
                    //         self.context.focused.set_focus(&mut self.context, true);
                    //     } else {
                    //         self.context.focused.set_focus(&mut self.context, false);
                    //         self.context.focused =
                    //             match self.context.focused.tree_iter(&self.tree).next() {
                    //                 Some(val) => val,
                    //                 None => Entity::root(),
                    //             };
                    //         self.context.focused.set_focus(&mut self.context, true);
                    //     }
                    // }

                    self.context.style.needs_restyle = true;
                }

                match s {
                    MouseButtonState::Pressed => {
                        if self.context.focused != Entity::null() {
                            self.context.event_queue.push_back(
                                Event::new(WindowEvent::KeyDown(
                                    event.code,
                                    Some(event.key.clone()),
                                ))
                                .target(self.context.focused)
                                .propagate(Propagation::Up),
                            );
                        } else {
                            self.context.event_queue.push_back(
                                Event::new(WindowEvent::KeyDown(
                                    event.code,
                                    Some(event.key.clone()),
                                ))
                                .target(self.context.hovered)
                                .propagate(Propagation::Up),
                            );
                        }

                        if let keyboard_types::Key::Character(written) = &event.key {
                            for chr in written.chars() {
                                self.context.event_queue.push_back(
                                    Event::new(WindowEvent::CharInput(chr))
                                        .target(self.context.focused)
                                        .propagate(Propagation::Up),
                                );
                            }
                        }
                    }

                    MouseButtonState::Released => {
                        if self.context.focused != Entity::null() {
                            self.context.event_queue.push_back(
                                Event::new(WindowEvent::KeyUp(event.code, Some(event.key)))
                                    .target(self.context.focused)
                                    .propagate(Propagation::Up),
                            );
                        } else {
                            self.context.event_queue.push_back(
                                Event::new(WindowEvent::KeyUp(event.code, Some(event.key)))
                                    .target(self.context.hovered)
                                    .propagate(Propagation::Up),
                            );
                        }
                    }
                }
            }
            baseview::Event::Window(event) => match event {
                baseview::WindowEvent::Focused => {
                    self.context.style.needs_restyle = true;
                    self.context.style.needs_relayout = true;
                    self.context.style.needs_redraw = true;
                }
                baseview::WindowEvent::Resized(window_info) => {
                    self.scale_factor = match self.scale_policy {
                        WindowScalePolicy::ScaleFactor(scale) => scale,
                        WindowScalePolicy::SystemScaleFactor => window_info.scale(),
                    };

                    self.scale_factor = 1.0;

                    let logical_size = (
                        (window_info.physical_size().width as f64 / self.scale_factor),
                        (window_info.physical_size().height as f64 / self.scale_factor),
                    );

                    let physical_size =
                        (window_info.physical_size().width, window_info.physical_size().height);

                    self.context
                        .style
                        .width
                        .insert(Entity::root(), Units::Pixels(logical_size.0 as f32));
                    self.context
                        .style
                        .height
                        .insert(Entity::root(), Units::Pixels(logical_size.1 as f32));

                    self.context.cache.set_width(Entity::root(), physical_size.0 as f32);
                    self.context.cache.set_height(Entity::root(), physical_size.1 as f32);

                    let mut bounding_box = BoundingBox::default();
                    bounding_box.w = physical_size.0 as f32;
                    bounding_box.h = physical_size.1 as f32;

                    self.context.cache.set_clip_region(Entity::root(), bounding_box);

                    self.context.style.needs_restyle = true;
                    self.context.style.needs_relayout = true;
                    self.context.style.needs_redraw = true;
                }
                baseview::WindowEvent::WillClose => {
                    self.context.event_queue.push_back(Event::new(WindowEvent::WindowClose));
                }
                _ => {}
            },
        }
    }

    pub fn rebuild(&mut self, builder: &Option<Box<dyn Fn(&mut Context) + Send>>) {
        if self.context.enviroment.needs_rebuild {
            self.context.current = Entity::root();
            self.context.count = 0;
            if let Some(builder) = &builder {
                (builder)(&mut self.context);
            }
            self.context.enviroment.needs_rebuild = false;
        }
    }

    pub fn handle_idle(&mut self, on_idle: &Option<Box<dyn Fn(&mut Context) + Send>>) {
        // if let Some(idle_callback) = on_idle {
        //     (idle_callback)(&mut self.context);
        // }

        if let Some(idle_callback) = on_idle {
            self.context.current = Entity::root();
            self.context.count = 0;
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
