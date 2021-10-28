use morphorm::Cache;

use crate::{Display, Entity, Event, Propagation, Context, Units, Visibility, WindowEvent};

/// Determines the hovered entity based on the mouse cursor position
pub fn apply_hover(cx: &mut Context) {
    //println!("Apply Hover");
    //let mut draw_tree: Vec<Entity> = cx.tree.into_iter().collect();

    // This should be cached somewhere probably
    //draw_tree.sort_by_cached_key(|entity| cx.data.get_z_index(*entity));

    let cursorx = cx.mouse.cursorx;
    let cursory = cx.mouse.cursory;

    let mut hovered_widget = Entity::root();

    for entity in cx.tree.into_iter() {

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

        cx.event_queue.push_back(Event::new(WindowEvent::MouseEnter).target(hovered_widget));
        cx.event_queue.push_back(Event::new(WindowEvent::MouseLeave).target(cx.hovered));

        cx.hovered = hovered_widget;
    }
}
