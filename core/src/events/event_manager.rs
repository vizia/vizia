use crate::{Context, Entity, Event, InternalEvent, Propagation, Tree, TreeExt};

/// Dispatches events to views.
///
/// The [EventManager] is responsible for taking the events in the event queue in state
/// and dispatching them to widgets based on the target and propagation metadata of the event.
/// The is struct is used internally by the application and should not be constructed directly.
#[doc(hidden)]
pub struct EventManager {
    // Queue of events to be processed
    event_queue: Vec<Event>,

    // A copy of the tree for iteration
    tree: Tree,
}

impl EventManager {
    pub fn new() -> Self {
        EventManager { event_queue: Vec::new(), tree: Tree::new() }
    }

    /// Flush the event queue, dispatching events to their targets.
    pub fn flush_events(&mut self, context: &mut Context) {
        // Clear the event queue in the event manager
        self.event_queue.clear();

        //state.removed_entities.clear();

        // Move events from state to event manager
        self.event_queue.extend(context.event_queue.drain(0..));

        if context.tree.changed {
            self.tree = context.tree.clone();
        }

        // Loop over the events in the event queue
        'events: for event in self.event_queue.iter_mut() {
            // handle internal events
            if let Some(msg) = event.message.downcast() {
                match msg {
                    InternalEvent::Redraw => context.style.needs_redraw = true,
                    InternalEvent::LoadImage { path, image, policy } => {
                        if let Some(image) = image.lock().unwrap().take() {
                            context.load_image(path.clone(), image, *policy);
                        }
                    }
                }
            }

            // if event.trace {
            //     println!("Event: {:?}", event);
            // }

            // Send events to any listeners
            let listeners =
                context.listeners.iter().map(|(entity, _)| *entity).collect::<Vec<Entity>>();
            for entity in listeners {
                if let Some(listener) = context.listeners.remove(&entity) {
                    if let Some(mut event_handler) = context.views.remove(&entity) {
                        let prev = context.current;
                        context.current = entity;
                        (listener)(event_handler.as_mut(), context, event);
                        context.current = prev;

                        context.views.insert(entity, event_handler);
                    }

                    context.listeners.insert(entity, listener);
                }

                if event.consumed {
                    continue 'events;
                }
            }

            // Define the target to prevent multiple mutable borrows error
            let target = event.target;

            // Send event to target
            visit_entity(context, target, event);

            if event.consumed {
                continue 'events;
            }

            // if event.trace {
            //     println!("Target: {} Parents: {:?} Tree: {:?}", target, target.parent_iter(&self.tree).collect::<Vec<_>>(), self.tree.parent);
            // }

            // Propagate up from target to root (not including target)
            if event.propagation == Propagation::Up {
                // Walk up the tree from parent to parent
                for entity in target.parent_iter(&self.tree) {
                    // Skip the target entity
                    if entity == event.target {
                        continue;
                    }

                    // Send event to all entities before the target
                    visit_entity(context, entity, event);

                    // Skip to the next event if the current event is consumed
                    if event.consumed {
                        continue 'events;
                    }
                }
            }

            if event.propagation == Propagation::Subtree {
                for entity in target.branch_iter(&self.tree) {
                    // Skip the target entity
                    if entity == event.target {
                        continue;
                    }

                    // Send event to all entities before the target
                    visit_entity(context, entity, event);

                    // Skip to the next event if the current event is consumed
                    if event.consumed {
                        continue 'events;
                    }
                }
            }
        }
    }
}

fn visit_entity(context: &mut Context, entity: Entity, event: &mut Event) {
    if let Some(mut view) = context.views.remove(&entity) {
        let prev = context.current;
        context.current = entity;
        view.event(context, event);
        context.current = prev;

        context.views.insert(entity, view);
    }

    if let Some(mut model_list) = context.data.remove(entity) {
        for (_, model) in model_list.data.iter_mut() {
            // if event.trace {
            //     println!("Event: {:?} -> Model {:?}", event, ty);
            // }
            let prev = context.current;
            context.current = entity;
            model.event(context, event);
            context.current = prev;
        }

        context.data.insert(entity, model_list).expect("Failed to insert data");
    }
}
