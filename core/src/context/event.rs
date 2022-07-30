use std::any::Any;
use std::collections::{HashMap, HashSet, VecDeque};
#[cfg(feature = "clipboard")]
use std::error::Error;

use femtovg::TextContext;
use fnv::FnvHashMap;

use crate::cache::CachedData;
use crate::events::ViewHandler;
use crate::id::GenerationalId;
use crate::input::{Modifiers, MouseState};
use crate::prelude::*;
use crate::resource::ResourceManager;
use crate::state::ModelDataStore;
use crate::storage::sparse_set::SparseSet;
use crate::style::Style;

#[cfg(feature = "clipboard")]
use copypasta::ClipboardProvider;

use super::DrawCache;

pub struct EventContext<'a> {
    pub(crate) current: Entity,
    pub(crate) captured: &'a mut Entity,
    pub(crate) focused: &'a mut Entity,
    pub(crate) hovered: &'a Entity,
    pub(crate) style: &'a mut Style,
    pub cache: &'a CachedData,
    pub draw_cache: &'a DrawCache,
    pub tree: &'a Tree,
    pub(crate) data: &'a SparseSet<ModelDataStore>,
    pub views: &'a FnvHashMap<Entity, Box<dyn ViewHandler>>,
    listeners:
        &'a mut HashMap<Entity, Box<dyn Fn(&mut dyn ViewHandler, &mut EventContext, &mut Event)>>,
    pub resource_manager: &'a ResourceManager,
    pub text_context: &'a TextContext,
    pub modifiers: &'a Modifiers,
    pub mouse: &'a MouseState,
    pub(crate) event_queue: &'a mut VecDeque<Event>,
    cursor_icon_locked: &'a mut bool,
    #[cfg(feature = "clipboard")]
    clipboard: &'a mut Box<dyn ClipboardProvider>,
}

impl<'a> EventContext<'a> {
    pub fn new(cx: &'a mut Context) -> Self {
        Self {
            current: cx.current,
            captured: &mut cx.captured,
            focused: &mut cx.focused,
            hovered: &cx.hovered,
            style: &mut cx.style,
            cache: &cx.cache,
            draw_cache: &cx.draw_cache,
            tree: &cx.tree,
            data: &cx.data,
            views: &cx.views,
            listeners: &mut cx.listeners,
            resource_manager: &cx.resource_manager,
            text_context: &cx.text_context,
            modifiers: &cx.modifiers,
            mouse: &cx.mouse,
            event_queue: &mut cx.event_queue,
            cursor_icon_locked: &mut cx.cursor_icon_locked,
            #[cfg(feature = "clipboard")]
            clipboard: &mut cx.clipboard,
        }
    }

    pub fn current(&self) -> Entity {
        self.current
    }

    /// Send an event containing a message up the tree from the current entity.
    pub fn emit<M: Message>(&mut self, message: M) {
        self.event_queue.push_back(
            Event::new(message)
                .target(self.current)
                .origin(self.current)
                .propagate(Propagation::Up),
        );
    }

    /// Send an event containing a message directly to a specified entity.
    pub fn emit_to<M: Message>(&mut self, target: Entity, message: M) {
        self.event_queue.push_back(
            Event::new(message).target(target).origin(self.current).propagate(Propagation::Direct),
        );
    }

    /// Send an event with custom origin and propagation information.
    pub fn emit_custom(&mut self, event: Event) {
        self.event_queue.push_back(event);
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

    pub fn set_active(&mut self, active: bool) {
        if let Some(pseudo_classes) = self.style.pseudo_classes.get_mut(self.current) {
            pseudo_classes.set(PseudoClass::ACTIVE, active);
        }

        self.style.needs_restyle = true;
        self.style.needs_relayout = true;
        self.style.needs_redraw = true;
    }

    pub fn capture(&mut self) {
        *self.captured = self.current;
    }

    pub fn release(&mut self) {
        *self.captured = Entity::null();
    }

    /// Sets application focus to the current entity.
    pub fn focus(&mut self) {
        *self.focused = self.current;
    }

    pub fn hovered(&self) -> Entity {
        *self.hovered
    }

    /// Returns true if the current entity is disabled.
    pub fn is_disabled(&self) -> bool {
        self.style.disabled.get(self.current()).cloned().unwrap_or_default()
    }

    /// Returns true if the mouse cursor is over the current entity.
    pub fn is_over(&self) -> bool {
        if let Some(pseudo_classes) = self.style.pseudo_classes.get(self.current) {
            pseudo_classes.contains(PseudoClass::OVER)
        } else {
            false
        }
    }

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

    /// Sets the hover flag of the current entity.
    pub fn set_hover(&mut self, flag: bool) {
        let current = self.current();
        if let Some(pseudo_classes) = self.style.pseudo_classes.get_mut(current) {
            pseudo_classes.set(PseudoClass::HOVER, flag);
        }

        self.style.needs_restyle = true;
        self.style.needs_relayout = true;
        self.style.needs_redraw = true;
    }

    /// Sets the checked flag of the current entity.
    pub fn set_checked(&mut self, flag: bool) {
        let current = self.current();
        if let Some(pseudo_classes) = self.style.pseudo_classes.get_mut(current) {
            pseudo_classes.set(PseudoClass::CHECKED, flag);
        }

        self.style.needs_restyle = true;
        self.style.needs_relayout = true;
        self.style.needs_redraw = true;
    }

    /// Sets the checked flag of the current entity.
    pub fn set_selected(&mut self, flag: bool) {
        let current = self.current();
        if let Some(pseudo_classes) = self.style.pseudo_classes.get_mut(current) {
            pseudo_classes.set(PseudoClass::SELECTED, flag);
        }

        self.style.needs_restyle = true;
        self.style.needs_relayout = true;
        self.style.needs_redraw = true;
    }

    /// Get the contents of the system clipboard. This may fail for a variety of backend-specific
    /// reasons.
    #[cfg(feature = "clipboard")]
    pub fn get_clipboard(&mut self) -> Result<String, Box<dyn Error + Send + Sync + 'static>> {
        self.clipboard.get_contents()
    }

    /// Set the contents of the system clipboard. This may fail for a variety of backend-specific
    /// reasons.
    #[cfg(feature = "clipboard")]
    pub fn set_clipboard(
        &mut self,
        text: String,
    ) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
        self.clipboard.set_contents(text)
    }

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
            self.style.classes.insert(current, class_list).expect("Failed to insert class name");
        }

        self.style.needs_restyle = true;
        self.style.needs_relayout = true;
        self.style.needs_redraw = true;
    }

    pub fn play_animation(&mut self, animation: Animation) {
        self.current.play_animation(self, animation);
    }

    pub fn environment(&self) -> &Environment {
        self.data::<Environment>().unwrap()
    }
}

impl<'a> DataContext for EventContext<'a> {
    fn data<T: 'static>(&self) -> Option<&T> {
        // return data for the static model
        if let Some(t) = <dyn Any>::downcast_ref::<T>(&()) {
            return Some(t);
        }

        for entity in self.current.parent_iter(&self.tree) {
            if let Some(data_list) = self.data.get(entity) {
                for (_, model) in data_list.data.iter() {
                    if let Some(data) = model.downcast_ref::<T>() {
                        return Some(data);
                    }
                }
            }

            if let Some(view_handler) = self.views.get(&entity) {
                if let Some(data) = view_handler.downcast_ref::<T>() {
                    return Some(data);
                }
            }
        }

        None
    }
}
