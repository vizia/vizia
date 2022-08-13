use crate::{DoubleEndedTreeTour, TourDirection, TourStep, Tree};
use vizia_id::GenerationalId;

pub struct TreeDepthIterator<'a, I>
where
    I: GenerationalId,
{
    tree: &'a Tree<I>,
    tours: DoubleEndedTreeTour<I>,
    depth_forward: usize,
    // depth_backward: usize,
}

impl<'a, I> TreeDepthIterator<'a, I>
where
    I: GenerationalId,
{
    pub fn full(tree: &'a Tree<I>) -> Self {
        Self::subtree(tree, I::root())
    }

    pub fn subtree(tree: &'a Tree<I>, root: I) -> Self {
        Self { tree, depth_forward: 0, tours: DoubleEndedTreeTour::new_same(Some(root)) }
    }
}

impl<'a, I> Iterator for TreeDepthIterator<'a, I>
where
    I: GenerationalId,
{
    type Item = (I, usize);

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
