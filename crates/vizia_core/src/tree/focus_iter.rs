use crate::prelude::*;
use crate::tree::*;

/// Should the user be able to navigate to the entity with tab?
pub fn is_navigatable(cx: &Context, node: Entity) -> bool {
    // Skip invisible widgets
    if cx.cache.get_visibility(node) == Visibility::Invisible {
        return false;
    }

    // Skip disabled widgets
    if cx.style.disabled.get(node).cloned().unwrap_or_default() {
        return false;
    }

    // Skip non-displayed widgets
    if cx.cache.get_display(node) == Display::None {
        return false;
    }

    // Skip nodes outside of the subtree
    if !node.is_descendant_of(&cx.tree, cx.lock_focus_to) {
        return false;
    }

    has_ability(cx, node, Abilities::KEYBOARD_NAVIGATABLE)
}

/// Is the entity focusable - some focusable entities are not in the tab order.
pub fn is_focusable(cx: &Context, node: Entity) -> bool {
    has_ability(cx, node, Abilities::FOCUSABLE)
}

fn has_ability(cx: &Context, node: Entity, ability: Abilities) -> bool {
    // Skip ignored widgets
    if cx.tree.is_ignored(node) {
        return false;
    }
    cx.style
        .abilities
        .get(node)
        .and_then(|abilities| Some(abilities.contains(ability)))
        .unwrap_or(false)
}

pub fn focus_forward<'a>(cx: &Context, node: Entity) -> Option<Entity> {
    TreeIterator {
        tree: &cx.tree,
        tours: DoubleEndedTreeTour::new(Some(node), Some(Entity::root())),
    }
    .skip(1)
    .filter(|node| is_navigatable(cx, *node))
    .next()
}

pub fn focus_backward<'a>(cx: &Context, node: Entity) -> Option<Entity> {
    let mut iter = TreeIterator {
        tree: &cx.tree,
        tours: DoubleEndedTreeTour::new_raw(
            TreeTour::new(Some(Entity::root())),
            TreeTour::with_direction(Some(node), TourDirection::Leaving),
        ),
        //tours: DoubleEndedTreeTour::new(Some(Entity::root()), Some(node)),
    };
    iter.next_back();
    iter.filter(|node| is_navigatable(cx, *node)).next_back()
}
