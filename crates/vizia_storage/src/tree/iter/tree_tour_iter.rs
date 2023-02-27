use crate::{TourDirection, TourStep, Tree, TreeTour};
use vizia_id::GenerationalId;

/// Generic iterator based on [`TreeTour`].
pub struct TreeTourIterator<'a, I, F>
where
    I: GenerationalId,
{
    tour: TreeTour<I>,
    tree: &'a Tree<I>,
    cb: F,
}

impl<'a, I, F, O> TreeTourIterator<'a, I, F>
where
    I: GenerationalId,
    F: FnMut(I, TourDirection) -> (Option<O>, TourStep),
{
    pub fn new(tree: &'a Tree<I>, start: Option<I>, cb: F) -> Self {
        Self { tour: TreeTour::new(start), tree, cb }
    }
}

impl<'a, I, F, O> Iterator for TreeTourIterator<'a, I, F>
where
    I: GenerationalId,
    F: FnMut(I, TourDirection) -> (Option<O>, TourStep),
{
    type Item = O;

    fn next(&mut self) -> Option<Self::Item> {
        self.tour.next_with(self.tree, &mut self.cb)
    }
}
