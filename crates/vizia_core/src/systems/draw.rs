use crate::{animation::Interpolator, cache::CachedData, prelude::*};
use morphorm::Node;
use skia_safe::{canvas::SaveLayerRec, ClipOp, ImageFilter, Matrix, Paint, Rect, SamplingOptions};
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use vizia_storage::{LayoutChildIterator, LayoutTreeIterator};
use vizia_style::{backdrop_filter, blend_mode, BlendMode};

pub(crate) fn transform_system(cx: &mut Context) {
    let iter = LayoutTreeIterator::full(&cx.tree);

    for entity in iter {
        let bounds = cx.cache.bounds.get(entity).copied().unwrap();
        if let Some(parent) = cx.tree.get_layout_parent(entity) {
            let parent_transform = cx.cache.transform.get(parent).copied().unwrap();
            if let Some(tx) = cx.cache.transform.get_mut(entity) {
                let scale_factor = cx.style.scale_factor();

                // Apply transform origin.
                let mut origin = cx
                    .style
                    .transform_origin
                    .get(entity)
                    .map(|transform_origin| {
                        let mut origin = skia_safe::Matrix::translate(bounds.top_left());
                        let offset = transform_origin.as_transform(bounds, scale_factor);
                        origin = offset * origin;
                        origin
                    })
                    .unwrap_or(skia_safe::Matrix::translate(bounds.center()));
                // transform = origin * transform;
                let mut transform = origin;
                origin = origin.invert().unwrap();

                // Apply translation.
                if let Some(translate) = cx.style.translate.get(entity) {
                    transform = transform * translate.as_transform(bounds, scale_factor);
                }

                // Apply rotation.
                if let Some(rotate) = cx.style.rotate.get(entity) {
                    transform = transform * rotate.as_transform(bounds, scale_factor);
                }

                // Apply scaling.
                if let Some(scale) = cx.style.scale.get(entity) {
                    transform = transform * scale.as_transform(bounds, scale_factor);
                }

                // Apply transform functions.
                if let Some(transforms) = cx.style.transform.get(entity) {
                    // Check if the transform is currently animating
                    // Get the animation state
                    // Manually interpolate the value to get the overall transform for the current frame
                    if let Some(animation_state) = cx.style.transform.get_active_animation(entity) {
                        if let Some(start) = animation_state.keyframes.first() {
                            if let Some(end) = animation_state.keyframes.last() {
                                let start_transform =
                                    start.value.as_transform(bounds, scale_factor);
                                let end_transform = end.value.as_transform(bounds, scale_factor);
                                let t = animation_state.t;
                                let animated_transform = skia_safe::Matrix::interpolate(
                                    &start_transform,
                                    &end_transform,
                                    t,
                                );
                                transform = transform * animated_transform;
                            }
                        }
                    } else {
                        transform = transform * transforms.as_transform(bounds, scale_factor);
                    }
                }

                transform = transform * origin;

                *tx = parent_transform * transform;
            }

            let overflowx = cx.style.overflowx.get(entity).copied().unwrap_or_default();
            let overflowy = cx.style.overflowy.get(entity).copied().unwrap_or_default();

            let scale = cx.style.scale_factor();

            let clip_bounds = cx
                .style
                .clip_path
                .get(entity)
                .map(|clip| match clip {
                    ClipPath::Auto => bounds,
                    ClipPath::Shape(rect) => bounds.shrink_sides(
                        rect.3.to_pixels(bounds.w, scale),
                        rect.0.to_pixels(bounds.h, scale),
                        rect.1.to_pixels(bounds.w, scale),
                        rect.2.to_pixels(bounds.h, scale),
                    ),
                })
                .unwrap_or(bounds);

            let root_bounds = cx.cache.get_bounds(Entity::root());

            let clip_bounds = match (overflowx, overflowy) {
                (Overflow::Visible, Overflow::Visible) => root_bounds,
                (Overflow::Hidden, Overflow::Visible) => {
                    let left = clip_bounds.left();
                    let right = clip_bounds.right();
                    let top = root_bounds.top();
                    let bottom = root_bounds.bottom();
                    BoundingBox::from_min_max(left, top, right, bottom)
                }
                (Overflow::Visible, Overflow::Hidden) => {
                    let left = root_bounds.left();
                    let right = root_bounds.right();
                    let top = clip_bounds.top();
                    let bottom = clip_bounds.bottom();
                    BoundingBox::from_min_max(left, top, right, bottom)
                }
                (Overflow::Hidden, Overflow::Hidden) => clip_bounds,
            };

            let transform =
                cx.cache.transform.get(entity).copied().unwrap_or(Matrix::new_identity());

            let rect: skia_safe::Rect = clip_bounds.into();
            let clip_bounds: BoundingBox = transform.map_rect(&rect).0.into();

            let parent_clip_bounds = cx.cache.clip_path.get(parent).copied().unwrap_or(root_bounds);

            if let Some(clip_path) = cx.cache.clip_path.get_mut(entity) {
                *clip_path = clip_bounds.intersection(&parent_clip_bounds);
            } else {
                cx.cache.clip_path.insert(entity, clip_bounds.intersection(&parent_clip_bounds));
            }
        }
    }
}

pub(crate) fn draw_system(cx: &mut Context) {
    transform_system(cx);

    let children = cx
        .style
        .redraw_list
        .iter()
        .flat_map(|entity| LayoutTreeIterator::subtree(&cx.tree, *entity))
        .collect::<Vec<_>>();

    cx.style.redraw_list.extend(children.iter());

    for entity in cx.style.redraw_list.iter() {
        // Skip binding views
        if cx.tree.is_ignored(*entity) {
            continue;
        }

        if entity.visible(&cx.style) {
            let mut draw_bounds = draw_bounds(&cx.style, &cx.cache, &cx.tree, *entity);

            if let Some(previous_draw_bounds) = cx.cache.draw_bounds.get(*entity) {
                draw_bounds = draw_bounds.union(previous_draw_bounds);
            }

            if draw_bounds.w != 0.0 && draw_bounds.h != 0.0 {
                if let Some(dr) = &mut cx.cache.dirty_rect {
                    *dr = dr.union(&draw_bounds);
                } else {
                    cx.cache.dirty_rect = Some(draw_bounds);
                }
            }
        }
    }

    if let Some(canvas) = cx.canvases.get_mut(&Entity::root()).map(|(s1, s2)| s2.canvas()) {
        canvas.save();
        if let Some(dirty_rect) = cx.cache.dirty_rect {
            let rect: Rect = dirty_rect.into();
            canvas.clip_rect(&rect, ClipOp::Intersect, false);
        }

        cx.resource_manager.mark_images_unused();

        let clear_color =
            cx.style.background_color.get(Entity::root()).cloned().unwrap_or(Color::transparent());
        // canvas.clear(clear_color);

        let mut queue = BinaryHeap::new();
        queue.push(ZEntity { index: 0, entity: Entity::root(), visible: true });
        while !queue.is_empty() {
            let zentity = queue.pop().unwrap();
            canvas.save();
            draw_entity(
                &mut DrawContext {
                    current: zentity.entity,
                    style: &cx.style,
                    cache: &mut cx.cache,
                    tree: &cx.tree,
                    data: &cx.data,
                    views: &mut cx.views,
                    resource_manager: &cx.resource_manager,
                    text_context: &mut cx.text_context,
                    text_config: &cx.text_config,
                    modifiers: &cx.modifiers,
                    mouse: &cx.mouse,
                },
                canvas,
                zentity.index,
                &mut queue,
                zentity.visible,
            );
            canvas.restore();
        }
        canvas.restore();
    };

    if let Some((canvas, surface)) =
        cx.canvases.get_mut(&Entity::root()).map(|(s1, s2)| (s1.canvas(), s2))
    {
        surface.draw(canvas, (0, 0), SamplingOptions::default(), None);

        // // Debug draw dirty rect
        // if let Some(dirty_rect) = cx.cache.dirty_rect {
        //     let path: Rect = dirty_rect.into();
        //     let mut paint = Paint::default();
        //     paint.set_style(skia_safe::PaintStyle::Stroke);
        //     paint.set_color(Color::red());
        //     paint.set_stroke_width(1.0);
        //     canvas.draw_rect(&path, &paint);
        // }
    }

    cx.style.redraw_list.clear();
    cx.cache.dirty_rect = None;

    let iter = LayoutTreeIterator::full(&cx.tree);
    for entity in iter {
        if entity.visible(&cx.style) {
            let draw_bounds = draw_bounds(&cx.style, &cx.cache, &cx.tree, entity);
            if let Some(dr) = cx.cache.draw_bounds.get_mut(entity) {
                *dr = draw_bounds;
            } else {
                cx.cache.draw_bounds.insert(entity, draw_bounds);
            }
        }
    }

    // canvas.flush();
}

fn draw_entity(
    cx: &mut DrawContext,
    canvas: &Canvas,
    current_z: i32,
    queue: &mut BinaryHeap<ZEntity>,
    visible: bool,
) {
    let current = cx.current;

    // Skip views with display: none.
    if cx.display() == Display::None {
        return;
    }

    let z_index = cx.z_index();

    if z_index > current_z {
        queue.push(ZEntity { index: z_index, entity: current, visible });
        return;
    }

    let backdrop_filter = cx.backdrop_filter();
    let blend_mode = cx.style.blend_mode.get(current).copied().unwrap_or_default();

    canvas.save();
    let layer_count =
        if cx.opacity() != 1.0 || backdrop_filter.is_some() || blend_mode != BlendMode::Normal {
            let mut paint = Paint::default();
            paint.set_alpha_f(cx.opacity());
            paint.set_blend_mode(blend_mode.into());

            let rect: Rect = cx.bounds().into();
            let mut filter = ImageFilter::crop(&rect, None, None).unwrap();

            let slr = if let Some(backdrop_filter) = backdrop_filter {
                match backdrop_filter {
                    Filter::Blur(radius) => {
                        let sigma = radius.to_px().unwrap() * cx.scale_factor() / 2.0;
                        filter = filter.blur(None, (sigma, sigma), None).unwrap();
                        SaveLayerRec::default().paint(&paint).backdrop(&filter)
                    }
                }
            } else {
                SaveLayerRec::default().paint(&paint)
            };

            Some(canvas.save_layer(&slr))
        } else {
            None
        };

    if let Some(transform) = cx.cache.transform.get(current) {
        canvas.set_matrix(&(transform.into()));
    }

    if let Some(clip_path) = cx.clip_path() {
        canvas.clip_path(&clip_path, ClipOp::Intersect, true);
    }

    let is_visible = match (visible, cx.visibility()) {
        (v, None) => v,
        (_, Some(Visibility::Hidden)) => false,
        (_, Some(Visibility::Visible)) => true,
    };

    // Draw the view
    if is_visible {
        if let Some(dirty_rect) = cx.cache.dirty_rect {
            let bounds = draw_bounds(cx.style, cx.cache, cx.tree, current);
            if bounds.intersects(&dirty_rect) {
                if let Some(view) = cx.views.remove(&current) {
                    view.draw(cx, canvas);
                    cx.views.insert(current, view);
                }
            }
        }
    }

    let child_iter = LayoutChildIterator::new(cx.tree, cx.current);

    // Draw its children
    for child in child_iter {
        cx.current = child;
        // TODO: Skip views with zero-sized bounding boxes here? Or let user decide if they want to skip?
        draw_entity(cx, canvas, current_z, queue, is_visible);
    }

    if let Some(count) = layer_count {
        canvas.restore_to_count(count);
    }
    canvas.restore();
    cx.current = current;

    // if let Some(dirty_rect) = cx.cache.dirty_rect {
    //     let bounds = cx.bounds();
    //     if bounds.intersects(&dirty_rect) {
    //         let path: Rect = bounds.into();
    //         let mut paint = Paint::default();
    //         paint.set_style(skia_safe::PaintStyle::Stroke);
    //         paint.set_color(Color::green());
    //         paint.set_stroke_width(1.0);
    //         canvas.draw_rect(&path, &paint);
    //     }
    // }
}

// Must be called after transform and clipping systems to be valid
pub(crate) fn draw_bounds(
    style: &Style,
    cache: &CachedData,
    tree: &Tree<Entity>,
    entity: Entity,
) -> BoundingBox {
    let mut layout_bounds = cache.bounds.get(entity).copied().unwrap();

    if let Some(shadows) = style.shadow.get(entity) {
        for shadow in shadows.iter().filter(|shadow| !shadow.inset) {
            let mut shadow_bounds = layout_bounds;

            let x = shadow.x_offset.to_px().unwrap() * style.scale_factor();
            let y = shadow.y_offset.to_px().unwrap() * style.scale_factor();

            shadow_bounds = shadow_bounds.offset(x, y);

            if let Some(blur_radius) =
                shadow.blur_radius.as_ref().map(|br| br.clone().to_px().unwrap() / 2.0)
            {
                shadow_bounds = shadow_bounds.expand(blur_radius * style.scale_factor());
            }

            if let Some(spread_radius) =
                shadow.spread_radius.as_ref().map(|sr| sr.clone().to_px().unwrap())
            {
                shadow_bounds = shadow_bounds.expand(spread_radius * style.scale_factor());
            }

            layout_bounds = layout_bounds.union(&shadow_bounds);
        }
    }

    let mut outline_bounds = layout_bounds;

    if let Some(outline_width) = style.outline_width.get(entity) {
        outline_bounds = outline_bounds
            .expand(outline_width.to_pixels(layout_bounds.diagonal(), style.scale_factor()));
    }

    if let Some(outline_offset) = style.outline_offset.get(entity) {
        outline_bounds = outline_bounds
            .expand(outline_offset.to_pixels(layout_bounds.diagonal(), style.scale_factor()));
    }

    layout_bounds = layout_bounds.union(&outline_bounds);

    let matrix = cache.transform.get(entity).copied().unwrap();
    // let transformed_bounds = bounds.transform(&matrix);
    let rect: Rect = layout_bounds.into();
    let tr = matrix.map_rect(rect).0;

    let dirty_bounds: BoundingBox = tr.into();

    let parent = tree.get_layout_parent(entity).unwrap_or(Entity::root());
    if let Some(clip_bounds) = cache.clip_path.get(parent) {
        dirty_bounds.intersection(clip_bounds)
    } else {
        dirty_bounds
    }
}

struct ZEntity {
    pub index: i32,
    pub entity: Entity,
    pub visible: bool,
}

impl Ord for ZEntity {
    fn cmp(&self, other: &Self) -> Ordering {
        other.index.cmp(&self.index)
    }
}
impl PartialOrd for ZEntity {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl PartialEq for ZEntity {
    fn eq(&self, other: &Self) -> bool {
        self.index == other.index
    }
}

impl Eq for ZEntity {}
