use std::sync::Arc;

use crate::{accessibility::IntoNode, prelude::*};
use accesskit::TreeUpdate;
use vizia_storage::LayoutTreeIterator;

// Updates node properties from view properties
// Should be run after layout so that things like bounding box are correct
// This system doesn't change the structure of the accessibility tree as this is done when views are built/removed
// TODO: Change this to incrementally update nodes when required instead of updating all nodes every frame
pub fn accessibility_system(cx: &mut Context, tree: &Tree<Entity>) {
    let iterator = LayoutTreeIterator::full(tree);

    for entity in iterator {
        let node_id = entity.accesskit_id();
        let node = cx.get_node(entity);

        cx.tree_updates.push(TreeUpdate {
            nodes: vec![(node_id, Arc::new(node))],
            tree: None,
            focus: cx.window_has_focus.then_some(cx.focused.accesskit_id()),
        });
    }
}
