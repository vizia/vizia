use crate::{animation::Interpolator, prelude::*};
use skia_safe::{ClipOp, Paint, Rect};
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use vizia_storage::{LayoutChildIterator, LayoutTreeIterator};
use vizia_style::display;

pub(crate) fn transform_system(cx: &mut Context) {
    let iter = LayoutTreeIterator::full(&cx.tree);

    for entity in iter {
        if let Some(parent) = cx.tree.get_layout_parent(entity) {
            let parent_transform = cx.cache.transform.get(parent).copied().unwrap();
            if let Some(tx) = cx.cache.transform.get_mut(entity) {
                let bounds = cx.cache.bounds.get(entity).copied().unwrap();
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
        }
    }
}

pub(crate) fn draw_system(cx: &mut Context) {
    transform_system(cx);

    for entity in cx.style.redraw_list.iter() {
        if let Some(bounds) = cx.cache.bounds.get(*entity) {
            if bounds.w != 0.0 && bounds.h != 0.0 {
                let matrix = cx.cache.transform.get(*entity).copied().unwrap();
                // let transformed_bounds = bounds.transform(&matrix);
                let rect: Rect = (*bounds).into();
                let tr = matrix.map_rect(rect).0;
                println!("{} {:?}", entity, tr);

                if let Some(dr) = &mut cx.cache.dirty_rect {
                    *dr = dr.union(&tr.into());
                } else {
                    cx.cache.dirty_rect = Some(tr.into());
                }
            }
        }
    }

    let canvas = cx.canvases.get_mut(&Entity::root()).unwrap().canvas();
    cx.resource_manager.mark_images_unused();

    let clear_color =
        cx.style.background_color.get(Entity::root()).cloned().unwrap_or(Color::transparent());
    canvas.clear(clear_color);

    let mut queue = BinaryHeap::new();
    queue.push(ZEntity { index: 0, entity: Entity::root(), opacity: 1.0, visible: true });
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
                opacity: zentity.opacity,
            },
            canvas,
            zentity.index,
            &mut queue,
            zentity.visible,
        );
        canvas.restore();
    }

    // Debug draw dirty rect
    if let Some(dirty_rect) = cx.cache.dirty_rect {
        let path: Rect = dirty_rect.into();
        let mut paint = Paint::default();
        paint.set_style(skia_safe::PaintStyle::Stroke);
        paint.set_color(Color::red());
        paint.set_stroke_width(1.0);
        canvas.draw_rect(&path, &paint);
        println!("{}", dirty_rect);
    }

    cx.style.redraw_list.clear();
    cx.cache.dirty_rect = None;

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

    // TODO: Looks like I'll need to keep track of the current transform manually instead of within femtovg
    // because elements with a higher z-index aren't getting the transform of their parent.
    let z_index = cx.style.z_index.get(current).copied().unwrap_or_default();
    if z_index > current_z {
        queue.push(ZEntity { index: z_index, entity: current, opacity: cx.opacity, visible });
        return;
    }

    canvas.save();
    let layer_count = if cx.opacity() != 1.0 {
        let rect: Rect = cx.bounds().into();
        Some(canvas.save_layer_alpha_f(None, cx.opacity()))
    } else {
        None
    };
    // canvas.save();
    // canvas.set_transform(&cx.transform());
    canvas.concat(&cx.transform());

    if let Some(clip_path) = cx.clip_path() {
        canvas.clip_path(&clip_path, ClipOp::Intersect, true);
    }
    // canvas.intersect_scissor(clip_region.x, clip_region.y, clip_region.w, clip_region.h);

    let is_visible = match (visible, cx.visibility()) {
        (v, None) => v,
        (_, Some(Visibility::Hidden)) => false,
        (_, Some(Visibility::Visible)) => true,
    };

    // Draw the view
    if is_visible {
        if let Some(view) = cx.views.remove(&current) {
            view.draw(cx, canvas);
            cx.views.insert(current, view);
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

    if let Some(dirty_rect) = cx.cache.dirty_rect {
        let bounds = cx.bounds();
        if bounds.intersects(&dirty_rect) {
            let path: Rect = bounds.into();
            let mut paint = Paint::default();
            paint.set_style(skia_safe::PaintStyle::Stroke);
            paint.set_color(Color::green());
            paint.set_stroke_width(1.0);
            canvas.draw_rect(&path, &paint);
        }
    }
}

struct ZEntity {
    pub index: i32,
    pub entity: Entity,
    pub opacity: f32,
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
