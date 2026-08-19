use morphorm::Node;

use crate::{animation::view_progress, layout::node::SubLayout, prelude::*};
use vizia_style::{AnimationScroller, AnimationTimeline, AnimationTimelineAxis};

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

fn nearest_scroll_source(cx: &Context, entity: Entity) -> Option<Entity> {
    let mut current = cx.tree.get_layout_parent(entity);
    while let Some(entity) = current {
        if cx.style.scroll_timeline_sources.contains_key(&entity) {
            return Some(entity);
        }
        current = cx.tree.get_layout_parent(entity);
    }
    None
}

fn root_scroll_source(cx: &Context, entity: Entity) -> Option<Entity> {
    let mut current = Some(entity);
    let mut result = None;
    while let Some(entity) = current {
        if cx.style.scroll_timeline_sources.contains_key(&entity) {
            result = Some(entity);
        }
        current = cx.tree.get_layout_parent(entity);
    }
    result
}

fn source_progress(cx: &Context, source: Entity, axis: AnimationTimelineAxis) -> Option<f32> {
    cx.entity_manager
        .is_alive(source)
        .then(|| cx.style.scroll_timeline_sources.get(&source).copied())
        .flatten()
        .map(|source| source.progress(axis))
}

fn view_timeline_progress(
    cx: &Context,
    entity: Entity,
    axis: AnimationTimelineAxis,
) -> Option<f32> {
    let source_entity = nearest_scroll_source(cx, entity)?;
    let source = cx.style.scroll_timeline_sources.get(&source_entity).copied()?;
    let subject = cx.cache.get_bounds(entity);
    let viewport = cx.cache.get_bounds(source_entity);
    Some(match axis {
        AnimationTimelineAxis::Block | AnimationTimelineAxis::Y => {
            let offset = (source.inner_height - source.container_height).max(0.0) * source.y;
            view_progress(subject.y, subject.h, viewport.y, viewport.h, offset)
        }
        AnimationTimelineAxis::Inline | AnimationTimelineAxis::X => {
            let offset = (source.inner_width - source.container_width).max(0.0) * source.x;
            view_progress(subject.x, subject.w, viewport.x, viewport.w, offset)
        }
    })
}

fn timeline_progress_changed(previous: Option<f32>, next: Option<f32>) -> bool {
    match (previous, next) {
        (Some(previous), Some(next)) => (previous - next).abs() > f32::EPSILON,
        (None, None) => false,
        _ => true,
    }
}

fn refresh_progress_timelines(cx: &mut Context) {
    let requests = cx
        .style
        .css_animation_instances
        .iter()
        .flat_map(|(entity, instances)| {
            instances.iter().map(move |instance| {
                (
                    *entity,
                    instance.instance_id,
                    instance.timeline.clone(),
                    instance.timeline_driven,
                    instance.timeline_progress,
                )
            })
        })
        .collect::<Vec<_>>();

    let mut samples = Vec::new();
    for (entity, instance_id, timeline, was_driven, previous_progress) in requests {
        let (driven, progress) = match timeline {
            AnimationTimeline::Auto => (false, None),
            AnimationTimeline::None => (true, None),
            AnimationTimeline::Named(name) => {
                let progress =
                    cx.style.named_scroll_timelines.get(&name).copied().and_then(|source| {
                        source_progress(cx, source, AnimationTimelineAxis::Block)
                    });
                (true, progress)
            }
            AnimationTimeline::Scroll { scroller, axis } => {
                let source = match scroller {
                    AnimationScroller::Self_ => {
                        cx.style.scroll_timeline_sources.contains_key(&entity).then_some(entity)
                    }
                    AnimationScroller::Nearest => nearest_scroll_source(cx, entity),
                    AnimationScroller::Root => root_scroll_source(cx, entity),
                };
                (true, source.and_then(|source| source_progress(cx, source, axis)))
            }
            AnimationTimeline::View { axis } => (true, view_timeline_progress(cx, entity, axis)),
        };

        if driven != was_driven || timeline_progress_changed(previous_progress, progress) {
            samples.push((entity, instance_id, driven, progress));
        }
    }

    for (entity, instance_id, driven, progress) in samples {
        cx.style.set_css_timeline_progress(entity, instance_id, driven, progress);
    }
}

pub(crate) fn animation_system(cx: &mut Context) -> bool {
    cx.style.play_pending_animations();

    process_auto_animations!(cx, cx.style.max_height, true);
    process_auto_animations!(cx, cx.style.max_width, false);
    process_auto_animations!(cx, cx.style.height, true);
    process_auto_animations!(cx, cx.style.width, false);

    // Tick all animations

    let time = Instant::now();
    refresh_progress_timelines(cx);

    let mut redraw_entities = Vec::new();
    let mut reflow_entities = Vec::new();
    let mut relayout_entities = Vec::new();
    let mut retransform_entities = Vec::new();
    let mut reclip_entities = Vec::new();
    let mut repath_entities = Vec::new();
    let mut has_active_layout_animations = false;

    // Properties which affect rendering
    // Opacity
    redraw_entities.extend(cx.style.opacity.tick(time));
    // Filters. Track only the entities which can require filter-aware dirty-bound work.
    let filter_entities = cx.style.filter.tick(time);
    cx.style.filter_entities.extend(filter_entities.iter().copied());
    redraw_entities.extend(filter_entities);
    let backdrop_filter_entities = cx.style.backdrop_filter.tick(time);
    cx.style.filter_entities.extend(backdrop_filter_entities.iter().copied());
    redraw_entities.extend(backdrop_filter_entities);
    // Corner Colour
    redraw_entities.extend(cx.style.border_top_color.tick(time));
    redraw_entities.extend(cx.style.border_right_color.tick(time));
    redraw_entities.extend(cx.style.border_bottom_color.tick(time));
    redraw_entities.extend(cx.style.border_left_color.tick(time));
    // Corner Radius and smoothing. Radius changes also affect rounded clipping.
    let corner_top_left = cx.style.corner_top_left_radius.tick(time);
    let corner_top_right = cx.style.corner_top_right_radius.tick(time);
    let corner_bottom_left = cx.style.corner_bottom_left_radius.tick(time);
    let corner_bottom_right = cx.style.corner_bottom_right_radius.tick(time);
    redraw_entities.extend(corner_top_left.iter().copied());
    redraw_entities.extend(corner_top_right.iter().copied());
    redraw_entities.extend(corner_bottom_left.iter().copied());
    redraw_entities.extend(corner_bottom_right.iter().copied());
    repath_entities.extend(corner_top_left.iter().copied());
    repath_entities.extend(corner_top_right.iter().copied());
    repath_entities.extend(corner_bottom_left.iter().copied());
    repath_entities.extend(corner_bottom_right.iter().copied());
    reclip_entities.extend(corner_top_left);
    reclip_entities.extend(corner_top_right);
    reclip_entities.extend(corner_bottom_left);
    reclip_entities.extend(corner_bottom_right);
    let corner_top_left_smoothing = cx.style.corner_top_left_smoothing.tick(time);
    let corner_top_right_smoothing = cx.style.corner_top_right_smoothing.tick(time);
    let corner_bottom_left_smoothing = cx.style.corner_bottom_left_smoothing.tick(time);
    let corner_bottom_right_smoothing = cx.style.corner_bottom_right_smoothing.tick(time);
    for entities in [
        &corner_top_left_smoothing,
        &corner_top_right_smoothing,
        &corner_bottom_left_smoothing,
        &corner_bottom_right_smoothing,
    ] {
        redraw_entities.extend(entities.iter().copied());
        reclip_entities.extend(entities.iter().copied());
        repath_entities.extend(entities.iter().copied());
    }
    // Background
    redraw_entities.extend(cx.style.background_color.tick(time));
    redraw_entities.extend(cx.style.background_image.tick(time));
    redraw_entities.extend(cx.style.background_position.tick(time));
    redraw_entities.extend(cx.style.background_repeat.tick(time));
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

    // Font and decoration colors are baked into Skia's paragraph text styles, so a redraw alone
    // would keep painting the paragraph with its previous colors. Rebuild the paragraph whenever
    // either animated value changes. Caret and selection colors are read directly while drawing.
    reflow_entities.extend(cx.style.font_color.tick(time));
    redraw_entities.extend(cx.style.caret_color.tick(time));
    redraw_entities.extend(cx.style.selection_color.tick(time));
    reflow_entities.extend(cx.style.text_decoration_color.tick(time));
    // Font Size
    reflow_entities.extend(cx.style.font_size.tick(time));
    // Letter Spacing
    reflow_entities.extend(cx.style.letter_spacing.tick(time));
    // Line Height
    reflow_entities.extend(cx.style.line_height.tick(time));

    // Properties which affect layout. Keep the animation frame loop alive while
    // avoiding a relayout when a stepped/paused animation sampled the same value.
    macro_rules! tick_layout {
        ($store:expr) => {{
            has_active_layout_animations |= $store.has_animations();
            relayout_entities.extend($store.tick_changed(time));
        }};
    }

    relayout_entities.extend(cx.style.display.tick(time));
    // Border Width
    tick_layout!(cx.style.border_top_width);
    tick_layout!(cx.style.border_right_width);
    tick_layout!(cx.style.border_bottom_width);
    tick_layout!(cx.style.border_left_width);
    // Space
    tick_layout!(cx.style.left);
    tick_layout!(cx.style.right);
    tick_layout!(cx.style.top);
    tick_layout!(cx.style.bottom);
    // Size
    tick_layout!(cx.style.width);
    tick_layout!(cx.style.height);
    // Min/Max Size
    tick_layout!(cx.style.max_width);
    tick_layout!(cx.style.max_height);
    tick_layout!(cx.style.min_width);
    tick_layout!(cx.style.min_height);
    // Min/Max Gap
    tick_layout!(cx.style.max_horizontal_gap);
    tick_layout!(cx.style.max_vertical_gap);
    tick_layout!(cx.style.min_horizontal_gap);
    tick_layout!(cx.style.min_vertical_gap);
    // Row/Col Between
    tick_layout!(cx.style.vertical_gap);
    tick_layout!(cx.style.horizontal_gap);
    // Child Space
    tick_layout!(cx.style.padding_left);
    tick_layout!(cx.style.padding_right);
    tick_layout!(cx.style.padding_top);
    tick_layout!(cx.style.padding_bottom);

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
    // Tick animations on custom letter-spacing properties
    for store in cx.style.custom_letter_spacing_props.values_mut() {
        reflow_entities.extend(store.tick(time));
    }
    // Tick animations on custom line-height properties
    for store in cx.style.custom_line_height_props.values_mut() {
        reflow_entities.extend(store.tick(time));
    }
    // Tick animations on custom units properties
    for store in cx.style.custom_units_props.values_mut() {
        has_active_layout_animations |= store.has_animations();
        relayout_entities.extend(store.tick_changed(time));
    }
    // Tick animations on custom opacity properties
    for store in cx.style.custom_opacity_props.values_mut() {
        redraw_entities.extend(store.tick(time));
    }
    // Tick animations on custom shadow properties.
    for store in cx.style.custom_shadow_props.values_mut() {
        redraw_entities.extend(store.tick(time));
    }

    // CSS animation lifecycle events are emitted once per named animation.
    let lifecycle_events = cx.style.tick_css_animation_events(time);
    for lifecycle_event in lifecycle_events {
        let entity = lifecycle_event.entity;
        cx.event_queue.push_back(
            Event::new(lifecycle_event).target(entity).origin(entity).propagate(Propagation::Up),
        );
    }

    for entity in relayout_entities.iter() {
        cx.style.needs_relayout(*entity);
    }

    for entity in redraw_entities.iter() {
        cx.needs_redraw(*entity);
    }

    for entity in reflow_entities.iter() {
        cx.style.text_construction.insert(*entity);
    }

    for entity in retransform_entities.iter() {
        cx.needs_retransform(*entity);
        cx.needs_redraw(*entity);
    }

    for entity in repath_entities {
        cx.cache.path.remove(entity);
    }

    for entity in reclip_entities.iter() {
        cx.needs_reclip(*entity);
        cx.needs_redraw(*entity);
    }

    has_active_layout_animations
        | !redraw_entities.is_empty()
        | !relayout_entities.is_empty()
        | !reflow_entities.is_empty()
        | !retransform_entities.is_empty()
        | !reclip_entities.is_empty()
}
