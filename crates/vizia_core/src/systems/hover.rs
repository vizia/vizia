use std::{cmp::Ordering, collections::BinaryHeap};

use crate::{cache::BoundingBox, prelude::*};
use femtovg::Transform2D;
use vizia_id::GenerationalId;
use vizia_storage::LayoutChildIterator;

// // Determines the hovered entity based on the mouse cursor position.
// pub fn _hover_system2(cx: &mut Context) {
//     let draw_tree = DrawIterator::full(&cx.tree);

//     let cursorx = cx.mouse.cursorx;
//     let cursory = cx.mouse.cursory;

//     let mut hovered_widget = Entity::root();

//     for entity in draw_tree {
//         let window_bounds = cx.cache.get_bounds(Entity::root());

//         // Skip if the entity is invisible or out of bounds
//         // Unfortunately we can't skip the subtree because even if a parent is invisible
//         // a child might be explicitly set to be visible.
//         if entity == Entity::root()
//             || cx.cache.get_display(entity) == Display::None
//             || cx.cache.get_opacity(entity) == 0.0
//             || !window_bounds.contains(&cx.cache.get_bounds(entity))
//         {
//             continue;
//         }

//         // Skip non-hoverable widgets
//         if !cx.cache.get_hoverability(entity) {
//             continue;
//         }

//         let mut transform = cx.cache.get_transform(entity);
//         transform.inverse();

//         let (tx, ty) = transform.transform_point(cursorx, cursory);

//         let posx = cx.cache.get_posx(entity);
//         let posy = cx.cache.get_posy(entity);
//         let width = cx.cache.get_width(entity);
//         let height = cx.cache.get_height(entity);

//         let clip_region = cx.cache.get_clip_region(entity);

//         if tx >= posx
//             && tx >= clip_region.x
//             && tx < (posx + width)
//             && tx < (clip_region.x + clip_region.w)
//             && ty >= posy
//             && ty >= clip_region.y
//             && ty < (posy + height)
//             && ty < (clip_region.y + clip_region.h)
//         {
//             hovered_widget = entity;

//             if !cx
//                 .style
//                 .pseudo_classes
//                 .get(entity)
//                 .cloned()
//                 .unwrap_or_default()
//                 .contains(PseudoClassFlags::OVER)
//             {
//                 cx.event_queue.push_back(
//                     Event::new(WindowEvent::MouseOver)
//                         .target(entity)
//                         .propagate(Propagation::Direct),
//                 );

//                 if let Some(pseudo_class) = cx.style.pseudo_classes.get_mut(entity) {
//                     pseudo_class.set(PseudoClassFlags::OVER, true);
//                 }
//             }
//         } else {
//             if cx
//                 .style
//                 .pseudo_classes
//                 .get(entity)
//                 .cloned()
//                 .unwrap_or_default()
//                 .contains(PseudoClassFlags::OVER)
//             {
//                 cx.event_queue.push_back(
//                     Event::new(WindowEvent::MouseOut).target(entity).propagate(Propagation::Direct),
//                 );

//                 if let Some(pseudo_class) = cx.style.pseudo_classes.get_mut(entity) {
//                     pseudo_class.set(PseudoClassFlags::OVER, false);
//                 }
//             }
//         }
//     }

//     if hovered_widget != cx.hovered {
//         // Useful for debugging
//         #[cfg(debug_assertions)]
//         println!(
//             "Hover changed to {:?} parent: {:?}, view: {}, posx: {}, posy: {} width: {} height: {}",
//             hovered_widget,
//             cx.tree.get_parent(hovered_widget),
//             cx.views
//                 .get(&hovered_widget)
//                 .map_or("<None>", |view| view.element().unwrap_or("<Unnamed>")),
//             cx.cache.get_posx(hovered_widget),
//             cx.cache.get_posy(hovered_widget),
//             cx.cache.get_width(hovered_widget),
//             cx.cache.get_height(hovered_widget),
//         );

//         let cursor = cx.style.cursor.get(hovered_widget).cloned().unwrap_or_default();
//         // TODO: Decide if not changing the cursor when the view is disabled is the correct thing to do
//         if !cx.cursor_icon_locked
//             && !cx.style.disabled.get(hovered_widget).cloned().unwrap_or_default()
//         {
//             cx.emit(WindowEvent::SetCursor(cursor));
//         }

//         // Set current hovered pseudoclass to true
//         if let Some(pseudo_classes) = cx.style.pseudo_classes.get_mut(hovered_widget) {
//             pseudo_classes.set(PseudoClassFlags::HOVER, true);
//         }

//         // Set previous hovered pseudoclass to false
//         let hovered = cx.hovered;
//         if let Some(pseudo_classes) = cx.style.pseudo_classes.get_mut(hovered) {
//             pseudo_classes.set(PseudoClassFlags::HOVER, false);
//         }

//         cx.event_queue.push_back(Event::new(WindowEvent::MouseEnter).target(hovered_widget));
//         cx.event_queue.push_back(Event::new(WindowEvent::MouseLeave).target(cx.hovered));

//         cx.hovered = hovered_widget;

//         cx.style.needs_restyle();
//     }
// }

pub fn hover_system(cx: &mut Context) {
    let mut queue = BinaryHeap::new();
    queue.push(ZEntity(0, Entity::root()));
    let mut hovered = Entity::root();
    let mut transform = Transform2D::identity();
    let clip_bounds = cx.cache.get_bounds(Entity::root());
    while !queue.is_empty() {
        let ZEntity(current_z, current) = queue.pop().unwrap();
        cx.with_current(current, |cx| {
            hover_entity(
                &mut EventContext::new(cx),
                current_z,
                &mut queue,
                &mut hovered,
                &mut transform,
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
        let h = cx.hovered;
        if let Some(pseudo_classes) = cx.style.pseudo_classes.get_mut(h) {
            pseudo_classes.set(PseudoClassFlags::HOVER, false);
        }

        cx.event_queue.push_back(Event::new(WindowEvent::MouseEnter).target(h));
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
    transform: &mut Transform2D,
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

    let mut bounds = cx.cache.get_bounds(cx.current);

    let cursorx = cx.mouse.cursorx;
    let cursory = cx.mouse.cursory;

    if let Some(t) = cx.transform() {
        transform.premultiply(&t);
    }

    let mut t = *transform;
    t.inverse();
    let (tx, ty) = t.transform_point(cursorx, cursory);

    let clipping = clip_bounds.intersection(&cx.clip_region());

    let b = bounds.intersection(&clipping);

    if tx >= b.left() && tx <= b.right() && ty >= b.top() && ty <= b.bottom() {
        *hovered = cx.current;

        if !cx
            .style
            .pseudo_classes
            .get(cx.current)
            .cloned()
            .unwrap_or_default()
            .contains(PseudoClassFlags::OVER)
        {
            // cx.event_queue.push_back(
            //     Event::new(WindowEvent::MouseOver)
            //         .target(cx.current)
            //         .propagate(Propagation::Direct),
            // );

            if let Some(pseudo_class) = cx.style.pseudo_classes.get_mut(cx.current) {
                pseudo_class.set(PseudoClassFlags::OVER, true);
            }
        }
    } else {
        if cx
            .style
            .pseudo_classes
            .get(cx.current)
            .cloned()
            .unwrap_or_default()
            .contains(PseudoClassFlags::OVER)
        {
            // cx.event_queue.push_back(
            //     Event::new(WindowEvent::MouseOut).target(cx.current).propagate(Propagation::Direct),
            // );

            if let Some(pseudo_class) = cx.style.pseudo_classes.get_mut(cx.current) {
                pseudo_class.set(PseudoClassFlags::OVER, false);
            }
        }
    }

    let child_iter = LayoutChildIterator::new(&cx.tree, cx.current);
    // let bounds = cx.bounds();
    for child in child_iter {
        cx.current = child;
        hover_entity(cx, current_z, queue, hovered, transform, &clipping);
    }
}

#[derive(Eq)]
pub struct ZEntity(i32, Entity);
impl Ord for ZEntity {
    fn cmp(&self, other: &Self) -> Ordering {
        other.0.cmp(&self.0)
    }
}
impl PartialOrd for ZEntity {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl PartialEq for ZEntity {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
