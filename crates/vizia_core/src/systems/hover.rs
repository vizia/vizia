use crate::cache::is_identity;
use crate::prelude::*;
use crate::systems::draw::{
    Stacking, StackingGroup, ZHeaps, ZOrder, ZQueue, clip_bounds, subtree_to_screen,
};
use log::debug;
use skia_safe::Matrix;
use vizia_storage::{DrawChildIterator, LayoutParentIterator};

/// The entities whose hover pseudo-classes were set by the previous run of the hover system.
#[derive(Debug, Default)]
pub(crate) struct HoverTracking {
    /// Entities with the `OVER` pseudo-class (the cursor is within their bounds).
    over: Vec<Entity>,
    /// Entities with the `HOVER` pseudo-class (the hovered entity and its ancestors).
    hover: Vec<Entity>,
}

// Determines the hovered entity based on the mouse cursor position.
pub fn hover_system(cx: &mut Context, window_entity: Entity) {
    let _scope = crate::profiling::scope("hover_system");
    cx.current = window_entity;

    // Cached subtree draw bounds are only up to date once the draw system has processed every
    // pending redraw, so only use them to skip subtrees away from the cursor when nothing is
    // pending.
    let prune = cx.windows.get(&window_entity).is_some_and(|window| window.redraw_list.is_empty());

    // Views are visited in the stacking order they are drawn in, so the last view hit is the
    // topmost one painted.
    let mut heaps = ZHeaps::default();
    let mut order = ZOrder::default();
    let mut queue = ZQueue::new(&mut heaps, &mut order);
    let pointer_events: bool =
        cx.style.pointer_events.get(window_entity).copied().unwrap_or_default().into();
    queue.defer_raised(
        0,
        window_entity,
        HoverParent { pointer_events, transform: Matrix::new_identity(), clip: None },
    );
    let mut hovered = window_entity;
    let mut over = Vec::new();
    while let Some(zentity) = queue.pop(true) {
        cx.with_current(zentity.entity, |cx| {
            hover_entity(
                &mut EventContext::new(cx),
                zentity.index,
                false,
                &zentity.state,
                prune,
                &mut queue,
                &mut hovered,
                &mut over,
            );
        });
    }

    let previous = cx.hover_tracking.remove(&window_entity).unwrap_or_default();

    // Clear the over state of entities which were not visited because they were skipped.
    for entity in previous.over {
        if !over.contains(&entity) && cx.entity_manager.is_alive(entity) {
            cx.set_pseudo_class(entity, PseudoClassFlags::OVER, false);
        }
    }

    // Set hover state for hovered view and ancestors
    let hover = LayoutParentIterator::new(&cx.tree, hovered)
        .filter(|ancestor| {
            cx.style
                .pseudo_classes
                .get(*ancestor)
                .is_some_and(|pseudo_classes| pseudo_classes.contains(PseudoClassFlags::OVER))
        })
        .collect::<Vec<_>>();

    for entity in previous.hover {
        if !hover.contains(&entity) && cx.entity_manager.is_alive(entity) {
            cx.set_pseudo_class(entity, PseudoClassFlags::HOVER, false);
        }
    }

    for &entity in &hover {
        cx.set_pseudo_class(entity, PseudoClassFlags::HOVER, true);
    }

    cx.hover_tracking.insert(window_entity, HoverTracking { over, hover });

    if hovered != cx.hovered {
        // Useful for debugging
        debug!(
            "Hover changed to {:?} parent: {:?}, view: {}, posx: {}, posy: {} width: {} height: {}",
            hovered,
            cx.tree.get_layout_parent(hovered),
            cx.views.get(&hovered).map_or("<None>", |view| view.element().unwrap_or("<Unnamed>")),
            cx.cache.get_posx(hovered),
            cx.cache.get_posy(hovered),
            cx.cache.get_width(hovered),
            cx.cache.get_height(hovered),
        );

        let cursor = cx.style.cursor.get(hovered).cloned().unwrap_or_default();

        if cx.cursor_icon_locked_by.is_none() {
            cx.emit(WindowEvent::SetCursor(cursor));
        }

        // Send mouse enter/leave events directly to entity.
        cx.event_queue.push_back(Event::new(WindowEvent::MouseEnter).direct(hovered));
        cx.event_queue.push_back(Event::new(WindowEvent::MouseLeave).direct(cx.hovered));

        // Send mouse over/out events to entity and ancestors.
        cx.event_queue.push_back(Event::new(WindowEvent::MouseOver).target(hovered));
        cx.event_queue.push_back(Event::new(WindowEvent::MouseOut).target(cx.hovered));

        cx.hovered = hovered;
    }
}

#[allow(clippy::too_many_arguments)]
fn hover_entity(
    cx: &mut EventContext,
    current_z: i32,
    ancestor_backdrop_clip: bool,
    parent: &HoverParent,
    prune: bool,
    queue: &mut ZQueue<'_, HoverParent>,
    hovered: &mut Entity,
    over: &mut Vec<Entity>,
) {
    // Skip if non-hoverable (will skip any descendants)
    let hoverable = cx
        .style
        .abilities
        .get(cx.current)
        .map(|abilitites| abilitites.contains(Abilities::HOVERABLE))
        .unwrap_or(true);

    if !hoverable {
        return;
    }

    // Skip if not displayed.
    // TODO: Should this skip descendants? Probably not...?
    if cx.style.display.get(cx.current).copied().unwrap_or_default() == Display::None
        && !cx.style.text_span.get(cx.current).copied().unwrap_or_default()
    {
        return;
    }

    // Disabled views (and their descendants through inherited disabled state) should never
    // become hover targets.
    if cx.style.disabled.get(cx.current).copied().unwrap_or_default() {
        cx.set_pseudo_class(cx.current, PseudoClassFlags::HOVER | PseudoClassFlags::OVER, false);
        return;
    }

    let cursor_x = cx.mouse.cursor_x;
    let cursor_y = cx.mouse.cursor_y;

    if cursor_x < 0.0 || cursor_y < 0.0 {
        return;
    }

    // Skip the whole subtree if the cursor is outside the bounds of everything drawn within it.
    // Subtree draw bounds include z-indexed and clip-escaping descendants and contain the
    // (clipped) bounds of every descendant.
    if prune
        && !cx.tree.is_window(cx.current)
        && let Some(bounds) = cx.cache.draw_bounds.get(cx.current)
    {
        let subtree_bounds = subtree_to_screen(bounds, &parent.transform, parent.clip);
        if !subtree_bounds.contains_point(cursor_x, cursor_y) {
            return;
        }
    }

    let pointer_events = cx
        .style
        .pointer_events
        .get(cx.current)
        .copied()
        .map(|pointer_events| match pointer_events {
            PointerEvents::Auto => true,
            PointerEvents::None => false,
        })
        .unwrap_or(parent.pointer_events);

    match Stacking::of(cx.style, cx.current, current_z, ancestor_backdrop_clip) {
        Stacking::Inline => {}
        Stacking::Raised(index) => {
            queue.defer_raised(index, cx.current, *parent);
            return;
        }
        Stacking::ClippingEscape(index) => {
            queue.defer_clipping_escape(index, cx.current, *parent);
            return;
        }
    }

    let bounds = cx.bounds();

    let transform = match cx.cache.local_transform.get(cx.current) {
        Some(local) if !is_identity(local) => parent.transform * *local,
        _ => parent.transform,
    };

    let bounds_rect: skia_safe::Rect = bounds.into();
    let transformed_bounds: BoundingBox = transform.map_rect(bounds_rect).0.into();

    let tx = cursor_x;
    let ty = cursor_y;
    let clipping = cx.clip_region();

    let b = transformed_bounds.intersection(&clipping);

    if pointer_events {
        let is_over = tx >= b.left() && tx < b.right() && ty >= b.top() && ty < b.bottom();
        if is_over {
            *hovered = cx.current;
            over.push(cx.current);
        }

        cx.set_pseudo_class(cx.current, PseudoClassFlags::OVER, is_over);
    }

    let current = cx.current;
    let mut child_iter = DrawChildIterator::new(cx.tree, current).peekable();
    // Most views are leaves, which have no descendants to queue.
    if child_iter.peek().is_none() {
        return;
    }
    let child_parent =
        HoverParent { pointer_events, transform, clip: clip_bounds(cx.cache, current) };
    // An opacity group composites its raised descendants, and the clip escapes it owns, before
    // the views above it are drawn, so they are hit beneath those views too.
    let group = StackingGroup::of(cx.style, current, ancestor_backdrop_clip);
    let mut group_heaps = queue.group_heaps(group);
    let mut child_queue = queue.descend(group, &mut group_heaps);
    for child in child_iter {
        cx.current = child;
        hover_entity(
            cx,
            current_z,
            group.backdrop_clipped,
            &child_parent,
            prune,
            &mut child_queue,
            hovered,
            over,
        );
    }

    if group.owns_raised {
        while let Some(zentity) = child_queue.pop(group.owns_escapes) {
            cx.current = zentity.entity;
            hover_entity(
                cx,
                zentity.index,
                group.backdrop_clipped,
                &zentity.state,
                prune,
                &mut child_queue,
                hovered,
                over,
            );
        }
    }
    queue.recycle(group_heaps);
    cx.current = current;
}

/// The state of the parent of a view being hit tested.
#[derive(Clone, Copy)]
struct HoverParent {
    pointer_events: bool,
    /// The transform of the parent in window coordinates.
    transform: Matrix,
    /// The clip bounds of the parent in window coordinates.
    clip: Option<BoundingBox>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::backend::BackendContext;

    fn rect(cx: &mut Context, x: f32, width: f32, color: Color) -> Entity {
        Element::new(cx)
            .position_type(PositionType::Absolute)
            .left(Pixels(x))
            .top(Pixels(0.0))
            .width(Pixels(width))
            .height(Pixels(16.0))
            .background_color(color)
            .entity()
    }

    fn backdrop_clip(cx: &mut Context, entity: Entity) {
        cx.style.backdrop_filter.insert(entity, Filter::Blur(Length::px(0.0)));
        cx.style.overflowx.insert(entity, Overflow::Hidden);
        cx.style.overflowy.insert(entity, Overflow::Hidden);
    }

    /// Paints the scene on a 32x16 window and returns the painted color and the hovered entity at
    /// each point.
    fn paint_and_hover(cx: Context, points: &[(f32, f32)]) -> Vec<(skia_safe::Color, Entity)> {
        let mut backend = BackendContext::new(cx);
        backend.0.windows.insert(Entity::root(), WindowState::default());
        backend.set_window_size(Entity::root(), 32.0, 16.0);
        backend.process_style_updates();
        backend.process_visual_updates();
        let mut surface = skia_safe::surfaces::raster_n32_premul((32, 16)).unwrap();
        let mut dirty = skia_safe::surfaces::raster_n32_premul((32, 16)).unwrap();
        assert!(backend.draw(Entity::root(), &mut surface, &mut dirty));
        let pixels = surface.peek_pixels().unwrap();

        let cx = backend.context();
        points
            .iter()
            .map(|&(x, y)| {
                cx.mouse.cursor_x = x;
                cx.mouse.cursor_y = y;
                hover_system(cx, Entity::root());
                (pixels.get_color((x as i32, y as i32)), cx.hovered)
            })
            .collect()
    }

    // A raised child of an opacity group is hit beneath a higher sibling, as it is painted.
    #[test]
    fn hover_keeps_opacity_group_raised_child_beneath_higher_sibling() {
        let mut cx = Context::new();
        let group = rect(&mut cx, 0.0, 16.0, Color::transparent());
        cx.style.opacity.insert(group, 0.5.into());
        cx.style.z_index.insert(group, 2);
        let child = cx.with_current(group, |cx| rect(cx, 0.0, 16.0, Color::blue()));
        cx.style.z_index.insert(child, 5);
        let sibling = rect(&mut cx, 8.0, 16.0, Color::green());
        cx.style.z_index.insert(sibling, 3);

        let hits = paint_and_hover(cx, &[(4.0, 8.0), (12.0, 8.0)]);
        assert_eq!(hits[0], (skia_safe::Color::from_argb(128, 0, 0, 255), child));
        assert_eq!(hits[1], (skia_safe::Color::from_rgb(0, 128, 0), sibling));
    }

    // A clip escape from a backdrop clip is hit above later siblings, as it is painted.
    #[test]
    fn hover_keeps_backdrop_clipping_escape_above_later_sibling() {
        let mut cx = Context::new();
        let backdrop = rect(&mut cx, 0.0, 16.0, Color::transparent());
        backdrop_clip(&mut cx, backdrop);
        let escape = cx.with_current(backdrop, |cx| rect(cx, 8.0, 16.0, Color::red()));
        cx.style.ignore_clipping.insert(escape, true);
        let sibling = rect(&mut cx, 16.0, 16.0, Color::green());

        let hits = paint_and_hover(cx, &[(20.0, 8.0), (28.0, 8.0)]);
        assert_eq!(hits[0], (skia_safe::Color::RED, escape));
        assert_eq!(hits[1], (skia_safe::Color::from_rgb(0, 128, 0), sibling));
    }

    // An opacity group drains a clip escape it owns beneath its raised descendants, as it paints.
    #[test]
    fn hover_keeps_group_owned_clipping_escape_beneath_raised_content() {
        let mut cx = Context::new();
        let group = rect(&mut cx, 0.0, 32.0, Color::transparent());
        cx.style.opacity.insert(group, 0.5.into());
        let backdrop = cx.with_current(group, |cx| rect(cx, 0.0, 16.0, Color::transparent()));
        backdrop_clip(&mut cx, backdrop);
        let (raised, escape) = cx.with_current(backdrop, |cx| {
            let raised = rect(cx, 0.0, 16.0, Color::blue());
            cx.style.z_index.insert(raised, 5);
            let escape = rect(cx, 8.0, 16.0, Color::red());
            cx.style.ignore_clipping.insert(escape, true);
            (raised, escape)
        });

        let hits = paint_and_hover(cx, &[(12.0, 8.0), (20.0, 8.0)]);
        assert_eq!(hits[0], (skia_safe::Color::from_argb(128, 0, 0, 255), raised));
        assert_eq!(hits[1], (skia_safe::Color::from_argb(128, 255, 0, 0), escape));
    }
}
