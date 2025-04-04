use crate::context::{InternalEvent, ResourceContext};
use crate::events::EventMeta;
use crate::prelude::*;
#[cfg(debug_assertions)]
use crate::systems::compute_matched_rules;
use crate::systems::{binding_system, hover_system};
use crate::tree::{focus_backward, focus_forward, is_navigatable};
#[cfg(debug_assertions)]
use log::debug;
use std::any::Any;
use vizia_storage::LayoutParentIterator;
#[cfg(debug_assertions)]
use vizia_storage::ParentIterator;
use vizia_storage::TreeIterator;

const DOUBLE_CLICK_INTERVAL: Duration = Duration::from_millis(500);

/// Dispatches events to views and models.
///
/// The [EventManager] is responsible for taking the events in the event queue in cx
/// and dispatching them to views and models based on the target and propagation metadata of the event.
#[doc(hidden)]
pub struct EventManager {
    // Queue of events to be processed.
    event_queue: Vec<Event>,
}

impl Default for EventManager {
    fn default() -> Self {
        Self::new()
    }
}

impl EventManager {
    pub fn new() -> Self {
        EventManager { event_queue: Vec::with_capacity(10) }
    }

    /// Flush the event queue, dispatching events to their targets.
    /// Returns whether there are still more events to process, i.e. the event handlers sent events.
    pub fn flush_events(
        &mut self,
        cx: &mut Context,
        mut window_event_callback: impl FnMut(&WindowEvent),
    ) {
        while {
            // Clear the event queue in the event manager.
            self.event_queue.clear();

            // Move events from cx to event manager. This is so the cx can be passed
            // mutably to the view when handling events.
            self.event_queue.extend(cx.event_queue.drain(0..));

            // Loop over the events in the event queue.
            'events: for event in self.event_queue.iter_mut() {
                // Handle internal events.
                event.take(|internal_event, _| match internal_event {
                    InternalEvent::Redraw => cx.needs_redraw(Entity::root()),
                    InternalEvent::LoadImage { path, image, policy } => {
                        if let Some(image) = image.lock().unwrap().take() {
                            ResourceContext::new(cx).load_image(path, image, policy);
                        }
                    }
                });

                // Send events to any global listeners.
                let mut global_listeners = vec![];
                std::mem::swap(&mut cx.global_listeners, &mut global_listeners);
                for listener in &global_listeners {
                    cx.with_current(Entity::root(), |cx| {
                        listener(&mut EventContext::new(cx), event)
                    });
                }
                std::mem::swap(&mut cx.global_listeners, &mut global_listeners);

                // Send events to any local listeners.
                let listeners = cx.listeners.keys().copied().collect::<Vec<Entity>>();
                for entity in listeners {
                    if let Some(listener) = cx.listeners.remove(&entity) {
                        if let Some(mut event_handler) = cx.views.remove(&entity) {
                            cx.with_current(entity, |cx| {
                                (listener)(
                                    event_handler.as_mut(),
                                    &mut EventContext::new(cx),
                                    event,
                                );
                            });

                            cx.views.insert(entity, event_handler);
                        }

                        cx.listeners.insert(entity, listener);
                    }

                    if event.meta.consumed {
                        continue 'events;
                    }
                }

                // Handle state updates for window events.
                event.map(|window_event, meta| {
                    if cx.windows.contains_key(&meta.origin) {
                        internal_state_updates(cx, window_event, meta);
                    }
                });

                // Skip to next event if the current event was consumed when handling internal state updates.
                if event.meta.consumed {
                    continue 'events;
                }

                let cx = &mut EventContext::new(cx);

                // Copy the target to prevent multiple mutable borrows error.
                let target = event.meta.target;

                // Send event to target.
                visit_entity(cx, target, event);

                // Skip to next event if the current event was consumed.
                if event.meta.consumed {
                    continue 'events;
                }

                // Propagate up from target to root (not including the target).
                if event.meta.propagation == Propagation::Up {
                    // Create a parent iterator and skip the first element which is the target.
                    let iter = target.parent_iter(cx.tree).skip(1);

                    for entity in iter {
                        // Send event to all ancestors of the target.
                        visit_entity(cx, entity, event);

                        // Skip to the next event if the current event was consumed.
                        if event.meta.consumed {
                            continue 'events;
                        }
                    }
                }

                // Propagate the event down the subtree from the target (not including the target).
                if event.meta.propagation == Propagation::Subtree {
                    // Create a branch (subtree) iterator and skip the first element which is the target.
                    let iter = target.branch_iter(cx.tree).skip(1);

                    for entity in iter {
                        // Send event to all entities in the subtree after the target.
                        visit_entity(cx, entity, event);

                        // Skip to the next event if the current event was consumed.
                        if event.meta.consumed {
                            continue 'events;
                        }
                    }
                }

                event.map(|window_event: &WindowEvent, _| {
                    (window_event_callback)(window_event);
                });
            }

            binding_system(cx);

            // Return true if there are new events in the queue.
            !cx.event_queue.is_empty()
        } {}
    }
}

fn visit_entity(cx: &mut EventContext, entity: Entity, event: &mut Event) {
    // Send event to models attached to the entity
    if let Some(ids) =
        cx.models.get(&entity).map(|models| models.keys().cloned().collect::<Vec<_>>())
    {
        for id in ids {
            if let Some(mut model) =
                cx.models.get_mut(&entity).and_then(|models| models.remove(&id))
            {
                cx.current = entity;

                model.event(cx, event);

                cx.models.get_mut(&entity).and_then(|models| models.insert(id, model));
            }
        }
    }

    // Return early if the event was consumed by a model
    if event.meta.consumed {
        return;
    }

    // Send event to the view attached to the entity
    if let Some(mut view) = cx.views.remove(&entity) {
        cx.current = entity;
        view.event(cx, event);

        cx.views.insert(entity, view);
    }
}

/// Update the internal state of the cx based on received window event and emit window event to relevant target.
fn internal_state_updates(cx: &mut Context, window_event: &WindowEvent, meta: &mut EventMeta) {
    cx.current = meta.target;

    match window_event {
        WindowEvent::Drop(drop_data) => {
            cx.drop_data = Some(drop_data.clone());
        }

        WindowEvent::MouseMove(x, y) => {
            if !x.is_nan() && !y.is_nan() {
                cx.mouse.previous_cursor_x = cx.mouse.cursor_x;
                cx.mouse.previous_cursor_y = cx.mouse.cursor_y;
                cx.mouse.cursor_x = *x;
                cx.mouse.cursor_y = *y;

                hover_system(cx, meta.origin);

                mutate_direct_or_up(meta, cx.captured, cx.hovered, false);
            }

            // if cx.mouse.cursor_x != cx.mouse.previous_cursor_x
            //     || cx.mouse.cursor_y != cx.mouse.previous_cursor_y
            // {
            // }

            // if let Some(dropped_file) = cx.dropped_file.take() {
            //     emit_direct_or_up(
            //         cx,
            //         WindowEvent::DroppedFile(dropped_file),
            //         cx.captured,
            //         cx.hovered,
            //         true,
            //     );
            // }
        }
        WindowEvent::MouseDown(button) => {
            // do direct state-updates
            match button {
                MouseButton::Left => {
                    cx.mouse.left.state = MouseButtonState::Pressed;

                    cx.mouse.left.pos_down = (cx.mouse.cursor_x, cx.mouse.cursor_y);
                    cx.mouse.left.pressed = cx.hovered;
                    cx.triggered = cx.hovered;

                    let disabled = cx.style.disabled.get(cx.hovered).copied().unwrap_or_default();

                    if let Some(pseudo_classes) = cx.style.pseudo_classes.get_mut(cx.triggered) {
                        if !disabled {
                            pseudo_classes.set(PseudoClassFlags::ACTIVE, true);
                            cx.needs_restyle(cx.triggered);
                        }
                    }
                    let focusable = cx
                        .style
                        .abilities
                        .get(cx.hovered)
                        .filter(|abilities| abilities.contains(Abilities::FOCUSABLE))
                        .is_some();

                    // Reset drag data
                    cx.drop_data = None;

                    cx.with_current(if focusable { cx.hovered } else { cx.focused }, |cx| {
                        cx.focus_with_visibility(false)
                    });
                }
                MouseButton::Right => {
                    cx.mouse.right.state = MouseButtonState::Pressed;
                    cx.mouse.right.pos_down = (cx.mouse.cursor_x, cx.mouse.cursor_y);
                    cx.mouse.right.pressed = cx.hovered;
                }
                MouseButton::Middle => {
                    cx.mouse.middle.state = MouseButtonState::Pressed;
                    cx.mouse.middle.pos_down = (cx.mouse.cursor_x, cx.mouse.cursor_y);
                    cx.mouse.middle.pressed = cx.hovered;
                }
                _ => {}
            }

            // emit trigger events
            if matches!(button, MouseButton::Left) {
                emit_direct_or_up(
                    cx,
                    WindowEvent::PressDown { mouse: true },
                    cx.captured,
                    cx.triggered,
                    true,
                );
            }

            // track double/triple -click
            let new_click_time = Instant::now();
            let click_duration = new_click_time - cx.click_time;
            let new_click_pos = (cx.mouse.cursor_x, cx.mouse.cursor_y);
            if click_duration <= DOUBLE_CLICK_INTERVAL
                && new_click_pos == cx.click_pos
                && *button == cx.click_button
            {
                if cx.clicks <= 2 {
                    cx.clicks += 1;
                    let event = if cx.clicks == 3 {
                        WindowEvent::MouseTripleClick(*button)
                    } else {
                        WindowEvent::MouseDoubleClick(*button)
                    };
                    meta.consume();
                    emit_direct_or_up(cx, event, cx.captured, cx.hovered, true);
                }
            } else {
                cx.clicks = 1;
            }
            cx.click_time = new_click_time;
            cx.click_pos = new_click_pos;
            cx.click_button = *button;
            mutate_direct_or_up(meta, cx.captured, cx.hovered, true);
        }
        WindowEvent::MouseUp(button) => {
            match button {
                MouseButton::Left => {
                    cx.mouse.left.pos_up = (cx.mouse.cursor_x, cx.mouse.cursor_y);
                    cx.mouse.left.released = cx.hovered;
                    cx.mouse.left.state = MouseButtonState::Released;
                }
                MouseButton::Right => {
                    cx.mouse.right.pos_up = (cx.mouse.cursor_x, cx.mouse.cursor_y);
                    cx.mouse.right.released = cx.hovered;
                    cx.mouse.right.state = MouseButtonState::Released;
                }
                MouseButton::Middle => {
                    cx.mouse.middle.pos_up = (cx.mouse.cursor_x, cx.mouse.cursor_y);
                    cx.mouse.middle.released = cx.hovered;
                    cx.mouse.middle.state = MouseButtonState::Released;
                }
                _ => {}
            }

            if matches!(button, MouseButton::Left) {
                if cx.hovered == cx.triggered {
                    let disabled = cx.style.disabled.get(cx.hovered).copied().unwrap_or_default();

                    if !disabled {
                        emit_direct_or_up(
                            cx,
                            WindowEvent::Press { mouse: true },
                            cx.captured,
                            cx.triggered,
                            true,
                        );
                    }
                }

                if let Some(pseudo_classes) = cx.style.pseudo_classes.get_mut(cx.triggered) {
                    pseudo_classes.set(PseudoClassFlags::ACTIVE, false);
                }

                cx.needs_restyle(cx.triggered);

                cx.triggered = Entity::null();
            }

            mutate_direct_or_up(meta, cx.captured, cx.hovered, true);
        }
        WindowEvent::MouseScroll(_, _) => {
            meta.target = cx.hovered;
        }
        WindowEvent::KeyDown(code, _) => {
            meta.target = cx.focused;

            #[cfg(debug_assertions)]
            if *code == Code::KeyP && cx.modifiers.ctrl() {
                for entity in TreeIterator::full(&cx.tree) {
                    if let Some(models) = cx.models.get(&entity) {
                        if !models.is_empty() {
                            debug!("Models for {}", entity);
                            for (_, model) in models.iter() {
                                debug!("M: {:?}", model.name())
                            }
                        }
                    }

                    if let Some(stores) = cx.stores.get(&entity) {
                        if !stores.is_empty() {
                            debug!("Stores for {}", entity);
                            for (_, store) in stores.iter() {
                                debug!("S: [{}] - Observers {:?}", store.name(), store.observers())
                            }
                        }
                    }
                }
            }

            #[cfg(debug_assertions)]
            if *code == Code::KeyI {
                debug!("Entity tree");
                let (tree, views, cache) = (&cx.tree, &cx.views, &cx.cache);
                let has_next_sibling = |entity| tree.get_next_sibling(entity).is_some();
                let root_indents = |entity: Entity| {
                    let parent_iter = ParentIterator::new(tree, Some(entity));
                    parent_iter
                        .skip(1)
                        .collect::<Vec<_>>()
                        .into_iter()
                        .rev()
                        .skip(1)
                        .map(|entity| if has_next_sibling(entity) { "│   " } else { "    " })
                        .collect::<String>()
                };
                let local_idents =
                    |entity| if has_next_sibling(entity) { "├── " } else { "└── " };
                let indents = |entity| root_indents(entity) + local_idents(entity);

                for entity in TreeIterator::full(tree).skip(1) {
                    if let Some(element_name) = views.get(&entity).and_then(|view| view.element()) {
                        let w = cache.get_bounds(entity).w;
                        let h = cache.get_bounds(entity).h;
                        let classes = cx.style.classes.get(entity);
                        let mut class_names = String::new();
                        if let Some(classes) = classes {
                            for class in classes.iter() {
                                class_names += &format!(".{}", class);
                            }
                        }
                        println!(
                            "{}{} {}{} [x: {} y: {} w: {} h: {}]",
                            indents(entity),
                            entity,
                            element_name,
                            class_names,
                            cache.get_bounds(entity).x,
                            cache.get_bounds(entity).y,
                            if w == f32::MAX { "inf".to_string() } else { w.to_string() },
                            if h == f32::MAX { "inf".to_string() } else { h.to_string() },
                        );
                    } else if let Some(binding_name) =
                        cx.bindings.get(&entity).map(|binding| format!("{:?}", binding))
                    {
                        println!(
                            "{}{} binding observing {}",
                            indents(entity),
                            entity,
                            binding_name,
                        );
                    } else {
                        println!(
                            "{}{} {}",
                            indents(entity),
                            entity,
                            if views.get(&entity).is_some() {
                                "unnamed view"
                            } else {
                                "no binding or view"
                            }
                        );
                    }
                }
            }

            #[cfg(debug_assertions)]
            if *code == Code::KeyS
                && cx.modifiers == Modifiers::CTRL | Modifiers::SHIFT | Modifiers::ALT
            {
                use crate::systems::compute_element_hash;
                use vizia_style::selectors::bloom::BloomFilter;

                let mut filter = BloomFilter::default();
                compute_element_hash(cx.hovered, &cx.tree, &cx.style, &mut filter);
                let result = compute_matched_rules(cx.hovered, &cx.style, &cx.tree, &filter);

                let entity = cx.hovered;
                debug!("/* Matched rules for Entity: {} Parent: {:?} View: {} posx: {} posy: {} width: {} height: {}",
                    entity,
                    entity.parent(&cx.tree),
                    cx
                        .views
                        .get(&entity)
                        .map_or("<None>", |view| view.element().unwrap_or("<Unnamed>")),
                    cx.cache.get_posx(entity),
                    cx.cache.get_posy(entity),
                    cx.cache.get_width(entity),
                    cx.cache.get_height(entity)
                );
                for rule in result.into_iter() {
                    for selectors in cx.style.rules.iter() {
                        if *selectors.0 == rule.0 {
                            debug!("{:?}", selectors.1.selector);
                        }
                    }
                }
            }

            #[cfg(debug_assertions)]
            if *code == Code::KeyT
                && cx.modifiers == Modifiers::CTRL | Modifiers::SHIFT | Modifiers::ALT
            {
                // debug!("Loaded font face info:");
                // for face in cx.text_context.font_system().db().faces() {
                //     debug!(
                //         "family: {:?}\npost_script_name: {:?}\nstyle: {:?}\nweight: {:?}\nstretch: {:?}\nmonospaced: {:?}\n",
                //         face.families,
                //         face.post_script_name,
                //         face.style,
                //         face.weight,
                //         face.stretch,
                //         face.monospaced,
                //     );
                // }
            }

            if *code == Code::F5 {
                EventContext::new(cx).reload_styles().unwrap();
            }

            if *code == Code::Tab {
                let lock_focus_to = cx.tree.lock_focus_within(cx.focused);
                if cx.modifiers.shift() {
                    let prev_focused = if let Some(prev_focused) =
                        focus_backward(&cx.tree, &cx.style, cx.focused, lock_focus_to)
                    {
                        prev_focused
                    } else {
                        TreeIterator::full(&cx.tree)
                            .filter(|node| {
                                is_navigatable(&cx.tree, &cx.style, *node, lock_focus_to)
                            })
                            .next_back()
                            .unwrap_or(Entity::root())
                    };

                    if prev_focused != cx.focused {
                        cx.set_focus_pseudo_classes(cx.focused, false, true);
                        cx.set_focus_pseudo_classes(prev_focused, true, true);
                        cx.event_queue.push_back(
                            Event::new(WindowEvent::FocusOut)
                                .target(cx.focused)
                                .origin(Entity::root()),
                        );
                        cx.event_queue.push_back(
                            Event::new(WindowEvent::FocusIn)
                                .target(prev_focused)
                                .origin(Entity::root()),
                        );

                        cx.focused = prev_focused;

                        if let Some(pseudo_classes) = cx.style.pseudo_classes.get_mut(cx.triggered)
                        {
                            pseudo_classes.set(PseudoClassFlags::ACTIVE, false);
                            cx.needs_restyle(cx.triggered);
                        }
                        cx.triggered = Entity::null();
                    }
                } else {
                    let next_focused = if let Some(next_focused) =
                        focus_forward(&cx.tree, &cx.style, cx.focused, lock_focus_to)
                    {
                        next_focused
                    } else {
                        TreeIterator::full(&cx.tree)
                            .find(|node| is_navigatable(&cx.tree, &cx.style, *node, lock_focus_to))
                            .unwrap_or(Entity::root())
                    };

                    if next_focused != cx.focused {
                        cx.set_focus_pseudo_classes(cx.focused, false, true);
                        cx.set_focus_pseudo_classes(next_focused, true, true);
                        cx.event_queue.push_back(
                            Event::new(WindowEvent::FocusOut)
                                .target(cx.focused)
                                .origin(Entity::root()),
                        );
                        cx.event_queue.push_back(
                            Event::new(WindowEvent::FocusIn)
                                .target(next_focused)
                                .origin(Entity::root()),
                        );

                        cx.focused = next_focused;

                        if let Some(pseudo_classes) = cx.style.pseudo_classes.get_mut(cx.triggered)
                        {
                            pseudo_classes.set(PseudoClassFlags::ACTIVE, false);
                            cx.needs_restyle(cx.triggered);
                        }
                        cx.triggered = Entity::null();
                    }
                }
            }

            if matches!(*code, Code::Enter | Code::NumpadEnter | Code::Space) {
                cx.triggered = cx.focused;
                if let Some(pseudo_classes) = cx.style.pseudo_classes.get_mut(cx.triggered) {
                    pseudo_classes.set(PseudoClassFlags::ACTIVE, true);
                }
                cx.with_current(cx.focused, |cx| cx.emit(WindowEvent::PressDown { mouse: false }));
            }
        }
        WindowEvent::KeyUp(code, _) => {
            meta.target = cx.focused;
            if matches!(code, Code::Enter | Code::NumpadEnter | Code::Space) {
                if cx.focused == cx.triggered {
                    cx.with_current(cx.triggered, |cx| {
                        cx.emit(WindowEvent::Press { mouse: false })
                    });
                }
                if let Some(pseudo_classes) = cx.style.pseudo_classes.get_mut(cx.triggered) {
                    pseudo_classes.set(PseudoClassFlags::ACTIVE, false);
                }
                cx.needs_restyle(cx.triggered);
                cx.triggered = Entity::null();
            }
        }
        WindowEvent::CharInput(_) => {
            meta.target = cx.focused;
        }
        WindowEvent::ImeActivate(_) => {
            meta.target = cx.focused;
        }
        WindowEvent::ImeCommit(_) => {
            meta.target = cx.focused;
        }
        WindowEvent::ImePreedit(_, _) => {
            meta.target = cx.focused;
        }
        WindowEvent::SetImeCursorArea(_, _) => {
            meta.target = cx.focused;
        }
        WindowEvent::WindowFocused(is_focused) => {
            if *is_focused {
                cx.set_focus_pseudo_classes(cx.focused, true, true);
                cx.needs_restyle(cx.focused);
                cx.needs_redraw(cx.focused);
            } else {
                cx.set_focus_pseudo_classes(cx.focused, false, true);
                cx.needs_restyle(cx.focused);

                cx.event_queue.push_back(
                    Event::new(WindowEvent::FocusVisibility(false))
                        .target(cx.focused)
                        .origin(Entity::root()), //.propagate(Propagation::Direct),
                );

                cx.event_queue.push_back(
                    Event::new(WindowEvent::MouseOut).target(cx.hovered).origin(Entity::root()), // .propagate(Propagation::Direct),
                );
            }
        }
        WindowEvent::MouseEnter => {
            if let Some(pseudo_class) = cx.style.pseudo_classes.get_mut(meta.origin) {
                pseudo_class.set(PseudoClassFlags::OVER, true);
            }
        }
        WindowEvent::MouseLeave => {
            if let Some(pseudo_class) = cx.style.pseudo_classes.get_mut(meta.origin) {
                pseudo_class.set(PseudoClassFlags::OVER, false);
            }

            let parent_iter = LayoutParentIterator::new(&cx.tree, cx.hovered);
            for ancestor in parent_iter {
                if let Some(pseudo_classes) = cx.style.pseudo_classes.get_mut(ancestor) {
                    pseudo_classes.set(PseudoClassFlags::HOVER, false);
                    cx.style.needs_restyle(ancestor);
                }
            }

            cx.hovered = Entity::null();
        }

        _ => {}
    }
}

fn mutate_direct_or_up(meta: &mut EventMeta, direct: Entity, up: Entity, root: bool) {
    if direct != Entity::null() {
        meta.target = direct;
        meta.propagation = Propagation::Direct;
    } else if up != Entity::root() || root {
        meta.target = up;
        meta.propagation = Propagation::Up;
    } else {
        meta.consume();
    }
}

fn emit_direct_or_up<M: Any + Send>(
    cx: &mut Context,
    message: M,
    direct: Entity,
    up: Entity,
    root: bool,
) {
    let mut event = Event::new(message);
    mutate_direct_or_up(&mut event.meta, direct, up, root);
    cx.emit_custom(event);
}
