
use unicode_segmentation::{GraphemeCursor, UnicodeSegmentation};


pub trait EditableText {
    fn prev_grapheme_offset(&self, from: usize) -> Option<usize>;
    fn next_grapheme_offset(&self, from: usize) -> Option<usize>;
}

impl EditableText for String {

    fn prev_grapheme_offset(&self, from: usize) -> Option<usize> {
        let mut cursor = GraphemeCursor::new(from, self.len(), true);
        cursor.prev_boundary(self, 0).unwrap()
    }

    fn next_grapheme_offset(&self, from: usize) -> Option<usize> {
        let mut cursor = GraphemeCursor::new(from, self.len(), true);
        cursor.next_boundary(self, 0).unwrap()
    }
}