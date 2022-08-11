use vizia_id::GenerationalId;
use crate::prelude::*;
use crate::tree::*;

/// Iterator for iterating the children of an entity.
pub struct ChildIterator<'a> {
    tree: &'a Tree,
    tours: DoubleEndedTreeTour,
}

impl<'a> ChildIterator<'a> {
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

impl<'a> Iterator for ChildIterator<'a> {
    type Item = Entity;
    fn next(&mut self) -> Option<Self::Item> {
        self.tours.next_with(self.tree, |node, direction| match direction {
            TourDirection::Entering => {
                if
                /* self.tree.ignored(node) */
                false {
                    (None, TourStep::EnterFirstChild)
                } else {
                    (Some(node), TourStep::LeaveCurrent)
                }
            }
            TourDirection::Leaving => (None, TourStep::EnterNextSibling),
        })
    }
}

impl<'a> DoubleEndedIterator for ChildIterator<'a> {
    fn next_back(&mut self) -> Option<Entity> {
        self.tours.next_back_with(self.tree, |node, direction| match direction {
            TourDirection::Entering => {
                if
                /* self.tree.ignored(node) */
                false {
                    (None, TourStep::EnterLastChild)
                } else {
                    (None, TourStep::LeaveCurrent)
                }
            }
            TourDirection::Leaving => {
                if
                /* self.tree.ignored(node) */
                false {
                    (None, TourStep::EnterPrevSibling)
                } else {
                    (Some(node), TourStep::EnterPrevSibling)
                }
            }
        })
    }
}
