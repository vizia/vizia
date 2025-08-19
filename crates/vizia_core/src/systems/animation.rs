use morphorm::Node;

use crate::{layout::node::SubLayout, prelude::*};

macro_rules! process_auto_animations {
    ($cx:expr, $property:expr, $height:expr) => {
        if let Some(animations) = $property.get_active_animations() {
            let mut entities = vec![];

            for animation in animations {
                if animation.keyframes.iter().any(|keyframe| keyframe.value == Units::Auto) {
                    for entity in animation.entities.iter() {
                        entities.push((*entity, animation.clone()));
                    }
                }
            }

            for (entity, mut animation) in entities {
                $property.stop_animation(entity, animation.id);
                $property.insert(entity, Units::Auto);

                let size = entity.layout(
                    &mut $cx.cache,
                    &$cx.tree,
                    &$cx.style,
                    &mut SubLayout {
                        text_context: &mut $cx.text_context,
                        resource_manager: &$cx.resource_manager,
                    },
                );

                $property.remove(entity);
                animation.keyframes.iter_mut().for_each(|keyframe| {
                    if keyframe.value == Units::Auto {
                        let parent = $cx.tree.get_parent(entity).unwrap_or(Entity::root());
                        let parent_layout_type =
                            $cx.style.layout_type.get(parent).copied().unwrap_or_default();
                        let value = if (parent_layout_type == LayoutType::Row) == $height {
                            size.cross
                        } else {
                            size.main
                        };
                        keyframe.value = Units::Pixels(value);
                    }
                });

                let id = $cx.style.animation_manager.create();
                $property.insert_animation(id, animation.clone());
                $property.play_animation(
                    entity,
                    id,
                    Animation {
                        duration: animation.duration,
                        delay: animation.delay,
                        ..Default::default()
                    },
                    animation.start_time,
                );
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

    // if let Some(animations) = cx.style.max_height.get_active_animations() {
    //     let mut entities = vec![];

    //     for animation in animations {
    //         if animation.keyframes.iter().any(|keyframe| keyframe.value == Units::Auto) {
    //             for entity in animation.entities.iter() {
    //                 entities.push((*entity, animation.clone()));
    //             }
    //         }
    //     }

    //     for (entity, mut animation) in entities {
    //         cx.style.max_height.stop_animation(entity, animation.id);
    //         cx.style.max_height.insert(entity, Units::Auto);

    //         let size = entity.layout(
    //             &mut cx.cache,
    //             &cx.tree,
    //             &cx.style,
    //             &mut SubLayout {
    //                 text_context: &mut cx.text_context,
    //                 resource_manager: &cx.resource_manager,
    //             },
    //         );

    //         cx.style.max_height.remove(entity);
    //         animation.keyframes.iter_mut().for_each(|keyframe| {
    //             if keyframe.value == Units::Auto {
    //                 keyframe.value = Units::Pixels(size.main);
    //             }
    //         });

    //         let id = cx.style.animation_manager.create();
    //         cx.style.max_height.insert_animation(id, animation.clone());
    //         cx.style.max_height.play_animation(
    //             entity,
    //             id,
    //             animation.start_time,
    //             animation.duration,
    //             animation.delay,
    //         );
    //     }
    // }

    // Tick all animations

    let time = Instant::now();

    let mut redraw_entities = Vec::new();
    let mut reflow_entities = Vec::new();
    let mut relayout_entities = Vec::new();

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
    redraw_entities.extend(cx.style.transform.tick(time));
    redraw_entities.extend(cx.style.transform_origin.tick(time));
    redraw_entities.extend(cx.style.translate.tick(time));
    redraw_entities.extend(cx.style.rotate.tick(time));
    redraw_entities.extend(cx.style.scale.tick(time));
    // Outline
    redraw_entities.extend(cx.style.outline_color.tick(time));
    redraw_entities.extend(cx.style.outline_offset.tick(time));
    redraw_entities.extend(cx.style.outline_width.tick(time));
    // Clip Path
    redraw_entities.extend(cx.style.clip_path.tick(time));

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

    !redraw_entities.is_empty() | !relayout_entities.is_empty() | !reflow_entities.is_empty()
}
