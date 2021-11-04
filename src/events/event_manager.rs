

// use crate::{BindEvent, Context, Display, Entity, Event, FontOrId, Propagation, State, Tree, TreeExt, Visibility, WindowEvent, entity};


use femtovg::{
    renderer::OpenGl, Canvas,
};

use crate::{Context, Event, EventCtx, Model, Propagation, Tree, TreeExt};


/// Dispatches events to widgets.
/// 
/// The [EventManager] is responsible for taking the events in the event queue in state
/// and dispatching them to widgets based on the target and propagation metadata of the event.
/// The is struct is used internally by the application and should not be constructed directly.
pub struct EventManager {

    // Queue of events to be processed
    event_queue: Vec<Event>,

    // A copy of the tree for iteration
    tree: Tree,
}

impl EventManager {
    pub fn new() -> Self {
        EventManager {
            event_queue: Vec::new(),

            tree: Tree::new(),
        }
    }

    pub fn flush_events(&mut self, context: &mut Context) {

        // Clear the event queue in the event manager
        self.event_queue.clear();

        //state.removed_entities.clear();

        // Move events from state to event manager
        self.event_queue.extend(context.event_queue.drain(0..));

        // Sort the events by order
        //self.event_queue.sort_by_cached_key(|event| event.order);

        self.tree = context.tree.clone();

        // Loop over the events in the event queue
        'events: for event in self.event_queue.iter_mut() {
            //println!("Event: {:?}", event);

            // Define the target to prevent multiple mutable borrows error
            let target = event.target;

            // Direct

            // Send event to target
            if let Some(mut view) = context.views.remove(&event.target) {
                context.current = event.target;
                view.event(&mut EventCtx {
                    tree: &mut context.tree,
                    event_queue: &mut context.event_queue,
                    current: event.target,
                }, event);

                if let Some(mut model_list) = context.data.model_data.remove(event.target) {
                    for model in model_list.iter_mut() {
                        model.event(context, event);
                    }

                    context.data.model_data.insert(event.target, model_list);

                }

                context.views.insert(event.target, view);

                if event.consumed {
                    continue 'events;
                }

            }

            // Propagate up from target to root (not including target)
            if event.propagation == Propagation::Up {
              
                // Walk up the tree from parent to parent
                for entity in target.parent_iter(&self.tree) {
          
                    // Skip the target entity
                    if entity == event.target {
                        continue;
                    }
                
                    // Send event to all entities before the target
                    if let Some(mut view) = context.views.remove(&entity) {
                        context.current = entity;
                        view.event(&mut EventCtx {
                            tree: &mut context.tree,
                            event_queue: &mut context.event_queue,
                            current: entity,
                        }, event);

                        if let Some(mut model_list) = context.data.model_data.remove(entity) {
                            for model in model_list.iter_mut() {
                                model.event(context, event);
                            }
        
                            context.data.model_data.insert(entity, model_list);
        
                        }

                        context.views.insert(entity, view);
                    }

                    // Skip to the next event if the current event is consumed
                    if event.consumed {
                        continue 'events;
                    }
                }
            }
        }
    }
}
