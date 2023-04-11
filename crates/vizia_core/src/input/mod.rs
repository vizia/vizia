//! Types used for handling input events such as mouse and keyboard.

mod keymap;
pub use keymap::*;

mod entry;
pub use entry::*;

pub use vizia_input::{Code, Key, Modifiers, MouseButton, MouseButtonData, MouseState};
