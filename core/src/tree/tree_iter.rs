use crate::{DoubleEndedTreeTour, Entity, TourDirection, TourStep, Tree};

/// Iterator for iterating through the tree in depth first preorder.
pub struct TreeIterator<'a> {
    tree: &'a Tree,
    tours: DoubleEndedTreeTour,
}

impl<'a> TreeIterator<'a> {
    pub fn full(tree: &'a Tree) -> Self {
        Self::subtree(tree, Entity::root())
    }

    pub fn subtree(tree: &'a Tree, root: Entity) -> Self {
        Self { tree, tours: DoubleEndedTreeTour::new_same(Some(root)) }
    }
}

impl<'a> Iterator for TreeIterator<'a> {
    type Item = Entity;
    fn next(&mut self) -> Option<Entity> {
        self.tours.next_with(self.tree, |node, direction| match direction {
            TourDirection::Entering => (Some(node), TourStep::EnterFirstChild),
            TourDirection::Leaving => (None, TourStep::EnterNextSibling),
        })
    }
}

impl<'a> DoubleEndedIterator for TreeIterator<'a> {
    fn next_back(&mut self) -> Option<Entity> {
        self.tours.next_back_with(self.tree, |node, direction| match direction {
            TourDirection::Entering => (None, TourStep::EnterLastChild),
            TourDirection::Leaving => (Some(node), TourStep::EnterPrevSibling),
        })
    }
}
