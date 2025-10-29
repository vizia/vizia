#![allow(dead_code)]

use std::{borrow::Cow, ops::Range};

use unicode_segmentation::{GraphemeCursor, UnicodeSegmentation};

pub trait EditableText: Sized {
    /// Replace range with new text.
    /// Can panic if supplied an invalid range.
    fn edit(&mut self, range: Range<usize>, new: impl Into<Self>);

    /// Get slice of text at range.
    fn slice(&self, range: Range<usize>) -> Option<Cow<str>>;

    /// Get length of text (in bytes).
    fn len(&self) -> usize;

    /// Get the previous word offset from the given offset, if it exists.
    fn prev_word_offset(&self, offset: usize) -> Option<usize>;

    /// Get the next word offset from the given offset, if it exists.
    fn next_word_offset(&self, offset: usize) -> Option<usize>;

    /// Get the next grapheme offset from the given offset, if it exists.
    fn prev_grapheme_offset(&self, offset: usize) -> Option<usize>;

    /// Get the next grapheme offset from the given offset, if it exists.
    fn next_grapheme_offset(&self, offset: usize) -> Option<usize>;

    fn current_grapheme_offset(&self, offset: usize) -> usize;

    /// Get the previous codepoint offset from the given offset, if it exists.
    fn prev_codepoint_offset(&self, offset: usize) -> Option<usize>;

    /// Get the next codepoint offset from the given offset, if it exists.
    fn next_codepoint_offset(&self, offset: usize) -> Option<usize>;

    fn prev_codepoint(&self, offset: usize) -> Option<char>;

    /// Get the preceding line break offset from the given offset
    fn preceding_line_break(&self, offset: usize) -> usize;

    /// Get the next line break offset from the given offset
    fn next_line_break(&self, offset: usize) -> usize;

    /// Returns `true` if this text has 0 length.
    fn is_empty(&self) -> bool;

    /// Construct an instance of this type from a `&str`.
    fn from_str(s: &str) -> Self;
}

impl EditableText for String {
    fn edit(&mut self, range: Range<usize>, new: impl Into<Self>) {
        self.replace_range(range, &new.into());
    }

    fn slice(&self, range: Range<usize>) -> Option<Cow<str>> {
        self.get(range).map(Cow::from)
    }

    fn len(&self) -> usize {
        self.len()
    }

    fn prev_grapheme_offset(&self, from: usize) -> Option<usize> {
        let mut c = GraphemeCursor::new(from, self.len(), true);
        c.prev_boundary(self, 0).unwrap()
    }

    fn next_grapheme_offset(&self, from: usize) -> Option<usize> {
        let mut c = GraphemeCursor::new(from, self.len(), true);
        c.next_boundary(self, 0).unwrap()
    }

    fn current_grapheme_offset(&self, from: usize) -> usize {
        if from == self.len() {
            self.graphemes(true).count()
        } else {
            let mut current = self.graphemes(true).count();

            let mut iter = self.grapheme_indices(true).peekable();
            let mut count = 0;
            while let Some((i, _)) = iter.next() {
                let ni = if let Some(next) = iter.peek() { next.0 } else { self.len() };

                if from >= i && from < ni {
                    current = count;
                    break;
                }

                count += 1;
            }

            current
        }
    }

    fn prev_codepoint_offset(&self, current_pos: usize) -> Option<usize> {
        if current_pos == 0 {
            None
        } else {
            let mut len = 1;
            while !self.is_char_boundary(current_pos - len) {
                len += 1;
            }

            Some(current_pos - len)
        }
    }

    fn next_codepoint_offset(&self, current_pos: usize) -> Option<usize> {
        if current_pos == self.len() {
            None
        } else {
            let b = self.as_bytes()[current_pos];
            Some(current_pos + len_utf8_from_first_byte(b))
        }
    }

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

    fn is_empty(&self) -> bool {
        self.is_empty()
    }

    fn from_str(s: &str) -> Self {
        s.to_string()
    }

    fn preceding_line_break(&self, from: usize) -> usize {
        let mut offset = from;

        for byte in self.get(0..from).unwrap_or("").bytes().rev() {
            if byte == 0x0a {
                return offset;
            }
            offset -= 1;
        }

        0
    }

    fn next_line_break(&self, from: usize) -> usize {
        let mut offset = from;

        for char in self.get(from..).unwrap_or("").bytes() {
            if char == 0x0a {
                return offset;
            }
            offset += 1;
        }

        self.len()
    }

    fn prev_codepoint(&self, offset: usize) -> Option<char> {
        if let Some(prev) = self.prev_codepoint_offset(offset) {
            self[prev..].chars().next()
        } else {
            None
        }
    }
}

pub fn len_utf8_from_first_byte(b: u8) -> usize {
    match b {
        b if b < 0x80 => 1,
        b if b < 0xe0 => 2,
        b if b < 0xf0 => 3,
        _ => 4,
    }
}
