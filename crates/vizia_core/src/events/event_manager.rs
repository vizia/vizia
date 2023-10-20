use crate::context::{InternalEvent, ResourceContext};
use crate::events::EventMeta;
use crate::prelude::*;
use crate::style::{Abilities, PseudoClassFlags};
#[cfg(debug_assertions)]
use crate::systems::compute_matched_rules;
use crate::systems::hover_system;
use crate::tree::{focus_backward, focus_forward, is_navigatable};
use instant::{Duration, Instant};
#[cfg(debug_assertions)]
use log::debug;
use std::any::Any;
use vizia_id::GenerationalId;
use vizia_storage::TreeIterator;
use vizia_storage::{LayoutParentIterator, TreeExt};

const DOUBLE_CLICK_INTERVAL: Duration = Duration::from_millis(500);

/// Dispatches events to views and models.
///
/// The [EventManager] is responsible for taking the events in the event queue in context
/// and dispatching them to views and models based on the target and propagation metadata of the event.
#[doc(hidden)]
pub(crate) struct EventManager {
    // Queue of events to be processed
    event_queue: Vec<Event>,
}

impl EventManager {
    pub fn new() -> Self {
        EventManager { event_queue: Vec::with_capacity(10) }
    }

    /// Flush the event queue, dispatching events to their targets.
    /// Returns whether there are still more events to process, i.e. the event handlers sent events.
    pub(crate) fn flush_events(&mut self, cx: &mut Context) -> bool {
        // Clear the event queue in the event manager
        self.event_queue.clear();

        // Move events from state to event manager
        self.event_queue.extend(cx.event_queue.drain(0..));

        // Loop over the events in the event queue
        'events: for event in self.event_queue.iter_mut() {
            // Handle internal events
            event.map(|internal_event, _| match internal_event {
                InternalEvent::Redraw => cx.needs_redraw(),
                InternalEvent::LoadImage { path, image, policy } => {
                    if let Some(image) = image.lock().unwrap().take() {
                        ResourceContext::new(cx).load_image(path.clone(), image, *policy);
                    }
                }
            });

            // Send events to any global listeners
            let mut global_listeners = vec![];
            std::mem::swap(&mut cx.global_listeners, &mut global_listeners);
            for listener in &global_listeners {
                cx.with_current(Entity::root(), |cx| listener(&mut EventContext::new(cx), event));
            }
            std::mem::swap(&mut cx.global_listeners, &mut global_listeners);

            // Send events to any local listeners
            let listeners = cx.listeners.keys().copied().collect::<Vec<Entity>>();
            for entity in listeners {
                if let Some(listener) = cx.listeners.remove(&entity) {
                    if let Some(mut event_handler) = cx.views.remove(&entity) {
                        cx.with_current(entity, |cx| {
                            (listener)(event_handler.as_mut(), &mut EventContext::new(cx), event);
                        });

                        cx.views.insert(entity, event_handler);
                    }

                    cx.listeners.insert(entity, listener);
                }

                if event.meta.consumed {
                    continue 'events;
                }
            }

            // Handle state updates for window events
            event.map(|window_event, meta| {
                if meta.origin == Entity::root() {
                    internal_state_updates(cx, window_event, meta);
                }
            });

            // Skip to next event if the current event was consumed when handling state updates.
            if event.meta.consumed {
                continue 'events;
            }

            let cx = &mut EventContext::new(cx);

            // Copy the target to prevent multiple mutable borrows error.
            let target = event.meta.target;

            // Send event to target
            visit_entity(cx, target, event);

            // Skip to next event if the current event was consumed.
            if event.meta.consumed {
                continue 'events;
            }

            // Propagate up from target to root (not including target)
            if event.meta.propagation == Propagation::Up {
                // Create a parent iterator and skip the first element which is the target.
                let iter = target.parent_iter(cx.tree).skip(1);
                // Walk up the tree from parent to parent
                for entity in iter {
                    // Send event to all entities before the target
                    visit_entity(cx, entity, event);

                    // Skip to the next event if the current event is consumed
                    if event.meta.consumed {
                        continue 'events;
                    }
                }
            }

            if event.meta.propagation == Propagation::Subtree {
                // Create a parent iterator and skip the first element which is the target.
                let iter = target.branch_iter(cx.tree).skip(1);
                // Walk down the subtree
                for entity in iter {
                    // Send event to all entities in the subtree after the target
                    visit_entity(cx, entity, event);

                    // Skip to the next event if the current event is consumed
                    if event.meta.consumed {
                        continue 'events;
                    }
                }
            }
        }

        // Return true if there are new events in the queue
        !cx.event_queue.is_empty()
    }
}

fn visit_entity(cx: &mut EventContext, entity: Entity, event: &mut Event) {
    // Send event to models attached to the entity
    if let Some(ids) = cx
        .data
        .get(&entity)
        .map(|model_data_store| model_data_store.models.keys().cloned().collect::<Vec<_>>())
    {
        for id in ids {
            if let Some(mut model) = cx
                .data
                .get_mut(&entity)
                .and_then(|model_data_store| model_data_store.models.remove(&id))
            {
                cx.current = entity;

                model.event(cx, event);

                cx.data
                    .get_mut(&entity)
                    .and_then(|model_data_store| model_data_store.models.insert(id, model));
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

/// Update the internal state of the context based on received window event and emit window event to relevant target.
fn internal_state_updates(context: &mut Context, window_event: &WindowEvent, meta: &mut EventMeta) {
    match window_event {
        WindowEvent::Drop(drop_data) => {
            context.drop_data = Some(drop_data.clone());
        }

        WindowEvent::MouseMove(x, y) => {
            if !x.is_nan() && !y.is_nan() {
                context.mouse.previous_cursorx = context.mouse.cursorx;
                context.mouse.previous_cursory = context.mouse.cursory;
                context.mouse.cursorx = *x;
                context.mouse.cursory = *y;

                mutate_direct_or_up(meta, context.captured, context.hovered, false);
            }

            // if context.mouse.cursorx != context.mouse.previous_cursorx
            //     || context.mouse.cursory != context.mouse.previous_cursory
            // {
            // }

            hover_system(context);
            // if let Some(dropped_file) = context.dropped_file.take() {
            //     emit_direct_or_up(
            //         context,
            //         WindowEvent::DroppedFile(dropped_file),
            //         context.captured,
            //         context.hovered,
            //         true,
            //     );
            // }
        }
        WindowEvent::MouseDown(button) => {
            // do direct state-updates
            match button {
                MouseButton::Left => {
                    context.mouse.left.state = MouseButtonState::Pressed;

                    context.mouse.left.pos_down = (context.mouse.cursorx, context.mouse.cursory);
                    context.mouse.left.pressed = context.hovered;
                    context.triggered = context.hovered;

                    let disabled =
                        context.style.disabled.get(context.hovered).copied().unwrap_or_default();

                    if let Some(pseudo_classes) =
                        context.style.pseudo_classes.get_mut(context.triggered)
                    {
                        if !disabled {
                            pseudo_classes.set(PseudoClassFlags::ACTIVE, true);
                        }
                    }
                    let focusable = context
                        .style
                        .abilities
                        .get(context.hovered)
                        .filter(|abilities| abilities.contains(Abilities::FOCUSABLE))
                        .is_some();

                    // Reset drag data
                    context.drop_data = None;

                    context.with_current(
                        if focusable { context.hovered } else { context.focused },
                        |cx| cx.focus_with_visibility(false),
                    );
                }
                MouseButton::Right => {
                    context.mouse.right.state = MouseButtonState::Pressed;
                    context.mouse.right.pos_down = (context.mouse.cursorx, context.mouse.cursory);
                    context.mouse.right.pressed = context.hovered;
                }
                MouseButton::Middle => {
                    context.mouse.middle.state = MouseButtonState::Pressed;
                    context.mouse.middle.pos_down = (context.mouse.cursorx, context.mouse.cursory);
                    context.mouse.middle.pressed = context.hovered;
                }
                _ => {}
            }

            // emit trigger events
            if matches!(button, MouseButton::Left) {
                emit_direct_or_up(
                    context,
                    WindowEvent::PressDown { mouse: true },
                    context.captured,
                    context.triggered,
                    true,
                );
            }

            // track double/triple -click
            let new_click_time = Instant::now();
            let click_duration = new_click_time - context.click_time;
            let new_click_pos = (context.mouse.cursorx, context.mouse.cursory);
            if click_duration <= DOUBLE_CLICK_INTERVAL
                && new_click_pos == context.click_pos
                && *button == context.click_button
            {
                if context.clicks <= 2 {
                    context.clicks += 1;
                    let event = if context.clicks == 3 {
                        WindowEvent::MouseTripleClick(*button)
                    } else {
                        WindowEvent::MouseDoubleClick(*button)
                    };
                    meta.consume();
                    emit_direct_or_up(context, event, context.captured, context.hovered, true);
                }
            } else {
                context.clicks = 1;
            }
            context.click_time = new_click_time;
            context.click_pos = new_click_pos;
            context.click_button = *button;
            mutate_direct_or_up(meta, context.captured, context.hovered, true);
        }
        WindowEvent::MouseUp(button) => {
            match button {
                MouseButton::Left => {
                    context.mouse.left.pos_up = (context.mouse.cursorx, context.mouse.cursory);
                    context.mouse.left.released = context.hovered;
                    context.mouse.left.state = MouseButtonState::Released;
                }
                MouseButton::Right => {
                    context.mouse.right.pos_up = (context.mouse.cursorx, context.mouse.cursory);
                    context.mouse.right.released = context.hovered;
                    context.mouse.right.state = MouseButtonState::Released;
                }
                MouseButton::Middle => {
                    context.mouse.middle.pos_up = (context.mouse.cursorx, context.mouse.cursory);
                    context.mouse.middle.released = context.hovered;
                    context.mouse.middle.state = MouseButtonState::Released;
                }
                _ => {}
            }

            if matches!(button, MouseButton::Left) {
                if context.hovered == context.triggered {
                    emit_direct_or_up(
                        context,
                        WindowEvent::Press { mouse: true },
                        context.captured,
                        context.triggered,
                        true,
                    );
                }

                if let Some(pseudo_classes) =
                    context.style.pseudo_classes.get_mut(context.triggered)
                {
                    pseudo_classes.set(PseudoClassFlags::ACTIVE, false);
                }
                context.needs_restyle();

                context.triggered = Entity::null();
            }

            mutate_direct_or_up(meta, context.captured, context.hovered, true);
        }
        WindowEvent::MouseScroll(_, _) => {
            meta.target = context.hovered;
        }
        WindowEvent::KeyDown(code, _) => {
            meta.target = context.focused;

            #[cfg(debug_assertions)]
            if *code == Code::KeyP && context.modifiers.contains(Modifiers::CTRL) {
                for entity in TreeIterator::full(&context.tree) {
                    if let Some(model_data_store) = context.data.get(&entity) {
                        if !model_data_store.models.is_empty() {
                            println!("Models for {}", entity);
                            for (_, model) in model_data_store.models.iter() {
                                println!("M: {:?}", model.name())
                            }
                        }

                        if !model_data_store.stores.is_empty() {
                            println!("Stores for {}", entity);
                            for (_, store) in model_data_store.stores.iter() {
                                println!(
                                    "S: [{}] - Observers {:?}",
                                    store.name(),
                                    store.observers()
                                )
                            }
                        }
                    }
                }
            }

            #[cfg(debug_assertions)]
            if *code == Code::KeyI && context.modifiers.contains(Modifiers::CTRL) {
                println!("Entity tree");
                let (tree, views, cache) = (&context.tree, &context.views, &context.cache);
                let has_next_sibling = |entity| tree.get_next_sibling(entity).is_some();
                let root_indents = |entity: Entity| {
                    entity
                        .parent_iter(tree)
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
                        let classes = context.style.classes.get(entity);
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
                        context.bindings.get(&entity).map(|binding| format!("{:?}", binding))
                    {
                        println!(
                            "{}{} binding observing {}",
                            indents(entity),
                            entity,
                            binding_name
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
                && context.modifiers == Modifiers::CTRL | Modifiers::SHIFT | Modifiers::ALT
            {
                let mut result = vec![];
                compute_matched_rules(context, context.hovered, &mut result);

                let entity = context.hovered;
                debug!("/* Matched rules for Entity: {} Parent: {:?} View: {} posx: {} posy: {} width: {} height: {}",
                    entity,
                    entity.parent(&context.tree),
                    context
                        .views
                        .get(&entity)
                        .map_or("<None>", |view| view.element().unwrap_or("<Unnamed>")),
                    context.cache.get_posx(entity),
                    context.cache.get_posy(entity),
                    context.cache.get_width(entity),
                    context.cache.get_height(entity)
                );
                for rule in result.into_iter() {
                    for selectors in context.style.rules.iter() {
                        if selectors.0 == rule.0 {
                            debug!("{:?}", selectors.1);
                        }
                    }
                }
            }

            #[cfg(debug_assertions)]
            if *code == Code::KeyT
                && context.modifiers == Modifiers::CTRL | Modifiers::SHIFT | Modifiers::ALT
            {
                debug!("Loaded font face info:");
                for face in context.text_context.font_system().db().faces() {
                    debug!(
                        "family: {:?}\npost_script_name: {:?}\nstyle: {:?}\nweight: {:?}\nstretch: {:?}\nmonospaced: {:?}\n",
                        face.families,
                        face.post_script_name,
                        face.style,
                        face.weight,
                        face.stretch,
                        face.monospaced,
                    );
                }
            }

            if *code == Code::F5 {
                EventContext::new(context).reload_styles().unwrap();
            }

            if *code == Code::Tab {
                let lock_focus_to = context.tree.lock_focus_within(context.focused);
                if context.modifiers.contains(Modifiers::SHIFT) {
                    let prev_focused = if let Some(prev_focused) = focus_backward(
                        &context.tree,
                        &context.style,
                        context.focused,
                        lock_focus_to,
                    ) {
                        prev_focused
                    } else {
                        TreeIterator::full(&context.tree)
                            .filter(|node| {
                                is_navigatable(&context.tree, &context.style, *node, lock_focus_to)
                            })
                            .next_back()
                            .unwrap_or(Entity::root())
                    };

                    if prev_focused != context.focused {
                        context.event_queue.push_back(
                            Event::new(WindowEvent::FocusOut)
                                .target(context.focused)
                                .origin(Entity::root()),
                        );
                        context.event_queue.push_back(
                            Event::new(WindowEvent::FocusIn)
                                .target(prev_focused)
                                .origin(Entity::root()),
                        );

                        if let Some(pseudo_classes) =
                            context.style.pseudo_classes.get_mut(context.triggered)
                        {
                            pseudo_classes.set(PseudoClassFlags::ACTIVE, false);
                        }
                        context.needs_restyle();
                        context.triggered = Entity::null();
                    }
                } else {
                    let next_focused = if let Some(next_focused) =
                        focus_forward(&context.tree, &context.style, context.focused, lock_focus_to)
                    {
                        next_focused
                    } else {
                        TreeIterator::full(&context.tree)
                            .find(|node| {
                                is_navigatable(&context.tree, &context.style, *node, lock_focus_to)
                            })
                            .unwrap_or(Entity::root())
                    };

                    if next_focused != context.focused {
                        context.event_queue.push_back(
                            Event::new(WindowEvent::FocusOut)
                                .target(context.focused)
                                .origin(Entity::root()),
                        );
                        context.event_queue.push_back(
                            Event::new(WindowEvent::FocusIn)
                                .target(next_focused)
                                .origin(Entity::root()),
                        );

                        if let Some(pseudo_classes) =
                            context.style.pseudo_classes.get_mut(context.triggered)
                        {
                            pseudo_classes.set(PseudoClassFlags::ACTIVE, false);
                        }
                        context.needs_restyle();
                        context.triggered = Entity::null();
                    }
                }
            }

            if matches!(*code, Code::Enter | Code::NumpadEnter | Code::Space) {
                context.triggered = context.focused;
                if let Some(pseudo_classes) =
                    context.style.pseudo_classes.get_mut(context.triggered)
                {
                    pseudo_classes.set(PseudoClassFlags::ACTIVE, true);
                }
                context.with_current(context.focused, |cx| {
                    cx.emit(WindowEvent::PressDown { mouse: false })
                });
            }
        }
        WindowEvent::KeyUp(code, _) => {
            meta.target = context.focused;
            if matches!(code, Code::Enter | Code::NumpadEnter | Code::Space) {
                if context.focused == context.triggered {
                    context.with_current(context.triggered, |cx| {
                        cx.emit(WindowEvent::Press { mouse: false })
                    });
                }
                if let Some(pseudo_classes) =
                    context.style.pseudo_classes.get_mut(context.triggered)
                {
                    pseudo_classes.set(PseudoClassFlags::ACTIVE, false);
                }
                context.needs_restyle();
                context.triggered = Entity::null();
            }
        }
        WindowEvent::CharInput(_) => {
            meta.target = context.focused;
        }
        WindowEvent::FocusOut => {
            context.set_focus_pseudo_classes(context.focused, false, true);
            context.focused = Entity::null();
        }
        WindowEvent::FocusIn => {
            context.focused = meta.target;
            context.set_focus_pseudo_classes(context.focused, true, true);
        }
        WindowEvent::MouseEnter => {
            if let Some(pseudo_class) = context.style.pseudo_classes.get_mut(Entity::root()) {
                pseudo_class.set(PseudoClassFlags::OVER, true);
            }
        }
        WindowEvent::MouseLeave => {
            if let Some(pseudo_class) = context.style.pseudo_classes.get_mut(Entity::root()) {
                pseudo_class.set(PseudoClassFlags::OVER, false);
            }

            let parent_iter = LayoutParentIterator::new(&context.tree, Some(context.hovered));
            for ancestor in parent_iter {
                if let Some(pseudo_classes) = context.style.pseudo_classes.get_mut(ancestor) {
                    pseudo_classes.set(PseudoClassFlags::HOVER, false);
                    context.style.needs_restyle();
                }
            }

            context.hovered = Entity::null();
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
    context: &mut Context,
    message: M,
    direct: Entity,
    up: Entity,
    root: bool,
) {
    let mut event = Event::new(message);
    mutate_direct_or_up(&mut event.meta, direct, up, root);
    context.emit_custom(event);
}
