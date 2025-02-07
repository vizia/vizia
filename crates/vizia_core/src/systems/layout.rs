use morphorm::Node;
use vizia_storage::LayoutTreeIterator;

use crate::layout::node::SubLayout;
use crate::prelude::*;

use super::{clipping_system, text_layout_system, text_system, transform_system};

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

        let cx = &mut EventContext::new(cx);

        let iter = LayoutTreeIterator::full(cx.tree);

        for entity in iter {
            cx.current = entity;
            if cx.style.display.get(entity).copied().unwrap_or_default() == Display::None {
                continue;
            }
            // Morphorm produces relative positions so convert to absolute.
            if let Some(parent) = cx.tree.get_layout_parent(entity) {
                let parent_bounds = cx.cache.get_bounds(parent);
                if let Some(bounds) = cx.cache.bounds.get_mut(entity) {
                    if let Some(relative_bounds) = cx.cache.relative_bounds.get(entity) {
                        let x = relative_bounds.x + parent_bounds.x;
                        let y = relative_bounds.y + parent_bounds.y;
                        let w = relative_bounds.w;
                        let h = relative_bounds.h;

                        let mut geo_changed = GeoChanged::empty();

                        if x != bounds.x {
                            geo_changed.set(GeoChanged::POSX_CHANGED, true);
                        }

                        if y != bounds.y {
                            geo_changed.set(GeoChanged::POSY_CHANGED, true);
                        }

                        if w != bounds.w {
                            geo_changed.set(GeoChanged::WIDTH_CHANGED, true);
                            cx.cache.path.remove(entity);
                        }

                        if h != bounds.h {
                            geo_changed.set(GeoChanged::HEIGHT_CHANGED, true);
                            cx.cache.path.remove(entity);
                        }

                        if let Some(geo) = cx.cache.geo_changed.get_mut(entity) {
                            *geo = geo_changed;
                        }

                        let new_bounds = BoundingBox { x, y, w, h };

                        *bounds = new_bounds;
                    }
                }
            }

            if let Some(geo) = cx.cache.geo_changed.get(entity).copied() {
                if !geo.is_empty() {
                    cx.needs_redraw();
                    cx.style.needs_text_layout(entity);
                    cx.style.needs_retransform(entity, &cx.tree);
                    cx.style.needs_reclip(entity, &cx.tree);

                    let mut event = Event::new(WindowEvent::GeometryChanged(geo))
                        .target(entity)
                        .origin(entity)
                        .propagate(Propagation::Direct);
                    visit_entity(cx, entity, &mut event);

                    // A relayout, retransform, or reclip, can cause the element under the cursor to change. So we push a mouse move event here to force
                    // a new event cycle and the hover system to trigger.
                    if let Some(proxy) = &cx.event_proxy {
                        let event = Event::new(WindowEvent::MouseMove(
                            cx.mouse.cursor_x,
                            cx.mouse.cursor_y,
                        ))
                        .target(Entity::root())
                        .origin(Entity::root())
                        .propagate(Propagation::Up);

                        proxy.send(event).expect("Failed to send event");
                    }
                }
            }

            if let Some(geo) = cx.cache.geo_changed.get_mut(entity) {
                *geo = GeoChanged::empty();
            }
        }

        cx.style.system_flags.set(SystemFlags::RELAYOUT, false);
    }

    text_layout_system(cx);
    transform_system(cx);
    clipping_system(cx);
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
