use morphorm::Node;

use crate::layout::node::SubLayout;
use crate::prelude::*;
use crate::style::SystemFlags;

/// Determines the size and position of views.
/// TODO: Currently relayout is done on an entire tree rather than incrementally.
/// Incremental relayout can be done by keeping a list of nodes that need relayout,
/// and when a node undergoes relayout remove the descendants that have been processed from the list,
/// then continue relayout on the remaining nodes in the list.
pub(crate) fn layout_system(cx: &mut Context) {
    // text_constraints_system(cx);

    if cx.style.system_flags.contains(SystemFlags::RELAYOUT) {
        Entity::root().layout(
            &mut cx.cache,
            &cx.tree,
            &cx.style,
            &mut SubLayout {
                text_context: &mut cx.text_context,
                resource_manager: &cx.resource_manager,
            },
        );

        // If layout has changed then redraw
        cx.style.system_flags.set(SystemFlags::REDRAW, true);

        for entity in cx.tree.into_iter() {
            if cx.text_context.has_buffer(entity) {
                let auto_width = cx.style.width.get(entity).copied().unwrap_or_default().is_auto();
                let auto_height =
                    cx.style.height.get(entity).copied().unwrap_or_default().is_auto();
                if !auto_width && !auto_height {
                    let width = cx.cache.bounds.get(entity).unwrap().w;
                    let child_left = cx
                        .style
                        .child_left
                        .get(entity)
                        .cloned()
                        .unwrap_or_default()
                        .to_px(width, 0.0)
                        * cx.scale_factor();
                    let child_right = cx
                        .style
                        .child_right
                        .get(entity)
                        .cloned()
                        .unwrap_or_default()
                        .to_px(width, 0.0)
                        * cx.scale_factor();
                    let width = width.ceil() - child_left - child_right;
                    cx.text_context.sync_styles(entity, &cx.style);
                    let (text_width, text_height) =
                        cx.text_context.with_buffer(entity, |fs, buf| {
                            buf.set_size(fs, width, f32::MAX);
                            let w = buf
                                .layout_runs()
                                .filter_map(|r| (!r.line_w.is_nan()).then_some(r.line_w))
                                .max_by(|f1, f2| f1.partial_cmp(f2).unwrap())
                                .unwrap_or_default();
                            let h = buf.layout_runs().len() as f32 * buf.metrics().line_height;
                            (w, h)
                        });
                    cx.text_context.set_bounds(
                        entity,
                        BoundingBox { w: text_width, h: text_height, ..Default::default() },
                    )
                } else {
                    let width = cx.cache.bounds.get(entity).unwrap().w;
                    let child_left = cx
                        .style
                        .child_left
                        .get(entity)
                        .cloned()
                        .unwrap_or_default()
                        .to_px(width, 0.0)
                        * cx.scale_factor();
                    let child_right = cx
                        .style
                        .child_right
                        .get(entity)
                        .cloned()
                        .unwrap_or_default()
                        .to_px(width, 0.0)
                        * cx.scale_factor();
                    let width = width.ceil() - child_left - child_right;
                    cx.text_context.with_buffer(entity, |fs, buffer| {
                        buffer.set_size(fs, width, f32::MAX);
                    })
                }
            }

            // Morphorm produces relative positions so convert to absolute.
            if let Some(parent) = cx.tree.get_layout_parent(entity) {
                let parent_bounds = cx.cache.get_bounds(parent);
                if let Some(bounds) = cx.cache.bounds.get_mut(entity) {
                    bounds.x += parent_bounds.x;
                    bounds.y += parent_bounds.y;
                }
            }
        }

        // Defer resetting the layout system flag to the geometry changed system
    }
}
