use morphorm::Cache;

use crate::{Display, Entity, Event, Propagation, Context, Units, Visibility, WindowEvent, PseudoClass};

/// Determines the hovered entity based on the mouse cursor position
pub fn apply_hover(cx: &mut Context) {
    //println!("Apply Hover");
    let mut draw_tree: Vec<Entity> = cx.tree.into_iter().collect();

    // This should be cached somewhere probably
    draw_tree.sort_by_cached_key(|entity| cx.cache.get_z_index(*entity));

    let cursorx = cx.mouse.cursorx;
    let cursory = cx.mouse.cursory;

    let mut hovered_widget = Entity::root();

    for entity in draw_tree.into_iter() {

        // Skip invisible widgets
        if cx.cache.get_visibility(entity) == Visibility::Invisible {
            continue;
        }

        // This shouldn't be here but there's a bug if it isn't
        if cx.cache.get_opacity(entity) == 0.0 {
            continue;
        }

        // Skip non-displayed widgets
        if cx.cache.get_display(entity) == Display::None {
            continue;
        }

        // Skip non-hoverable widgets
        if cx.cache.get_hoverable(entity) != true {
            continue;
        }

        let posx = cx.cache.get_posx(entity);
        let posy = cx.cache.get_posy(entity);
        let width = cx.cache.get_width(entity);
        let height = cx.cache.get_height(entity);

        let bounds = cx.cache.get_bounds(entity);

        if cursorx >= bounds.x
            && cursorx < (bounds.x + bounds.w)
            && cursory >= bounds.y
            && cursory < (posy + height)
        {
            hovered_widget = entity;
        }
    }

    if hovered_widget != cx.hovered {
        // Useful for debugging

        #[cfg(debug_assertions)]
        println!(
            "Hover changed to {:?} parent: {:?}, posx: {}, posy: {} width: {} height: {}",
            hovered_widget,
            cx.tree.get_parent(hovered_widget),
            cx.cache.get_posx(hovered_widget),
            cx.cache.get_posy(hovered_widget),
            cx.cache.get_width(hovered_widget),
            cx.cache.get_height(hovered_widget),
        );

        // Set current hovered pseudoclass to true
        if let Some(pseudo_classes) = cx.style.borrow_mut().pseudo_classes.get_mut(hovered_widget) {
            pseudo_classes.set(PseudoClass::HOVER, true);
        }

        // Set previous hovered pseudoclass to false
        if let Some(pseudo_classes) = cx.style.borrow_mut().pseudo_classes.get_mut(cx.hovered) {
            pseudo_classes.set(PseudoClass::HOVER, false);
        }

        cx.event_queue.push_back(Event::new(WindowEvent::MouseEnter).target(hovered_widget));
        cx.event_queue.push_back(Event::new(WindowEvent::MouseLeave).target(cx.hovered));

        cx.hovered = hovered_widget;
    }
}
