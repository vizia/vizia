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
