use std::any::Any;
use std::collections::VecDeque;
use std::ops::Range;

use femtovg::{ImageId, TextContext};
use fnv::FnvHashMap;
use morphorm::Units;

use crate::cache::CachedData;
use crate::events::ViewHandler;
use crate::input::{Modifiers, MouseState};
use crate::prelude::*;
use crate::resource::{ImageOrId, ResourceManager};
use crate::state::ModelDataStore;
use crate::storage::sparse_set::SparseSet;
use crate::style::{LinearGradient, Style};
use crate::text::Selection;

pub struct EventContext<'a> {
    pub(crate) current: Entity,
    captured: &'a mut Entity,
    focused: &'a mut Entity,
    pub(crate) hovered: &'a Entity,
    pub style: &'a mut Style,
    // Should event handling be able to directly mutate the cache?
    pub cache: &'a mut CachedData,
    pub tree: &'a Tree,
    pub(crate) data: &'a SparseSet<ModelDataStore>,
    pub views: &'a FnvHashMap<Entity, Box<dyn ViewHandler>>,
    pub resource_manager: &'a ResourceManager,
    pub text_context: &'a TextContext,
    pub modifiers: &'a Modifiers,
    pub mouse: &'a MouseState,
    event_queue: &'a mut VecDeque<Event>,
    cursor_icon_locked: &'a mut bool,
}

impl<'a> EventContext<'a> {
    pub fn new(cx: &'a mut Context) -> Self {
        Self {
            current: cx.current,
            captured: &mut cx.captured,
            focused: &mut cx.focused,
            hovered: &cx.hovered,
            style: &mut cx.style,
            cache: &mut cx.cache,
            tree: &cx.tree,
            data: &cx.data,
            views: &cx.views,
            resource_manager: &cx.resource_manager,
            text_context: &cx.text_context,
            modifiers: &cx.modifiers,
            mouse: &cx.mouse,
            event_queue: &mut cx.event_queue,
            cursor_icon_locked: &mut cx.cursor_icon_locked,
        }
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
    pub fn send_event(&mut self, event: Event) {
        self.event_queue.push_back(event);
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

    /// Sets application focus to the current entity
    pub fn focus(&mut self) {
        *self.focused = self.current;
    }

    /// Returns true if the current entity is disabled
    pub fn is_disabled(&self) -> bool {
        self.style.disabled.get(self.current()).cloned().unwrap_or_default()
    }

    /// Returns true if the mouse cursor is over the current entity
    pub fn is_over(&self) -> bool {
        if let Some(pseudo_classes) = self.style.pseudo_classes.get(self.current) {
            pseudo_classes.contains(PseudoClass::OVER)
        } else {
            false
        }
    }

    /// Prevents the cursor icon from changing until the lock is released
    pub fn lock_cursor_icon(&mut self) {
        *self.cursor_icon_locked = true;
    }

    /// Releases any cursor icon lock, allowing the cursor icon to be changed
    pub fn unlock_cursor_icon(&mut self) {
        *self.cursor_icon_locked = false;
        let hovered = *self.hovered;
        let cursor = self.style.cursor.get(hovered).cloned().unwrap_or_default();
        self.emit(WindowEvent::SetCursor(cursor));
    }

    /// Returns true if the cursor icon is locked
    pub fn is_cursor_icon_locked(&self) -> bool {
        *self.cursor_icon_locked
    }

    /// Sets the hover flag of the current entity
    pub fn set_hover(&mut self, flag: bool) {
        let current = self.current();
        if let Some(pseudo_classes) = self.style.pseudo_classes.get_mut(current) {
            pseudo_classes.set(PseudoClass::HOVER, flag);
        }

        self.style.needs_restyle = true;
        self.style.needs_relayout = true;
        self.style.needs_redraw = true;
    }

    /// Sets the checked flag of the current entity
    pub fn set_checked(&mut self, flag: bool) {
        let current = self.current();
        if let Some(pseudo_classes) = self.style.pseudo_classes.get_mut(current) {
            pseudo_classes.set(PseudoClass::CHECKED, flag);
        }

        self.style.needs_restyle = true;
        self.style.needs_relayout = true;
        self.style.needs_redraw = true;
    }

    /// Sets the checked flag of the current entity
    pub fn set_selected(&mut self, flag: bool) {
        let current = self.current();
        if let Some(pseudo_classes) = self.style.pseudo_classes.get_mut(current) {
            pseudo_classes.set(PseudoClass::SELECTED, flag);
        }

        self.style.needs_restyle = true;
        self.style.needs_relayout = true;
        self.style.needs_redraw = true;
    }
}
