use unicode_segmentation::{GraphemeCursor, UnicodeSegmentation};

use std::ops::Range;

pub trait EditableText: Clone {
    fn as_str(&self) -> &str;
    /// Replace range with new text
    fn edit(&mut self, range: Range<usize>, txt: impl Into<String>);
    /// Length of the text
    fn len(&self) -> usize;
    /// Get the previous grapheme offset from the current offset if it exists
    fn prev_grapheme_offset(&self, current: usize) -> Option<usize>;
    /// Get the next grapheme offset from the current offset if it exists
    fn next_grapheme_offset(&self, current: usize) -> Option<usize>;
    /// Get the prev word offset from the current offset if it exists
    fn prev_word_offset(&self, current: usize) -> Option<usize>;
    fn next_word_offset(&self, current: usize) -> Option<usize>;

    // fn prev_codepoint_offset(&self, from: usize) -> Option<usize>;
    // fn next_codepoint_offset(&self, from: usize) -> Option<usize>;
}

impl EditableText for String {
    fn as_str(&self) -> &str {
        self.as_str()
    }

    fn edit(&mut self, range: Range<usize>, txt: impl Into<String>) {
        self.replace_range(range, &txt.into());
    }

    fn len(&self) -> usize {
        self.len()
    }

    fn prev_grapheme_offset(&self, from: usize) -> Option<usize> {
        let mut cursor = GraphemeCursor::new(from, self.len(), true);
        cursor.prev_boundary(self, 0).unwrap()
    }

    fn next_grapheme_offset(&self, from: usize) -> Option<usize> {
        let mut cursor = GraphemeCursor::new(from, self.len(), true);
        cursor.next_boundary(self, 0).unwrap()
    }

    // fn prev_codepoint_offset(&self, from: usize) -> Option<usize> {
    //     let mut c = self.cursor(from).unwrap();
    //     c.prev()
    // }

    // fn next_codepoint_offset(&self, from: usize) -> Option<usize> {
    //     let mut c = self.cursor(from).unwrap();
    //     if c.next().is_some() {
    //         Some(c.pos())
    //     } else {
    //         None
    //     }
    // }

    fn prev_word_offset(&self, from: usize) -> Option<usize> {
        let mut offset = from;
        let mut passed_alphanumeric = false;
        for prev_grapheme in self.get(0..from)?.graphemes(true).rev() {
            let is_alphanumeric = prev_grapheme.chars().next()?.is_alphanumeric();
            if is_alphanumeric {
                passed_alphanumeric = true;
            } else if passed_alphanumeric {
                return Some(offset);
            }
            offset -= prev_grapheme.len();

            if offset == 0 {
                return Some(0);
            }
        }
        None
    }

    fn next_word_offset(&self, from: usize) -> Option<usize> {
        let mut offset = from;
        let mut passed_alphanumeric = false;
        for next_grapheme in self.get(from..)?.graphemes(true) {
            let is_alphanumeric = next_grapheme.chars().next()?.is_alphanumeric();
            if is_alphanumeric {
                passed_alphanumeric = true;
            } else if passed_alphanumeric {
                return Some(offset);
            }
            offset += next_grapheme.len();
        }
        Some(self.len())
    }
}

#[cfg(test)]
mod tests {
    use crate::EditableText;

    #[test]
    fn prev_word_offset() {
        let a = String::from("This is some text");
        assert_eq!(Some(0), a.prev_word_offset(5));
    }
}
