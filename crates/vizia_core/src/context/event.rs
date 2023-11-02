use std::any::{Any, TypeId};
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
#[cfg(feature = "clipboard")]
use std::error::Error;
use std::rc::Rc;

use femtovg::Transform2D;
use fnv::FnvHashMap;
use instant::{Duration, Instant};
use vizia_storage::TreeIterator;
use vizia_style::{ClipPath, Filter, Scale, Translate};

use crate::animation::{AnimId, Interpolator};
use crate::cache::CachedData;
use crate::environment::ThemeMode;
use crate::events::{TimedEvent, TimedEventHandle, Timer, TimerState, ViewHandler};
use crate::model::ModelDataStore;
use crate::prelude::*;
use crate::resource::ResourceManager;
use crate::style::{Abilities, IntoTransform, PseudoClassFlags, Style, SystemFlags};
use crate::tree::{focus_backward, focus_forward, is_navigatable};
use crate::window::DropData;
use vizia_id::GenerationalId;
use vizia_input::{Modifiers, MouseState};

use crate::context::EmitContext;
use crate::text::TextContext;
#[cfg(feature = "clipboard")]
use copypasta::ClipboardProvider;

use super::{DARK_THEME, LIGHT_THEME};

/// A context used when handling events.
///
/// The [`EventContext`] is provided by the [`event`](crate::prelude::View::event) method in [`View`], or the [`event`](crate::model::Model::event) method in [`Model`], and can be used to mutably access the
/// desired style and layout properties of the current view.
///
/// # Example
/// ```
/// # use vizia_core::prelude::*;
/// # use vizia_core::vg;
/// # let cx = &mut Context::default();
///
/// pub struct CustomView {}
///
/// impl CustomView {
///     pub fn new(cx: &mut Context) -> Handle<Self> {
///         Self{}.build(cx, |_|{})
///     }
/// }
///
/// impl View for CustomView {
///     fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
///         event.map(|window_event, _| match window_event {
///             WindowEvent::Press{..} => {
///                 // Change the view background color to red when pressed.
///                 cx.set_background_color(Color::red());
///             }
///
///             _=> {}
///         });
///     }
/// }
/// ```
pub struct EventContext<'a> {
    pub(crate) current: Entity,
    pub(crate) captured: &'a mut Entity,
    pub(crate) focused: &'a mut Entity,
    pub(crate) hovered: &'a Entity,
    pub(crate) triggered: &'a mut Entity,
    pub(crate) style: &'a mut Style,
    pub(crate) entity_identifiers: &'a HashMap<String, Entity>,
    pub cache: &'a mut CachedData,
    pub(crate) tree: &'a Tree<Entity>,
    pub(crate) data: &'a mut FnvHashMap<Entity, ModelDataStore>,
    pub(crate) views: &'a mut FnvHashMap<Entity, Box<dyn ViewHandler>>,
    pub(crate) listeners:
        &'a mut HashMap<Entity, Box<dyn Fn(&mut dyn ViewHandler, &mut EventContext, &mut Event)>>,
    pub(crate) resource_manager: &'a mut ResourceManager,
    pub(crate) text_context: &'a mut TextContext,
    pub(crate) modifiers: &'a Modifiers,
    pub(crate) mouse: &'a MouseState<Entity>,
    pub(crate) event_queue: &'a mut VecDeque<Event>,
    pub(crate) event_schedule: &'a mut BinaryHeap<TimedEvent>,
    pub(crate) next_event_id: &'a mut usize,
    pub(crate) timers: &'a mut Vec<TimerState>,
    pub(crate) running_timers: &'a mut BinaryHeap<TimerState>,
    cursor_icon_locked: &'a mut bool,
    window_size: &'a mut WindowSize,
    user_scale_factor: &'a mut f64,
    #[cfg(feature = "clipboard")]
    clipboard: &'a mut Box<dyn ClipboardProvider>,
    pub(crate) event_proxy: &'a mut Option<Box<dyn crate::context::EventProxy>>,
    pub(crate) ignore_default_theme: &'a bool,
    pub(crate) drop_data: &'a mut Option<DropData>,
}

macro_rules! get_length_property {
    (
        $(#[$meta:meta])*
        $name:ident
    ) => {
        $(#[$meta])*
        pub fn $name(&self) -> f32 {
            if let Some(length) = self.style.$name.get(self.current) {
                let bounds = self.bounds();

                let px = length.to_pixels(bounds.w.min(bounds.h), self.scale_factor());
                return px.round();
            }

            0.0
        }
    };
}

impl<'a> EventContext<'a> {
    pub fn new(cx: &'a mut Context) -> Self {
        Self {
            current: cx.current,
            captured: &mut cx.captured,
            focused: &mut cx.focused,
            hovered: &cx.hovered,
            triggered: &mut cx.triggered,
            entity_identifiers: &cx.entity_identifiers,
            style: &mut cx.style,
            cache: &mut cx.cache,
            tree: &cx.tree,
            data: &mut cx.data,
            views: &mut cx.views,
            listeners: &mut cx.listeners,
            resource_manager: &mut cx.resource_manager,
            text_context: &mut cx.text_context,
            modifiers: &cx.modifiers,
            mouse: &cx.mouse,
            event_queue: &mut cx.event_queue,
            event_schedule: &mut cx.event_schedule,
            next_event_id: &mut cx.next_event_id,
            timers: &mut cx.timers,
            running_timers: &mut cx.running_timers,
            cursor_icon_locked: &mut cx.cursor_icon_locked,
            window_size: &mut cx.window_size,
            user_scale_factor: &mut cx.user_scale_factor,
            #[cfg(feature = "clipboard")]
            clipboard: &mut cx.clipboard,
            event_proxy: &mut cx.event_proxy,
            ignore_default_theme: &cx.ignore_default_theme,
            drop_data: &mut cx.drop_data,
        }
    }

    pub fn new_with_current(cx: &'a mut Context, current: Entity) -> Self {
        Self {
            current,
            captured: &mut cx.captured,
            focused: &mut cx.focused,
            hovered: &cx.hovered,
            triggered: &mut cx.triggered,
            entity_identifiers: &cx.entity_identifiers,
            style: &mut cx.style,
            cache: &mut cx.cache,
            tree: &cx.tree,
            data: &mut cx.data,
            views: &mut cx.views,
            listeners: &mut cx.listeners,
            resource_manager: &mut cx.resource_manager,
            text_context: &mut cx.text_context,
            modifiers: &cx.modifiers,
            mouse: &cx.mouse,
            event_queue: &mut cx.event_queue,
            event_schedule: &mut cx.event_schedule,
            next_event_id: &mut cx.next_event_id,
            timers: &mut cx.timers,
            running_timers: &mut cx.running_timers,
            cursor_icon_locked: &mut cx.cursor_icon_locked,
            window_size: &mut cx.window_size,
            user_scale_factor: &mut cx.user_scale_factor,
            #[cfg(feature = "clipboard")]
            clipboard: &mut cx.clipboard,
            event_proxy: &mut cx.event_proxy,
            ignore_default_theme: &cx.ignore_default_theme,
            drop_data: &mut cx.drop_data,
        }
    }

    pub fn get_view<V: View>(&self) -> Option<&V> {
        self.views.get(&self.current).and_then(|view| view.downcast_ref::<V>())
    }

    /// Returns the [Entity] id associated with the given identifier.
    pub fn resolve_entity_identifier(&self, id: &str) -> Option<Entity> {
        self.entity_identifiers.get(id).cloned()
    }

    /// Returns the [Entity] id of the current view.
    pub fn current(&self) -> Entity {
        self.current
    }

    /// Returns a reference to the keyboard modifiers state.
    pub fn modifiers(&self) -> &Modifiers {
        self.modifiers
    }

    /// Returns a reference to the mouse state.
    pub fn mouse(&self) -> &MouseState<Entity> {
        self.mouse
    }

    pub fn nth_child(&self, n: usize) -> Option<Entity> {
        self.tree.get_child(self.current, n)
    }

    pub fn last_child(&self) -> Option<Entity> {
        self.tree.get_last_child(self.current).copied()
    }

    pub fn with_current<T>(&mut self, entity: Entity, f: impl FnOnce(&mut Self) -> T) -> T {
        let prev = self.current();
        self.current = entity;
        let ret = (f)(self);
        self.current = prev;
        ret
    }

    // Returns true if in a drop state.
    pub fn has_drop_data(&self) -> bool {
        self.drop_data.is_some()
    }

    /// Returns the bounds of the current view.
    pub fn bounds(&self) -> BoundingBox {
        self.cache.get_bounds(self.current)
    }

    pub fn set_bounds(&mut self, bounds: BoundingBox) {
        self.cache.set_bounds(self.current, bounds);
    }

    /// Returns the scale factor.
    pub fn scale_factor(&self) -> f32 {
        self.style.dpi_factor as f32
    }

    /// Converts logical points to physical pixels.
    pub fn logical_to_physical(&self, logical: f32) -> f32 {
        self.style.logical_to_physical(logical)
    }

    /// Convert physical pixels to logical points.
    pub fn physical_to_logical(&self, physical: f32) -> f32 {
        self.style.physical_to_logical(physical)
    }

    /// Returns the clip bounds of the current view.
    pub fn clip_region(&self) -> BoundingBox {
        let bounds = self.bounds();
        let overflowx = self.style.overflowx.get(self.current).copied().unwrap_or_default();
        let overflowy = self.style.overflowy.get(self.current).copied().unwrap_or_default();

        // let root_bounds = self.cache.get_bounds(Entity::root());

        let scale = self.scale_factor();

        let clip_bounds = self
            .style
            .clip_path
            .get(self.current)
            .map(|clip| match clip {
                ClipPath::Auto => bounds,
                ClipPath::Shape(rect) => bounds.shrink_sides(
                    rect.3.to_pixels(bounds.w, scale),
                    rect.0.to_pixels(bounds.h, scale),
                    rect.1.to_pixels(bounds.w, scale),
                    rect.2.to_pixels(bounds.h, scale),
                ),
            })
            .unwrap_or(bounds);

        let root_bounds: BoundingBox =
            BoundingBox { x: -f32::MAX / 2.0, y: -f32::MAX / 2.0, w: f32::MAX, h: f32::MAX };

        match (overflowx, overflowy) {
            (Overflow::Visible, Overflow::Visible) => root_bounds,
            (Overflow::Hidden, Overflow::Visible) => {
                let left = clip_bounds.left();
                let right = clip_bounds.right();
                let top = root_bounds.top();
                let bottom = root_bounds.bottom();
                BoundingBox::from_min_max(left, top, right, bottom)
            }
            (Overflow::Visible, Overflow::Hidden) => {
                let left = root_bounds.left();
                let right = root_bounds.right();
                let top = clip_bounds.top();
                let bottom = clip_bounds.bottom();
                BoundingBox::from_min_max(left, top, right, bottom)
            }
            (Overflow::Hidden, Overflow::Hidden) => clip_bounds,
        }
    }

    /// Returns the transform of the current view.
    pub fn transform(&self) -> Transform2D {
        let mut transform = Transform2D::identity();

        let bounds = self.bounds();
        let scale_factor = self.scale_factor();

        // Apply transform origin.
        let mut origin = self
            .style
            .transform_origin
            .get(self.current)
            .map(|transform_origin| {
                let mut origin = Transform2D::new_translation(bounds.left(), bounds.top());
                let offset = transform_origin.as_transform(bounds, scale_factor);
                origin.premultiply(&offset);
                origin
            })
            .unwrap_or(Transform2D::new_translation(bounds.center().0, bounds.center().1));
        transform.premultiply(&origin);
        origin.inverse();

        // Apply translation.
        if let Some(translate) = self.style.translate.get(self.current) {
            transform.premultiply(&translate.as_transform(bounds, scale_factor));
        }

        // Apply rotation.
        if let Some(rotate) = self.style.rotate.get(self.current) {
            transform.premultiply(&rotate.as_transform(bounds, scale_factor));
        }

        // Apply scaling.
        if let Some(scale) = self.style.scale.get(self.current) {
            transform.premultiply(&scale.as_transform(bounds, scale_factor));
        }

        // Apply transform functions.
        if let Some(transforms) = self.style.transform.get(self.current) {
            // Check if the transform is currently animating
            // Get the animation state
            // Manually interpolate the value to get the overall transform for the current frame
            if let Some(animation_state) = self.style.transform.get_active_animation(self.current) {
                if let Some(start) = animation_state.keyframes.first() {
                    if let Some(end) = animation_state.keyframes.last() {
                        let start_transform = start.value.as_transform(bounds, scale_factor);
                        let end_transform = end.value.as_transform(bounds, scale_factor);
                        let t = animation_state.t;
                        let animated_transform =
                            Transform2D::interpolate(&start_transform, &end_transform, t);
                        transform.premultiply(&animated_transform);
                    }
                }
            } else {
                transform.premultiply(&transforms.as_transform(bounds, scale_factor));
            }
        }

        transform.premultiply(&origin);

        transform
    }

    /// Trigger an animation with the given id to play on the current view.
    pub fn play_animation(&mut self, anim_id: impl AnimId, duration: Duration) {
        if let Some(animation_id) = anim_id.get(self) {
            self.style.enqueue_animation(self.current, animation_id, duration);
        }
    }

    /// Trigger an animation with the given id to play on a target view.
    pub fn play_animation_for(&mut self, anim_id: impl AnimId, target: &str, duration: Duration) {
        if let Some(target_entity) = self.resolve_entity_identifier(target) {
            if let Some(animation_id) = anim_id.get(self) {
                self.style.enqueue_animation(target_entity, animation_id, duration)
            }
        }
    }

    /// Returns true if the current view is currently animating with the given animation id.
    pub fn is_animating(&self, anim_id: impl AnimId) -> bool {
        if let Some(animation_id) = anim_id.get(self) {
            return self.style.is_animating(self.current, animation_id);
        }

        false
    }

    /// Add a listener to an entity.
    ///
    /// A listener can be used to handle events which would not normally propagate to the entity.
    /// For example, mouse events when a different entity has captured them. Useful for things like
    /// closing a popup when clicking outside of its bounding box.
    pub fn add_listener<F, W>(&mut self, listener: F)
    where
        W: View,
        F: 'static + Fn(&mut W, &mut EventContext, &mut Event),
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

    /// Sets the language used by the application for localization.
    pub fn set_language(&mut self, lang: LanguageIdentifier) {
        if let Some(mut model_data_store) = self.data.remove(&Entity::root()) {
            if let Some(model) = model_data_store.models.get_mut(&TypeId::of::<Environment>()) {
                model.event(self, &mut Event::new(EnvironmentEvent::SetLocale(lang)));
            }

            self.data.insert(Entity::root(), model_data_store);
        }
    }

    /// Capture mouse input for the current view.
    pub fn capture(&mut self) {
        *self.captured = self.current;
    }

    /// Release mouse input capture for the current view.
    pub fn release(&mut self) {
        if self.current == *self.captured {
            *self.captured = Entity::null();
        }
    }

    /// Enables or disables PseudoClassFlags for the focus of an entity
    fn set_focus_pseudo_classes(&mut self, focused: Entity, enabled: bool, focus_visible: bool) {
        if let Some(pseudo_classes) = self.style.pseudo_classes.get_mut(focused) {
            pseudo_classes.set(PseudoClassFlags::FOCUS, enabled);
            if !enabled || focus_visible {
                pseudo_classes.set(PseudoClassFlags::FOCUS_VISIBLE, enabled);
            }
        }

        for ancestor in focused.parent_iter(self.tree) {
            let entity = ancestor;
            if let Some(pseudo_classes) = self.style.pseudo_classes.get_mut(entity) {
                pseudo_classes.set(PseudoClassFlags::FOCUS_WITHIN, enabled);
            }
        }
    }

    /// Sets application focus to the current view with the specified focus visibility.
    pub fn focus_with_visibility(&mut self, focus_visible: bool) {
        let old_focus = self.focused();
        let new_focus = self.current();
        self.set_focus_pseudo_classes(old_focus, false, focus_visible);
        if self.current() != self.focused() {
            self.emit_to(old_focus, WindowEvent::FocusOut);
            self.emit_to(new_focus, WindowEvent::FocusIn);
            *self.focused = self.current();
        }
        self.set_focus_pseudo_classes(new_focus, true, focus_visible);

        self.style.needs_restyle();
    }

    /// Sets application focus to the current view using the previous focus visibility.
    ///
    /// Focused elements receive keyboard input events and can be selected with the `:focus` CSS pseudo-class selector.
    pub fn focus(&mut self) {
        let focused = self.focused();
        let old_focus_visible = self
            .style
            .pseudo_classes
            .get_mut(focused)
            .filter(|class| class.contains(PseudoClassFlags::FOCUS_VISIBLE))
            .is_some();
        self.focus_with_visibility(old_focus_visible)
    }

    /// Moves the keyboard focus to the next navigable view.
    pub fn focus_next(&mut self) {
        let lock_focus_to = self.tree.lock_focus_within(*self.focused);
        let next_focused = if let Some(next_focused) =
            focus_forward(self.tree, self.style, *self.focused, lock_focus_to)
        {
            next_focused
        } else {
            TreeIterator::full(self.tree)
                .find(|node| is_navigatable(self.tree, self.style, *node, lock_focus_to))
                .unwrap_or(Entity::root())
        };

        if next_focused != *self.focused {
            self.event_queue.push_back(
                Event::new(WindowEvent::FocusOut).target(*self.focused).origin(Entity::root()),
            );
            self.event_queue.push_back(
                Event::new(WindowEvent::FocusIn).target(next_focused).origin(Entity::root()),
            );

            if let Some(pseudo_classes) = self.style.pseudo_classes.get_mut(*self.triggered) {
                pseudo_classes.set(PseudoClassFlags::ACTIVE, false);
            }
            self.needs_restyle();
            *self.triggered = Entity::null();
        }
    }

    pub fn focus_prev(&mut self) {
        let lock_focus_to = self.tree.lock_focus_within(*self.focused);
        let prev_focused = if let Some(prev_focused) =
            focus_backward(self.tree, self.style, *self.focused, lock_focus_to)
        {
            prev_focused
        } else {
            TreeIterator::full(self.tree)
                .filter(|node| is_navigatable(self.tree, self.style, *node, lock_focus_to))
                .next_back()
                .unwrap_or(Entity::root())
        };

        if prev_focused != *self.focused {
            self.event_queue.push_back(
                Event::new(WindowEvent::FocusOut).target(*self.focused).origin(Entity::root()),
            );
            self.event_queue.push_back(
                Event::new(WindowEvent::FocusIn).target(prev_focused).origin(Entity::root()),
            );

            if let Some(pseudo_classes) = self.style.pseudo_classes.get_mut(*self.triggered) {
                pseudo_classes.set(PseudoClassFlags::ACTIVE, false);
            }
            self.needs_restyle();
            *self.triggered = Entity::null();
        }
    }

    /// Returns the currently hovered view.
    pub fn hovered(&self) -> Entity {
        *self.hovered
    }

    /// Returns the currently focused view.
    pub fn focused(&self) -> Entity {
        *self.focused
    }

    // PseudoClass Getters

    /// Returns true if the current view is being hovered.
    pub fn is_hovered(&self) -> bool {
        self.hovered() == self.current
    }

    /// Returns true if the current view is active.
    pub fn is_active(&self) -> bool {
        if let Some(pseudo_classes) = self.style.pseudo_classes.get(self.current) {
            pseudo_classes.contains(PseudoClassFlags::ACTIVE)
        } else {
            false
        }
    }

    /// Returns true if the mouse cursor is over the current view.
    pub fn is_over(&self) -> bool {
        if let Some(pseudo_classes) = self.style.pseudo_classes.get(self.current) {
            pseudo_classes.contains(PseudoClassFlags::OVER)
        } else {
            false
        }
    }

    /// Returns true if the current view is focused.
    pub fn is_focused(&self) -> bool {
        self.focused() == self.current
    }

    pub fn is_draggable(&self) -> bool {
        self.style
            .abilities
            .get(self.current)
            .map(|abilities| abilities.contains(Abilities::DRAGGABLE))
            .unwrap_or_default()
    }

    /// Returns true if the current view is disabled.
    pub fn is_disabled(&self) -> bool {
        self.style.disabled.get(self.current()).cloned().unwrap_or_default()
    }

    /// Returns true if the current view is checked.
    pub fn is_checked(&self) -> bool {
        if let Some(pseudo_classes) = self.style.pseudo_classes.get(self.current) {
            pseudo_classes.contains(PseudoClassFlags::CHECKED)
        } else {
            false
        }
    }

    /// Returns true if the view is in a read-only state.
    pub fn is_read_only(&self) -> bool {
        if let Some(pseudo_classes) = self.style.pseudo_classes.get(self.current) {
            pseudo_classes.contains(PseudoClassFlags::READ_ONLY)
        } else {
            false
        }
    }

    //

    /// Prevents the cursor icon from changing until the lock is released.
    pub fn lock_cursor_icon(&mut self) {
        *self.cursor_icon_locked = true;
    }

    /// Releases any cursor icon lock, allowing the cursor icon to be changed.
    pub fn unlock_cursor_icon(&mut self) {
        *self.cursor_icon_locked = false;
        let hovered = *self.hovered;
        let cursor = self.style.cursor.get(hovered).cloned().unwrap_or_default();
        self.emit(WindowEvent::SetCursor(cursor));
    }

    /// Returns true if the cursor icon is locked.
    pub fn is_cursor_icon_locked(&self) -> bool {
        *self.cursor_icon_locked
    }

    pub fn set_drop_data(&mut self, data: impl Into<DropData>) {
        *self.drop_data = Some(data.into())
    }

    /// Get the contents of the system clipboard.
    ///
    /// This may fail for a variety of backend-specific reasons.
    #[cfg(feature = "clipboard")]
    pub fn get_clipboard(&mut self) -> Result<String, Box<dyn Error + Send + Sync + 'static>> {
        self.clipboard.get_contents()
    }

    /// Set the contents of the system clipboard.
    ///
    /// This may fail for a variety of backend-specific reasons.
    #[cfg(feature = "clipboard")]
    pub fn set_clipboard(
        &mut self,
        text: String,
    ) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
        self.clipboard.set_contents(text)
    }

    /// Toggles the addition/removal of a class name for the current view.
    ///
    /// # Example
    /// ```rust
    /// # use vizia_core::prelude::*;
    /// # let context = &mut Context::default();
    /// # let mut cx = &mut EventContext::new(context);
    /// cx.toggle_class("foo", true);
    /// ```
    pub fn toggle_class(&mut self, class_name: &str, applied: bool) {
        let current = self.current();
        if let Some(class_list) = self.style.classes.get_mut(current) {
            if applied {
                class_list.insert(class_name.to_string());
            } else {
                class_list.remove(class_name);
            }
        } else if applied {
            let mut class_list = HashSet::new();
            class_list.insert(class_name.to_string());
            self.style.classes.insert(current, class_list);
        }

        self.style.needs_restyle();
    }

    /// Returns a reference to the [Environment] model.
    pub fn environment(&self) -> &Environment {
        self.data::<Environment>().unwrap()
    }

    /// Sets the current [theme mode](ThemeMode).
    pub fn set_theme_mode(&mut self, theme_mode: ThemeMode) {
        if !self.ignore_default_theme {
            match theme_mode {
                ThemeMode::LightMode => {
                    self.resource_manager.themes[1] = String::from(LIGHT_THEME);
                }

                ThemeMode::DarkMode => {
                    self.resource_manager.themes[1] = String::from(DARK_THEME);
                }
            }
        }
    }

    /// Marks the current view as needing to be redrawn.
    pub fn needs_redraw(&mut self) {
        self.style.needs_redraw();
    }

    /// Marks the current view as needing a layout computation.
    pub fn needs_relayout(&mut self) {
        self.style.needs_relayout();
        self.style.needs_redraw();
    }

    pub fn needs_restyle(&mut self) {
        self.style.needs_restyle();
    }

    /// Reloads the stylesheets linked to the application.
    pub fn reload_styles(&mut self) -> Result<(), std::io::Error> {
        if self.resource_manager.themes.is_empty() && self.resource_manager.styles.is_empty() {
            return Ok(());
        }

        self.style.remove_rules();

        self.style.clear_style_rules();

        let mut overall_theme = String::new();

        // Reload built-in themes
        for theme in self.resource_manager.themes.iter() {
            overall_theme += theme;
        }

        for style_string in self.resource_manager.styles.iter().flat_map(|style| style.get_style())
        {
            overall_theme += &style_string;
        }

        self.style.parse_theme(&overall_theme);

        self.style.needs_restyle();
        self.style.needs_relayout();
        self.style.needs_redraw();

        Ok(())
    }

    /// Spawns a thread and provides a [ContextProxy] for sending events back to the main thread.
    pub fn spawn<F>(&self, target: F)
    where
        F: 'static + Send + FnOnce(&mut ContextProxy),
    {
        let mut cxp = ContextProxy {
            current: self.current,
            event_proxy: self.event_proxy.as_ref().map(|p| p.make_clone()),
        };

        std::thread::spawn(move || target(&mut cxp));
    }

    pub fn modify<V: View>(&mut self, f: impl FnOnce(&mut V)) {
        if let Some(view) = self
            .views
            .get_mut(&self.current)
            .and_then(|view_handler| view_handler.downcast_mut::<V>())
        {
            (f)(view);
        }
    }

    /// Returns the window's size in logical pixels, before
    /// [`user_scale_factor()`][Self::user_scale_factor()] gets applied to it. If this value changed
    /// during a frame then the window will be resized and a [`WindowEvent::GeometryChanged`] will
    /// be emitted.
    pub fn window_size(&self) -> WindowSize {
        *self.window_size
    }

    /// Change the window size. A [`WindowEvent::GeometryChanged`] will be emitted when the window
    /// has actually changed in size.
    pub fn set_window_size(&mut self, new_size: WindowSize) {
        *self.window_size = new_size;
    }

    /// A scale factor used for uniformly scaling the window independently of any HiDPI scaling.
    /// `window_size` gets multplied with this factor to get the actual logical window size. If this
    /// changes during a frame, then the window will be resized at the end of the frame and a
    /// [`WindowEvent::GeometryChanged`] will be emitted. This can be initialized using
    /// [`WindowDescription::user_scale_factor`](vizia_window::WindowDescription::user_scale_factor).
    pub fn user_scale_factor(&self) -> f64 {
        *self.user_scale_factor
    }

    /// Change the user scale factor size. A [`WindowEvent::GeometryChanged`] will be emitted when the
    /// window has actually changed in size.
    pub fn set_user_scale_factor(&mut self, new_factor: f64) {
        *self.user_scale_factor = new_factor;
        self.style.system_flags.set(SystemFlags::RELAYOUT, true);
        self.style.system_flags.set(SystemFlags::REFLOW, true);
    }

    // TODO: Abstract this to shared trait for all contexts

    // Getters

    /// Returns the background color of the view.
    ///
    /// Returns a transparent color if the view does not have a background color.
    pub fn background_color(&mut self) -> Color {
        self.style.background_color.get(self.current).copied().unwrap_or_default()
    }

    // Setters

    pub fn set_id(&mut self, id: &str) {
        self.style.ids.insert(self.current, id.to_string())
    }

    // Pseudoclass Setters

    /// Sets the hover state of the current view.
    ///
    /// Hovered elements can be selected with the `:hover` CSS pseudo-class selector:
    /// ```css
    /// element:hover {
    ///     background-color: red;
    /// }
    /// ```
    /// Typically this is set by the hover system and should not be set manually.
    pub fn set_hover(&mut self, flag: bool) {
        let current = self.current();
        if let Some(pseudo_classes) = self.style.pseudo_classes.get_mut(current) {
            pseudo_classes.set(PseudoClassFlags::HOVER, flag);
        }

        self.style.needs_restyle();
    }

    /// Set the active state for the current view.
    ///
    /// Active elements can be selected with the `:active` CSS pseudo-class selector:
    /// ```css
    /// element:active {
    ///     background-color: red;
    /// }
    /// ```
    pub fn set_active(&mut self, active: bool) {
        if let Some(pseudo_classes) = self.style.pseudo_classes.get_mut(self.current) {
            pseudo_classes.set(PseudoClassFlags::ACTIVE, active);
        }

        self.style.needs_restyle();
    }

    pub fn set_read_only(&mut self, flag: bool) {
        let current = self.current();
        if let Some(pseudo_classes) = self.style.pseudo_classes.get_mut(current) {
            pseudo_classes.set(PseudoClassFlags::READ_ONLY, flag);
        }

        self.style.needs_restyle();
    }

    pub fn set_read_write(&mut self, flag: bool) {
        let current = self.current();
        if let Some(pseudo_classes) = self.style.pseudo_classes.get_mut(current) {
            pseudo_classes.set(PseudoClassFlags::READ_WRITE, flag);
        }

        self.style.needs_restyle();
    }

    /// Sets the checked state of the current view.
    ///
    /// Checked elements can be selected with the `:checked` CSS pseudo-class selector:
    /// ```css
    /// element:checked {
    ///     background-color: red;
    /// }
    /// ```
    pub fn set_checked(&mut self, flag: bool) {
        let current = self.current();
        if let Some(pseudo_classes) = self.style.pseudo_classes.get_mut(current) {
            pseudo_classes.set(PseudoClassFlags::CHECKED, flag);
        }

        self.style.needs_restyle();
    }

    /// Sets the valid state of the current view.
    ///
    /// Checked elements can be selected with the `:checked` CSS pseudo-class selector:
    /// ```css
    /// element:checked {
    ///     background-color: red;
    /// }
    /// ```
    pub fn set_valid(&mut self, flag: bool) {
        let current = self.current();
        if let Some(pseudo_classes) = self.style.pseudo_classes.get_mut(current) {
            pseudo_classes.set(PseudoClassFlags::VALID, flag);
            pseudo_classes.set(PseudoClassFlags::INVALID, !flag);
        }

        self.style.needs_restyle();
    }

    pub fn set_placeholder_shown(&mut self, flag: bool) {
        let current = self.current();
        if let Some(pseudo_classes) = self.style.pseudo_classes.get_mut(current) {
            pseudo_classes.set(PseudoClassFlags::PLACEHOLDER_SHOWN, flag);
        }

        self.style.needs_restyle();
    }

    // TODO: Move me
    pub fn is_valid(&self) -> bool {
        self.style
            .pseudo_classes
            .get(self.current)
            .map(|pseudo_classes| pseudo_classes.contains(PseudoClassFlags::VALID))
            .unwrap_or_default()
    }

    pub fn is_placeholder_shown(&self) -> bool {
        self.style
            .pseudo_classes
            .get(self.current)
            .map(|pseudo_classes| pseudo_classes.contains(PseudoClassFlags::PLACEHOLDER_SHOWN))
            .unwrap_or_default()
    }

    // Accessibility Properties

    /// Sets the accessibility name of the view.
    pub fn set_name(&mut self, name: &str) {
        self.style.name.insert(self.current, name.to_string());
    }

    /// Sets the accessibility role of the view.
    pub fn set_role(&mut self, role: Role) {
        self.style.role.insert(self.current, role);
    }

    /// Sets the accessibility default action verb of the view.
    pub fn set_default_action_verb(&mut self, default_action_verb: DefaultActionVerb) {
        self.style.default_action_verb.insert(self.current, default_action_verb);
    }

    /// Sets the view to be an accessibility live region.
    pub fn set_live(&mut self, live: Live) {
        self.style.live.insert(self.current, live);
    }

    /// Sets the view, by id name, which labels the current view for accessibility.  
    pub fn labelled_by(&mut self, id: &str) {
        if let Some(entity) = self.resolve_entity_identifier(id) {
            self.style.labelled_by.insert(self.current, entity);
        }
    }

    /// Sets whether the view should be explicitely hidden from accessibility.
    pub fn set_hidden(&mut self, hidden: bool) {
        self.style.hidden.insert(self.current, hidden)
    }

    /// Sets a text value used for accessbility for the current view.
    pub fn text_value(&mut self, text: &str) {
        self.style.text_value.insert(self.current, text.to_string());
    }

    /// Sets a numeric value used for accessibility for the current view.
    pub fn numeric_value(&mut self, value: f64) {
        self.style.numeric_value.insert(self.current, value);
    }

    // DISPLAY

    /// Sets the display type of the current view.
    ///
    /// A display value of `Display::None` causes the view to be ignored by both layout and rendering.
    pub fn set_display(&mut self, display: Display) {
        self.style.display.insert(self.current, display);
    }

    /// Sets the visibility of the current view.
    ///
    /// The layout system will still compute the size and position of an invisible (hidden) view.
    pub fn set_visibility(&mut self, visibility: Visibility) {
        self.style.visibility.insert(self.current, visibility);
    }

    /// Sets the opacity of the current view.
    ///
    /// Expects a number between 0.0 (transparent) and 1.0 (opaque).
    pub fn set_opacity(&mut self, opacity: f32) {
        self.style.opacity.insert(self.current, Opacity(opacity));
    }

    /// Sets the z-index of the current view.
    pub fn set_z_index(&mut self, z_index: i32) {
        self.style.z_index.insert(self.current, z_index);
    }

    /// Sets the clip path of the current view.
    pub fn set_clip_path(&mut self, clip_path: ClipPath) {
        self.style.clip_path.insert(self.current, clip_path);
    }

    /// Sets the overflow type on the horizontal axis of the current view.
    pub fn set_overflowx(&mut self, overflowx: impl Into<Overflow>) {
        self.style.overflowx.insert(self.current, overflowx.into());
    }

    /// Sets the overflow type on the vertical axis of the current view.
    pub fn set_overflowy(&mut self, overflowy: impl Into<Overflow>) {
        self.style.overflowy.insert(self.current, overflowy.into());
    }

    // TRANSFORM

    /// Sets the transform of the current view.
    pub fn set_transform(&mut self, transform: impl Into<Vec<Transform>>) {
        self.style.transform.insert(self.current, transform.into());
    }

    /// Sets the transform origin of the current view.
    pub fn set_transform_origin(&mut self, transform_origin: Translate) {
        self.style.transform_origin.insert(self.current, transform_origin);
    }

    /// Sets the translation of the current view.
    pub fn set_translate(&mut self, translate: impl Into<Translate>) {
        self.style.translate.insert(self.current, translate.into());
    }

    /// Sets the rotation of the current view.
    pub fn set_rotate(&mut self, angle: impl Into<Angle>) {
        self.style.rotate.insert(self.current, angle.into());
    }

    /// Sets the scale of the current view.
    pub fn set_scale(&mut self, scale: impl Into<Scale>) {
        self.style.scale.insert(self.current, scale.into());
    }

    // FILTER

    /// Sets the backdrop filter of the current view.
    pub fn set_backdrop_filter(&mut self, filter: Filter) {
        self.style.backdrop_filter.insert(self.current, filter);
    }

    // BOX SHADOW

    // TODO

    // BACKGROUND

    pub fn set_background_color(&mut self, background_color: Color) {
        self.style.background_color.insert(self.current, background_color);
        self.needs_redraw();
    }

    // SPACE

    pub fn set_left(&mut self, left: Units) {
        self.style.left.insert(self.current, left);
        self.needs_relayout();
        self.needs_redraw();
    }

    pub fn set_top(&mut self, left: Units) {
        self.style.top.insert(self.current, left);
        self.needs_relayout();
        self.needs_redraw();
    }

    pub fn set_right(&mut self, left: Units) {
        self.style.right.insert(self.current, left);
        self.needs_relayout();
        self.needs_redraw();
    }

    pub fn set_bottom(&mut self, left: Units) {
        self.style.bottom.insert(self.current, left);
        self.needs_relayout();
        self.needs_redraw();
    }

    // TEXT

    /// Sets the text of the current view.
    pub fn set_text(&mut self, text: &str) {
        self.text_context.set_text(self.current, text);

        self.style.needs_text_layout.insert(self.current, true);
        self.needs_relayout();
        self.needs_redraw();
    }

    pub fn set_pointer_events(&mut self, pointer_events: impl Into<PointerEvents>) {
        self.style.pointer_events.insert(self.current, pointer_events.into());
    }

    // GETTERS
    get_length_property!(
        /// Returns the border width of the current view in physical pixels.
        border_width
    );

    /// Returns the font-size of the current view in physical pixels.
    pub fn font_size(&self) -> f32 {
        self.logical_to_physical(
            self.style.font_size.get(self.current).copied().map(|f| f.0).unwrap_or(16.0),
        )
    }

    /// Adds a timer to the application.
    ///
    /// `interval` - The time between ticks of the timer.
    /// `duration` - An optional duration for the timer. Pass `None` for a continuos timer.
    /// `callback` - A callback which is called on when the timer is started, ticks, and stops. Disambiguated by the `TimerAction` parameter of the callback.
    ///
    /// Returns a `Timer` id which can be used to start and stop the timer.  
    ///
    /// # Example
    /// Creates a timer which calls the provided callback every second for 5 seconds:
    /// ```rust
    /// # use vizia_core::prelude::*;
    /// # use instant::{Instant, Duration};
    /// # let cx = &mut Context::default();
    /// let timer = cx.add_timer(Duration::from_secs(1), Some(Duration::from_secs(5)), |cx, reason|{
    ///     match reason {
    ///         TimerAction::Start => {
    ///             println!("Start timer");
    ///         }
    ///     
    ///         TimerAction::Tick(delta) => {
    ///             println!("Tick timer: {:?}", delta);
    ///         }
    ///
    ///         TimerAction::Stop => {
    ///             println!("Stop timer");
    ///         }
    ///     }
    /// });
    /// ```
    pub fn add_timer(
        &mut self,
        interval: Duration,
        duration: Option<Duration>,
        callback: impl Fn(&mut EventContext, TimerAction) + 'static,
    ) -> Timer {
        let id = Timer(self.timers.len());
        self.timers.push(TimerState {
            entity: Entity::root(),
            id,
            time: Instant::now(),
            interval,
            duration,
            start_time: Instant::now(),
            callback: Rc::new(callback),
            ticking: false,
            stopping: false,
        });

        id
    }

    /// Starts a timer with the provided timer id.
    ///
    /// Events sent within the timer callback provided in `add_timer()` will target the current view.
    pub fn start_timer(&mut self, timer: Timer) {
        let current = self.current;
        if !self.timer_is_running(timer) {
            let timer_state = self.timers[timer.0].clone();
            // Copy timer state from pending to playing
            self.running_timers.push(timer_state);
        }

        self.modify_timer(timer, |timer_state| {
            let now = instant::Instant::now();
            timer_state.start_time = now;
            timer_state.time = now;
            timer_state.entity = current;
            timer_state.ticking = false;
            timer_state.stopping = false;
        });
    }

    /// Modifies the state of an existing timer with the provided `Timer` id.
    pub fn modify_timer(&mut self, timer: Timer, timer_function: impl Fn(&mut TimerState)) {
        while let Some(next_timer_state) = self.running_timers.peek() {
            if next_timer_state.id == timer {
                let mut timer_state = self.running_timers.pop().unwrap();

                (timer_function)(&mut timer_state);

                self.running_timers.push(timer_state);

                return;
            }
        }

        for pending_timer in self.timers.iter_mut() {
            if pending_timer.id == timer {
                (timer_function)(pending_timer);
            }
        }
    }

    /// Returns true if the timer with the provided timer id is currently running.
    pub fn timer_is_running(&mut self, timer: Timer) -> bool {
        for timer_state in self.running_timers.iter() {
            if timer_state.id == timer {
                return true;
            }
        }

        false
    }

    /// Stops the timer with the given timer id.
    ///
    /// Any events emitted in response to the timer stopping, as determined by the callback provided in `add_timer()`, will target the view which called `start_timer()`.
    pub fn stop_timer(&mut self, timer: Timer) {
        let mut running_timers = self.running_timers.clone();

        for timer_state in running_timers.iter() {
            if timer_state.id == timer {
                self.with_current(timer_state.entity, |cx| {
                    (timer_state.callback)(cx, TimerAction::Stop);
                });
            }
        }

        *self.running_timers =
            running_timers.drain().filter(|timer_state| timer_state.id != timer).collect();
    }
}

impl<'a> DataContext for EventContext<'a> {
    fn data<T: 'static>(&self) -> Option<&T> {
        // Return data for the static model.
        if let Some(t) = <dyn Any>::downcast_ref::<T>(&()) {
            return Some(t);
        }

        for entity in self.current.parent_iter(self.tree) {
            // Return model data.
            if let Some(model_data_store) = self.data.get(&entity) {
                if let Some(model) = model_data_store.models.get(&TypeId::of::<T>()) {
                    return model.downcast_ref::<T>();
                }
            }

            // Return view data.
            if let Some(view_handler) = self.views.get(&entity) {
                if let Some(data) = view_handler.downcast_ref::<T>() {
                    return Some(data);
                }
            }
        }

        None
    }
}

impl<'a> EmitContext for EventContext<'a> {
    fn emit<M: Any + Send>(&mut self, message: M) {
        self.event_queue.push_back(
            Event::new(message)
                .target(self.current)
                .origin(self.current)
                .propagate(Propagation::Up),
        );
    }

    fn emit_to<M: Any + Send>(&mut self, target: Entity, message: M) {
        self.event_queue.push_back(
            Event::new(message).target(target).origin(self.current).propagate(Propagation::Direct),
        );
    }

    fn emit_custom(&mut self, event: Event) {
        self.event_queue.push_back(event);
    }

    fn schedule_emit<M: Any + Send>(&mut self, message: M, at: Instant) -> TimedEventHandle {
        self.schedule_emit_custom(
            Event::new(message)
                .target(self.current)
                .origin(self.current)
                .propagate(Propagation::Up),
            at,
        )
    }
    fn schedule_emit_to<M: Any + Send>(
        &mut self,
        target: Entity,
        message: M,
        at: Instant,
    ) -> TimedEventHandle {
        self.schedule_emit_custom(
            Event::new(message).target(target).origin(self.current).propagate(Propagation::Direct),
            at,
        )
    }
    fn schedule_emit_custom(&mut self, event: Event, at: Instant) -> TimedEventHandle {
        let handle = TimedEventHandle(*self.next_event_id);
        self.event_schedule.push(TimedEvent { event, time: at, ident: handle });
        *self.next_event_id += 1;
        handle
    }
    fn cancel_scheduled(&mut self, handle: TimedEventHandle) {
        *self.event_schedule =
            self.event_schedule.drain().filter(|item| item.ident != handle).collect();
    }
}

/// Trait for querying properties of the tree from a context.
pub trait TreeProps {
    /// Returns the id of the parent of the current view.
    fn parent(&self) -> Entity;
    /// Returns the id of the first_child of the current view.
    fn first_child(&self) -> Entity;
}

impl<'a> TreeProps for EventContext<'a> {
    fn parent(&self) -> Entity {
        self.tree.get_layout_parent(self.current).unwrap()
    }

    fn first_child(&self) -> Entity {
        self.tree.get_layout_first_child(self.current).unwrap()
    }
}
