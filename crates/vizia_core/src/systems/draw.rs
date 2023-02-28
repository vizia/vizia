use crate::cache::BoundingBox;
use crate::context::Context;
use crate::prelude::*;
use crate::style::Transform2D;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use vizia_id::GenerationalId;
use vizia_style::Clip;

pub fn draw_system(cx: &mut Context) {
    let canvas = cx.canvases.get_mut(&Entity::root()).unwrap();
    cx.resource_manager.mark_images_unused();
    let window_width = cx.cache.get_width(Entity::root());
    let window_height = cx.cache.get_height(Entity::root());
    let clear_color =
        cx.style.background_color.get(Entity::root()).cloned().unwrap_or(RGBA::WHITE.into());
    canvas.clear_rect(0, 0, window_width as u32, window_height as u32, clear_color.into());
    let mut queue = BinaryHeap::new();
    queue.push(ZEntity(0, Entity::root()));
    while !queue.is_empty() {
        let ZEntity(current_z, current) = queue.pop().unwrap();
        canvas.save();
        draw_entity(
            &mut DrawContext {
                current,
                captured: &cx.captured,
                focused: &cx.focused,
                hovered: &cx.hovered,
                style: &cx.style,
                cache: &mut cx.cache,
                draw_cache: &mut cx.draw_cache,
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
            current_z,
            &mut queue,
            true,
        );
        canvas.restore();
    }

    canvas.flush();
}

fn draw_entity(
    cx: &mut DrawContext,
    canvas: &mut Canvas,
    current_z: i32,
    queue: &mut BinaryHeap<ZEntity>,
    visible: bool,
) {
    let current = cx.current;
    if cx.cache.get_display(current) == Display::None || cx.cache.get_opacity(current) == 0.0 {
        return;
    }

    let bounds = cx.cache.get_bounds(current);

    let z_order = cx.tree.z_order(current);
    if z_order > current_z {
        queue.push(ZEntity(z_order, current));
        return;
    }

    canvas.save();

    if let Some(transforms) = cx.style.transform.get(current) {
        let mut translate = Transform2D::with_translate(bounds.center().0, bounds.center().1);

        let mut transform = Transform2D::identity();
        transform.premultiply(&translate);

        translate.inverse();

        // Check if the transform is currently animating
        // Get the animation state
        // Manually interpolate the value to get the overall transform for the current frame
        if let Some(animation_state) = cx.style.transform.get_active_animation(current) {
            if let Some(start) = animation_state.keyframes.first() {
                if let Some(end) = animation_state.keyframes.last() {
                    let start_transform =
                        Transform2D::from_style_transforms(&start.1, bounds, cx.scale_factor());
                    let end_transform =
                        Transform2D::from_style_transforms(&end.1, bounds, cx.scale_factor());
                    let t = animation_state.t;
                    let animated_transform =
                        Transform2D::interpolate(&start_transform, &end_transform, t);
                    transform.premultiply(&animated_transform);
                }
            }
        } else {
            transform.premultiply(&Transform2D::from_style_transforms(
                transforms,
                bounds,
                cx.scale_factor(),
            ));
        }

        transform.premultiply(&translate);

        let mut trans = femtovg::Transform2D::identity();
        canvas.set_transform(&trans.new(
            transform[0],
            transform[1],
            transform[2],
            transform[3],
            transform[4],
            transform[5],
        ));
    }

    let overflowx = cx.style.overflowx.get(current).copied().unwrap_or_default();
    let overflowy = cx.style.overflowy.get(current).copied().unwrap_or_default();

    let root_bounds = cx.cache.get_bounds(Entity::root());

    let clip_bounds = cx
        .style
        .clip
        .get(current)
        .map(|clip| match clip {
            Clip::Auto => bounds,
            Clip::Shape(rect) => bounds.shrink_sides(
                rect.3.to_px().unwrap() * cx.style.dpi_factor as f32,
                rect.0.to_px().unwrap() * cx.style.dpi_factor as f32,
                rect.1.to_px().unwrap() * cx.style.dpi_factor as f32,
                rect.2.to_px().unwrap() * cx.style.dpi_factor as f32,
            ),
        })
        .unwrap_or(bounds);

    let clipping = match (overflowx, overflowy) {
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

    canvas.intersect_scissor(clipping.x, clipping.y, clipping.w, clipping.h);

    let is_visible = match (visible, cx.style.visibility.get(current).copied()) {
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
    // Draw its children
    for child in current.child_iter(&cx.tree) {
        cx.current = child;
        draw_entity(cx, canvas, current_z, queue, is_visible);
    }

    canvas.restore();
    cx.current = current;
}

#[derive(Eq)]
pub struct ZEntity(i32, Entity);
impl Ord for ZEntity {
    fn cmp(&self, other: &Self) -> Ordering {
        other.0.cmp(&self.0)
    }
}
impl PartialOrd for ZEntity {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl PartialEq for ZEntity {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
