use crate::prelude::*;

pub(crate) fn animation_system(cx: &mut Context) -> bool {
    cx.style.play_pending_animations();

    // Tick all animations

    let time = Instant::now();

    let mut redraw_entities = Vec::new();
    let mut reflow_entities = Vec::new();
    let mut relayout_entities = Vec::new();
    let mut retransform_entities = Vec::new();
    let mut reclip_entities = Vec::new();

    // Properties which affect rendering
    // Opacity
    redraw_entities.extend(cx.style.opacity.tick(time));
    // Corner Colour
    redraw_entities.extend(cx.style.border_color.tick(time));
    // Corner Radius
    redraw_entities.extend(cx.style.corner_top_left_radius.tick(time));
    redraw_entities.extend(cx.style.corner_top_right_radius.tick(time));
    redraw_entities.extend(cx.style.corner_bottom_left_radius.tick(time));
    redraw_entities.extend(cx.style.corner_bottom_right_radius.tick(time));
    // Background
    redraw_entities.extend(cx.style.background_color.tick(time));
    redraw_entities.extend(cx.style.background_image.tick(time));
    redraw_entities.extend(cx.style.background_size.tick(time));
    // Box Shadow
    redraw_entities.extend(cx.style.shadow.tick(time));
    // Transform
    retransform_entities.extend(cx.style.transform.tick(time));
    retransform_entities.extend(cx.style.transform_origin.tick(time));
    retransform_entities.extend(cx.style.translate.tick(time));
    retransform_entities.extend(cx.style.rotate.tick(time));
    retransform_entities.extend(cx.style.scale.tick(time));
    // Outline
    redraw_entities.extend(cx.style.outline_color.tick(time));
    redraw_entities.extend(cx.style.outline_offset.tick(time));
    redraw_entities.extend(cx.style.outline_width.tick(time));
    // Clip Path
    reclip_entities.extend(cx.style.clip_path.tick(time));

    redraw_entities.extend(cx.style.fill.tick(time));

    // Font Color
    reflow_entities.extend(cx.style.font_color.tick(time));
    // Font Size
    reflow_entities.extend(cx.style.font_size.tick(time));

    // Properties which affect layout
    relayout_entities.extend(cx.style.display.tick(time));
    // Border Width
    relayout_entities.extend(cx.style.border_width.tick(time));
    // Space
    relayout_entities.extend(cx.style.left.tick(time));
    relayout_entities.extend(cx.style.right.tick(time));
    relayout_entities.extend(cx.style.top.tick(time));
    relayout_entities.extend(cx.style.bottom.tick(time));
    // Size
    relayout_entities.extend(cx.style.width.tick(time));
    relayout_entities.extend(cx.style.height.tick(time));
    // Min/Max Size
    relayout_entities.extend(cx.style.max_width.tick(time));
    relayout_entities.extend(cx.style.max_height.tick(time));
    relayout_entities.extend(cx.style.min_width.tick(time));
    relayout_entities.extend(cx.style.min_height.tick(time));
    // Min/Max Gap
    relayout_entities.extend(cx.style.max_horizontal_gap.tick(time));
    relayout_entities.extend(cx.style.max_vertical_gap.tick(time));
    relayout_entities.extend(cx.style.min_horizontal_gap.tick(time));
    relayout_entities.extend(cx.style.min_vertical_gap.tick(time));
    // Row/Col Between
    relayout_entities.extend(cx.style.vertical_gap.tick(time));
    relayout_entities.extend(cx.style.horizontal_gap.tick(time));
    // Child Space
    relayout_entities.extend(cx.style.padding_left.tick(time));
    relayout_entities.extend(cx.style.padding_right.tick(time));
    relayout_entities.extend(cx.style.padding_top.tick(time));
    relayout_entities.extend(cx.style.padding_bottom.tick(time));

    if !relayout_entities.is_empty() {
        cx.style.system_flags.set(SystemFlags::RELAYOUT, true);
    }

    for entity in redraw_entities.iter() {
        cx.needs_redraw(*entity);
    }

    for entity in reflow_entities.iter() {
        cx.style.text_construction.insert(*entity).unwrap();
    }

    for entity in retransform_entities.iter() {
        cx.style.needs_retransform(*entity, &cx.tree);
    }

    for entity in reclip_entities.iter() {
        cx.style.needs_reclip(*entity, &cx.tree);
    }

    !redraw_entities.is_empty()
        | !relayout_entities.is_empty()
        | !reflow_entities.is_empty()
        | !retransform_entities.is_empty()
        | !reclip_entities.is_empty()
}
