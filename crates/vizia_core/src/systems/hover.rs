use std::{cmp::Ordering, collections::BinaryHeap};

use crate::prelude::*;
use log::debug;
use vizia_storage::{DrawChildIterator, LayoutParentIterator};

// Determines the hovered entity based on the mouse cursor position.
pub fn hover_system(cx: &mut Context, window_entity: Entity) {
    cx.current = window_entity;

    // if let Some(pseudo_classes) = cx.style.pseudo_classes.get(window_entity) {
    //     if !pseudo_classes.contains(PseudoClassFlags::OVER) {
    //         return;
    //     }
    // }

    let mut queue = BinaryHeap::new();
    let pointer_events: bool =
        cx.style.pointer_events.get(window_entity).copied().unwrap_or_default().into();
    queue.push(ZEntity { index: 0, pointer_events, entity: window_entity });
    let mut hovered = window_entity;
    while !queue.is_empty() {
        let zentity = queue.pop().unwrap();
        cx.with_current(zentity.entity, |cx| {
            hover_entity(
                &mut EventContext::new(cx),
                zentity.index,
                zentity.pointer_events,
                &mut queue,
                &mut hovered,
            );
        });
    }

    // Set hover state for hovered view and ancestors
    let parent_iter = LayoutParentIterator::new(&cx.tree, hovered);
    for ancestor in parent_iter {
        if let Some(pseudo_classes) = cx.style.pseudo_classes.get_mut(ancestor) {
            if pseudo_classes.contains(PseudoClassFlags::OVER)
                && !pseudo_classes.contains(PseudoClassFlags::HOVER)
            {
                pseudo_classes.set(PseudoClassFlags::HOVER, true);
            }
        }
    }

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

        cx.style.needs_restyle(cx.hovered);
        cx.style.needs_restyle(hovered);

        cx.hovered = hovered;
    }
}

fn hover_entity(
    cx: &mut EventContext,
    current_z: i32,
    parent_pointer_events: bool,
    queue: &mut BinaryHeap<ZEntity>,
    hovered: &mut Entity,
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

    let cursor_x = cx.mouse.cursor_x;
    let cursor_y = cx.mouse.cursor_y;

    if cursor_x < 0.0 || cursor_y < 0.0 {
        return;
    }

    // let mut transform = parent_transform;

    let transform = cx.transform();

    let bounds_rect: skia_safe::Rect = bounds.into();
    let transformed_bounds: BoundingBox = transform.map_rect(bounds_rect).0.into();

    let tx = cursor_x;
    let ty = cursor_y;
    let clipping = cx.clip_region();

    let b = transformed_bounds.intersection(&clipping);

    if let Some(pseudo_classes) = cx.style.pseudo_classes.get_mut(cx.current) {
        pseudo_classes.set(PseudoClassFlags::HOVER, false);
    }

    if pointer_events {
        if tx >= b.left() && tx < b.right() && ty >= b.top() && ty < b.bottom() {
            *hovered = cx.current;

            if !cx
                .style
                .pseudo_classes
                .get(cx.current)
                .copied()
                .unwrap_or_default()
                .contains(PseudoClassFlags::OVER)
            {
                if let Some(pseudo_class) = cx.style.pseudo_classes.get_mut(cx.current) {
                    pseudo_class.set(PseudoClassFlags::OVER, true);

                    cx.needs_restyle();
                }
            }
        } else if cx
            .style
            .pseudo_classes
            .get(cx.current)
            .copied()
            .unwrap_or_default()
            .contains(PseudoClassFlags::OVER)
        {
            if let Some(pseudo_class) = cx.style.pseudo_classes.get_mut(cx.current) {
                pseudo_class.set(PseudoClassFlags::OVER, false);

                cx.needs_restyle();
            }
        }
    }

    let child_iter = DrawChildIterator::new(cx.tree, cx.current);
    for child in child_iter {
        cx.current = child;
        hover_entity(cx, current_z, pointer_events, queue, hovered);
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
