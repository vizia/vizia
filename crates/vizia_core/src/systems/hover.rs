use std::{cmp::Ordering, collections::BinaryHeap};

use crate::prelude::*;
use log::debug;
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

    let mut queue = BinaryHeap::new();
    let pointer_events: bool =
        cx.style.pointer_events.get(window_entity).copied().unwrap_or_default().into();
    queue.push(ZEntity { index: 0, pointer_events, entity: window_entity });
    let mut hovered = window_entity;
    let mut over = Vec::new();
    while let Some(zentity) = queue.pop() {
        cx.with_current(zentity.entity, |cx| {
            hover_entity(
                &mut EventContext::new(cx),
                zentity.index,
                zentity.pointer_events,
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

        if !cx.cursor_icon_locked {
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

fn hover_entity(
    cx: &mut EventContext,
    current_z: i32,
    parent_pointer_events: bool,
    prune: bool,
    queue: &mut BinaryHeap<ZEntity>,
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
    if prune && !cx.tree.is_window(cx.current) {
        if let Some(subtree_bounds) = cx.cache.subtree_draw_bounds.get(cx.current) {
            if !subtree_bounds.contains_point(cursor_x, cursor_y) {
                return;
            }
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
        .unwrap_or(parent_pointer_events);

    // Push to queue if the z-index is higher than the current z-index.
    let z_index = cx.style.z_index.get(cx.current).copied().unwrap_or_default();
    if z_index > current_z {
        queue.push(ZEntity { index: z_index, entity: cx.current, pointer_events });
        return;
    }

    let bounds = cx.bounds();

    let transform = cx.transform();

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

    let child_iter = DrawChildIterator::new(cx.tree, cx.current);
    for child in child_iter {
        cx.current = child;
        hover_entity(cx, current_z, pointer_events, prune, queue, hovered, over);
    }
}
struct ZEntity {
    pub index: i32,
    pub pointer_events: bool,
    pub entity: Entity,
}

impl Ord for ZEntity {
    fn cmp(&self, other: &Self) -> Ordering {
        other.index.cmp(&self.index)
    }
}
impl PartialOrd for ZEntity {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl PartialEq for ZEntity {
    fn eq(&self, other: &Self) -> bool {
        self.index == other.index
    }
}

impl Eq for ZEntity {}
