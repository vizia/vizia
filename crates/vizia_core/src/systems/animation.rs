use crate::prelude::*;

pub fn animation_system(cx: &mut Context) -> bool {
    let time = instant::Instant::now();

    // Properties which affect rendering
    let needs_redraw = cx.style.opacity.tick(time)
        | cx.style.border_color.tick(time)
        | cx.style.border_top_left_radius.tick(time)
        | cx.style.border_top_right_radius.tick(time)
        | cx.style.border_bottom_left_radius.tick(time)
        | cx.style.border_bottom_right_radius.tick(time)
        | cx.style.background_color.tick(time)
        | cx.style.background_gradient.tick(time)
        | cx.style.box_shadow.tick(time)
        | cx.style.font_color.tick(time)
        | cx.style.transform.tick(time);

    // Properties which affect layout
    let needs_relayout = cx.style.border_width.tick(time)
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

    cx.style.needs_redraw |= needs_redraw | needs_relayout;
    cx.style.needs_relayout |= needs_relayout;

    return needs_redraw | needs_relayout;
}
