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
    pub cursorx: f32,
    /// The vertical mouse cursor position of the frame.
    pub cursory: f32,
    /// The horizontal mouse cursor position of the previous frame.
    pub previous_cursorx: f32,
    /// The vertical mouse cursor position of the previous frame.
    pub previous_cursory: f32,
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
            cursorx: -1.0,
            cursory: -1.0,
            previous_cursorx: -1.0,
            previous_cursory: -1.0,
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
    pub fn frame_delta(&self) -> (f32, f32) {
        (self.cursorx - self.previous_cursorx, self.cursory - self.previous_cursory)
    }

    /// Returns the delta of the mouse cursor position of the current frame and the frame the `button` got pressed.
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
