use crate::{accessibility::IntoNode, context::AccessContext, prelude::*};
use accesskit::{
    NodeBuilder, NodeId, Rect, TextDirection, TextPosition, TextSelection, TreeUpdate,
};
use cosmic_text::Edit;
use unicode_segmentation::UnicodeSegmentation;
use vizia_storage::LayoutTreeIterator;

// Updates node properties from view properties
// Should be run after layout so that things like bounding box are correct
// This system doesn't change the structure of the accessibility tree as this is done when views are built/removed
// TODO: Change this to incrementally update nodes when required instead of updating all nodes every frame
pub fn accessibility_system(cx: &mut Context) {
    let iterator = LayoutTreeIterator::full(&cx.tree);

    for entity in iterator {
        let mut node_builder = cx.style.accesskit_node_builders.get(entity).cloned().unwrap();

        let navigable = cx
            .style
            .abilities
            .get(entity)
            .copied()
            .unwrap_or_default()
            .contains(Abilities::NAVIGABLE);

        if node_builder.role() == Role::Unknown && !navigable {
            continue;
        }

        let bounds = cx.cache.get_bounds(entity);

        node_builder.set_bounds(Rect {
            x0: bounds.x as f64,
            y0: bounds.y as f64,
            x1: (bounds.x + bounds.w) as f64,
            y1: (bounds.y + bounds.h) as f64,
        });

        if let Some(disabled) = cx.style.disabled.get(entity).copied() {
            if disabled {
                node_builder.set_disabled();
            } else {
                node_builder.clear_disabled();
            }
        }

        let focusable = cx
            .style
            .abilities
            .get(entity)
            .map(|flags| flags.contains(Abilities::NAVIGABLE))
            .unwrap_or(false);

        if focusable {
            node_builder.add_action(Action::Focus);
        } else {
            node_builder.remove_action(Action::Focus);
        }

        if let Some(view) = cx.views.remove(&entity) {
            let mut access_context = AccessContext {
                current: entity,
                style: &mut cx.style,
                cache: &cx.cache,
                text_context: &mut cx.text_context,
            };

            let mut children = Vec::new();

            view.accessibility(&mut access_context, &mut node_builder, &mut children);

            let child_ids = children.iter().map(|(id, _)| *id).collect::<Vec<_>>();
            if !child_ids.is_empty() {
                node_builder.set_children(child_ids);
            }

            let mut nodes = vec![(
                access_context.node_id(),
                node_builder.build(&mut cx.style.accesskit_node_classes),
            )];

            // If child nodes were generated then append them to the nodes list
            if !children.is_empty() {
                nodes.extend(children.into_iter().map(|(id, builder)| {
                    (id, builder.build(&mut cx.style.accesskit_node_classes))
                }));
            }

            cx.tree_updates.push(TreeUpdate {
                nodes,
                tree: None,
                focus: cx.window_has_focus.then_some(cx.focused.accesskit_id()),
            });

            cx.views.insert(entity, view);
        }
    }
}
