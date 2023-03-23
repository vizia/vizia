use crate::context::Context;
use crate::prelude::*;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use vizia_id::GenerationalId;
use vizia_storage::LayoutChildIterator;

pub fn draw_system(cx: &mut Context) {
    let canvas = cx.canvases.get_mut(&Entity::root()).unwrap();
    cx.resource_manager.mark_images_unused();
    let window_width = cx.cache.get_width(Entity::root());
    let window_height = cx.cache.get_height(Entity::root());
    let clear_color =
        cx.style.background_color.get(Entity::root()).cloned().unwrap_or(RGBA::WHITE.into());
    canvas.set_size(window_width as u32, window_height as u32, 1.0);
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

    if cx.display() == Display::None {
        return;
    }

    let z_order = cx.tree.z_order(current);
    if z_order > current_z {
        queue.push(ZEntity(z_order, current));
        return;
    }

    canvas.save();

    if let Some(transform) = cx.transform() {
        canvas.set_transform(&transform);
    }

    let clip_region = cx.clip_region();

    canvas.intersect_scissor(clip_region.x, clip_region.y, clip_region.w, clip_region.h);

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

    let child_iter = LayoutChildIterator::new(&cx.tree, cx.current);

    // Draw its children
    for child in child_iter {
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
