use hashbrown::HashSet;
use morphorm::Node;
use vizia_storage::LayoutTreeIterator;

use crate::cache::CachedData;
use crate::events::ProxyEvent;
use crate::layout::node::SubLayout;
use crate::prelude::*;
use crate::tree::minimal_layout_dirty_roots;

use super::{clipping_system, text_layout_system, text_system, transform_system};

/// Returns whether an entity is a valid restart point for incremental layout, mirroring
/// morphorm's `NodeExt::is_restartable`.
fn is_restartable(style: &Style, entity: Entity) -> bool {
    let stable = |units| matches!(units, Units::Pixels(_) | Units::Stretch(_));
    let width =
        style.width.get_resolved(entity, &style.custom_units_props).unwrap_or(Units::Stretch(1.0));
    let height =
        style.height.get_resolved(entity, &style.custom_units_props).unwrap_or(Units::Stretch(1.0));
    let min_width = style
        .min_width
        .get_resolved(entity, &style.custom_units_props)
        .unwrap_or(Units::Pixels(0.0));
    let max_width = style
        .max_width
        .get_resolved(entity, &style.custom_units_props)
        .unwrap_or(Units::Pixels(f32::MAX));
    let min_height = style
        .min_height
        .get_resolved(entity, &style.custom_units_props)
        .unwrap_or(Units::Pixels(0.0));
    let max_height = style
        .max_height
        .get_resolved(entity, &style.custom_units_props)
        .unwrap_or(Units::Pixels(f32::MAX));

    stable(width)
        && stable(height)
        && stable(min_width)
        && stable(max_width)
        && stable(min_height)
        && stable(max_height)
}

/// Returns the ancestor that morphorm restarts layout from for a dirty `entity`, mirroring
/// `NodeExt::find_relayout_root`. Used to determine which views undergo layout for the debug overlay.
fn layout_restart_root(
    cache: &CachedData,
    style: &Style,
    tree: &Tree<Entity>,
    entity: Entity,
) -> Entity {
    let mut root = match tree.get_layout_parent(entity) {
        Some(parent) => parent,
        None => return entity,
    };
    // If the dirty node is itself absolutely positioned it is out of its parent's flow, so a change
    // to it cannot alter the parent's size — restart from the parent without walking up (mirrors
    // morphorm).
    if style.position_type.get(entity).copied().unwrap_or_default()
        == morphorm::PositionType::Absolute
    {
        return root;
    }
    while let Some(parent) = tree.get_layout_parent(root) {
        // An absolutely-positioned ancestor does not affect its parent's size, but its position can
        // depend on its own size, so restart at its parent.
        if style.position_type.get(root).copied().unwrap_or_default()
            == morphorm::PositionType::Absolute
        {
            root = parent;
            break;
        }
        if is_restartable(style, root) {
            break;
        }
        root = parent;
    }

    // Match Morphorm's first-layout fallback when a non-root restart point has no cached size.
    if tree.get_layout_parent(root).is_some()
        && cache.get_width(root) == 0.0
        && cache.get_height(root) == 0.0
    {
        while let Some(parent) = tree.get_layout_parent(root) {
            root = parent;
        }
    }

    root
}

#[derive(Clone, Copy)]
struct LayoutWork {
    dirty: Entity,
    restart: Entity,
}

fn layout_work(
    cache: &CachedData,
    style: &Style,
    tree: &Tree<Entity>,
    dirty: &HashSet<Entity>,
) -> Vec<LayoutWork> {
    let mut work = Vec::<LayoutWork>::new();

    for dirty in minimal_layout_dirty_roots(tree, dirty) {
        let restart = layout_restart_root(cache, style, tree, dirty);

        if work
            .iter()
            .any(|item| item.restart == restart || restart.is_descendant_of(tree, item.restart))
        {
            continue;
        }

        work.retain(|item| !item.restart.is_descendant_of(tree, restart));
        work.push(LayoutWork { dirty, restart });
    }

    work
}

fn invalidate_geometry_subtrees(
    style: &mut Style,
    tree: &Tree<Entity>,
    geometry_changed: &HashSet<Entity>,
) {
    for root in minimal_layout_dirty_roots(tree, geometry_changed) {
        for entity in LayoutTreeIterator::subtree(tree, root) {
            style.needs_retransform(entity);
            style.needs_reclip(entity);
        }
    }
}

/// Determines the size and position of views.
///
/// Relayout is performed incrementally: only the subtrees rooted at the dirty nodes (tracked in
/// [`Style::relayout`](crate::style::Style)) are recomputed. Dirty descendants covered by a dirty
/// ancestor are collapsed away via [`minimal_layout_dirty_roots`], and for each remaining root
/// `Node::layout` walks up to the best ancestor to restart layout from before recomputing that
/// subtree. Marking the root dirty performs a full relayout.
pub(crate) fn layout_system(cx: &mut Context) {
    text_system(cx);

    if !cx.style.relayout.is_empty() {
        let dirty = std::mem::take(&mut cx.style.relayout);

        // Invalidate every original dirty subtree before collapsing nodes that share a restart
        // root. Morphorm then safely reuses unchanged siblings and repeated identical constraints.
        for entity in &dirty {
            entity.invalidate_layout_cache(&mut cx.cache, &cx.tree);
        }

        let layout_work = layout_work(&cx.cache, &cx.style, &cx.tree, &dirty);

        // Debug overlay: record which views undergo layout (the subtree of each restart root).
        // The overlay persists until the next layout pass, so schedule a redraw of the previously
        // highlighted views to erase their outlines before recording the new set.
        if cx.style.debug_layout {
            for entity in std::mem::take(&mut cx.style.laid_out) {
                cx.needs_redraw(entity);
            }
            let mut laid_out = HashSet::new();
            for work in &layout_work {
                for entity in LayoutTreeIterator::subtree(&cx.tree, work.restart) {
                    laid_out.insert(entity);
                }
            }
            cx.style.laid_out = laid_out;
        }

        for work in &layout_work {
            work.dirty.layout(
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
        let mut geometry_changed = HashSet::new();

        for work in layout_work {
            let iter = LayoutTreeIterator::subtree(cx.tree, work.restart);

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
                        geometry_changed.insert(entity);

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
        }

        invalidate_geometry_subtrees(cx.style, cx.tree, &geometry_changed);

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

#[cfg(test)]
mod tests {
    use super::*;

    fn add_entity(cx: &mut Context, parent: Entity) -> Entity {
        let entity = cx.entity_manager.create();
        cx.tree.add(entity, parent).unwrap();
        cx.cache.add(entity);
        cx.cache.set_bounds(entity, BoundingBox { x: 0.0, y: 0.0, w: 100.0, h: 100.0 });
        entity
    }

    #[test]
    fn constrained_ancestor_is_not_restartable() {
        let mut cx = Context::new();
        let ancestor = add_entity(&mut cx, Entity::root());

        cx.style.min_width.insert(ancestor, Units::Percentage(50.0));

        assert!(!is_restartable(&cx.style, ancestor));
    }

    #[test]
    fn absolute_ancestor_restarts_layout_from_its_parent() {
        let mut cx = Context::new();
        let absolute = add_entity(&mut cx, Entity::root());
        let auto_child = add_entity(&mut cx, absolute);
        let dirty = add_entity(&mut cx, auto_child);

        cx.style.position_type.insert(absolute, PositionType::Absolute);
        cx.style.width.insert(auto_child, Units::Auto);

        assert_eq!(layout_restart_root(&cx.cache, &cx.style, &cx.tree, dirty), Entity::root());
    }

    #[test]
    fn uninitialized_restart_subtree_falls_back_to_root() {
        let mut cx = Context::new();
        let restart = add_entity(&mut cx, Entity::root());
        let dirty = add_entity(&mut cx, restart);

        cx.cache.set_bounds(restart, BoundingBox::default());

        assert_eq!(layout_restart_root(&cx.cache, &cx.style, &cx.tree, dirty), Entity::root());
    }

    #[test]
    fn layout_work_deduplicates_shared_restart_subtree() {
        let mut cx = Context::new();
        let restart = add_entity(&mut cx, Entity::root());
        let first = add_entity(&mut cx, restart);
        let second = add_entity(&mut cx, restart);
        let dirty = HashSet::from([first, second]);

        let work = layout_work(&cx.cache, &cx.style, &cx.tree, &dirty);

        assert_eq!(work.len(), 1);
        assert_eq!(work[0].restart, restart);
    }

    #[test]
    fn layout_work_collapses_nested_restart_subtrees() {
        let mut cx = Context::new();
        let outer = add_entity(&mut cx, Entity::root());
        let outer_dirty = add_entity(&mut cx, outer);
        let inner = add_entity(&mut cx, outer);
        let inner_dirty = add_entity(&mut cx, inner);
        let dirty = HashSet::from([outer_dirty, inner_dirty]);

        let work = layout_work(&cx.cache, &cx.style, &cx.tree, &dirty);

        assert_eq!(work.len(), 1);
        assert_eq!(work[0].restart, outer);
    }

    #[test]
    fn layout_work_keeps_disjoint_restart_subtrees() {
        let mut cx = Context::new();
        let first_restart = add_entity(&mut cx, Entity::root());
        let first_dirty = add_entity(&mut cx, first_restart);
        let second_restart = add_entity(&mut cx, Entity::root());
        let second_dirty = add_entity(&mut cx, second_restart);
        let dirty = HashSet::from([first_dirty, second_dirty]);

        let work = layout_work(&cx.cache, &cx.style, &cx.tree, &dirty);
        let restart_roots = work.into_iter().map(|work| work.restart).collect::<HashSet<_>>();

        assert_eq!(restart_roots, HashSet::from([first_restart, second_restart]));
    }

    #[test]
    fn geometry_invalidation_collapses_overlapping_changed_subtrees() {
        let mut cx = Context::new();
        let parent = add_entity(&mut cx, Entity::root());
        let child = add_entity(&mut cx, parent);
        let grandchild = add_entity(&mut cx, child);
        let unaffected = add_entity(&mut cx, Entity::root());
        cx.style.retransform.clear();
        cx.style.reclip.clear();

        invalidate_geometry_subtrees(
            &mut cx.style,
            &cx.tree,
            &HashSet::from([parent, child, grandchild]),
        );

        assert_eq!(cx.style.retransform, HashSet::from([parent, child, grandchild]));
        assert_eq!(cx.style.reclip, HashSet::from([parent, child, grandchild]));
        assert!(!cx.style.retransform.contains(&unaffected));
        assert!(!cx.style.reclip.contains(&unaffected));
    }
}
