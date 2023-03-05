use crate::context::Context;
use crate::prelude::*;
use vizia_id::GenerationalId;
use vizia_storage::DrawIterator;

pub fn draw_system(cx: &mut Context) {
    let canvas = cx.canvases.get_mut(&Entity::root()).unwrap();
    cx.resource_manager.mark_images_unused();

    let window_width = cx.cache.get_width(Entity::root());
    let window_height = cx.cache.get_height(Entity::root());

    canvas.set_size(window_width as u32, window_height as u32, 1.0);
    let clear_color =
        cx.style.background_color.get(Entity::root()).cloned().unwrap_or(Color::white());
    canvas.clear_rect(0, 0, window_width as u32, window_height as u32, clear_color.into());

    let draw_tree = DrawIterator::full(&cx.tree);

    for entity in draw_tree {
        let window_bounds = cx.cache.get_bounds(Entity::root());

        // Skip if the entity is invisible or out of bounds
        // Unfortunately we can't skip the subtree because even if a parent is invisible
        // a child might be explicitly set to be visible.
        if entity == Entity::root()
            || cx.cache.get_visibility(entity) == Visibility::Invisible
            || cx.cache.get_display(entity) == Display::None
            || cx.cache.get_opacity(entity) == 0.0
            || !window_bounds.intersects(&cx.cache.get_bounds(entity))
        {
            continue;
        }

        // Apply clipping
        let clip_region = cx.cache.get_clip_region(entity);

        // Skips drawing views with zero-sized clip regions
        // This skips calling the `draw` method of the view
        if clip_region.height() == 0.0 || clip_region.width() == 0.0 {
            continue;
        }

        canvas.scissor(clip_region.x, clip_region.y, clip_region.w, clip_region.h);

        // Apply transform
        let transform = cx.cache.get_transform(entity);
        canvas.save();
        canvas.set_transform(&transform);

        if let Some(view) = cx.views.remove(&entity) {
            cx.current = entity;
            view.draw(
                &mut DrawContext {
                    current: cx.current,
                    captured: &cx.captured,
                    focused: &cx.focused,
                    hovered: &cx.hovered,
                    style: &cx.style,
                    cache: &mut cx.cache,
                    draw_cache: &mut cx.draw_cache,
                    tree: &cx.tree,
                    data: &cx.data,
                    views: &cx.views,
                    resource_manager: &cx.resource_manager,
                    text_context: &mut cx.text_context,
                    text_config: &cx.text_config,
                    modifiers: &cx.modifiers,
                    mouse: &cx.mouse,
                },
                canvas,
            );

            cx.views.insert(entity, view);
        }

        canvas.restore();

        // Uncomment this for debug outlines
        // TODO - Hook this up to a key in debug mode
        // let mut path = Path::new();
        // path.rect(bounds.x, bounds.y, bounds.w, bounds.h);
        // let mut paint = Paint::color(femtovg::Color::rgb(255, 0, 0));
        // paint.set_line_width(1.0);
        // canvas.stroke_path(&mut path, paint);
    }

    canvas.flush();

    //cx.resource_manager.evict_unused_images();
}
