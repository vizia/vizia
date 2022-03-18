use instant::{Duration, Instant};
use std::collections::hash_map::Entry;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::{Debug, Formatter};
use std::sync::Mutex;

#[cfg(feature = "clipboard")]
use copypasta::ClipboardContext;
use femtovg::TextContext;
use fnv::FnvHashMap;
use keyboard_types::Code;
use unic_langid::LanguageIdentifier;
// use fluent_bundle::{FluentBundle, FluentResource};
// use unic_langid::LanguageIdentifier;

use crate::{
    apply_clipping, apply_hover, apply_inline_inheritance, apply_layout, apply_shared_inheritance,
    apply_styles, apply_text_constraints, apply_transform, apply_visibility, apply_z_ordering,
    focus_backward, focus_forward, geometry_changed, is_focusable, storage::sparse_set::SparseSet,
    CachedData, Canvas, Color, Display, Entity, Enviroment, Event, FontOrId, IdManager, ImageOrId,
    ImageRetentionPolicy, Message, ModelDataStore, Modifiers, MouseButton, MouseButtonState,
    MouseState, PropSet, Propagation, ResourceManager, StoredImage, Style, Tree, TreeDepthIterator,
    TreeExt, TreeIterator, View, ViewHandler, Visibility, WindowEvent,
};
use crate::{AnimExt, Animation, AnimationBuilder};

static DEFAULT_THEME: &str = include_str!("default_theme.css");
const DOUBLE_CLICK_INTERVAL: Duration = Duration::from_millis(500);

pub struct Context {
    pub entity_manager: IdManager<Entity>,
    pub tree: Tree,
    pub current: Entity,
    pub count: usize,
    //pub views: HashMap<Entity, Box<dyn ViewHandler>>,
    pub views: FnvHashMap<Entity, Box<dyn ViewHandler>>,
    //pub views: SparseSet<Box<dyn ViewHandler>>,
    pub data: SparseSet<ModelDataStore>,
    pub event_queue: VecDeque<Event>,
    pub listeners: HashMap<Entity, Box<dyn Fn(&mut dyn ViewHandler, &mut Context, &mut Event)>>,
    pub style: Style,
    pub cache: CachedData,

    pub enviroment: Enviroment,

    pub mouse: MouseState,
    pub modifiers: Modifiers,

    pub captured: Entity,
    pub hovered: Entity,
    pub focused: Entity,

    pub resource_manager: ResourceManager,

    pub text_context: TextContext,

    pub event_proxy: Option<Box<dyn EventProxy>>,

    #[cfg(feature = "clipboard")]
    pub clipboard: ClipboardContext,

    click_time: Instant,
    double_click: bool,
    click_pos: (f32, f32),
}

impl Context {
    pub fn new() -> Self {
        let mut cache = CachedData::default();
        cache.add(Entity::root()).expect("Failed to add entity to cache");

        let mut result = Self {
            entity_manager: IdManager::new(),
            tree: Tree::new(),
            current: Entity::root(),
            count: 0,
            views: FnvHashMap::default(),
            data: SparseSet::new(),
            style: Style::default(),
            cache,
            enviroment: Enviroment::new(),
            event_queue: VecDeque::new(),
            listeners: HashMap::default(),
            mouse: MouseState::default(),
            modifiers: Modifiers::empty(),
            captured: Entity::null(),
            hovered: Entity::root(),
            focused: Entity::root(),
            resource_manager: ResourceManager::new(),
            text_context: TextContext::default(),

            event_proxy: None,

            #[cfg(feature = "clipboard")]
            clipboard: ClipboardContext::new().expect("Failed to init clipboard"),

            click_time: Instant::now(),
            double_click: false,
            click_pos: (0.0, 0.0),
        };

        result.entity_manager.create();
        result.add_theme(DEFAULT_THEME);

        result
    }

    /// Causes mouse events to propagate to the current entity until released
    pub fn capture(&mut self) {
        self.captured = self.current;
    }

    /// Releases the mouse events capture
    pub fn release(&mut self) {
        self.captured = Entity::null();
    }

    pub fn remove_children(&mut self, entity: Entity) {
        let children = entity.child_iter(&self.tree).collect::<Vec<_>>();
        for child in children.into_iter() {
            self.remove(child);
        }
    }

    /// Remove any extra children that were cached in the tree but are no longer required
    pub fn remove_trailing_children(&mut self) {
        while let Some(child) = self.tree.get_child(self.current, self.count) {
            self.remove(child);
        }
    }

    pub fn remove(&mut self, entity: Entity) {
        let delete_list = entity.branch_iter(&self.tree).collect::<Vec<_>>();

        if !delete_list.is_empty() {
            self.style.needs_restyle = true;
            self.style.needs_relayout = true;
            self.style.needs_redraw = true;
        }

        for entity in delete_list.iter().rev() {
            for model_store in self.data.dense.iter_mut().map(|entry| &mut entry.value) {
                for (_, lens) in model_store.lenses_dedup.iter_mut() {
                    lens.remove_observer(entity);
                }
                for lens in model_store.lenses_dup.iter_mut() {
                    lens.remove_observer(entity);
                }

                model_store.lenses_dedup.retain(|_, lenswrap| lenswrap.num_observers() != 0);
                model_store.lenses_dup.retain(|lenswrap| lenswrap.num_observers() != 0);
            }

            for image in self.resource_manager.images.values_mut() {
                // no need to drop them here. garbage collection happens after draw (policy based)
                image.observers.remove(entity);
            }

            self.tree.remove(*entity).expect("");
            self.cache.remove(*entity);
            self.style.remove(*entity);
            self.data.remove(*entity);
            self.views.remove(entity);
            self.entity_manager.destroy(*entity);

            if self.captured == *entity {
                self.captured = Entity::null();
            }
        }
    }

    /// Get stored data from the context.
    pub fn data<T: 'static>(&self) -> Option<&T> {
        // return data for the static model
        if let Some(t) = ().as_any().downcast_ref::<T>() {
            return Some(t);
        }

        for entity in self.current.parent_iter(&self.tree) {
            //println!("Current: {} {:?}", entity, entity.parent(&self.tree));
            if let Some(data_list) = self.data.get(entity) {
                for (_, model) in data_list.data.iter() {
                    if let Some(data) = model.downcast_ref::<T>() {
                        return Some(data);
                    }
                }
            }
        }

        None
    }

    pub fn emit<M: Message>(&mut self, message: M) {
        self.event_queue.push_back(
            Event::new(message)
                .target(self.current)
                .origin(self.current)
                .propagate(Propagation::Up),
        );
    }

    pub fn emit_to<M: Message>(&mut self, target: Entity, message: M) {
        self.event_queue.push_back(
            Event::new(message).target(target).origin(self.current).propagate(Propagation::Direct),
        );
    }

    pub fn add_listener<F, W>(&mut self, listener: F)
    where
        W: View,
        F: 'static + Fn(&mut W, &mut Context, &mut Event),
    {
        self.listeners.insert(
            self.current,
            Box::new(move |event_handler, context, event| {
                if let Some(widget) = event_handler.downcast_mut::<W>() {
                    (listener)(widget, context, event);
                }
            }),
        );
    }

    pub fn emit_trace<M: Message>(&mut self, message: M) {
        self.event_queue.push_back(
            Event::new(message)
                .target(self.current)
                .origin(self.current)
                .propagate(Propagation::Up)
                .trace(),
        );
    }

    /// Add a font from memory to the application
    pub fn add_font_mem(&mut self, name: &str, data: &[u8]) {
        // TODO - return error
        if self.resource_manager.fonts.contains_key(name) {
            println!("Font already exists");
            return;
        }

        self.resource_manager.fonts.insert(name.to_owned(), FontOrId::Font(data.to_vec()));
    }

    /// Sets the global default font for the application
    pub fn set_default_font(&mut self, name: &str) {
        self.style.default_font = name.to_string();
    }

    pub fn add_theme(&mut self, theme: &str) {
        self.resource_manager.themes.push(theme.to_owned());

        self.reload_styles().expect("Failed to reload styles");
    }

    pub fn remove_user_themes(&mut self) {
        self.resource_manager.themes.clear();

        self.add_theme(DEFAULT_THEME);
    }

    pub fn add_stylesheet(&mut self, path: &str) -> Result<(), std::io::Error> {
        let style_string = std::fs::read_to_string(path)?;
        self.resource_manager.stylesheets.push(path.to_owned());
        self.style.parse_theme(&style_string);

        Ok(())
    }

    pub fn has_animations(&self) -> bool {
        self.style.display.has_animations()
            | self.style.visibility.has_animations()
            | self.style.opacity.has_animations()
            | self.style.rotate.has_animations()
            | self.style.translate.has_animations()
            | self.style.scale.has_animations()
            | self.style.border_width.has_animations()
            | self.style.border_color.has_animations()
            | self.style.border_radius_top_left.has_animations()
            | self.style.border_radius_top_right.has_animations()
            | self.style.border_radius_bottom_left.has_animations()
            | self.style.border_radius_bottom_right.has_animations()
            | self.style.background_color.has_animations()
            | self.style.outer_shadow_h_offset.has_animations()
            | self.style.outer_shadow_v_offset.has_animations()
            | self.style.outer_shadow_blur.has_animations()
            | self.style.outer_shadow_color.has_animations()
            | self.style.font_color.has_animations()
            | self.style.font_size.has_animations()
            | self.style.left.has_animations()
            | self.style.right.has_animations()
            | self.style.top.has_animations()
            | self.style.bottom.has_animations()
            | self.style.width.has_animations()
            | self.style.height.has_animations()
            | self.style.max_width.has_animations()
            | self.style.max_height.has_animations()
            | self.style.min_width.has_animations()
            | self.style.min_height.has_animations()
            | self.style.min_left.has_animations()
            | self.style.max_left.has_animations()
            | self.style.min_right.has_animations()
            | self.style.max_right.has_animations()
            | self.style.min_top.has_animations()
            | self.style.max_top.has_animations()
            | self.style.min_bottom.has_animations()
            | self.style.max_bottom.has_animations()
            | self.style.row_between.has_animations()
            | self.style.col_between.has_animations()
            | self.style.child_left.has_animations()
            | self.style.child_right.has_animations()
            | self.style.child_top.has_animations()
            | self.style.child_bottom.has_animations()
    }

    pub fn apply_animations(&mut self) {
        let time = std::time::Instant::now();

        self.style.display.tick(time);
        self.style.visibility.tick(time);
        self.style.opacity.tick(time);
        self.style.rotate.tick(time);
        self.style.translate.tick(time);
        self.style.scale.tick(time);
        self.style.border_width.tick(time);
        self.style.border_color.tick(time);
        self.style.border_radius_top_left.tick(time);
        self.style.border_radius_top_right.tick(time);
        self.style.border_radius_bottom_left.tick(time);
        self.style.border_radius_bottom_right.tick(time);
        self.style.background_color.tick(time);
        self.style.outer_shadow_h_offset.tick(time);
        self.style.outer_shadow_v_offset.tick(time);
        self.style.outer_shadow_blur.tick(time);
        self.style.outer_shadow_color.tick(time);
        self.style.font_color.tick(time);
        self.style.font_size.tick(time);
        self.style.left.tick(time);
        self.style.right.tick(time);
        self.style.top.tick(time);
        self.style.bottom.tick(time);
        self.style.width.tick(time);
        self.style.height.tick(time);
        self.style.max_width.tick(time);
        self.style.max_height.tick(time);
        self.style.min_width.tick(time);
        self.style.min_height.tick(time);
        self.style.min_left.tick(time);
        self.style.max_left.tick(time);
        self.style.min_right.tick(time);
        self.style.max_right.tick(time);
        self.style.min_top.tick(time);
        self.style.max_top.tick(time);
        self.style.min_bottom.tick(time);
        self.style.max_bottom.tick(time);
        self.style.row_between.tick(time);
        self.style.col_between.tick(time);
        self.style.child_left.tick(time);
        self.style.child_right.tick(time);
        self.style.child_top.tick(time);
        self.style.child_bottom.tick(time);

        self.style.needs_relayout = true;
    }

    pub fn add_animation(&mut self, duration: std::time::Duration) -> AnimationBuilder {
        let id = self.style.animation_manager.create();
        AnimationBuilder::new(id, self, duration)
    }

    pub fn play_animation(&mut self, animation: Animation) {
        self.current.play_animation(self, animation);
    }

    pub fn reload_styles(&mut self) -> Result<(), std::io::Error> {
        if self.resource_manager.themes.is_empty() && self.resource_manager.stylesheets.is_empty() {
            return Ok(());
        }

        self.style.remove_rules();

        self.style.rules.clear();

        self.style.clear_style_rules();

        let mut overall_theme = String::new();

        // Reload the stored themes
        for (index, theme) in self.resource_manager.themes.iter().enumerate() {
            if !self.enviroment.include_default_theme && index == 0 {
                continue;
            }

            //self.style.parse_theme(theme);
            overall_theme += theme;
        }

        // Reload the stored stylesheets
        for stylesheet in self.resource_manager.stylesheets.iter() {
            let theme = std::fs::read_to_string(stylesheet)?;
            overall_theme += &theme;
        }

        self.style.parse_theme(&overall_theme);

        //self.enviroment.needs_rebuild = true;

        Ok(())
    }

    pub fn set_image_loader<F: 'static + Fn(&mut Context, &str)>(&mut self, loader: F) {
        self.resource_manager.image_loader = Some(Box::new(loader));
    }

    fn get_image_internal(&mut self, path: &str) -> &mut StoredImage {
        if let Some(img) = self.resource_manager.images.get_mut(path) {
            img.used = true;
            // borrow checker hack
            return self.resource_manager.images.get_mut(path).unwrap();
        }

        if let Some(callback) = self.resource_manager.image_loader.take() {
            callback(self, path);
            self.resource_manager.image_loader = Some(callback);
        }

        if let Some(img) = self.resource_manager.images.get_mut(path) {
            img.used = true;
            // borrow checker hack
            return self.resource_manager.images.get_mut(path).unwrap();
        } else {
            self.resource_manager.images.insert(
                path.to_owned(),
                StoredImage {
                    image: ImageOrId::Image(
                        image::load_from_memory_with_format(
                            include_bytes!("../resources/broken_image.png"),
                            image::ImageFormat::Png,
                        )
                        .unwrap(),
                        femtovg::ImageFlags::NEAREST,
                    ),
                    retention_policy: ImageRetentionPolicy::Forever,
                    used: true,
                    dirty: false,
                    observers: HashSet::new(),
                },
            );
            self.resource_manager.images.get_mut(path).unwrap()
        }
    }

    pub fn get_image(&mut self, path: &str) -> &mut ImageOrId {
        &mut self.get_image_internal(path).image
    }

    pub fn add_image_observer(&mut self, path: &str, observer: Entity) {
        self.get_image_internal(path).observers.insert(observer);
    }

    pub fn load_image(
        &mut self,
        path: String,
        image: image::DynamicImage,
        policy: ImageRetentionPolicy,
    ) {
        match self.resource_manager.images.entry(path) {
            Entry::Occupied(mut occ) => {
                occ.get_mut().image = ImageOrId::Image(
                    image,
                    femtovg::ImageFlags::REPEAT_X | femtovg::ImageFlags::REPEAT_Y,
                );
                occ.get_mut().dirty = true;
                occ.get_mut().retention_policy = policy;
            }
            Entry::Vacant(vac) => {
                vac.insert(StoredImage {
                    image: ImageOrId::Image(
                        image,
                        femtovg::ImageFlags::REPEAT_X | femtovg::ImageFlags::REPEAT_Y,
                    ),
                    retention_policy: policy,
                    used: true,
                    dirty: false,
                    observers: HashSet::new(),
                });
            }
        }
        self.style.needs_redraw = true;
        self.style.needs_relayout = true;
    }

    pub fn evict_image(&mut self, path: &str) {
        self.resource_manager.images.remove(path);
        self.style.needs_redraw = true;
        self.style.needs_relayout = true;
    }

    pub fn add_translation(&mut self, lang: LanguageIdentifier, ftl: String) {
        self.resource_manager.add_translation(lang, ftl)
    }

    pub fn spawn<F>(&self, target: F)
    where
        F: 'static + Send + Fn(&mut ContextProxy),
    {
        let mut cxp = ContextProxy {
            current: self.current,
            event_proxy: self.event_proxy.as_ref().map(|p| p.make_clone()),
        };

        std::thread::spawn(move || target(&mut cxp));
    }

    /// For each binding or data observer, check if its data has changed, and if so, rerun its
    /// builder/body.
    pub fn process_data_updates(&mut self) {
        let mut observers: Vec<Entity> = Vec::new();

        for model_store in self.data.dense.iter_mut().map(|entry| &mut entry.value) {
            for (_, model) in model_store.data.iter() {
                for lens in model_store.lenses_dup.iter_mut() {
                    if lens.update(model) {
                        observers.extend(lens.observers().iter())
                    }
                }

                for (_, lens) in model_store.lenses_dedup.iter_mut() {
                    if lens.update(model) {
                        observers.extend(lens.observers().iter());
                    }
                }
            }
        }
        for img in self.resource_manager.images.values_mut() {
            if img.dirty {
                observers.extend(img.observers.iter());
                img.dirty = false;
            }
        }

        for observer in observers.iter() {
            if let Some(mut view) = self.views.remove(observer) {
                let prev = self.current;
                let prev_count = self.count;
                self.current = *observer;
                self.count = 0;
                view.body(self);
                self.current = prev;
                self.count = prev_count;
                self.views.insert(*observer, view);
            }
        }
    }

    pub fn process_style_updates(&mut self) {
        // Not ideal
        let tree = self.tree.clone();

        apply_inline_inheritance(self, &tree);

        if self.style.needs_restyle {
            apply_styles(self, &tree);
            self.style.needs_restyle = false;
        }

        apply_shared_inheritance(self, &tree);
    }

    /// Massages the style system until everything is coherent
    pub fn process_visual_updates(&mut self) {
        // Not ideal
        let tree = self.tree.clone();

        apply_z_ordering(self, &tree);
        apply_visibility(self, &tree);

        // Layout
        if self.style.needs_relayout {
            apply_text_constraints(self, &tree);

            apply_layout(&mut self.cache, &self.tree, &self.style);
            self.style.needs_relayout = false;
        }

        apply_transform(self, &tree);
        apply_hover(self);
        apply_clipping(self, &tree);

        // Emit any geometry changed events
        geometry_changed(self, &tree);
    }

    pub fn draw(&mut self, canvas: &mut Canvas, dpi_factor: f32) {
        self.resource_manager.mark_images_unused();

        let window_width = self.cache.get_width(Entity::root());
        let window_height = self.cache.get_height(Entity::root());

        canvas.set_size(window_width as u32, window_height as u32, dpi_factor);
        let clear_color =
            self.style.background_color.get(Entity::root()).cloned().unwrap_or(Color::white());
        canvas.clear_rect(0, 0, window_width as u32, window_height as u32, clear_color.into());

        // filter for widgets that should be drawn
        let tree_iter = self.tree.into_iter();
        let mut draw_tree: Vec<Entity> = tree_iter
            .filter(|&entity| {
                entity != Entity::root()
                    && self.cache.get_visibility(entity) != Visibility::Invisible
                    && self.cache.get_display(entity) != Display::None
                    && !self.tree.is_ignored(entity)
                    && self.cache.get_opacity(entity) > 0.0
                    && {
                        let bounds = self.cache.get_bounds(entity);
                        !(bounds.x > window_width
                            || bounds.y > window_height
                            || bounds.x + bounds.w <= 0.0
                            || bounds.y + bounds.h <= 0.0)
                    }
            })
            .collect();

        // Sort the tree by z order
        draw_tree.sort_by_cached_key(|entity| self.cache.get_z_index(*entity));

        for entity in draw_tree.into_iter() {
            // Apply clipping
            let clip_region = self.cache.get_clip_region(entity);
            canvas.scissor(clip_region.x, clip_region.y, clip_region.w, clip_region.h);

            // Apply transform
            let transform = self.cache.get_transform(entity);
            canvas.save();
            canvas.set_transform(
                transform[0],
                transform[1],
                transform[2],
                transform[3],
                transform[4],
                transform[5],
            );

            if let Some(view) = self.views.remove(&entity) {
                self.current = entity;
                view.draw(self, canvas);

                self.views.insert(entity, view);
            }

            canvas.restore();

            // Uncomment this for debug outlines
            // TODO - Hook this up to a key in debug mode
            // let mut path = Path::new();
            // path.rect(bounds.x, bounds.y, bounds.w, bounds.h);
            // let mut paint = Paint::color(femtovg::Color::rgb(255, 0, 0));
            // paint.set_line_width(1.0);
            // canvas.stroke_path(&mut path, paint);
        }

        canvas.flush();

        self.resource_manager.evict_unused_images();
    }

    /// This method is in charge of receiving raw WindowEvents and dispatching them to the
    /// appropriate points in the tree.
    pub fn dispatch_system_event(&mut self, event: WindowEvent) {
        match &event {
            WindowEvent::MouseMove(x, y) => {
                self.mouse.cursorx = *x;
                self.mouse.cursory = *y;

                apply_hover(self);

                self.dispatch_direct_or_hovered(event, self.captured, false);
            }
            WindowEvent::MouseDown(button) => {
                match button {
                    MouseButton::Left => self.mouse.left.state = MouseButtonState::Pressed,
                    MouseButton::Right => self.mouse.right.state = MouseButtonState::Pressed,
                    MouseButton::Middle => self.mouse.middle.state = MouseButtonState::Pressed,
                    _ => {}
                }

                let new_click_time = Instant::now();
                let click_duration = new_click_time - self.click_time;
                let new_click_pos = (self.mouse.cursorx, self.mouse.cursory);

                if click_duration <= DOUBLE_CLICK_INTERVAL && new_click_pos == self.click_pos {
                    if !self.double_click {
                        self.dispatch_direct_or_hovered(
                            WindowEvent::MouseDoubleClick(*button),
                            self.captured,
                            true,
                        );
                        self.double_click = true;
                    }
                } else {
                    self.double_click = false;
                }

                self.click_time = new_click_time;
                self.click_pos = new_click_pos;

                match button {
                    MouseButton::Left => {
                        self.mouse.left.pos_down = (self.mouse.cursorx, self.mouse.cursory);
                        self.mouse.left.pressed = self.hovered;
                    }
                    MouseButton::Right => {
                        self.mouse.right.pos_down = (self.mouse.cursorx, self.mouse.cursory);
                        self.mouse.right.pressed = self.hovered;
                    }
                    MouseButton::Middle => {
                        self.mouse.middle.pos_down = (self.mouse.cursorx, self.mouse.cursory);
                        self.mouse.middle.pressed = self.hovered;
                    }
                    _ => {}
                }

                self.dispatch_direct_or_hovered(event, self.captured, true);
            }
            WindowEvent::MouseUp(button) => {
                match button {
                    MouseButton::Left => {
                        self.mouse.left.pos_up = (self.mouse.cursorx, self.mouse.cursory);
                        self.mouse.left.released = self.hovered;
                    }
                    MouseButton::Right => {
                        self.mouse.right.pos_up = (self.mouse.cursorx, self.mouse.cursory);
                        self.mouse.right.released = self.hovered;
                    }
                    MouseButton::Middle => {
                        self.mouse.middle.pos_up = (self.mouse.cursorx, self.mouse.cursory);
                        self.mouse.middle.released = self.hovered;
                    }
                    _ => {}
                }
                self.dispatch_direct_or_hovered(event, self.captured, true);
            }
            WindowEvent::MouseScroll(_, _) => {
                self.event_queue
                    .push_back(Event::new(event).target(self.hovered).propagate(Propagation::Up));
            }
            WindowEvent::KeyDown(code, _) => {
                #[cfg(debug_assertions)]
                if *code == Code::KeyH {
                    for entity in self.tree.into_iter() {
                        println!("Entity: {} Parent: {:?} View: {} posx: {} posy: {} width: {} height: {}", entity, entity.parent(&self.tree), self.views.get(&entity).map_or("<None>".to_owned(), |view| view.element().unwrap_or("<Unnamed>".to_owned())), self.cache.get_posx(entity), self.cache.get_posy(entity), self.cache.get_width(entity), self.cache.get_height(entity));
                    }
                }

                #[cfg(debug_assertions)]
                if *code == Code::KeyI {
                    let iter = TreeDepthIterator::full(&self.tree);
                    for (entity, level) in iter {
                        if let Some(element_name) = self.views.get(&entity).unwrap().element() {
                            println!(
                                "{:indent$} {} {} {:?} {:?} {:?} {:?}",
                                "",
                                entity,
                                element_name,
                                self.cache.get_visibility(entity),
                                self.cache.get_display(entity),
                                self.cache.get_bounds(entity),
                                self.cache.get_clip_region(entity),
                                indent = level
                            );
                        }
                    }
                }

                if *code == Code::F5 {
                    self.reload_styles().unwrap();
                }

                if *code == Code::Tab {
                    self.focused.set_focus(self, false);

                    if self.modifiers.contains(Modifiers::SHIFT) {
                        let prev_focused = if let Some(prev_focused) =
                            focus_backward(&self.tree, &self.style, self.focused)
                        {
                            prev_focused
                        } else {
                            TreeIterator::full(&self.tree)
                                .filter(|node| is_focusable(&self.style, *node))
                                .next_back()
                                .unwrap_or(Entity::root())
                        };

                        if prev_focused != self.focused {
                            self.event_queue
                                .push_back(Event::new(WindowEvent::FocusOut).target(self.focused));
                            self.event_queue
                                .push_back(Event::new(WindowEvent::FocusIn).target(prev_focused));
                            self.focused = prev_focused;
                        }
                    } else {
                        let next_focused = if let Some(next_focused) =
                            focus_forward(&self.tree, &self.style, self.focused)
                        {
                            next_focused
                        } else {
                            Entity::root()
                        };

                        if next_focused != self.focused {
                            self.event_queue
                                .push_back(Event::new(WindowEvent::FocusOut).target(self.focused));
                            self.event_queue
                                .push_back(Event::new(WindowEvent::FocusIn).target(next_focused));
                            self.focused = next_focused;
                        }
                    }
                    self.focused.set_focus(self, true);
                }

                self.dispatch_direct_or_hovered(event, self.focused, true);
            }
            WindowEvent::KeyUp(_, _) | WindowEvent::CharInput(_) => {
                self.dispatch_direct_or_hovered(event, self.focused, true);
            }
            _ => {}
        }
    }

    fn dispatch_direct_or_hovered(&mut self, event: WindowEvent, target: Entity, root: bool) {
        if target != Entity::null() {
            self.event_queue
                .push_back(Event::new(event).target(target).propagate(Propagation::Direct));
        } else if self.hovered != Entity::root() || root {
            self.event_queue
                .push_back(Event::new(event).target(self.hovered).propagate(Propagation::Up));
        }
    }
}

/// A bundle of data representing a snapshot of the context when a thread was spawned. It supports
/// a small subset of context operations. You will get one of these passed to you when you create a
/// new thread with `cx.spawn()`.
pub struct ContextProxy {
    pub current: Entity,
    pub event_proxy: Option<Box<dyn EventProxy>>,
}

#[derive(Debug)]
pub enum ProxyEmitError {
    Unsupported,
    EventLoopClosed,
}

impl std::fmt::Display for ProxyEmitError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ProxyEmitError::Unsupported => {
                f.write_str("The current runtime does not support proxying events")
            }
            ProxyEmitError::EventLoopClosed => {
                f.write_str("Sending an event to an event loop which has been closed")
            }
        }
    }
}

impl std::error::Error for ProxyEmitError {}

impl ContextProxy {
    pub fn emit<M: Message>(&mut self, message: M) -> Result<(), ProxyEmitError> {
        if let Some(proxy) = &self.event_proxy {
            let event = Event::new(message)
                .target(self.current)
                .origin(self.current)
                .propagate(Propagation::Up);

            proxy.send(event).map_err(|_| ProxyEmitError::EventLoopClosed)
        } else {
            Err(ProxyEmitError::Unsupported)
        }
    }

    pub fn emit_to<M: Message>(
        &mut self,
        target: Entity,
        message: M,
    ) -> Result<(), ProxyEmitError> {
        if let Some(proxy) = &self.event_proxy {
            let event = Event::new(message)
                .target(target)
                .origin(self.current)
                .propagate(Propagation::Direct);

            proxy.send(event).map_err(|_| ProxyEmitError::EventLoopClosed)
        } else {
            Err(ProxyEmitError::Unsupported)
        }
    }

    pub fn redraw(&mut self) -> Result<(), ProxyEmitError> {
        self.emit(InternalEvent::Redraw)
    }

    pub fn load_image(
        &mut self,
        path: String,
        image: image::DynamicImage,
        policy: ImageRetentionPolicy,
    ) -> Result<(), ProxyEmitError> {
        self.emit(InternalEvent::LoadImage { path, image: Mutex::new(Some(image)), policy })
    }
}

pub trait EventProxy: Send {
    fn send(&self, event: Event) -> Result<(), ()>;
    fn make_clone(&self) -> Box<dyn EventProxy>;
}

pub(crate) enum InternalEvent {
    Redraw,
    LoadImage {
        path: String,
        image: Mutex<Option<image::DynamicImage>>,
        policy: ImageRetentionPolicy,
    },
}
