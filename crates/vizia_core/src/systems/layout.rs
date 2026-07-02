use hashbrown::HashSet;
use morphorm::Node;
use vizia_storage::LayoutTreeIterator;

use crate::events::ProxyEvent;
use crate::layout::node::SubLayout;
use crate::prelude::*;
use crate::tree::minimal_layout_dirty_roots;

use super::{clipping_system, text_layout_system, text_system, transform_system};

/// Returns whether an entity is a valid restart point for incremental layout, mirroring
/// morphorm's `NodeExt::is_restartable`: its width and height must both be `Pixels` or `Stretch`.
fn is_restartable(style: &Style, entity: Entity) -> bool {
    let width =
        style.width.get_resolved(entity, &style.custom_units_props).unwrap_or(Units::Stretch(1.0));
    let height =
        style.height.get_resolved(entity, &style.custom_units_props).unwrap_or(Units::Stretch(1.0));
    matches!(width, Units::Pixels(_) | Units::Stretch(_))
        && matches!(height, Units::Pixels(_) | Units::Stretch(_))
}

/// Returns the ancestor that morphorm restarts layout from for a dirty `entity`, mirroring
/// `NodeExt::find_relayout_root`. Used to determine which views undergo layout for the debug overlay.
fn layout_restart_root(style: &Style, tree: &Tree<Entity>, entity: Entity) -> Entity {
    let mut root = match tree.get_layout_parent(entity) {
        Some(parent) => parent,
        None => return entity,
    };
    while let Some(parent) = tree.get_layout_parent(root) {
        if is_restartable(style, root) {
            break;
        }
        root = parent;
    }
    root
}

/// Determines the size and position of views.
///
/// Relayout is performed incrementally: only the subtrees rooted at the dirty nodes (tracked in
/// [`Style::relayout`](crate::style::Style)) are recomputed. Dirty descendants covered by a dirty
/// ancestor are collapsed away via [`minimal_layout_dirty_roots`], and for each remaining root
/// `Node::layout` walks up to the best ancestor to restart layout from before recomputing that
/// subtree. Marking the root dirty (or an empty dirty set) performs a full relayout.
pub(crate) fn layout_system(cx: &mut Context) {
    text_system(cx);

    if cx.style.system_flags.contains(SystemFlags::RELAYOUT) {
        // Determine the minimal set of subtree roots to lay out. An empty dirty set (a relayout
        // flag raised without a specific entity) falls back to a full relayout from the root.
        let relayout_roots: Vec<Entity> = if cx.style.relayout.is_empty() {
            vec![Entity::root()]
        } else {
            let dirty = std::mem::take(&mut cx.style.relayout);
            minimal_layout_dirty_roots(&cx.tree, &dirty).into_iter().collect()
        };
        cx.style.relayout.clear();

        // Debug overlay: record which views undergo layout (the subtree of each restart root).
        // The overlay persists until the next layout pass, so schedule a redraw of the previously
        // highlighted views to erase their outlines before recording the new set.
        if cx.style.debug_layout {
            for entity in std::mem::take(&mut cx.style.laid_out) {
                cx.needs_redraw(entity);
            }
            let mut laid_out = HashSet::new();
            for &root in &relayout_roots {
                let restart = layout_restart_root(&cx.style, &cx.tree, root);
                for entity in LayoutTreeIterator::subtree(&cx.tree, restart) {
                    laid_out.insert(entity);
                }
            }
            cx.style.laid_out = laid_out;
        }

        for entity in relayout_roots {
            entity.layout(
                &mut cx.cache,
                &cx.tree,
                &cx.style,
                &mut SubLayout {
                    text_context: &mut cx.text_context,
                    resource_manager: &cx.resource_manager,
                },
            );
        }

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

                        // if new_bounds != *bounds && *bounds != BoundingBox::default() {
                        //     cx.needs_redraw();
                        // }

                        *bounds = new_bounds;
                    }
                }
            }

            if let Some(geo) = cx.cache.geo_changed.get(entity).copied() {
                if !geo.is_empty() && cx.style.text.contains(entity) {
                    cx.style.needs_text_layout(entity);
                }

                if !geo.is_empty()
                // && cx.style.text.get(entity).is_some()
                {
                    cx.needs_redraw();

                    cx.needs_retransform();
                    cx.needs_reclip();

                    // If the entity clips its children (Overflow::Hidden or ClipPath::Shape)
                    // and its geometry changed, the clip path has changed too, so invalidate
                    // all descendants' cached draw_bounds.
                    if matches!(cx.style.overflowx.get(entity), Some(Overflow::Hidden))
                        || matches!(cx.style.overflowy.get(entity), Some(Overflow::Hidden))
                        || matches!(cx.style.clip_path.get(entity), Some(ClipPath::Shape(_)))
                    {
                        for descendant in LayoutTreeIterator::subtree(cx.tree, entity).skip(1) {
                            cx.cache.draw_bounds.remove(descendant);
                        }
                    }
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
        // However, skip this during capture to avoid sending stale coordinates that interfere with drag operations.
        if let Some(proxy) = &cx.event_proxy {
            if cx.captured.is_null() {
                let event =
                    ProxyEvent::new(WindowEvent::MouseMove(cx.mouse.cursor_x, cx.mouse.cursor_y))
                        .target(Entity::root())
                        .origin(Entity::root())
                        .propagate(Propagation::Up);

                proxy.send(event).expect("Failed to send event");
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
