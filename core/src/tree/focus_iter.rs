use crate::prelude::*;
use crate::tree::*;

pub fn is_navigatable(cx: &Context, node: Entity) -> bool {
    // Skip invisible widgets
    if cx.cache_ref().get_visibility(node) == Visibility::Invisible {
        return false;
    }

    // Skip disabled widgets
    if cx.style_ref().disabled.get(node).cloned().unwrap_or_default() {
        return false;
    }

    // Skip non-displayed widgets
    if cx.cache_ref().get_display(node) == Display::None {
        return false;
    }

    // Skip ignored widgets
    if cx.tree_ref().is_ignored(node) {
        return false;
    }
    cx.style_ref()
        .abilities
        .get(node)
        .and_then(|abilities| Some(abilities.contains(Abilities::KEYBOARD_NAVIGATABLE)))
        .unwrap_or(false)
}

pub fn focus_forward<'a>(cx: &Context, node: Entity) -> Option<Entity> {
    TreeIterator {
        tree: cx.tree_ref(),
        tours: DoubleEndedTreeTour::new(Some(node), Some(Entity::root())),
    }
    .skip(1)
    .filter(|node| is_navigatable(cx, *node))
    .next()
}

pub fn focus_backward<'a>(cx: &Context, node: Entity) -> Option<Entity> {
    let mut iter = TreeIterator {
        tree: cx.tree_ref(),
        tours: DoubleEndedTreeTour::new_raw(
            TreeTour::new(Some(Entity::root())),
            TreeTour::with_direction(Some(node), TourDirection::Leaving),
        ),
        //tours: DoubleEndedTreeTour::new(Some(Entity::root()), Some(node)),
    };
    iter.next_back();
    iter.filter(|node| is_navigatable(cx, *node)).next_back()
}
