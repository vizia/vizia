use crate::{Entity, GenerationalId, Tree};

/// Generic iterator based on [`TreeTour`].
pub struct TreeTourIterator<'a, F> {
    tour: TreeTour,
    tree: &'a Tree,
    cb: F,
}

impl<'a, O, F> TreeTourIterator<'a, F>
where
    F: FnMut(Entity, TourDirection) -> (Option<O>, TourStep),
{
    pub fn new(tree: &'a Tree, start: Option<Entity>, cb: F) -> Self {
        Self { tour: TreeTour::new(start), tree, cb }
    }
}

impl<'a, O, F> Iterator for TreeTourIterator<'a, F>
where
    F: FnMut(Entity, TourDirection) -> (Option<O>, TourStep),
{
    type Item = O;
    fn next(&mut self) -> Option<O> {
        self.tour.next_with(self.tree, &mut self.cb)
    }
}

/// Modular tree traversal helper. Based on the Euler tour technique.
#[derive(Clone, PartialEq, Eq)]
pub struct TreeTour {
    current: Option<Entity>,
    direction: TourDirection,
}

/// Current traversal direction.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum TourDirection {
    Entering,
    Leaving,
}

impl TourDirection {
    pub fn opposite(self) -> Self {
        match self {
            TourDirection::Entering => TourDirection::Leaving,
            TourDirection::Leaving => TourDirection::Entering,
        }
    }
}

/// Where to go next after entering or leaving a node.
#[non_exhaustive]
pub enum TourStep {
    /// Leave this node instead of descending to the children. The next call will be
    /// `cb(current, TourDirection::Leaving)`. Should not be returned if the current direction is
    /// already `TourDirection::Leaving`.
    LeaveCurrent,
    /// Enter the first child. If there are no children, has the same effect as `LeaveCurrent`.
    EnterFirstChild,
    /// Enter the last child. If there are no children, has the same effect as `LeaveCurrent`.
    EnterLastChild,
    /// Leave the parent's subtree without considering further siblings. The next call will be
    /// `cb(parent, TourDirection::Leaving)`, unless there is no parent, in which case iteration
    /// stops.
    LeaveParent,
    /// Enter next sibling. If there is no such sibling, has the same effect as `LeaveParent`.
    EnterNextSibling,
    /// Enter previous sibling. If there is no such sibling, has the same effect as `LeaveParent`.
    EnterPrevSibling,
    /// Stop iteration after this node. The callback will not be called again.
    Break,
}

impl TreeTour {
    pub fn new(start: Option<Entity>) -> Self {
        Self { current: start, direction: TourDirection::Entering }
    }

    /// Traverse tree until next item is yielded, or iteration stops.
    ///
    /// The callback is called each time a node is "entered" or "left", as described by the second
    /// parameter. The first element of the returned tuple indicates what, if anything, should be
    /// yielded at this step. The second element indicates how the traversal should continue.
    pub fn next_with<O, F>(&mut self, tree: &Tree, mut cb: F) -> Option<O>
    where
        F: FnMut(Entity, TourDirection) -> (Option<O>, TourStep),
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
                        self.current = Some(child);
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
pub struct DoubleEndedTreeTour {
    forward: TreeTour,
    backward: TreeTour,
}

impl DoubleEndedTreeTour {
    pub fn new(forward_start: Option<Entity>, backward_start: Option<Entity>) -> Self {
        Self { forward: TreeTour::new(forward_start), backward: TreeTour::new(backward_start) }
    }

    pub fn new_same(start_both: Option<Entity>) -> Self {
        Self { forward: TreeTour::new(start_both), backward: TreeTour::new(start_both) }
    }

    pub fn next_with<O, F>(&mut self, tree: &Tree, mut cb: F) -> Option<O>
    where
        F: FnMut(Entity, TourDirection) -> (Option<O>, TourStep),
    {
        self.forward.next_with(tree, |current, direction| {
            let (item, action) = cb(current, direction);
            if self.backward.current == Some(current)
                && self.backward.direction == direction.opposite()
            {
                self.backward.current = None;
                (item, TourStep::Break)
            } else {
                (item, action)
            }
        })
    }

    pub fn next_back_with<O, F>(&mut self, tree: &Tree, mut cb: F) -> Option<O>
    where
        F: FnMut(Entity, TourDirection) -> (Option<O>, TourStep),
    {
        self.backward.next_with(tree, |current, direction| {
            let (item, action) = cb(current, direction);
            if self.forward.current == Some(current)
                && self.forward.direction == direction.opposite()
            {
                self.forward.current = None;
                (item, TourStep::Break)
            } else {
                (item, action)
            }
        })
    }
}
