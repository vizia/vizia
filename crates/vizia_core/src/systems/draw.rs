use crate::animation::Interpolator;
use crate::context::Context;
use crate::prelude::*;
use crate::style::Transform2D;
use vizia_id::GenerationalId;
use vizia_storage::DrawIterator;

pub fn draw_system(cx: &mut Context) {
    let canvas = cx.canvases.get_mut(&Entity::root()).unwrap();
    cx.resource_manager.mark_images_unused();

    let window_width = cx.cache.get_width(Entity::root());
    let window_height = cx.cache.get_height(Entity::root());

    canvas.set_size(window_width as u32, window_height as u32, 1.0);
    let clear_color =
        cx.style.background_color.get(Entity::root()).cloned().unwrap_or(Color::rgb(255, 255, 255));
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

        let bounds = cx.cache.get_bounds(entity);

        // let x = bounds.x + (bounds.w / 2.0);
        // let y = bounds.y + (bounds.h / 2.0);
        // let mut translate = Transform2D::with_translate(x, y);

        // let mut transform = Transform2D::identity();
        // transform.premultiply(&translate);

        // translate.inverse();

        // if let Some(transforms) = cx.style.transform.get(entity) {
        //     // Check if the transform is currently animating
        //     // Get the animation state
        //     // Manually interpolate the value to get the overall transform for the current frame

        //     if let Some(animation_state) = cx.style.transform.get_active_animation(entity) {
        //         if let Some(start) = animation_state.keyframes.first() {
        //             if let Some(end) = animation_state.keyframes.last() {
        //                 let start_transform = Transform2D::from_style_transforms(&start.1, bounds);
        //                 let end_transform = Transform2D::from_style_transforms(&end.1, bounds);
        //                 let t = animation_state.t;
        //                 let animated_transform =
        //                     Transform2D::interpolate(&start_transform, &end_transform, t);
        //                 transform.premultiply(&animated_transform);
        //             }
        //         }
        //     } else {
        //         transform.premultiply(&Transform2D::from_style_transforms(transforms, bounds));
        //     }
        // }

        // transform.premultiply(&translate);

        let transform = cx.cache.get_transform(entity);
        // let (clipx, clipy) = transform.transform_point(clip_region.x, clip_region.y);
        // let (clipw, cliph) = transform.transform_point(clip_region.w, clip_region.h);

        // Apply transform
        canvas.save();
        // canvas.set_transform(1.0, 0.0, 0.0, 1.0, bounds.center().0, bounds.center().1);
        // canvas.reset_transform();
        canvas.set_transform(
            transform[0],
            transform[1],
            transform[2],
            transform[3],
            transform[4],
            transform[5],
        );
        // canvas.reset_transform();
        // canvas.set_transform(1.0, 0.0, 0.5, 1.0, -50.0, 0.0);

        // canvas.set_transform(1.0, 0.0, 0.0, 1.0, -bounds.center().0, -bounds.center().1);

        // let overlfow = cx.style.overflowx.get(entity).copied().unwrap_or_default();
        // if overlfow == Overflow::Hidden {
        //     println!("Set clip for {} {:?} {:?}", entity, bounds, transform);
        //     canvas.intersect_scissor(bounds.x, bounds.y, bounds.w, bounds.h);
        // }

        canvas.scissor(clip_region.x, clip_region.y, clip_region.w, clip_region.h);
        // canvas.intersect_scissor(
        //     parent_bounds.x,
        //     parent_bounds.y,
        //     parent_bounds.w,
        //     parent_bounds.h,
        // );

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
    }

    canvas.flush();

    //cx.resource_manager.evict_unused_images();
}
