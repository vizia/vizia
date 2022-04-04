use crate::{Code, Modifiers};

/// A keybinding used inside of a [`Keymap`](crate::Keymap).
#[derive(Default, Copy, Clone, Debug)]
pub struct KeyBinding {
    /// The modifiers that have to be pressed in order to active its associated action.
    pub modifiers: Modifiers,
    /// The button that has to be pressed in order to active its associated action.
    pub button: Code,
}

impl KeyBinding {
    /// Creates a new keybinding.
    ///
    /// # Examples
    ///
    /// ```
    /// # use vizia_core::*;
    /// #
    /// let keybinding = KeyBinding::new(Modifiers::empty(), Code::KeyA);
    /// ```
    pub fn new(modifiers: Modifiers, button: Code) -> Self {
        Self { modifiers, button }
    }
}
