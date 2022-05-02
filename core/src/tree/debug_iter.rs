use crate::prelude::*;
use crate::tree::*;

pub struct TreeDepthIterator<'a> {
    tree: &'a Tree,
    tours: DoubleEndedTreeTour,
    depth_forward: usize,
    // depth_backward: usize,
}

impl<'a> TreeDepthIterator<'a> {
    pub fn full(tree: &'a Tree) -> Self {
        Self::subtree(tree, Entity::root())
    }

    pub fn subtree(tree: &'a Tree, root: Entity) -> Self {
        Self { tree, depth_forward: 0, tours: DoubleEndedTreeTour::new_same(Some(root)) }
    }
}

impl<'a> Iterator for TreeDepthIterator<'a> {
    type Item = (Entity, usize);
    fn next(&mut self) -> Option<Self::Item> {
        self.tours.next_with(self.tree, |node, direction| match direction {
            TourDirection::Entering => {
                self.depth_forward += 1;
                // if yielding:
                (Some((node, self.depth_forward - 1)), TourStep::EnterFirstChild)
            }
            TourDirection::Leaving => {
                self.depth_forward -= 1;
                // if yielding:
                (None, TourStep::EnterNextSibling)
            }
        })
    }
}
