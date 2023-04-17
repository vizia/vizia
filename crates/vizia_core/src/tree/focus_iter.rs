use crate::context::Context;
use crate::entity::Entity;
use crate::style::{Abilities, Display};
use vizia_id::GenerationalId;
use vizia_storage::{DoubleEndedTreeTour, TourDirection, TreeExt, TreeIterator, TreeTour};

/// Should the user be able to navigate to the entity with tab?
pub(crate) fn is_navigatable(cx: &Context, node: Entity, lock_focus_to: Entity) -> bool {
    // Skip invisible widgets
    // if cx.cache.get_visibility(node) == Visibility::Hidden {
    //     return false;
    // }

    // Skip disabled widgets
    if cx.style.disabled.get(node).cloned().unwrap_or_default() {
        return false;
    }

    // Skip non-displayed widgets
    if cx.style.display.get(node).copied().unwrap_or_default() == Display::None {
        return false;
    }

    // Skip nodes outside of the subtree
    if !node.is_descendant_of(&cx.tree, lock_focus_to) {
        return false;
    }

    has_ability(cx, node, Abilities::NAVIGABLE)
}

fn has_ability(cx: &Context, node: Entity, ability: Abilities) -> bool {
    // Skip ignored widgets
    if cx.tree.is_ignored(node) {
        return false;
    }
    cx.style.abilities.get(node).map(|abilities| abilities.contains(ability)).unwrap_or(false)
}

pub fn focus_forward<'a>(cx: &Context, node: Entity, lock_focus_to: Entity) -> Option<Entity> {
    TreeIterator::new(&cx.tree, DoubleEndedTreeTour::new(Some(node), Some(Entity::root())))
        .skip(1)
        .find(|node| is_navigatable(cx, *node, lock_focus_to))
}

pub fn focus_backward<'a>(cx: &Context, node: Entity, lock_focus_to: Entity) -> Option<Entity> {
    let mut iter = TreeIterator::new(
        &cx.tree,
        DoubleEndedTreeTour::new_raw(
            TreeTour::new(Some(Entity::root())),
            TreeTour::with_direction(Some(node), TourDirection::Leaving),
        ),
        //tours: DoubleEndedTreeTour::new(Some(Entity::root()), Some(node)),
    );
    iter.next_back();
    iter.filter(|node| is_navigatable(cx, *node, lock_focus_to)).next_back()
}
