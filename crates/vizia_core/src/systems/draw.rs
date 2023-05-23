use crate::prelude::*;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use vizia_id::GenerationalId;
use vizia_storage::LayoutChildIterator;

pub(crate) fn draw_system(cx: &mut Context) {
    let canvas = cx.canvases.get_mut(&Entity::root()).unwrap();
    cx.resource_manager.mark_images_unused();
    let window_width = cx.cache.get_width(Entity::root());
    let window_height = cx.cache.get_height(Entity::root());
    let clear_color =
        cx.style.background_color.get(Entity::root()).cloned().unwrap_or(RGBA::TRANSPARENT.into());
    canvas.set_size(window_width as u32, window_height as u32, 1.0);
    canvas.clear_rect(0, 0, window_width as u32, window_height as u32, clear_color.into());

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

    // Skip views with display: none.
    if cx.display() == Display::None {
        return;
    }

    // TODO: Looks like I'll need to keep track of the current transform manually instead of within femtovg
    // because elements with a higher z-index aren't getting the transform of their parent.
    let z_index = cx.tree.z_index(current);
    if z_index > current_z {
        queue.push(ZEntity { index: z_index, entity: current, opacity: cx.opacity, visible });
        return;
    }

    canvas.save();

    canvas.set_transform(&cx.transform());

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

    let child_iter = LayoutChildIterator::new(cx.tree, cx.current);

    let parent_opacity = cx.opacity();
    // Draw its children
    for child in child_iter {
        cx.current = child;
        let opactiy = cx.style.opacity.get(child).copied().unwrap_or(Opacity(1.0)).0;
        cx.opacity = parent_opacity * opactiy;
        // TODO: Skip views with zero-sized bounding boxes here? Or let user decide if they want to skip?
        draw_entity(cx, canvas, current_z, queue, is_visible);
    }

    canvas.restore();
    cx.current = current;
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
