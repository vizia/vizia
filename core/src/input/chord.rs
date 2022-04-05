use crate::{Code, Modifiers};

/// A key chord used inside of a [`Keymap`](crate::Keymap).
#[derive(Default, Copy, Clone, Debug)]
pub struct KeyChord {
    /// The modifiers that have to be pressed in order to active its associated action.
    pub modifiers: Modifiers,
    /// The button that has to be pressed in order to active its associated action.
    pub button: Code,
}

impl KeyChord {
    /// Creates a new key chord.
    ///
    /// # Examples
    ///
    /// ```
    /// # use vizia_core::*;
    /// #
    /// let key_chord = KeyChord::new(Modifiers::empty(), Code::KeyA);
    /// ```
    pub fn new(modifiers: Modifiers, button: Code) -> Self {
        Self { modifiers, button }
    }
}
