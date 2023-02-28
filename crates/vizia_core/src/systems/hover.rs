use crate::prelude::*;
use vizia_id::GenerationalId;
use vizia_storage::DrawIterator;

// Determines the hovered entity based on the mouse cursor position.
pub fn hover_system(cx: &mut Context) {
    let draw_tree = DrawIterator::full(&cx.tree);

    let cursorx = cx.mouse.cursorx;
    let cursory = cx.mouse.cursory;

    let mut hovered_widget = Entity::root();

    for entity in draw_tree {
        let window_bounds = cx.cache.get_bounds(Entity::root());

        // Skip if the entity is invisible or out of bounds
        // Unfortunately we can't skip the subtree because even if a parent is invisible
        // a child might be explicitly set to be visible.
        if entity == Entity::root()
            || cx.cache.get_visibility(entity) == Visibility::Hidden
            || cx.cache.get_display(entity) == Display::None
            || cx.cache.get_opacity(entity) == 0.0
            || !window_bounds.contains(&cx.cache.get_bounds(entity))
        {
            continue;
        }

        // Skip non-hoverable widgets
        if !cx.cache.get_hoverability(entity) {
            continue;
        }

        let mut transform = cx.cache.get_transform(entity);
        transform.inverse();

        let (tx, ty) = transform.transform_point(cursorx, cursory);

        let posx = cx.cache.get_posx(entity);
        let posy = cx.cache.get_posy(entity);
        let width = cx.cache.get_width(entity);
        let height = cx.cache.get_height(entity);

        let clip_region = cx.cache.get_clip_region(entity);

        if tx >= posx
            && tx >= clip_region.x
            && tx < (posx + width)
            && tx < (clip_region.x + clip_region.w)
            && ty >= posy
            && ty >= clip_region.y
            && ty < (posy + height)
            && ty < (clip_region.y + clip_region.h)
        {
            hovered_widget = entity;
            if !cx
                .style
                .pseudo_classes
                .get(entity)
                .cloned()
                .unwrap_or_default()
                .contains(PseudoClassFlags::OVER)
                == false
            {
                cx.event_queue.push_back(
                    Event::new(WindowEvent::MouseOver)
                        .target(entity)
                        .propagate(Propagation::Direct),
                );

                if let Some(pseudo_class) = cx.style.pseudo_classes.get_mut(entity) {
                    pseudo_class.set(PseudoClassFlags::OVER, true);
                }
            }
        } else {
            if cx
                .style
                .pseudo_classes
                .get(entity)
                .cloned()
                .unwrap_or_default()
                .contains(PseudoClassFlags::OVER)
                == true
            {
                cx.event_queue.push_back(
                    Event::new(WindowEvent::MouseOut).target(entity).propagate(Propagation::Direct),
                );

                if let Some(pseudo_class) = cx.style.pseudo_classes.get_mut(entity) {
                    pseudo_class.set(PseudoClassFlags::OVER, false);
                }
            }
        }
    }

    if hovered_widget != cx.hovered {
        // Useful for debugging

        #[cfg(debug_assertions)]
        println!(
            "Hover changed to {:?} parent: {:?}, view: {}, posx: {}, posy: {} width: {} height: {}",
            hovered_widget,
            cx.tree.get_parent(hovered_widget),
            cx.views
                .get(&hovered_widget)
                .map_or("<None>", |view| view.element().unwrap_or("<Unnamed>")),
            cx.cache.get_posx(hovered_widget),
            cx.cache.get_posy(hovered_widget),
            cx.cache.get_width(hovered_widget),
            cx.cache.get_height(hovered_widget),
        );

        let cursor = cx.style.cursor.get(hovered_widget).cloned().unwrap_or_default();
        // TODO: Decide if not changing the cursor when the view is disabled is the correct thing to do
        if !cx.cursor_icon_locked
            && !cx.style.disabled.get(hovered_widget).cloned().unwrap_or_default()
        {
            cx.emit(WindowEvent::SetCursor(cursor));
        }

        // Set current hovered pseudoclass to true
        if let Some(pseudo_classes) = cx.style.pseudo_classes.get_mut(hovered_widget) {
            pseudo_classes.set(PseudoClassFlags::HOVER, true);
        }

        // Set previous hovered pseudoclass to false
        let hovered = cx.hovered;
        if let Some(pseudo_classes) = cx.style.pseudo_classes.get_mut(hovered) {
            pseudo_classes.set(PseudoClassFlags::HOVER, false);
        }

        cx.event_queue.push_back(Event::new(WindowEvent::MouseEnter).target(hovered_widget));
        cx.event_queue.push_back(Event::new(WindowEvent::MouseLeave).target(cx.hovered));

        cx.hovered = hovered_widget;

        cx.style.needs_restyle();
    }
}
