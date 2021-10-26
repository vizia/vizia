//! # Tree of Widgets
//! 
//! The [Tree] struct describes the visual hierarchy of widgets built into the application. A series of iterators
//! are used to traverse the tree, which is used by nearly all systems, including for example, for calculating layout, 
//! propagating events, and rendering the UI. 

mod tree;
pub use tree::*;

mod parent_iter;
pub use parent_iter::ParentIterator;

mod child_iter;
pub use child_iter::ChildIterator;

mod branch_iter;
pub use branch_iter::BranchIterator;

mod tree_iter;
pub use tree_iter::TreeIterator;

mod tree_ext;
pub use tree_ext::*;