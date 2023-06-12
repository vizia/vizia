use crate::{DoubleEndedTreeTour, TourDirection, TourStep, Tree};
use vizia_id::GenerationalId;

// A TreeTour that returns a ref instead of a value
// TODO: Probably replace the normal TreeTour with this?
#[derive(Clone, PartialEq, Eq)]
pub struct TreeTourRef<'a, I>
where
    I: GenerationalId,
{
    current: Option<&'a I>,
    start: Option<&'a I>,
    direction: TourDirection,
}

impl<'a, I> TreeTourRef<'a, I>
where
    I: GenerationalId + std::fmt::Debug,
{
    pub fn new(start: Option<&'a I>) -> Self {
        Self { current: start, start, direction: TourDirection::Entering }
    }

    /// Traverse tree until next item is yielded, or iteration stops.
    ///
    /// The callback is called each time a node is "entered" or "left", as described by the second
    /// parameter. The first element of the returned tuple indicates what, if anything, should be
    /// yielded at this step. The second element indicates how the traversal should continue.
    pub fn next_with<O: 'a + std::fmt::Debug, F>(
        &mut self,
        tree: &'a Tree<I>,
        mut cb: F,
    ) -> Option<&'a O>
    where
        F: FnMut(&'a I, TourDirection) -> (Option<&'a O>, TourStep),
    {
        while let Some(current) = self.current {
            let (item, action) = cb(current, self.direction);
            match action {
                TourStep::LeaveCurrent => {
                    assert!(
                        self.direction == TourDirection::Entering,
                        "tried to leave current node again in tree traversal"
                    );
                    self.direction = TourDirection::Leaving;
                }
                TourStep::EnterFirstChild => {
                    if let Some(child) = tree.first_child[current.index()].as_ref() {
                        self.direction = TourDirection::Entering;
                        self.current = Some(child);
                    } else {
                        self.direction = TourDirection::Leaving;
                    }
                }
                TourStep::EnterLastChild => {
                    if let Some(child) = tree.get_last_child(*current) {
                        self.direction = TourDirection::Entering;
                        self.current = Some(child);
                    } else {
                        self.direction = TourDirection::Leaving;
                    }
                }
                TourStep::LeaveParent => {
                    self.direction = TourDirection::Leaving;
                    self.current = tree.parent[current.index()].as_ref();
                }
                TourStep::EnterNextSibling => {
                    if let Some(sibling) = tree.next_sibling[current.index()].as_ref() {
                        self.direction = TourDirection::Entering;
                        self.current = Some(sibling);
                    } else {
                        self.direction = TourDirection::Leaving;
                        self.current = tree.parent[current.index()].as_ref();
                        if let Some(start) = self.start {
                            if self.current == tree.parent[start.index()].as_ref() {
                                self.current = None;
                            }
                        }
                    }
                }
                TourStep::EnterPrevSibling => {
                    if let Some(sibling) = tree.prev_sibling[current.index()].as_ref() {
                        self.direction = TourDirection::Entering;
                        self.current = Some(sibling);
                    } else {
                        self.direction = TourDirection::Leaving;
                        self.current = tree.parent[current.index()].as_ref();
                    }
                }
                TourStep::Break => {
                    self.current = None;
                }
            }
            if item.is_some() {
                return item;
            }
        }
        None
    }
}

pub struct MorphormChildIter<'a, I>
where
    I: GenerationalId,
{
    tree: &'a Tree<I>,
    tours: TreeTourRef<'a, I>,
}

impl<'a, I> MorphormChildIter<'a, I>
where
    I: GenerationalId + std::fmt::Debug,
{
    pub fn new(tree: &'a Tree<I>, node: I) -> Self {
        Self { tree, tours: TreeTourRef::new(tree.first_child[node.index()].as_ref()) }
    }
}

impl<'a, I> Iterator for MorphormChildIter<'a, I>
where
    I: GenerationalId + std::fmt::Debug,
{
    type Item = &'a I;

    fn next(&mut self) -> Option<Self::Item> {
        self.tours.next_with(self.tree, |node, direction| match direction {
            TourDirection::Entering => {
                if self.tree.is_ignored(*node) {
                    (None, TourStep::EnterFirstChild)
                } else {
                    (Some(node), TourStep::LeaveCurrent)
                }
            }
            TourDirection::Leaving => (None, TourStep::EnterNextSibling),
        })
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
                tree.get_last_child(node).copied(),
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
