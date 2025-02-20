use crate::{animation::Interpolator, cache::CachedData, prelude::*};
use morphorm::Node;
use skia_safe::{
    canvas::SaveLayerRec, ClipOp, ImageFilter, Matrix, Paint, Rect, SamplingOptions, Surface,
};
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use vizia_storage::{DrawChildIterator, LayoutTreeIterator};
use vizia_style::BlendMode;

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

            let root_bounds = BoundingBox::from_min_max(
                -f32::MAX / 2.0,
                -f32::MAX / 2.0,
                f32::MAX / 2.0,
                f32::MAX / 2.0,
            );

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
            let clip_bounds: BoundingBox = transform.map_rect(rect).0.into();

            let parent_clip_bounds = cx.cache.clip_path.get(parent).copied().unwrap_or(root_bounds);

            if let Some(clip_path) = cx.cache.clip_path.get_mut(entity) {
                *clip_path = clip_bounds.intersection(&parent_clip_bounds);
            } else {
                cx.cache.clip_path.insert(entity, clip_bounds.intersection(&parent_clip_bounds));
            }
        }
    }
}

pub(crate) fn draw_system(
    cx: &mut Context,
    window_entity: Entity,
    surface: &mut Surface,
    dirty_surface: &mut Surface,
) -> bool {
    if cx.windows.is_empty() {
        return false;
    }

    if !cx.entity_manager.is_alive(window_entity) {
        return false;
    }

    transform_system(cx);

    let window = cx.windows.get_mut(&window_entity).unwrap();

    let mut dirty_rect = std::mem::take(&mut window.dirty_rect);
    let redraw_list = std::mem::take(&mut window.redraw_list);

    // if redraw_list.is_empty() {
    //     return false;
    // }

    for &entity in &redraw_list {
        // Skip binding views
        if cx.tree.is_ignored(entity) {
            continue;
        }

        if cx.tree.is_window(entity) && entity != window_entity {
            continue;
        }

        if entity.visible(&cx.style) {
            let draw_bounds = draw_bounds(&cx.style, &cx.cache, &cx.tree, entity);

            let mut dirty_bounds = draw_bounds;

            if let Some(previous_draw_bounds) = cx.cache.draw_bounds.get(entity) {
                dirty_bounds = dirty_bounds.union(previous_draw_bounds);
            }

            if dirty_bounds.w != 0.0 && dirty_bounds.h != 0.0 {
                if let Some(dr) = &mut dirty_rect {
                    *dr = dr.union(&dirty_bounds);
                } else {
                    dirty_rect = Some(dirty_bounds);
                }
            }

            if let Some(dr) = cx.cache.draw_bounds.get_mut(entity) {
                *dr = draw_bounds;
            } else {
                cx.cache.draw_bounds.insert(entity, draw_bounds);
            }
        }
    }

    // if dirty_rect.is_none() {
    //     return true;
    // }

    let canvas = dirty_surface.canvas();

    canvas.save();

    if let Some(rect) = dirty_rect.map(Rect::from) {
        canvas.clip_rect(rect, ClipOp::Intersect, false);
        canvas.clear(Color::transparent());
    }

    cx.resource_manager.mark_images_unused();

    let mut queue = BinaryHeap::new();
    queue.push(ZEntity { index: 0, entity: window_entity, visible: true });

    while let Some(zentity) = queue.pop() {
        canvas.save();
        draw_entity(
            &mut DrawContext {
                current: zentity.entity,
                style: &cx.style,
                cache: &mut cx.cache,
                tree: &cx.tree,
                models: &cx.models,
                views: &mut cx.views,
                resource_manager: &cx.resource_manager,
                text_context: &mut cx.text_context,
                modifiers: &cx.modifiers,
                mouse: &cx.mouse,
                windows: &mut cx.windows,
            },
            &dirty_rect,
            canvas,
            zentity.index,
            &mut queue,
            zentity.visible,
        );
        canvas.restore();
    }

    canvas.restore();

    surface.canvas().clear(Color::transparent());
    dirty_surface.draw(surface.canvas(), (0, 0), SamplingOptions::default(), None);

    // Debug draw dirty rect
    // if let Some(rect) = dirty_rect.map(Rect::from) {
    //     let mut paint = Paint::default();
    //     paint.set_style(skia_safe::PaintStyle::Stroke);
    //     paint.set_color(Color::red());
    //     paint.set_stroke_width(1.0);
    //     surface.canvas().draw_rect(rect, &paint);
    // }

    true
}

fn draw_entity(
    cx: &mut DrawContext,
    dirty_rect: &Option<BoundingBox>,
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
            let mut filter = ImageFilter::crop(rect, None, None).unwrap();

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
        if let Some(dirty_rect) = dirty_rect {
            let bounds = draw_bounds(cx.style, cx.cache, cx.tree, current);
            if bounds.intersects(dirty_rect) {
                if let Some(view) = cx.views.remove(&current) {
                    view.draw(cx, canvas);
                    cx.views.insert(current, view);
                }
            }
        }
    }

    let child_iter = DrawChildIterator::new(cx.tree, cx.current);

    // Draw its children
    for child in child_iter {
        cx.current = child;
        // TODO: Skip views with zero-sized bounding boxes here? Or let user decide if they want to skip?
        draw_entity(cx, dirty_rect, canvas, current_z, queue, is_visible);
    }

    if let Some(count) = layer_count {
        canvas.restore_to_count(count);
    }
    canvas.restore();
    cx.current = current;
}

// Must be called after transform and clipping systems to be valid.
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

            let scale_factor = style.scale_factor();

            if let Some(blur_radius) =
                shadow.blur_radius.as_ref().map(|br| br.clone().to_px().unwrap() * scale_factor)
            {
                shadow_bounds = shadow_bounds.expand(blur_radius);
            }

            if let Some(spread_radius) =
                shadow.spread_radius.as_ref().map(|sr| sr.clone().to_px().unwrap() * scale_factor)
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

    let matrix = cache.transform.get(entity).copied().unwrap_or_default();

    let rect: Rect = layout_bounds.into();
    let tr = matrix.map_rect(rect).0;

    let mut dirty_bounds: BoundingBox = tr.into();

    dirty_bounds = dirty_bounds.expand(1.0);

    //
    if style.overflowx.get(entity).copied().unwrap_or_default() == Overflow::Visible
        || style.overflowy.get(entity).copied().unwrap_or_default() == Overflow::Visible
    {
        let child_iter = DrawChildIterator::new(tree, entity);
        for child in child_iter {
            dirty_bounds = dirty_bounds.union(&draw_bounds(style, cache, tree, child));
        }
    }

    let z_index = style.z_index.get(entity).copied().unwrap_or_default();

    let parent = tree
        .get_layout_parent(entity)
        .unwrap_or(tree.get_parent_window(entity).unwrap_or(Entity::root()));
    if let Some(clip_bounds) = cache.clip_path.get(parent) {
        if z_index != 0 {
            dirty_bounds
        } else {
            dirty_bounds.intersection(clip_bounds)
        }
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
