use crate::{TourDirection, TourStep, Tree};
use vizia_id::GenerationalId;

/// Modular tree traversal helper. Based on the Euler tour technique.
#[derive(Clone, PartialEq, Eq)]
pub struct TreeTour<I>
where
    I: GenerationalId,
{
    current: Option<I>,
    direction: TourDirection,
}

impl<I> TreeTour<I>
where
    I: GenerationalId,
{
    pub fn new(start: Option<I>) -> Self {
        Self { current: start, direction: TourDirection::Entering }
    }

    pub fn with_direction(start: Option<I>, direction: TourDirection) -> Self {
        Self { current: start, direction }
    }

    /// Traverse tree until next item is yielded, or iteration stops.
    ///
    /// The callback is called each time a node is "entered" or "left", as described by the second
    /// parameter. The first element of the returned tuple indicates what, if anything, should be
    /// yielded at this step. The second element indicates how the traversal should continue.
    pub fn next_with<O, F>(&mut self, tree: &Tree<I>, mut cb: F) -> Option<O>
    where
        F: FnMut(I, TourDirection) -> (Option<O>, TourStep),
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
                    if let Some(child) = tree.first_child[current.index()] {
                        self.direction = TourDirection::Entering;
                        self.current = Some(child);
                    } else {
                        self.direction = TourDirection::Leaving;
                    }
                }
                TourStep::EnterLastChild => {
                    if let Some(child) = tree.get_last_child(current) {
                        self.direction = TourDirection::Entering;
                        self.current = Some(*child);
                    } else {
                        self.direction = TourDirection::Leaving;
                    }
                }
                TourStep::LeaveParent => {
                    self.direction = TourDirection::Leaving;
                    self.current = tree.parent[current.index()];
                }
                TourStep::EnterNextSibling => {
                    if let Some(sibling) = tree.next_sibling[current.index()] {
                        self.direction = TourDirection::Entering;
                        self.current = Some(sibling);
                    } else {
                        self.direction = TourDirection::Leaving;
                        self.current = tree.parent[current.index()];
                    }
                }
                TourStep::EnterPrevSibling => {
                    if let Some(sibling) = tree.prev_sibling[current.index()] {
                        self.direction = TourDirection::Entering;
                        self.current = Some(sibling);
                    } else {
                        self.direction = TourDirection::Leaving;
                        self.current = tree.parent[current.index()];
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

/// Double ended version of [`TreeTour`].
///
/// # Correctness
///
/// In order for iteration to stop when the ends meet, the forward and backward tours should
/// traverse the same nodes, but in reverse order. This can be accomplished in any way you like.
/// Which of the nodes are yielded does not matter, but if the forward direction yield something on
/// entering, the backward direction should yield that item when leaving, and vice versa.
pub struct DoubleEndedTreeTour<I>
where
    I: GenerationalId,
{
    forward: TreeTour<I>,
    backward: TreeTour<I>,
}

impl<I> DoubleEndedTreeTour<I>
where
    I: GenerationalId,
{
    pub fn new(forward_start: Option<I>, backward_start: Option<I>) -> Self {
        Self { forward: TreeTour::new(forward_start), backward: TreeTour::new(backward_start) }
    }

    pub fn new_same(start_both: Option<I>) -> Self {
        Self { forward: TreeTour::new(start_both), backward: TreeTour::new(start_both) }
    }

    pub fn new_raw(forward: TreeTour<I>, backward: TreeTour<I>) -> Self {
        Self { forward, backward }
    }

    pub fn next_with<O, F>(&mut self, tree: &Tree<I>, mut cb: F) -> Option<O>
    where
        F: FnMut(I, TourDirection) -> (Option<O>, TourStep),
    {
        self.forward.next_with(tree, |current, direction| {
            let (item, action) = cb(current, direction);
            if self.backward.current == Some(current) && self.backward.direction != direction {
                self.backward.current = None;
                (item, TourStep::Break)
            } else {
                (item, action)
            }
        })
    }

    pub fn next_back_with<O, F>(&mut self, tree: &Tree<I>, mut cb: F) -> Option<O>
    where
        F: FnMut(I, TourDirection) -> (Option<O>, TourStep),
    {
        self.backward.next_with(tree, |current, direction| {
            let (item, action) = cb(current, direction);
            if self.forward.current == Some(current) && self.forward.direction != direction {
                self.forward.current = None;
                (item, TourStep::Break)
            } else {
                (item, action)
            }
        })
    }
}
