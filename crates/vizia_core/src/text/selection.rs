use std::ops::Range;

#[derive(Debug, Clone, Copy)]
pub struct Selection {
    pub anchor: usize,
    pub active: usize,
    pub h_pos: Option<f32>,
}

impl Selection {
    pub fn new(anchor: usize, active: usize) -> Self {
        Selection { anchor, active, h_pos: None }
    }

    /// Construct a new selection from this selection, with the provided h_pos.
    ///
    /// # Note
    ///
    /// `h_pos` is used to track the *pixel* location of the cursor when moving
    /// vertically; lines may have available cursor positions at different
    /// positions, and arrowing down and then back up should always result
    /// in a cursor at the original starting location; doing this correctly
    /// requires tracking this state.
    ///
    /// You *probably* don't need to use this, unless you are implementing a new
    /// text field, or otherwise implementing vertical cursor motion, in which
    /// case you will want to set this during vertical motion if it is not
    /// already set.
    pub fn with_h_pos(mut self, h_pos: Option<f32>) -> Self {
        self.h_pos = h_pos;
        self
    }

    pub fn caret(caret: usize) -> Self {
        Selection { anchor: caret, active: caret, h_pos: None }
    }

    pub fn min(&self) -> usize {
        usize::min(self.anchor, self.active)
    }

    pub fn max(&self) -> usize {
        usize::max(self.anchor, self.active)
    }

    pub fn range(&self) -> Range<usize> {
        self.min()..self.max()
    }

    pub fn is_caret(&self) -> bool {
        self.min() == self.max()
    }
}

pub trait TextSelection: Sized {}

impl TextSelection for Selection {}
