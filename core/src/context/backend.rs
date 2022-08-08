use std::collections::HashSet;

use femtovg::{renderer::OpenGl, Canvas, TextContext};
use fnv::FnvHashMap;
use instant::{Duration, Instant};
use morphorm::layout;

use super::EventProxy;
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
    style::{apply_transform, Style},
    style_system::{
        apply_clipping, apply_inline_inheritance, apply_shared_inheritance, apply_styles,
        apply_text_constraints, apply_visibility, apply_z_ordering,
    },
    systems::{draw_system::draw_system, image_system::image_system},
    tree::{focus_backward, focus_forward, is_navigatable, TreeDepthIterator, TreeIterator},
};
#[cfg(feature = "clipboard")]
use copypasta::ClipboardProvider;

const DOUBLE_CLICK_INTERVAL: Duration = Duration::from_millis(500);

/// Trait used by backend implementations to set up the application context
pub trait BackendContext {
    fn add_main_window(
        &mut self,
        window_description: &WindowDescription,
        canvas: Canvas<OpenGl>,
        scale_factor: f32,
    );
    fn environment(&self) -> &Environment;
    fn context(&mut self) -> &mut Context;
    fn remove_children(&mut self);
    fn draw(&mut self);
    fn load_images(&mut self);
    fn set_current(&mut self, e: Entity);
    fn with_current(&mut self, e: Entity, f: impl FnOnce(&mut Context));
    fn set_event_proxy(&mut self, proxy: Box<dyn EventProxy>);
    #[cfg(feature = "clipboard")]
    fn set_clipboard_provider(&mut self, clipboard: Box<dyn ClipboardProvider>);
    fn send_event(&mut self, event: Event);
    fn has_queued_events(&self) -> bool;
    fn synchronize_fonts(&mut self);
    fn has_animations(&self) -> bool;
    fn apply_animations(&mut self);
    fn process_data_updates(&mut self);
    fn process_style_updates(&mut self);
    fn process_visual_updates(&mut self);
    fn dispatch_system_event(&mut self, event: WindowEvent);
    fn dispatch_direct_or_hovered(&mut self, event: WindowEvent, target: Entity, root: bool);
    fn views(&mut self) -> &mut FnvHashMap<Entity, Box<dyn ViewHandler>>;
    fn style(&mut self) -> &mut Style;
    fn cache(&mut self) -> &mut CachedData;
    fn modifiers(&mut self) -> &mut Modifiers;
}

impl BackendContext for Context {
    fn views(&mut self) -> &mut FnvHashMap<Entity, Box<dyn ViewHandler>> {
        &mut self.views
    }

    fn style(&mut self) -> &mut Style {
        &mut self.style
    }

    fn cache(&mut self) -> &mut CachedData {
        &mut self.cache
    }

    fn modifiers(&mut self) -> &mut Modifiers {
        &mut self.modifiers
    }

    fn add_main_window(
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

        self.add_font_mem("roboto", regular_font);
        self.add_font_mem("roboto-bold", bold_font);
        self.add_font_mem("icons", icon_font);
        self.add_font_mem("emoji", emoji_font);
        self.add_font_mem("arabic", arabic_font);
        self.add_font_mem("material", material_font);

        self.style.default_font = String::from("roboto");

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

        self.style.dpi_factor = scale_factor as f64;

        self.cache.set_width(Entity::root(), physical_width);
        self.cache.set_height(Entity::root(), physical_height);

        self.style
            .width
            .insert(Entity::root(), Units::Pixels(window_description.inner_size.width as f32));
        self.style
            .height
            .insert(Entity::root(), Units::Pixels(window_description.inner_size.height as f32));

        self.style.pseudo_classes.insert(Entity::root(), PseudoClass::default()).unwrap();
        self.style.disabled.insert(Entity::root(), false);

        let bounding_box =
            BoundingBox { w: physical_width, h: physical_height, ..Default::default() };

        self.cache.set_clip_region(Entity::root(), bounding_box);

        self.canvases.insert(Entity::root(), canvas);
    }

    fn environment(&self) -> &Environment {
        self.data::<Environment>().unwrap()
    }

    fn context(&mut self) -> &mut Context {
        self
    }

    fn remove_children(&mut self) {
        self.remove_children(Entity::root());
    }

    fn draw(&mut self) {
        draw_system(self);
    }

    fn load_images(&mut self) {
        image_system(self);
    }

    /// Set the current entity. This is useful in user code when you're performing black magic and
    /// want to trick other parts of the code into thinking you're processing some other part of the
    /// tree.
    fn set_current(&mut self, e: Entity) {
        self.current = e;
    }

    /// Temporarily sets the current entity, calls the provided closure, and then resets the current entity back to previous.
    fn with_current(&mut self, e: Entity, f: impl FnOnce(&mut Context)) {
        let prev = self.current;
        self.current = e;
        f(self);
        self.current = prev;
    }

    /// You should not call this method unless you are writing a windowing backend, in which case
    /// you should consult the existing windowing backends for usage information.
    fn set_event_proxy(&mut self, proxy: Box<dyn EventProxy>) {
        if self.event_proxy.is_some() {
            panic!("Set the event proxy twice. This should never happen.");
        }

        self.event_proxy = Some(proxy);
    }

    /// You should not call this method unless you are writing a windowing backend, in which case
    /// you should consult the existing windowing backends for usage information.
    #[cfg(feature = "clipboard")]
    fn set_clipboard_provider(&mut self, clipboard: Box<dyn ClipboardProvider>) {
        self.clipboard = clipboard;
    }

    /// Send an event with custom origin and propagation information.
    fn send_event(&mut self, event: Event) {
        self.event_queue.push_back(event);
    }

    /// Check whether there are any events in the queue waiting for the next event dispatch cycle.
    fn has_queued_events(&self) -> bool {
        !self.event_queue.is_empty()
    }

    /// Ensure all FontOrId entires are loaded into the contexts and become Ids.
    fn synchronize_fonts(&mut self) {
        if let Some(canvas) = self.canvases.get_mut(&Entity::root()) {
            for (name, font) in self.resource_manager.fonts.iter_mut() {
                match font {
                    FontOrId::Font(data) => {
                        let id1 = canvas
                            .add_font_mem(&data.clone())
                            .expect(&format!("Failed to load font file for: {}", name));
                        let id2 = self.text_context.add_font_mem(&data.clone()).expect("failed");
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

    fn has_animations(&self) -> bool {
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

    fn apply_animations(&mut self) {
        let time = instant::Instant::now();

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

        /// For each binding or data observer, check if its data has changed, and if so, rerun its
    /// builder/body.
    fn process_data_updates(&mut self) {
        let mut observers: HashSet<Entity> = HashSet::new();

        for entity in self.tree.into_iter() {
            if let Some(model_store) = self.data.get_mut(entity) {
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

                for lens in model_store.lenses_dup.iter_mut() {
                    if let Some(view_handler) = self.views.get(&entity) {
                        if lens.update_view(view_handler) {
                            observers.extend(lens.observers().iter())
                        }
                    }
                }

                for (_, lens) in model_store.lenses_dedup.iter_mut() {
                    if let Some(view_handler) = self.views.get(&entity) {
                        if lens.update_view(view_handler) {
                            observers.extend(lens.observers().iter())
                        }
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

        let ordered_observers =
            self.tree.into_iter().filter(|ent| observers.contains(&ent)).collect::<Vec<_>>();

        for observer in ordered_observers.into_iter() {
            if !self.entity_manager.is_alive(observer) {
                continue;
            }

            if let Some(mut view) = self.views.remove(&observer) {
                let prev = self.current;
                self.current = observer;
                view.body(self);
                self.current = prev;
                self.views.insert(observer, view);
            }
        }
    }

    fn process_style_updates(&mut self) {
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
    fn process_visual_updates(&mut self) {
        // Not ideal
        let tree = self.tree.clone();

        image_system(self);

        apply_z_ordering(self, &tree);
        apply_visibility(self, &tree);

        // Layout
        if self.style.needs_relayout {
            apply_text_constraints(self, &tree);

            // hack!
            let mut store = (Style::default(), TextContext::default(), ResourceManager::default());
            std::mem::swap(&mut store.0, &mut self.style);
            std::mem::swap(&mut store.1, &mut self.text_context);
            std::mem::swap(&mut store.2, &mut self.resource_manager);

            layout(&mut self.cache, &self.tree, &store);
            std::mem::swap(&mut store.0, &mut self.style);
            std::mem::swap(&mut store.1, &mut self.text_context);
            std::mem::swap(&mut store.2, &mut self.resource_manager);
            self.style.needs_relayout = false;
        }

        apply_transform(self, &tree);
        apply_hover(self);
        apply_clipping(self, &tree);

        // Emit any geometry changed events
        geometry_changed(self, &tree);
    }

    /// This method is in charge of receiving raw WindowEvents and dispatching them to the
    /// appropriate points in the tree.
    fn dispatch_system_event(&mut self, event: WindowEvent) {
        match &event {
            WindowEvent::MouseMove(x, y) => {
                self.mouse.previous_cursorx = self.mouse.cursorx;
                self.mouse.previous_cursory = self.mouse.cursory;
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
                        self.mouse.left.state = MouseButtonState::Released;
                    }
                    MouseButton::Right => {
                        self.mouse.right.pos_up = (self.mouse.cursorx, self.mouse.cursory);
                        self.mouse.right.released = self.hovered;
                        self.mouse.right.state = MouseButtonState::Released;
                    }
                    MouseButton::Middle => {
                        self.mouse.middle.pos_up = (self.mouse.cursorx, self.mouse.cursory);
                        self.mouse.middle.released = self.hovered;
                        self.mouse.middle.state = MouseButtonState::Released;
                    }
                    _ => {}
                }
                self.dispatch_direct_or_hovered(event, self.captured, true);
            }
            WindowEvent::MouseScroll(_, _) => {
                self.event_queue.push_back(Event::new(event).target(self.hovered));
            }
            WindowEvent::KeyDown(code, _) => {
                #[cfg(debug_assertions)]
                if *code == Code::KeyH {
                    for entity in self.tree.into_iter() {
                        println!("Entity: {} Parent: {:?} View: {} posx: {} posy: {} width: {} height: {}", entity, entity.parent(&self.tree), self.views.get(&entity).map_or("<None>", |view| view.element().unwrap_or("<Unnamed>")), self.cache.get_posx(entity), self.cache.get_posy(entity), self.cache.get_width(entity), self.cache.get_height(entity));
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
                    let focused = self.focused;
                    if let Some(pseudo_classes) = self.style().pseudo_classes.get_mut(focused) {
                        pseudo_classes.set(PseudoClass::FOCUS, false);
                    }

                    if self.modifiers.contains(Modifiers::SHIFT) {
                        let prev_focused = if let Some(prev_focused) =
                            focus_backward(&self.tree, &self.style, self.focused)
                        {
                            prev_focused
                        } else {
                            TreeIterator::full(&self.tree)
                                .filter(|node| is_navigatable(&self.style, *node))
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
                            TreeIterator::full(&self.tree)
                                .filter(|node| is_navigatable(&self.style, *node))
                                .next()
                                .unwrap_or(Entity::root())
                        };

                        if next_focused != self.focused {
                            self.event_queue
                                .push_back(Event::new(WindowEvent::FocusOut).target(self.focused));
                            self.event_queue
                                .push_back(Event::new(WindowEvent::FocusIn).target(next_focused));
                            self.focused = next_focused;
                        }
                    }

                    let focused = self.focused;
                    if let Some(pseudo_classes) = self.style().pseudo_classes.get_mut(focused) {
                        pseudo_classes.set(PseudoClass::FOCUS, true);
                    }

                    self.style().needs_relayout = true;
                    self.style().needs_redraw = true;
                    self.style().needs_restyle = true;
                }

                self.event_queue.push_back(Event::new(event).target(self.focused));
            }
            WindowEvent::KeyUp(_, _) | WindowEvent::CharInput(_) => {
                self.event_queue.push_back(Event::new(event).target(self.focused));
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
