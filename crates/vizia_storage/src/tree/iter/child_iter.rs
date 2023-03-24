use crate::{DoubleEndedTreeTour, TourDirection, TourStep, Tree};
use vizia_id::GenerationalId;

pub struct MorphormChildIter<'a, I>
where
    I: GenerationalId,
{
    tree: &'a Tree<I>,
    current: Option<&'a I>,
}

impl<'a, I> MorphormChildIter<'a, I>
where
    I: GenerationalId,
{
    pub fn new(tree: &'a Tree<I>, node: I) -> Self {
        Self {
            tree,
            current: tree.first_child.get(node.index()).map(|child| child.as_ref()).flatten(),
        }
    }
}

impl<'a, I> Iterator for MorphormChildIter<'a, I>
where
    I: GenerationalId,
{
    type Item = &'a I;

    fn next(&mut self) -> Option<Self::Item> {
        let ret = self.current;
        if let Some(current) = self.current {
            self.current = self
                .tree
                .next_sibling
                .get(current.index())
                .map(|sibling| sibling.as_ref())
                .flatten()
        }
        if let Some(current) = ret {
            if self.tree.is_ignored(*current) {
                while let Some(firt_child) = self
                    .tree
                    .first_child
                    .get(current.index())
                    .map(|first_child| first_child.as_ref())
                    .flatten()
                {
                    if !self.tree.is_ignored(*firt_child) {
                        return Some(firt_child);
                    }
                }

                return self.next();
            }
        }
        ret
    }
}

/// Iterator for iterating the children of an entity.
pub struct ChildIterator<'a, I>
where
    I: GenerationalId,
{
    tree: &'a Tree<I>,
    tours: DoubleEndedTreeTour<I>,
}

impl<'a, I> ChildIterator<'a, I>
where
    I: GenerationalId,
{
    pub fn new(tree: &'a Tree<I>, node: I) -> Self {
        Self {
            tree,
            tours: DoubleEndedTreeTour::new(
                tree.first_child[node.index()],
                tree.get_last_child(node),
            ),
        }
    }
}

impl<'a, I> Iterator for ChildIterator<'a, I>
where
    I: GenerationalId,
{
    type Item = I;

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

impl<'a, I> DoubleEndedIterator for ChildIterator<'a, I>
where
    I: GenerationalId,
{
    fn next_back(&mut self) -> Option<Self::Item> {
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
