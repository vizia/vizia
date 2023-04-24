use std::hash::Hash;

use std::cmp::{Eq, PartialEq};

use bitflags::bitflags;

bitflags! {
    /// A bitflag of possible pseudoclasses.
    #[derive(Debug, Clone, Copy)]
    pub struct PseudoClassFlags: u32 {
        const HOVER = 1;
        const ACTIVE = 1 << 1;
        const OVER = 1 << 2;
        const FOCUS = 1 << 3;
        const FOCUS_VISIBLE = 1 << 4;
        const FOCUS_WITHIN = 1 << 5;
        const READ_ONLY = 1 << 6;
        const READ_WRITE = 1 << 7;
        const PLACEHOLDER_SHOWN = 1 << 8;
        const DEFAULT = 1 << 9;
        const CHECKED = 1 << 10;
        const INDETERMINATE = 1 << 11;
        const BLANK = 1 << 12;
        const VALID = 1 << 13;
        const INVALID = 1 << 14;
        const IN_RANGE = 1 << 15;
        const OUT_OF_RANGE = 1 << 16;
        const REQUIRED = 1 << 17;
        const OPTIONAL = 1 << 18;
        const USER_VALID = 1 << 19;
        const USER_INVALID = 1 << 20;
    }
}

impl Default for PseudoClassFlags {
    fn default() -> Self {
        PseudoClassFlags::empty()
    }
}

// TODO
impl std::fmt::Display for PseudoClassFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.contains(PseudoClassFlags::HOVER) {
            write!(f, ":hover")?;
        }
        if self.contains(PseudoClassFlags::OVER) {
            write!(f, ":over")?;
        }
        if self.contains(PseudoClassFlags::ACTIVE) {
            write!(f, ":active")?;
        }
        if self.contains(PseudoClassFlags::FOCUS) {
            write!(f, ":focus")?;
        }
        if self.contains(PseudoClassFlags::CHECKED) {
            write!(f, ":checked")?;
        }
        if self.contains(PseudoClassFlags::FOCUS_WITHIN) {
            write!(f, ":focus-within")?;
        }
        if self.contains(PseudoClassFlags::FOCUS_VISIBLE) {
            write!(f, ":focus-visible")?;
        }

        Ok(())
    }
}
