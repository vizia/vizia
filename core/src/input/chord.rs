use std::hash::Hash;
use crate::prelude::*;

/// A key chord used inside of a [`Keymap`](crate::prelude::Keymap).
///
/// This type is part of the prelude.
#[derive(Default, Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct KeyChord {
    /// The modifiers that have to be pressed in order to active its associated actions.
    pub modifiers: Modifiers,
    /// The button that has to be pressed in order to active its associated actions.
    pub code: Code,
}

impl KeyChord {
    /// Creates a new key chord.
    ///
    /// # Examples
    ///
    /// ```
    /// # use vizia_core::prelude::*;
    /// #
    /// let key_chord = KeyChord::new(Modifiers::empty(), Code::KeyA);
    /// ```
    pub fn new(modifiers: Modifiers, code: Code) -> Self {
        Self { modifiers, code }
    }
}
