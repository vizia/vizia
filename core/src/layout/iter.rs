use crate::{DoubleEndedTreeTour, Entity, GenerationalId, TourDirection, TourStep, Tree};

/// Iterator for iterating through the tree in depth first preorder.
pub struct LayoutTreeIterator<'a> {
    tree: &'a Tree,
    tours: DoubleEndedTreeTour,
}

impl<'a> LayoutTreeIterator<'a> {
    pub fn full(tree: &'a Tree) -> Self {
        Self::subtree(tree, Entity::root())
    }

    pub fn subtree(tree: &'a Tree, root: Entity) -> Self {
        Self { tree, tours: DoubleEndedTreeTour::new_same(Some(root)) }
    }
}

impl<'a> Iterator for LayoutTreeIterator<'a> {
    type Item = Entity;
    fn next(&mut self) -> Option<Entity> {
        self.tours.next_with(self.tree, |node, direction| match direction {
            TourDirection::Entering => {
                if self.tree.is_ignored(node) {
                    (None, TourStep::EnterFirstChild)
                } else {
                    (Some(node), TourStep::EnterFirstChild)
                }
            }
            TourDirection::Leaving => (None, TourStep::EnterNextSibling),
        })
    }
}

impl<'a> DoubleEndedIterator for LayoutTreeIterator<'a> {
    fn next_back(&mut self) -> Option<Entity> {
        self.tours.next_back_with(self.tree, |node, direction| match direction {
            TourDirection::Entering => (None, TourStep::EnterLastChild),
            TourDirection::Leaving => {
                if self.tree.is_ignored(node) {
                    (None, TourStep::EnterPrevSibling)
                } else {
                    (Some(node), TourStep::EnterPrevSibling)
                }
            }
        })
    }
}

pub struct LayoutChildIterator<'a> {
    tree: &'a Tree,
    tours: DoubleEndedTreeTour,
}

impl<'a> LayoutChildIterator<'a> {
    pub fn new(tree: &'a Tree, node: Entity) -> Self {
        Self {
            tree,
            tours: DoubleEndedTreeTour::new(
                tree.first_child[node.index()],
                tree.get_last_child(node),
            ),
        }
    }
}

impl<'a> Iterator for LayoutChildIterator<'a> {
    type Item = Entity;
    fn next(&mut self) -> Option<Self::Item> {
        self.tours.next_with(self.tree, |node, direction| match direction {
            TourDirection::Entering => {
                if self.tree.is_ignored(node) {
                    (None, TourStep::EnterFirstChild)
                } else {
                    (Some(node), TourStep::LeaveCurrent)
                }
            }
            TourDirection::Leaving => (None, TourStep::EnterNextSibling),
        })
    }
}

impl<'a> DoubleEndedIterator for LayoutChildIterator<'a> {
    fn next_back(&mut self) -> Option<Entity> {
        self.tours.next_back_with(self.tree, |node, direction| match direction {
            TourDirection::Entering => {
                if self.tree.is_ignored(node) {
                    (None, TourStep::EnterLastChild)
                } else {
                    (None, TourStep::LeaveCurrent)
                }
            }
            TourDirection::Leaving => {
                if self.tree.is_ignored(node) {
                    (None, TourStep::EnterPrevSibling)
                } else {
                    (Some(node), TourStep::EnterPrevSibling)
                }
            }
        })
    }
}
