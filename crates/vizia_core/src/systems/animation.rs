use morphorm::Node;

use crate::{layout::node::SubLayout, prelude::*};

macro_rules! process_auto_animations {
    ($cx:expr, $property:expr, $height:expr) => {
        if let Some(animations) = $property.get_active_animations() {
            let mut entities = vec![];

            for animation in animations {
                if animation.keyframes.iter().any(|keyframe| keyframe.value == Units::Auto) {
                    for entity in animation.entities.iter() {
                        let current_bounds = $cx.cache.get_bounds(*entity);
                        let current_measured =
                            if $height { current_bounds.h } else { current_bounds.w };
                        entities.push((*entity, animation.clone(), current_measured));
                    }
                }
            }

            if entities.is_empty() {
                // No auto keyframes for this property in the current frame.
            } else {
                // Resolve auto values against a root layout pass so wrapped text is measured using
                // real parent constraints (especially width) rather than isolated node layout.
                for (entity, animation, _) in entities.iter() {
                    $property.stop_animation(*entity, animation.id);
                    $property.insert(*entity, Units::Auto);
                }

                Entity::root().layout(
                    &mut $cx.cache,
                    &$cx.tree,
                    &$cx.style,
                    &mut SubLayout {
                        text_context: &mut $cx.text_context,
                        resource_manager: &$cx.resource_manager,
                    },
                );

                for (entity, mut animation, current_measured) in entities {
                    $property.remove(entity);

                    let measured_target =
                        if let Some(bounds) = $cx.cache.relative_bounds.get(entity) {
                            if $height { bounds.h } else { bounds.w }
                        } else {
                            let bounds = $cx.cache.get_bounds(entity);
                            if $height { bounds.h } else { bounds.w }
                        };

                    animation.keyframes.iter_mut().for_each(|keyframe| {
                        if keyframe.value == Units::Auto {
                            // Preserve transition direction: start keyframes resolve from current
                            // geometry, later keyframes resolve from target auto geometry.
                            let measured = if keyframe.time <= 0.0 {
                                current_measured
                            } else {
                                measured_target
                            };
                            keyframe.value = Units::Pixels(measured);
                        }
                    });

                    let id = $cx.style.animation_manager.create();
                    $property.insert_animation(id, animation.clone());
                    $property.play_animation(
                        entity,
                        id,
                        animation.start_time,
                        animation.duration,
                        animation.delay,
                    );
                }
            }
        }
    };
}

pub(crate) fn animation_system(cx: &mut Context) -> bool {
    cx.style.play_pending_animations();

    process_auto_animations!(cx, cx.style.max_height, true);
    process_auto_animations!(cx, cx.style.max_width, false);
    process_auto_animations!(cx, cx.style.height, true);
    process_auto_animations!(cx, cx.style.width, false);

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

    // Tick animations on custom color properties
    for store in cx.style.custom_color_props.values_mut() {
        redraw_entities.extend(store.tick(time));
    }
    // Tick animations on custom length properties
    for store in cx.style.custom_length_props.values_mut() {
        redraw_entities.extend(store.tick(time));
    }
    // Tick animations on custom font-size properties
    for store in cx.style.custom_font_size_props.values_mut() {
        reflow_entities.extend(store.tick(time));
    }
    // Tick animations on custom units properties
    for store in cx.style.custom_units_props.values_mut() {
        relayout_entities.extend(store.tick(time));
    }
    // Tick animations on custom opacity properties
    for store in cx.style.custom_opacity_props.values_mut() {
        redraw_entities.extend(store.tick(time));
    }

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
        cx.needs_retransform(*entity);
        cx.needs_redraw(*entity);
    }

    for entity in reclip_entities.iter() {
        cx.needs_reclip(*entity);
        cx.needs_redraw(*entity);
    }

    !redraw_entities.is_empty()
        | !relayout_entities.is_empty()
        | !reflow_entities.is_empty()
        | !retransform_entities.is_empty()
        | !reclip_entities.is_empty()
}
