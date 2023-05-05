use std::{cmp::Ordering, collections::BinaryHeap};

use crate::{
    prelude::*,
    style::{Abilities, PseudoClassFlags},
};
use femtovg::Transform2D;
use vizia_id::GenerationalId;
use vizia_storage::LayoutChildIterator;

// Determines the hovered entity based on the mouse cursor position.
pub(crate) fn hover_system(cx: &mut Context) {
    let mut queue = BinaryHeap::new();
    queue.push(ZEntity { index: 0, entity: Entity::root() });
    let mut hovered = Entity::root();
    let transform = Transform2D::identity();
    let clip_bounds = cx.cache.get_bounds(Entity::root());
    while !queue.is_empty() {
        let zentity = queue.pop().unwrap();
        cx.with_current(zentity.entity, |cx| {
            hover_entity(
                &mut EventContext::new(cx),
                zentity.index,
                &mut queue,
                &mut hovered,
                transform,
                &clip_bounds,
            );
        });
    }

    if hovered != cx.hovered {
        // Useful for debugging
        #[cfg(debug_assertions)]
        println!(
            "Hover changed to {:?} parent: {:?}, view: {}, posx: {}, posy: {} width: {} height: {}",
            hovered,
            cx.tree.get_parent(hovered),
            cx.views.get(&hovered).map_or("<None>", |view| view.element().unwrap_or("<Unnamed>")),
            cx.cache.get_posx(hovered),
            cx.cache.get_posy(hovered),
            cx.cache.get_width(hovered),
            cx.cache.get_height(hovered),
        );

        let cursor = cx.style.cursor.get(hovered).cloned().unwrap_or_default();
        // TODO: Decide if not changing the cursor when the view is disabled is the correct thing to do
        if !cx.cursor_icon_locked && !cx.style.disabled.get(hovered).cloned().unwrap_or_default() {
            cx.emit(WindowEvent::SetCursor(cursor));
        }

        // Set current hovered pseudoclass to true
        if let Some(pseudo_classes) = cx.style.pseudo_classes.get_mut(hovered) {
            pseudo_classes.set(PseudoClassFlags::HOVER, true);
        }

        // Set previous hovered pseudoclass to false
        if let Some(pseudo_classes) = cx.style.pseudo_classes.get_mut(cx.hovered) {
            pseudo_classes.set(PseudoClassFlags::HOVER, false);
        }

        cx.event_queue.push_back(Event::new(WindowEvent::MouseEnter).target(hovered));
        cx.event_queue.push_back(Event::new(WindowEvent::MouseLeave).target(cx.hovered));

        cx.hovered = hovered;

        cx.style.needs_restyle();
    }
}

fn hover_entity(
    cx: &mut EventContext,
    current_z: i32,
    queue: &mut BinaryHeap<ZEntity>,
    hovered: &mut Entity,
    parent_transform: Transform2D,
    clip_bounds: &BoundingBox,
) {
    // Skip non-hoverable
    let hoverable = cx
        .style
        .abilities
        .get(cx.current)
        .map(|abilitites| abilitites.contains(Abilities::HOVERABLE))
        .unwrap_or(true);

    if !hoverable {
        return;
    }

    if cx.style.display.get(cx.current).copied().unwrap_or_default() == Display::None {
        return;
    }

    let z_index = cx.tree.z_index(cx.current);
    if z_index > current_z {
        queue.push(ZEntity { index: z_index, entity: cx.current });
        return;
    }

    let bounds = cx.bounds();

    let cursorx = cx.mouse.cursorx;
    let cursory = cx.mouse.cursory;

    let mut transform = parent_transform;

    transform.premultiply(&cx.transform());

    // println!("{} {:?} {:?} {:?}", cx.current, bounds, transform, parent_transform);

    let mut t = transform;
    t.inverse();
    let (tx, ty) = t.transform_point(cursorx, cursory);

    let clipping = clip_bounds.intersection(&cx.clip_region());

    let b = bounds.intersection(&clipping);

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
            cx.event_queue.push_back(
                Event::new(WindowEvent::MouseOver)
                    .target(cx.current)
                    .propagate(Propagation::Direct),
            );

            if let Some(pseudo_class) = cx.style.pseudo_classes.get_mut(cx.current) {
                pseudo_class.set(PseudoClassFlags::OVER, true);
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
        cx.event_queue.push_back(
            Event::new(WindowEvent::MouseOut).target(cx.current).propagate(Propagation::Direct),
        );

        if let Some(pseudo_class) = cx.style.pseudo_classes.get_mut(cx.current) {
            pseudo_class.set(PseudoClassFlags::OVER, false);
        }
    }

    let child_iter = LayoutChildIterator::new(cx.tree, cx.current);
    for child in child_iter {
        cx.current = child;
        hover_entity(cx, current_z, queue, hovered, transform, &clipping);
    }
}

struct ZEntity {
    pub index: i32,
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
