use crate::prelude::*;

pub fn has_animations(cx: &Context) -> bool {
    let mut custom_is_animating = false;
    for (_, prop) in cx.style.custom_color_props.iter() {
        if prop.has_animations() {
            custom_is_animating = true;
        }
    }

    cx.style.display.has_animations()
        | cx.style.visibility.has_animations()
        | cx.style.opacity.has_animations()
        | cx.style.rotate.has_animations()
        | cx.style.translate.has_animations()
        | cx.style.scale.has_animations()
        | cx.style.border_width.has_animations()
        | cx.style.border_color.has_animations()
        | cx.style.border_radius_top_left.has_animations()
        | cx.style.border_radius_top_right.has_animations()
        | cx.style.border_radius_bottom_left.has_animations()
        | cx.style.border_radius_bottom_right.has_animations()
        | cx.style.background_color.has_animations()
        | cx.style.outer_shadow_h_offset.has_animations()
        | cx.style.outer_shadow_v_offset.has_animations()
        | cx.style.outer_shadow_blur.has_animations()
        | cx.style.outer_shadow_color.has_animations()
        | cx.style.font_color.has_animations()
        | cx.style.font_size.has_animations()
        | cx.style.left.has_animations()
        | cx.style.right.has_animations()
        | cx.style.top.has_animations()
        | cx.style.bottom.has_animations()
        | cx.style.width.has_animations()
        | cx.style.height.has_animations()
        | cx.style.max_width.has_animations()
        | cx.style.max_height.has_animations()
        | cx.style.min_width.has_animations()
        | cx.style.min_height.has_animations()
        | cx.style.min_left.has_animations()
        | cx.style.max_left.has_animations()
        | cx.style.min_right.has_animations()
        | cx.style.max_right.has_animations()
        | cx.style.min_top.has_animations()
        | cx.style.max_top.has_animations()
        | cx.style.min_bottom.has_animations()
        | cx.style.max_bottom.has_animations()
        | cx.style.row_between.has_animations()
        | cx.style.col_between.has_animations()
        | cx.style.child_left.has_animations()
        | cx.style.child_right.has_animations()
        | cx.style.child_top.has_animations()
        | cx.style.child_bottom.has_animations()
        | custom_is_animating
}

pub fn animation_system(cx: &mut Context) {
    let time = instant::Instant::now();

    cx.style.display.tick(time);
    cx.style.visibility.tick(time);
    cx.style.opacity.tick(time);
    cx.style.rotate.tick(time);
    cx.style.translate.tick(time);
    cx.style.scale.tick(time);
    cx.style.border_width.tick(time);
    cx.style.border_color.tick(time);
    cx.style.border_radius_top_left.tick(time);
    cx.style.border_radius_top_right.tick(time);
    cx.style.border_radius_bottom_left.tick(time);
    cx.style.border_radius_bottom_right.tick(time);
    cx.style.background_color.tick(time);
    cx.style.outer_shadow_h_offset.tick(time);
    cx.style.outer_shadow_v_offset.tick(time);
    cx.style.outer_shadow_blur.tick(time);
    cx.style.outer_shadow_color.tick(time);
    cx.style.font_color.tick(time);
    cx.style.font_size.tick(time);
    cx.style.left.tick(time);
    cx.style.right.tick(time);
    cx.style.top.tick(time);
    cx.style.bottom.tick(time);
    cx.style.width.tick(time);
    cx.style.height.tick(time);
    cx.style.max_width.tick(time);
    cx.style.max_height.tick(time);
    cx.style.min_width.tick(time);
    cx.style.min_height.tick(time);
    cx.style.min_left.tick(time);
    cx.style.max_left.tick(time);
    cx.style.min_right.tick(time);
    cx.style.max_right.tick(time);
    cx.style.min_top.tick(time);
    cx.style.max_top.tick(time);
    cx.style.min_bottom.tick(time);
    cx.style.max_bottom.tick(time);
    cx.style.row_between.tick(time);
    cx.style.col_between.tick(time);
    cx.style.child_left.tick(time);
    cx.style.child_right.tick(time);
    cx.style.child_top.tick(time);
    cx.style.child_bottom.tick(time);

    cx.style.needs_relayout = true;

    for (_, prop) in cx.style.custom_color_props.iter_mut() {
        prop.tick(time);
    }
}
