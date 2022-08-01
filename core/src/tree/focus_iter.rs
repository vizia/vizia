use crate::prelude::*;
use crate::style::Style;
use crate::tree::*;

pub fn is_navigatable<'a>(style: &'a Style, node: Entity) -> bool {
    style
        .abilities
        .get(node)
        .and_then(|abilities| Some(abilities.contains(Abilities::KEYBOARD_NAVIGATABLE)))
        .unwrap_or(false)
}

pub fn focus_forward<'a>(tree: &'a Tree, style: &'a Style, node: Entity) -> Option<Entity> {
    TreeIterator { tree, tours: DoubleEndedTreeTour::new(Some(node), Some(Entity::root())) }
        .skip(1)
        .filter(|node| is_navigatable(style, *node))
        .next()
}

pub fn focus_backward<'a>(tree: &'a Tree, style: &'a Style, node: Entity) -> Option<Entity> {
    let mut iter = TreeIterator {
        tree,
        tours: DoubleEndedTreeTour::new_raw(
            TreeTour::new(Some(Entity::root())),
            TreeTour::with_direction(Some(node), TourDirection::Leaving),
        ),
        //tours: DoubleEndedTreeTour::new(Some(Entity::root()), Some(node)),
    };
    iter.next_back();
    iter.filter(|node| is_navigatable(style, *node)).next_back()
}
