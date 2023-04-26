#![allow(clippy::module_inception)]

mod entry;
mod index;
mod sparse_set;

pub use self::{
    entry::Entry,
    index::SparseSetIndex,
    sparse_set::{SparseSet, SparseSetGeneric},
};
