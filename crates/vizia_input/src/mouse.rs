use vizia_id::GenerationalId;

/// A mouse button.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum MouseButton {
    /// The left mouse button.
    Left,
    /// The right mouse button.
    Right,
    /// The middle mouse button.
    Middle,
    /// Another mouse button with the associated button number.
    Other(u16),

    Back,

    Forward,
}

/// The state of a mouse button.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum MouseButtonState {
    /// Represents a pressed mouse button.
    Pressed,
    /// Represents a released mouse button.
    Released,
}

/// Data which describes the current state of a mouse button.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct MouseButtonData<I>
where
    I: GenerationalId,
{
    /// The state of the mouse button (pressed/released).
    pub state: MouseButtonState,
    /// The position of the mouse cursor when the mouse button was last pressed.
    pub pos_down: (f32, f32),
    /// The position of the mouse cursor when the mouse button was last released.
    pub pos_up: (f32, f32),
    /// The hovered entity when the mouse button was last pressed.
    pub pressed: I,
    /// The hovered entity when the mouse button was last released.
    pub released: I,
}

impl<I> Default for MouseButtonData<I>
where
    I: GenerationalId,
{
    fn default() -> Self {
        MouseButtonData {
            state: MouseButtonState::Released,
            pos_down: (0.0, 0.0),
            pos_up: (0.0, 0.0),
            pressed: I::null(),
            released: I::null(),
        }
    }
}

/// The current state of the mouse cursor and buttons.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct MouseState<I>
where
    I: GenerationalId,
{
    /// The horizontal mouse cursor position of the frame.
    pub cursor_x: f32,
    /// The vertical mouse cursor position of the frame.
    pub cursor_y: f32,
    /// The horizontal mouse cursor position of the previous frame.
    pub previous_cursor_x: f32,
    /// The vertical mouse cursor position of the previous frame.
    pub previous_cursor_y: f32,
    /// The state of the left mouse button.
    pub left: MouseButtonData<I>,
    /// The state of the right mouse button.
    pub right: MouseButtonData<I>,
    /// The state of the middle mouse button.
    pub middle: MouseButtonData<I>,
}

impl<I> Default for MouseState<I>
where
    I: GenerationalId,
{
    fn default() -> Self {
        MouseState {
            cursor_x: -1.0,
            cursor_y: -1.0,
            previous_cursor_x: -1.0,
            previous_cursor_y: -1.0,
            left: MouseButtonData::default(),
            right: MouseButtonData::default(),
            middle: MouseButtonData::default(),
        }
    }
}

impl<I> MouseState<I>
where
    I: GenerationalId,
{
    /// Returns the delta of the mouse cursor position of the current and previous frame.
    pub fn delta(&self) -> (f32, f32) {
        (self.cursor_x - self.previous_cursor_x, self.cursor_y - self.previous_cursor_y)
    }

    /// Returns the delta of the mouse cursor position of the current frame and the frame the `button` got pressed.
    pub fn button_delta(&self, button: MouseButton) -> (f32, f32) {
        match button {
            MouseButton::Left => {
                (self.cursor_x - self.left.pos_down.0, self.cursor_y - self.left.pos_down.1)
            }

            MouseButton::Right => {
                (self.cursor_x - self.right.pos_down.0, self.cursor_y - self.right.pos_down.1)
            }

            MouseButton::Middle => {
                (self.cursor_x - self.middle.pos_down.0, self.cursor_y - self.middle.pos_down.1)
            }

            _ => (0.0, 0.0),
        }
    }
}
