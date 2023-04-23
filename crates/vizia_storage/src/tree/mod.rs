#![allow(clippy::new_without_default)]
#![allow(clippy::module_inception)]

//! # Tree of Widgets
//!
//! The [Tree] struct describes the visual hierarchy of widgets built into the application. A series of iterators
//! are used to traverse the tree, which is used by nearly all systems, including for example, for calculating layout,
//! propagating events, and rendering the UI.

mod iter;
mod tour_direction;
mod tour_step;
mod tree;
mod tree_error;
mod tree_ext;
mod tree_tour;

pub use self::{
    iter::*,
    tour_direction::TourDirection,
    tour_step::TourStep,
    tree::Tree,
    tree_error::TreeError,
    tree_ext::TreeExt,
    tree_tour::{DoubleEndedTreeTour, TreeTour},
};
