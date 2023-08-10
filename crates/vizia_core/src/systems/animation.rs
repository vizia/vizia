use crate::{prelude::*, style::SystemFlags};

pub(crate) fn animation_system(cx: &mut Context) -> bool {
    cx.style.play_pending_animations();

    // Tick all animations

    let time = instant::Instant::now();

    // Properties which affect rendering
    let needs_redraw =
        // Opacity
        cx.style.opacity.tick(time)
        // Border Colour
        | cx.style.border_color.tick(time)
        // Border Radius
        | cx.style.border_top_left_radius.tick(time)
        | cx.style.border_top_right_radius.tick(time)
        | cx.style.border_bottom_left_radius.tick(time)
        | cx.style.border_bottom_right_radius.tick(time)
        // Background
        | cx.style.background_color.tick(time)
        | cx.style.background_image.tick(time)
        | cx.style.background_size.tick(time)
        // Box Shadow
        | cx.style.box_shadow.tick(time)
        // Font Color
        | cx.style.font_color.tick(time)
        // Transform
        | cx.style.transform.tick(time)
        | cx.style.transform_origin.tick(time)
        | cx.style.translate.tick(time)
        | cx.style.rotate.tick(time)
        | cx.style.scale.tick(time)
        // Outline
        | cx.style.outline_color.tick(time)
        | cx.style.outline_offset.tick(time)
        | cx.style.outline_width.tick(time)
        // Clip Path
        | cx.style.clip_path.tick(time);

    // Properties which affect layout
    let needs_relayout = cx.style.display.tick(time)
        // Border Width
        | cx.style.border_width.tick(time)
        // Font Size
        | cx.style.font_size.tick(time)
        // Space
        | cx.style.left.tick(time)
        | cx.style.right.tick(time)
        | cx.style.top.tick(time)
        | cx.style.bottom.tick(time)
        // Size
        | cx.style.width.tick(time)
        | cx.style.height.tick(time)
        // Min/Max Size
        | cx.style.max_width.tick(time)
        | cx.style.max_height.tick(time)
        | cx.style.min_width.tick(time)
        | cx.style.min_height.tick(time)
        // Min/Max Space
        | cx.style.min_left.tick(time)
        | cx.style.max_left.tick(time)
        | cx.style.min_right.tick(time)
        | cx.style.max_right.tick(time)
        | cx.style.min_top.tick(time)
        | cx.style.max_top.tick(time)
        | cx.style.min_bottom.tick(time)
        | cx.style.max_bottom.tick(time)
        // Row/Col Between
        | cx.style.row_between.tick(time)
        | cx.style.col_between.tick(time)
        // Child Space
        | cx.style.child_left.tick(time)
        | cx.style.child_right.tick(time)
        | cx.style.child_top.tick(time)
        | cx.style.child_bottom.tick(time);

    if needs_relayout {
        cx.style.system_flags.set(SystemFlags::RELAYOUT, true);
    }

    if needs_redraw {
        cx.style.system_flags.set(SystemFlags::REDRAW, true);
    }

    needs_redraw | needs_relayout
}
