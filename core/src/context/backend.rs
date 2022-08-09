use std::collections::HashSet;

use femtovg::{renderer::OpenGl, Canvas, TextContext};
use fnv::FnvHashMap;
use instant::{Duration, Instant};
use morphorm::layout;

use super::EventProxy;
#[cfg(debug_assertions)]
use crate::tree::TreeDepthIterator;
use crate::{
    cache::{BoundingBox, CachedData},
    environment::Environment,
    events::ViewHandler,
    fonts,
    hover_system::apply_hover,
    id::GenerationalId,
    layout::geometry_changed,
    prelude::*,
    resource::{FontOrId, ResourceManager},
    state::ModelOrView,
    style::{apply_transform, Style},
    style_system::{
        apply_clipping, apply_inline_inheritance, apply_shared_inheritance, apply_styles,
        apply_text_constraints, apply_visibility, apply_z_ordering,
    },
    systems::{draw_system::draw_system, image_system::image_system},
    tree::{focus_backward, focus_forward, is_navigatable, TreeIterator},
};

#[cfg(feature = "clipboard")]
use copypasta::ClipboardProvider;

const DOUBLE_CLICK_INTERVAL: Duration = Duration::from_millis(500);

pub struct BackendContext<'a>(pub &'a mut Context);

impl<'a> BackendContext<'a> {
    pub fn new(cx: &'a mut Context) -> Self {
        Self(cx)
    }

    pub fn views(&mut self) -> &mut FnvHashMap<Entity, Box<dyn ViewHandler>> {
        &mut self.0.views
    }

    pub fn style(&mut self) -> &mut Style {
        &mut self.0.style
    }

    pub fn cache(&mut self) -> &mut CachedData {
        &mut self.0.cache
    }

    pub fn modifiers(&mut self) -> &mut Modifiers {
        &mut self.0.modifiers
    }

    pub fn add_main_window(
        &mut self,
        window_description: &WindowDescription,
        mut canvas: Canvas<OpenGl>,
        scale_factor: f32,
    ) {
        // Add default fonts
        let regular_font = fonts::ROBOTO_REGULAR;
        let bold_font = fonts::ROBOTO_BOLD;
        let icon_font = fonts::ENTYPO;
        let emoji_font = fonts::OPEN_SANS_EMOJI;
        let arabic_font = fonts::AMIRI_REGULAR;
        let material_font = fonts::MATERIAL_ICONS_REGULAR;

        self.0.add_font_mem("roboto", regular_font);
        self.0.add_font_mem("roboto-bold", bold_font);
        self.0.add_font_mem("icons", icon_font);
        self.0.add_font_mem("emoji", emoji_font);
        self.0.add_font_mem("arabic", arabic_font);
        self.0.add_font_mem("material", material_font);

        self.0.style.default_font = String::from("roboto");

        let physical_width = window_description.inner_size.width as f32 * scale_factor;
        let physical_height = window_description.inner_size.height as f32 * scale_factor;

        // Scale factor is set to 1.0 here because scaling is applied prior to rendering
        canvas.set_size(physical_width as u32, physical_height as u32, 1.0);
        canvas.clear_rect(
            0,
            0,
            physical_width as u32,
            physical_height as u32,
            femtovg::Color::rgb(255, 0, 0),
        );

        self.0.style.dpi_factor = scale_factor as f64;

        self.0.cache.set_width(Entity::root(), physical_width);
        self.0.cache.set_height(Entity::root(), physical_height);

        self.0
            .style
            .width
            .insert(Entity::root(), Units::Pixels(window_description.inner_size.width as f32));
        self.0
            .style
            .height
            .insert(Entity::root(), Units::Pixels(window_description.inner_size.height as f32));

        self.0.style.pseudo_classes.insert(Entity::root(), PseudoClass::default()).unwrap();
        self.0.style.disabled.insert(Entity::root(), false);

        let bounding_box =
            BoundingBox { w: physical_width, h: physical_height, ..Default::default() };

        self.0.cache.set_clip_region(Entity::root(), bounding_box);

        self.0.canvases.insert(Entity::root(), canvas);
    }

    pub fn environment(&self) -> &Environment {
        self.0.data::<Environment>().unwrap()
    }

    pub fn context(&mut self) -> &mut Context {
        self.0
    }

    pub fn remove_all_children(&mut self) {
        self.0.remove_children(Entity::root());
    }

    pub fn draw(&mut self) {
        draw_system(self.0);
    }

    pub fn load_images(&mut self) {
        image_system(self.0);
    }

    /// Set the current entity. This is useful in user code when you're performing black magic and
    /// want to trick other parts of the code into thinking you're processing some other part of the
    /// tree.
    pub fn set_current(&mut self, e: Entity) {
        self.0.current = e;
    }

    /// Temporarily sets the current entity, calls the provided closure, and then resets the current entity back to previous.
    pub fn with_current(&mut self, e: Entity, f: impl FnOnce(&mut Context)) {
        let prev = self.0.current;
        self.0.current = e;
        f(self.0);
        self.0.current = prev;
    }

    /// You should not call this method unless you are writing a windowing backend, in which case
    /// you should consult the existing windowing backends for usage information.
    pub fn set_event_proxy(&mut self, proxy: Box<dyn EventProxy>) {
        if self.0.event_proxy.is_some() {
            panic!("Set the event proxy twice. This should never happen.");
        }

        self.0.event_proxy = Some(proxy);
    }

    /// You should not call this method unless you are writing a windowing backend, in which case
    /// you should consult the existing windowing backends for usage information.
    #[cfg(feature = "clipboard")]
    pub fn set_clipboard_provider(&mut self, clipboard: Box<dyn ClipboardProvider>) {
        self.0.clipboard = clipboard;
    }

    /// Send an event with custom origin and propagation information.
    pub fn send_event(&mut self, event: Event) {
        self.0.event_queue.push_back(event);
    }

    /// Check whether there are any events in the queue waiting for the next event dispatch cycle.
    pub fn has_queued_events(&self) -> bool {
        !self.0.event_queue.is_empty()
    }

    /// Ensure all FontOrId entires are loaded into the contexts and become Ids.
    pub fn synchronize_fonts(&mut self) {
        if let Some(canvas) = self.0.canvases.get_mut(&Entity::root()) {
            for (name, font) in self.0.resource_manager.fonts.iter_mut() {
                match font {
                    FontOrId::Font(data) => {
                        let id1 = canvas
                            .add_font_mem(&data.clone())
                            .expect(&format!("Failed to load font file for: {}", name));
                        let id2 = self.0.text_context.add_font_mem(&data.clone()).expect("failed");
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
        }
    }

    pub fn has_animations(&self) -> bool {
        self.0.style.display.has_animations()
            | self.0.style.visibility.has_animations()
            | self.0.style.opacity.has_animations()
            | self.0.style.rotate.has_animations()
            | self.0.style.translate.has_animations()
            | self.0.style.scale.has_animations()
            | self.0.style.border_width.has_animations()
            | self.0.style.border_color.has_animations()
            | self.0.style.border_radius_top_left.has_animations()
            | self.0.style.border_radius_top_right.has_animations()
            | self.0.style.border_radius_bottom_left.has_animations()
            | self.0.style.border_radius_bottom_right.has_animations()
            | self.0.style.background_color.has_animations()
            | self.0.style.outer_shadow_h_offset.has_animations()
            | self.0.style.outer_shadow_v_offset.has_animations()
            | self.0.style.outer_shadow_blur.has_animations()
            | self.0.style.outer_shadow_color.has_animations()
            | self.0.style.font_color.has_animations()
            | self.0.style.font_size.has_animations()
            | self.0.style.left.has_animations()
            | self.0.style.right.has_animations()
            | self.0.style.top.has_animations()
            | self.0.style.bottom.has_animations()
            | self.0.style.width.has_animations()
            | self.0.style.height.has_animations()
            | self.0.style.max_width.has_animations()
            | self.0.style.max_height.has_animations()
            | self.0.style.min_width.has_animations()
            | self.0.style.min_height.has_animations()
            | self.0.style.min_left.has_animations()
            | self.0.style.max_left.has_animations()
            | self.0.style.min_right.has_animations()
            | self.0.style.max_right.has_animations()
            | self.0.style.min_top.has_animations()
            | self.0.style.max_top.has_animations()
            | self.0.style.min_bottom.has_animations()
            | self.0.style.max_bottom.has_animations()
            | self.0.style.row_between.has_animations()
            | self.0.style.col_between.has_animations()
            | self.0.style.child_left.has_animations()
            | self.0.style.child_right.has_animations()
            | self.0.style.child_top.has_animations()
            | self.0.style.child_bottom.has_animations()
    }

    pub fn apply_animations(&mut self) {
        let time = instant::Instant::now();

        self.0.style.display.tick(time);
        self.0.style.visibility.tick(time);
        self.0.style.opacity.tick(time);
        self.0.style.rotate.tick(time);
        self.0.style.translate.tick(time);
        self.0.style.scale.tick(time);
        self.0.style.border_width.tick(time);
        self.0.style.border_color.tick(time);
        self.0.style.border_radius_top_left.tick(time);
        self.0.style.border_radius_top_right.tick(time);
        self.0.style.border_radius_bottom_left.tick(time);
        self.0.style.border_radius_bottom_right.tick(time);
        self.0.style.background_color.tick(time);
        self.0.style.outer_shadow_h_offset.tick(time);
        self.0.style.outer_shadow_v_offset.tick(time);
        self.0.style.outer_shadow_blur.tick(time);
        self.0.style.outer_shadow_color.tick(time);
        self.0.style.font_color.tick(time);
        self.0.style.font_size.tick(time);
        self.0.style.left.tick(time);
        self.0.style.right.tick(time);
        self.0.style.top.tick(time);
        self.0.style.bottom.tick(time);
        self.0.style.width.tick(time);
        self.0.style.height.tick(time);
        self.0.style.max_width.tick(time);
        self.0.style.max_height.tick(time);
        self.0.style.min_width.tick(time);
        self.0.style.min_height.tick(time);
        self.0.style.min_left.tick(time);
        self.0.style.max_left.tick(time);
        self.0.style.min_right.tick(time);
        self.0.style.max_right.tick(time);
        self.0.style.min_top.tick(time);
        self.0.style.max_top.tick(time);
        self.0.style.min_bottom.tick(time);
        self.0.style.max_bottom.tick(time);
        self.0.style.row_between.tick(time);
        self.0.style.col_between.tick(time);
        self.0.style.child_left.tick(time);
        self.0.style.child_right.tick(time);
        self.0.style.child_top.tick(time);
        self.0.style.child_bottom.tick(time);

        self.0.style.needs_relayout = true;
    }

    /// For each binding or data observer, check if its data has changed, and if so, rerun its
    /// builder/body.
    pub fn process_data_updates(&mut self) {
        let mut observers: HashSet<Entity> = HashSet::new();

        for entity in self.0.tree.into_iter() {
            if let Some(model_data_store) = self.0.data.get_mut(entity) {
                // Determine observers of model data
                for (_, model) in model_data_store.models.iter() {
                    let model = ModelOrView::Model(model.as_ref());

                    for (_, store) in model_data_store.stores.iter_mut() {
                        if store.update(model) {
                            observers.extend(store.observers().iter())
                        }
                    }
                }

                // Determine observers of view data
                for (_, store) in model_data_store.stores.iter_mut() {
                    if let Some(view_handler) = self.0.views.get(&entity) {
                        let view_model = ModelOrView::View(view_handler.as_ref());

                        if store.update(view_model) {
                            observers.extend(store.observers().iter())
                        }
                    }
                }
            }
        }

        for img in self.0.resource_manager.images.values_mut() {
            if img.dirty {
                observers.extend(img.observers.iter());
                img.dirty = false;
            }
        }

        let ordered_observers =
            self.0.tree.into_iter().filter(|ent| observers.contains(&ent)).collect::<Vec<_>>();

        // Update observers in tree order
        for observer in ordered_observers.into_iter() {
            if !self.0.entity_manager.is_alive(observer) {
                continue;
            }

            if let Some(mut binding) = self.0.bindings.remove(&observer) {
                let prev = self.0.current;
                self.0.current = observer;
                binding.update(self.0);
                self.0.current = prev;
                self.0.bindings.insert(observer, binding);
            }
        }
    }

    pub fn process_style_updates(&mut self) {
        // Not ideal
        let tree = self.0.tree.clone();

        apply_inline_inheritance(self.0, &tree);

        if self.0.style.needs_restyle {
            apply_styles(self.0, &tree);
            self.0.style.needs_restyle = false;
        }

        apply_shared_inheritance(self.0, &tree);
    }

    /// Massages the style system until everything is coherent
    pub fn process_visual_updates(&mut self) {
        // Not ideal
        let tree = self.0.tree.clone();

        image_system(self.0);

        apply_z_ordering(self.0, &tree);
        apply_visibility(self.0, &tree);

        // Layout
        if self.0.style.needs_relayout {
            apply_text_constraints(self.0, &tree);

            // hack!
            let mut store = (Style::default(), TextContext::default(), ResourceManager::default());
            std::mem::swap(&mut store.0, &mut self.0.style);
            std::mem::swap(&mut store.1, &mut self.0.text_context);
            std::mem::swap(&mut store.2, &mut self.0.resource_manager);

            layout(&mut self.0.cache, &self.0.tree, &store);
            std::mem::swap(&mut store.0, &mut self.0.style);
            std::mem::swap(&mut store.1, &mut self.0.text_context);
            std::mem::swap(&mut store.2, &mut self.0.resource_manager);
            self.0.style.needs_relayout = false;
        }

        apply_transform(self.0, &tree);
        apply_hover(self.0);
        apply_clipping(self.0, &tree);

        // Emit any geometry changed events
        geometry_changed(self.0, &tree);
    }

    /// This method is in charge of receiving raw WindowEvents and dispatching them to the
    /// appropriate points in the tree.
    pub fn dispatch_system_event(&mut self, event: WindowEvent) {
        match &event {
            WindowEvent::MouseMove(x, y) => {
                self.0.mouse.previous_cursorx = self.0.mouse.cursorx;
                self.0.mouse.previous_cursory = self.0.mouse.cursory;
                self.0.mouse.cursorx = *x;
                self.0.mouse.cursory = *y;

                apply_hover(self.0);

                self.dispatch_direct_or_hovered(event, self.0.captured, false);
            }
            WindowEvent::MouseDown(button) => {
                match button {
                    MouseButton::Left => self.0.mouse.left.state = MouseButtonState::Pressed,
                    MouseButton::Right => self.0.mouse.right.state = MouseButtonState::Pressed,
                    MouseButton::Middle => self.0.mouse.middle.state = MouseButtonState::Pressed,
                    _ => {}
                }

                let new_click_time = Instant::now();
                let click_duration = new_click_time - self.0.click_time;
                let new_click_pos = (self.0.mouse.cursorx, self.0.mouse.cursory);

                if click_duration <= DOUBLE_CLICK_INTERVAL && new_click_pos == self.0.click_pos {
                    if !self.0.double_click {
                        self.dispatch_direct_or_hovered(
                            WindowEvent::MouseDoubleClick(*button),
                            self.0.captured,
                            true,
                        );
                        self.0.double_click = true;
                    }
                } else {
                    self.0.double_click = false;
                }

                self.0.click_time = new_click_time;
                self.0.click_pos = new_click_pos;

                match button {
                    MouseButton::Left => {
                        self.0.mouse.left.pos_down = (self.0.mouse.cursorx, self.0.mouse.cursory);
                        self.0.mouse.left.pressed = self.0.hovered;
                    }
                    MouseButton::Right => {
                        self.0.mouse.right.pos_down = (self.0.mouse.cursorx, self.0.mouse.cursory);
                        self.0.mouse.right.pressed = self.0.hovered;
                    }
                    MouseButton::Middle => {
                        self.0.mouse.middle.pos_down = (self.0.mouse.cursorx, self.0.mouse.cursory);
                        self.0.mouse.middle.pressed = self.0.hovered;
                    }
                    _ => {}
                }

                self.dispatch_direct_or_hovered(event, self.0.captured, true);
            }
            WindowEvent::MouseUp(button) => {
                match button {
                    MouseButton::Left => {
                        self.0.mouse.left.pos_up = (self.0.mouse.cursorx, self.0.mouse.cursory);
                        self.0.mouse.left.released = self.0.hovered;
                        self.0.mouse.left.state = MouseButtonState::Released;
                    }
                    MouseButton::Right => {
                        self.0.mouse.right.pos_up = (self.0.mouse.cursorx, self.0.mouse.cursory);
                        self.0.mouse.right.released = self.0.hovered;
                        self.0.mouse.right.state = MouseButtonState::Released;
                    }
                    MouseButton::Middle => {
                        self.0.mouse.middle.pos_up = (self.0.mouse.cursorx, self.0.mouse.cursory);
                        self.0.mouse.middle.released = self.0.hovered;
                        self.0.mouse.middle.state = MouseButtonState::Released;
                    }
                    _ => {}
                }
                self.dispatch_direct_or_hovered(event, self.0.captured, true);
            }
            WindowEvent::MouseScroll(_, _) => {
                self.0.event_queue.push_back(Event::new(event).target(self.0.hovered));
            }
            WindowEvent::KeyDown(code, _) => {
                #[cfg(debug_assertions)]
                if *code == Code::KeyH {
                    for entity in self.0.tree.into_iter() {
                        println!("Entity: {} Parent: {:?} View: {} posx: {} posy: {} width: {} height: {}", entity, entity.parent(&self.0.tree), self.0.views.get(&entity).map_or("<None>", |view| view.element().unwrap_or("<Unnamed>")), self.0.cache.get_posx(entity), self.0.cache.get_posy(entity), self.0.cache.get_width(entity), self.0.cache.get_height(entity));
                    }
                }

                #[cfg(debug_assertions)]
                if *code == Code::KeyI {
                    let iter = TreeDepthIterator::full(&self.0.tree);
                    for (entity, level) in iter {
                        if let Some(element_name) = self.0.views.get(&entity).unwrap().element() {
                            println!(
                                "{:indent$} {} {} {:?} {:?} {:?} {:?}",
                                "",
                                entity,
                                element_name,
                                self.0.cache.get_visibility(entity),
                                self.0.cache.get_display(entity),
                                self.0.cache.get_bounds(entity),
                                self.0.cache.get_clip_region(entity),
                                indent = level
                            );
                        }
                    }
                }

                if *code == Code::F5 {
                    self.0.reload_styles().unwrap();
                }

                if *code == Code::Tab {
                    let focused = self.0.focused;
                    if let Some(pseudo_classes) = self.style().pseudo_classes.get_mut(focused) {
                        pseudo_classes.set(PseudoClass::FOCUS, false);
                    }

                    if self.0.modifiers.contains(Modifiers::SHIFT) {
                        let prev_focused = if let Some(prev_focused) =
                            focus_backward(&self.0.tree, &self.0.style, self.0.focused)
                        {
                            prev_focused
                        } else {
                            TreeIterator::full(&self.0.tree)
                                .filter(|node| is_navigatable(&self.0.style, *node))
                                .next_back()
                                .unwrap_or(Entity::root())
                        };

                        if prev_focused != self.0.focused {
                            self.0.event_queue.push_back(
                                Event::new(WindowEvent::FocusOut).target(self.0.focused),
                            );
                            self.0
                                .event_queue
                                .push_back(Event::new(WindowEvent::FocusIn).target(prev_focused));
                            self.0.focused = prev_focused;
                        }
                    } else {
                        let next_focused = if let Some(next_focused) =
                            focus_forward(&self.0.tree, &self.0.style, self.0.focused)
                        {
                            next_focused
                        } else {
                            TreeIterator::full(&self.0.tree)
                                .filter(|node| is_navigatable(&self.0.style, *node))
                                .next()
                                .unwrap_or(Entity::root())
                        };

                        if next_focused != self.0.focused {
                            self.0.event_queue.push_back(
                                Event::new(WindowEvent::FocusOut).target(self.0.focused),
                            );
                            self.0
                                .event_queue
                                .push_back(Event::new(WindowEvent::FocusIn).target(next_focused));
                            self.0.focused = next_focused;
                        }
                    }

                    let focused = self.0.focused;
                    if let Some(pseudo_classes) = self.style().pseudo_classes.get_mut(focused) {
                        pseudo_classes.set(PseudoClass::FOCUS, true);
                    }

                    self.style().needs_relayout = true;
                    self.style().needs_redraw = true;
                    self.style().needs_restyle = true;
                }

                self.0.event_queue.push_back(Event::new(event).target(self.0.focused));
            }
            WindowEvent::KeyUp(_, _) | WindowEvent::CharInput(_) => {
                self.0.event_queue.push_back(Event::new(event).target(self.0.focused));
            }
            _ => {}
        }
    }

    pub fn dispatch_direct_or_hovered(&mut self, event: WindowEvent, target: Entity, root: bool) {
        if target != Entity::null() {
            self.0
                .event_queue
                .push_back(Event::new(event).target(target).propagate(Propagation::Direct));
        } else if self.0.hovered != Entity::root() || root {
            self.0
                .event_queue
                .push_back(Event::new(event).target(self.0.hovered).propagate(Propagation::Up));
        }
    }
}
