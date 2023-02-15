use crate::{prelude::*, style::SystemFlags};

pub fn animation_system(cx: &mut Context) -> bool {
    let time = instant::Instant::now();

    // Properties which affect visibility
    let needs_rehide =
        cx.style.display.tick(time) | cx.style.visibility.tick(time) | cx.style.opacity.tick(time);

    // Properties which affect rendering
    let needs_redraw = cx.style.rotate.tick(time)
        | cx.style.translate.tick(time)
        | cx.style.scale.tick(time)
        | cx.style.border_color.tick(time)
        | cx.style.border_radius_top_left.tick(time)
        | cx.style.border_radius_top_right.tick(time)
        | cx.style.border_radius_bottom_left.tick(time)
        | cx.style.border_radius_bottom_right.tick(time)
        | cx.style.background_color.tick(time)
        | cx.style.outer_shadow_h_offset.tick(time)
        | cx.style.outer_shadow_v_offset.tick(time)
        | cx.style.outer_shadow_blur.tick(time)
        | cx.style.outer_shadow_color.tick(time)
        | cx.style.font_color.tick(time);

    // Properties which affect layout
    let needs_relayout = cx.style.border_width_left.tick(time)
        | cx.style.border_width_right.tick(time)
        | cx.style.border_width_top.tick(time)
        | cx.style.border_width_bottom.tick(time)
        | cx.style.font_size.tick(time)
        | cx.style.left.tick(time)
        | cx.style.right.tick(time)
        | cx.style.top.tick(time)
        | cx.style.bottom.tick(time)
        | cx.style.width.tick(time)
        | cx.style.height.tick(time)
        | cx.style.max_width.tick(time)
        | cx.style.max_height.tick(time)
        | cx.style.min_width.tick(time)
        | cx.style.min_height.tick(time)
        | cx.style.min_left.tick(time)
        | cx.style.max_left.tick(time)
        | cx.style.min_right.tick(time)
        | cx.style.max_right.tick(time)
        | cx.style.min_top.tick(time)
        | cx.style.max_top.tick(time)
        | cx.style.min_bottom.tick(time)
        | cx.style.max_bottom.tick(time)
        | cx.style.row_between.tick(time)
        | cx.style.col_between.tick(time)
        | cx.style.child_left.tick(time)
        | cx.style.child_right.tick(time)
        | cx.style.child_top.tick(time)
        | cx.style.child_bottom.tick(time);

    if needs_relayout {
        cx.style.system_flags.set(SystemFlags::RELAYOUT, true);
    }

    if needs_rehide {
        cx.style.system_flags.set(SystemFlags::REHIDE, true);
    }

    if needs_redraw {
        cx.style.system_flags.set(SystemFlags::REDRAW, true);
    }

    return needs_redraw | needs_relayout;
}
