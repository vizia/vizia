use std::ops::Range;

use parley::{Affinity, editing::Cursor as ParleyCursor, editing::Selection as ParleySelection};

use crate::text::ShapedText;

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

    pub fn to_parley(&self, shaped: &ShapedText) -> ParleySelection {
        let layout = &shaped.pre_shaped.parley_layout;
        let text_len = shaped.pre_shaped.text.len();
        let anchor =
            ParleyCursor::from_byte_index(layout, self.anchor.min(text_len), Affinity::Downstream);
        let focus =
            ParleyCursor::from_byte_index(layout, self.active.min(text_len), Affinity::Downstream);

        ParleySelection::new(anchor, focus)
    }

    pub fn from_parley(selection: ParleySelection, text_len: usize, h_pos: Option<f32>) -> Self {
        Selection::new(
            selection.anchor().index().min(text_len),
            selection.focus().index().min(text_len),
        )
        .with_h_pos(h_pos)
    }
}
