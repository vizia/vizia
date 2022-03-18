use crate::Entity;

/// A mouse button.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
    Other(u16),
}

/// The state of a mouse button.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum MouseButtonState {
    Pressed,
    Released,
}

/// Data which describes the current state of a mouse button.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct MouseButtonData {
    /// The state of the mouse button (pressed/released)
    pub state: MouseButtonState,
    /// The position of the mouse cursor when the mouse button was last pressed
    pub pos_down: (f32, f32),
    /// The position of the mouse cursor when the mouse button was last released
    pub pos_up: (f32, f32),
    /// The hovered entity when the mouse button was last pressed
    pub pressed: Entity,
    /// The hovered entity when the mouse button was last released
    pub released: Entity,
}

impl Default for MouseButtonData {
    fn default() -> Self {
        MouseButtonData {
            state: MouseButtonState::Released,
            pos_down: (0.0, 0.0),
            pos_up: (0.0, 0.0),
            pressed: Entity::null(),
            released: Entity::null(),
        }
    }
}

/// The current state of the mouse cursor and buttons.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct MouseState {
    pub cursorx: f32,
    pub cursory: f32,

    pub left: MouseButtonData,
    pub right: MouseButtonData,
    pub middle: MouseButtonData,
}

impl Default for MouseState {
    fn default() -> Self {
        MouseState {
            cursorx: -1.0,
            cursory: -1.0,
            left: MouseButtonData::default(),
            right: MouseButtonData::default(),
            middle: MouseButtonData::default(),
        }
    }
}

impl MouseState {
    pub fn delta(&self, button: MouseButton) -> (f32, f32) {
        match button {
            MouseButton::Left => {
                (self.cursorx - self.left.pos_down.0, self.cursory - self.left.pos_down.1)
            }

            MouseButton::Right => {
                (self.cursorx - self.right.pos_down.0, self.cursory - self.right.pos_down.1)
            }

            MouseButton::Middle => {
                (self.cursorx - self.middle.pos_down.0, self.cursory - self.middle.pos_down.1)
            }

            _ => (0.0, 0.0),
        }
    }
}
