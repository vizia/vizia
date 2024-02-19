use morphorm::Node;

use crate::layout::node::SubLayout;
use crate::prelude::*;

use super::{text_layout_system, text_system};

/// Determines the size and position of views.
/// TODO: Currently relayout is done on an entire tree rather than incrementally.
/// Incremental relayout can be done by keeping a list of nodes that need relayout,
/// and when a node undergoes relayout remove the descendants that have been processed from the list,
/// then continue relayout on the remaining nodes in the list.
pub(crate) fn layout_system(cx: &mut Context) {
    text_system(cx);

    if cx.style.system_flags.contains(SystemFlags::RELAYOUT) {
        // Perform layout on the whole tree.
        Entity::root().layout(
            &mut cx.cache,
            &cx.tree,
            &cx.style,
            &mut SubLayout {
                text_context: &mut cx.text_context,
                resource_manager: &cx.resource_manager,
            },
        );

        // If layout has changed then redraw
        cx.style.system_flags.set(SystemFlags::REDRAW, true);

        let cx = &mut EventContext::new(cx);

        for entity in cx.tree.into_iter() {
            // if let Some(paragraph) =

            // Morphorm produces relative positions so convert to absolute.
            if let Some(parent) = cx.tree.get_layout_parent(entity) {
                let parent_bounds = cx.cache.get_bounds(parent);
                if let Some(bounds) = cx.cache.bounds.get_mut(entity) {
                    if let Some(relative_position) = cx.cache.relative_position.get(entity) {
                        bounds.x = relative_position.x + parent_bounds.x;
                        bounds.y = relative_position.y + parent_bounds.y;
                    }
                }
            }

            if let Some(geo) = cx.cache.geo_changed.get(entity).copied() {
                if !geo.is_empty()
                // && cx.style.text.get(entity).is_some()
                {
                    cx.style.needs_text_layout(entity);
                }

                // TODO: Use geo changed to determine whether an entity needs to be redrawn.

                if !geo.is_empty() {
                    let mut event = Event::new(WindowEvent::GeometryChanged(geo))
                        .target(entity)
                        .origin(entity)
                        .propagate(Propagation::Direct);
                    visit_entity(cx, entity, &mut event);
                }
            }

            if let Some(geo) = cx.cache.geo_changed.get_mut(entity) {
                *geo = GeoChanged::empty();
            }
        }

        // A relayout, retransform, or reclip, can cause the element under the cursor to change. So we push a mouse move event here to force
        // a new event cycle and the hover system to trigger.
        if let Some(proxy) = &cx.event_proxy {
            let event = Event::new(WindowEvent::MouseMove(f32::NAN, f32::NAN))
                .target(Entity::root())
                .origin(Entity::root())
                .propagate(Propagation::Up);

            proxy.send(event).expect("Failed to send event");
        }

        cx.style.system_flags.set(SystemFlags::RELAYOUT, false);
    }

    text_layout_system(cx);
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
