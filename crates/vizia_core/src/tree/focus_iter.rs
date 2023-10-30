use crate::entity::Entity;
use crate::prelude::Style;
use crate::style::{Abilities, Display};
use vizia_id::GenerationalId;
use vizia_storage::{
    DoubleEndedTreeTour, FocusTreeIterator, TourDirection, Tree, TreeExt, TreeTour,
};

/// Should the user be able to navigate to the entity with tab?
pub(crate) fn is_navigatable(
    tree: &Tree<Entity>,
    style: &Style,
    node: Entity,
    lock_focus_to: Entity,
) -> bool {
    // Skip invisible widgets
    // if cx.cache.get_visibility(node) == Visibility::Hidden {
    //     return false;
    // }

    // Skip disabled widgets
    if style.disabled.get(node).cloned().unwrap_or_default() {
        return false;
    }

    // Skip non-displayed widgets
    if style.display.get(node).copied().unwrap_or_default() == Display::None {
        return false;
    }

    // Skip nodes outside of the subtree
    if !node.is_descendant_of(tree, lock_focus_to) {
        return false;
    }

    // Skip ignored widgets
    if tree.is_ignored(node) {
        return false;
    }

    style
        .abilities
        .get(node)
        .map(|abilities| abilities.contains(Abilities::NAVIGABLE))
        .unwrap_or(false)
}

/// Get the next entity to be focused during forward keyboard navigation.
pub(crate) fn focus_forward(
    tree: &Tree<Entity>,
    style: &Style,
    node: Entity,
    lock_focus_to: Entity,
) -> Option<Entity> {
    FocusTreeIterator::new(
        tree,
        DoubleEndedTreeTour::new(Some(node), Some(Entity::root())),
        |node| {
            style.display.get(node).copied().unwrap_or_default() == Display::None
            // false
        },
    )
    .skip(1)
    .find(|node| is_navigatable(tree, style, *node, lock_focus_to))
}

/// Get the next entity to be focused during backward keybaord navigation.
pub(crate) fn focus_backward(
    tree: &Tree<Entity>,
    style: &Style,
    node: Entity,
    lock_focus_to: Entity,
) -> Option<Entity> {
    let mut iter = FocusTreeIterator::new(
        tree,
        DoubleEndedTreeTour::new_raw(
            TreeTour::new(Some(Entity::root())),
            TreeTour::with_direction(Some(node), TourDirection::Leaving),
        ),
        |node| {
            // Check if any ancestors are not displayed.
            // TODO: Think of a better way to do thus.
            for ancestor in node.parent_iter(tree) {
                if style.display.get(ancestor).copied().unwrap_or_default() == Display::None {
                    return true;
                }
            }

            false
        },
    );
    iter.next_back();
    iter.filter(|node| is_navigatable(tree, style, *node, lock_focus_to)).next_back()
}
