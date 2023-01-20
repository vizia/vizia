use std::hash::Hash;

use std::cmp::{Eq, PartialEq};

use bitflags::bitflags;

bitflags! {
    /// A bitflag of possible pseudoclasses.
    pub struct PseudoClassFlags: u16 {
        const HOVER = 1;
        const OVER = 1 << 1;
        const ACTIVE = 1 << 2;
        const FOCUS = 1 << 3;
        const DISABLED = 1 << 4;
        const CHECKED = 1 << 5;
        const SELECTED = 1 << 6;
        const CUSTOM = 1 << 7;
        const FOCUS_WITHIN = 1<<8;
        const FOCUS_VISIBLE = 1 << 9;
        const ROOT = 1 << 10;
    }
}

impl Default for PseudoClassFlags {
    fn default() -> Self {
        PseudoClassFlags::empty()
    }
}

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
        if self.contains(PseudoClassFlags::DISABLED) {
            write!(f, ":disabled")?;
        }
        if self.contains(PseudoClassFlags::CHECKED) {
            write!(f, ":checked")?;
        }
        if self.contains(PseudoClassFlags::SELECTED) {
            write!(f, ":selected")?;
        }
        if self.contains(PseudoClassFlags::FOCUS_WITHIN) {
            write!(f, ":focus-within")?;
        }
        if self.contains(PseudoClassFlags::FOCUS_VISIBLE) {
            write!(f, ":focus-visible")?;
        }
        if self.contains(PseudoClassFlags::ROOT) {
            write!(f, ":root")?;
        }

        Ok(())
    }
}
