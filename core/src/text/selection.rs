use std::ops::Range;

use crate::Data;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Selection {
    pub anchor: usize,
    pub active: usize,
}

impl Selection {
    pub fn new(anchor: usize, active: usize) -> Self {
        Self { anchor, active }
    }

    pub fn caret(index: usize) -> Self {
        Self { anchor: index, active: index }
    }

    pub fn min(&self) -> usize {
        self.anchor.min(self.active)
    }

    pub fn max(&self) -> usize {
        self.anchor.max(self.active)
    }

    pub fn range(&self) -> Range<usize> {
        self.min()..self.max()
    }

    pub fn len(&self) -> usize {
        self.max() - self.min()
    }

    pub fn is_caret(&self) -> bool {
        self.active == self.anchor
    }
}

impl Data for Selection {
    fn same(&self, other: &Self) -> bool {
        *self == *other
    }
}
