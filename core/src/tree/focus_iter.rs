use crate::prelude::*;
use crate::style::Style;
use crate::tree::*;

pub fn is_focusable<'a>(style: &'a Style, node: Entity) -> bool {
    style
        .abilities
        .get(node)
        .and_then(|abilities| Some(abilities.contains(Abilities::FOCUSABLE)))
        .unwrap_or(true)
}

pub fn focus_forward<'a>(tree: &'a Tree, style: &'a Style, node: Entity) -> Option<Entity> {
    TreeIterator { tree, tours: DoubleEndedTreeTour::new(Some(node), Some(Entity::root())) }
        .filter(|node| is_focusable(style, *node))
        .nth(1)
}

pub fn focus_backward<'a>(tree: &'a Tree, style: &'a Style, node: Entity) -> Option<Entity> {
    TreeIterator {
        tree,
        tours: DoubleEndedTreeTour::new_raw(
            TreeTour::new(Some(Entity::root())),
            TreeTour::with_direction(Some(node), TourDirection::Leaving),
        ),
        //tours: DoubleEndedTreeTour::new(Some(Entity::root()), Some(node)),
    }
    .filter(|node| is_focusable(style, *node))
    .nth_back(1)
}
