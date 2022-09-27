use crate::context::InternalEvent;
use crate::events::EventMeta;
use crate::prelude::*;
use crate::systems::{compute_matched_rules, hover_system};
use crate::tree::{focus_backward, focus_forward, is_navigatable};
use instant::{Duration, Instant};
use std::any::Any;
use vizia_id::GenerationalId;
use vizia_storage::TreeExt;
use vizia_storage::TreeIterator;

const DOUBLE_CLICK_INTERVAL: Duration = Duration::from_millis(500);

/// Dispatches events to views and models.
///
/// The [EventManager] is responsible for taking the events in the event queue in context
/// and dispatching them to views and models based on the target and propagation metadata of the event.
#[doc(hidden)]
pub struct EventManager {
    // Queue of events to be processed
    event_queue: Vec<Event>,

    // A copy of the tree for iteration
    tree: Tree<Entity>,
}

impl EventManager {
    pub fn new() -> Self {
        EventManager { event_queue: Vec::new(), tree: Tree::new() }
    }

    /// Flush the event queue, dispatching events to their targets.
    /// Returns whether there are still more events to process, i.e. the event handlers sent events.
    pub fn flush_events(&mut self, context: &mut Context) -> bool {
        // Clear the event queue in the event manager
        self.event_queue.clear();

        // Move events from state to event manager
        self.event_queue.extend(context.event_queue.drain(0..));

        if context.tree.changed {
            self.tree = context.tree.clone();
        }

        // Loop over the events in the event queue
        'events: for event in self.event_queue.iter_mut() {
            // handle internal events
            event.map(|internal_event, _| match internal_event {
                InternalEvent::Redraw => context.need_redraw(),
                InternalEvent::LoadImage { path, image, policy } => {
                    if let Some(image) = image.lock().unwrap().take() {
                        context.load_image(path.clone(), image, *policy);
                    }
                }
            });
            // handle state updates for window events
            event.map(|window_event, meta| {
                if meta.origin == Entity::root() {
                    internal_state_updates(context, &window_event, meta);
                }
            });

            if event.meta.consumed {
                continue 'events;
            }

            // Send events to any global listeners
            let mut global_listeners = vec![];
            std::mem::swap(&mut context.global_listeners, &mut global_listeners);
            for listener in &global_listeners {
                context
                    .with_current(Entity::root(), |cx| listener(&mut EventContext::new(cx), event));
            }
            std::mem::swap(&mut context.global_listeners, &mut global_listeners);

            // Send events to any local listeners
            let listeners =
                context.listeners.iter().map(|(entity, _)| *entity).collect::<Vec<Entity>>();
            for entity in listeners {
                if let Some(listener) = context.listeners.remove(&entity) {
                    if let Some(mut event_handler) = context.views.remove(&entity) {
                        context.with_current(entity, |context| {
                            (listener)(
                                event_handler.as_mut(),
                                &mut EventContext::new(context),
                                event,
                            );
                        });

                        context.views.insert(entity, event_handler);
                    }

                    context.listeners.insert(entity, listener);
                }

                if event.meta.consumed {
                    continue 'events;
                }
            }

            // Define the target to prevent multiple mutable borrows error
            let target = event.meta.target;

            // Send event to target
            visit_entity(context, target, event);

            if event.meta.consumed {
                continue 'events;
            }

            // Propagate up from target to root (not including target)
            if event.meta.propagation == Propagation::Up {
                // Walk up the tree from parent to parent
                for entity in target.parent_iter(&self.tree) {
                    // Skip the target entity
                    if entity == event.meta.target {
                        continue;
                    }

                    // Send event to all entities before the target
                    visit_entity(context, entity, event);

                    // Skip to the next event if the current event is consumed
                    if event.meta.consumed {
                        continue 'events;
                    }
                }
            }

            if event.meta.propagation == Propagation::Subtree {
                for entity in target.branch_iter(&self.tree) {
                    // Skip the target entity
                    if entity == event.meta.target {
                        continue;
                    }

                    // Send event to all entities before the target
                    visit_entity(context, entity, event);

                    // Skip to the next event if the current event is consumed
                    if event.meta.consumed {
                        continue 'events;
                    }
                }
            }
        }

        !context.event_queue.is_empty()
    }
}

fn visit_entity(cx: &mut Context, entity: Entity, event: &mut Event) {
    if let Some(mut view) = cx.views.remove(&entity) {
        cx.with_current(entity, |cx| {
            view.event(&mut EventContext::new(cx), event);
        });

        cx.views.insert(entity, view);
    }

    if let Some(ids) = cx.data.get(entity).and_then(|model_data_store| {
        Some(model_data_store.models.keys().cloned().collect::<Vec<_>>())
    }) {
        for id in ids {
            if let Some(mut model) = cx
                .data
                .get_mut(entity)
                .and_then(|model_data_store| model_data_store.models.remove(&id))
            {
                let mut context = EventContext::new(cx);
                context.current = entity;

                model.event(&mut context, event);

                cx.data
                    .get_mut(entity)
                    .and_then(|model_data_store| model_data_store.models.insert(id, model));
            }
        }
    }
}

fn internal_state_updates(context: &mut Context, window_event: &WindowEvent, meta: &mut EventMeta) {
    match window_event {
        WindowEvent::MouseMove(x, y) => {
            context.mouse.previous_cursorx = context.mouse.cursorx;
            context.mouse.previous_cursory = context.mouse.cursory;
            context.mouse.cursorx = *x;
            context.mouse.cursory = *y;

            hover_system(context);
            mutate_direct_or_up(meta, context.captured, context.hovered, false);
        }
        WindowEvent::MouseDown(button) => {
            // do direct state-updates
            match button {
                MouseButton::Left => {
                    context.mouse.left.state = MouseButtonState::Pressed;

                    context.mouse.left.pos_down = (context.mouse.cursorx, context.mouse.cursory);
                    context.mouse.left.pressed = context.hovered;
                    context.triggered = context.hovered;
                    let focusable = context
                        .style
                        .abilities
                        .get(context.hovered)
                        .filter(|abilities| abilities.contains(Abilities::FOCUSABLE))
                        .is_some();

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
                    WindowEvent::TriggerDown { mouse: true },
                    context.captured,
                    context.triggered,
                    true,
                );
            }

            // track double-click
            let new_click_time = Instant::now();
            let click_duration = new_click_time - context.click_time;
            let new_click_pos = (context.mouse.cursorx, context.mouse.cursory);
            if click_duration <= DOUBLE_CLICK_INTERVAL && new_click_pos == context.click_pos {
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
                emit_direct_or_up(
                    context,
                    WindowEvent::TriggerUp { mouse: true },
                    context.captured,
                    context.triggered,
                    true,
                );
            }

            mutate_direct_or_up(meta, context.captured, context.hovered, true);
        }
        WindowEvent::MouseScroll(_, _) => {
            meta.target = context.hovered;
        }
        WindowEvent::KeyDown(code, _) => {
            meta.target = context.focused;

            #[cfg(debug_assertions)]
            if *code == Code::KeyH {
                for entity in context.tree.into_iter() {
                    println!(
                        "Entity: {} Parent: {:?} View: {} posx: {} posy: {} width: {} height: {}",
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
                        println!(
                            "{}{} {} {:?} display={:?} bounds={} clip={}",
                            indents(entity),
                            entity,
                            element_name,
                            cache.get_visibility(entity),
                            cache.get_display(entity),
                            cache.get_bounds(entity),
                            cache.get_clip_region(entity),
                        );
                    } else if let Some(binding_name) =
                        context.bindings.get(&entity).and_then(|binding| binding.name())
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
                compute_matched_rules(context, &context.tree, context.hovered, &mut result);

                let entity = context.hovered;
                println!("/* Matched rules for Entity: {} Parent: {:?} View: {} posx: {} posy: {} width: {} height: {}",
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
                    println!("{}", rule);
                }
            }

            if *code == Code::F5 {
                context.reload_styles().unwrap();
            }

            if *code == Code::Tab {
                let lock_focus_to = context.tree.lock_focus_within(context.focused);
                if context.modifiers.contains(Modifiers::SHIFT) {
                    let prev_focused = if let Some(prev_focused) =
                        focus_backward(&context, context.focused, lock_focus_to)
                    {
                        prev_focused
                    } else {
                        TreeIterator::full(&context.tree)
                            .filter(|node| is_navigatable(&context, *node, lock_focus_to))
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
                    }
                } else {
                    let next_focused = if let Some(next_focused) =
                        focus_forward(&context, context.focused, lock_focus_to)
                    {
                        next_focused
                    } else {
                        TreeIterator::full(&context.tree)
                            .filter(|node| is_navigatable(&context, *node, lock_focus_to))
                            .next()
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
                    }
                }

                context.style.needs_relayout = true;
                context.style.needs_redraw = true;
                context.style.needs_restyle = true;
            }

            if matches!(*code, Code::Enter | Code::NumpadEnter | Code::Space) {
                context.triggered = context.focused;
                context.with_current(context.focused, |cx| {
                    cx.emit(WindowEvent::TriggerDown { mouse: false })
                });
            }
        }
        WindowEvent::KeyUp(code, _) => {
            meta.target = context.focused;
            if matches!(code, Code::Enter | Code::NumpadEnter | Code::Space) {
                context.with_current(context.triggered, |cx| {
                    cx.emit(WindowEvent::TriggerUp { mouse: false })
                });
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
        _ => {}
    }
}

pub fn mutate_direct_or_up(meta: &mut EventMeta, direct: Entity, up: Entity, root: bool) {
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

pub fn emit_direct_or_up<M: Any + Send>(
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
