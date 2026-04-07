use crate::prelude::*;

/// A direction for resizing a resizable stack from one of its edges.
#[derive(PartialEq, Clone, Copy)]
pub enum ResizeStackDirection {
    Left,
    Right,
    Top,
    Bottom,
}

impl ResizeStackDirection {
    fn is_horizontal(self) -> bool {
        matches!(self, Self::Left | Self::Right)
    }

    fn is_vertical(self) -> bool {
        matches!(self, Self::Top | Self::Bottom)
    }

    fn resizes_from_leading_edge(self) -> bool {
        matches!(self, Self::Left | Self::Top)
    }
}

/// A view that can be resized by clicking and dragging from one of its edges.
///
/// The `ResizableStack` struct allows users to create a resizable container in a user interface.
/// It supports resizing in either a horizontal or vertical direction, as specified
/// by the `direction` field. The resizing behavior is controlled via the `on_drag` callback, which
/// is triggered during a drag operation.
pub struct ResizableStack {
    /// Tracks whether the edge of the view is currently being dragged.
    is_dragging: bool,

    /// A callback function that is triggered when the view is being dragged.
    /// The callback receives a mutable reference to the event context and the new size
    /// of the stack as a floating-point value.
    on_drag: Box<dyn Fn(&mut EventContext, f32)>,

    /// An optional callback function that is called when the stack is reset.
    /// This callback is triggered when the user double-clicks the resize handle,
    /// allowing the stack to return to its default size.
    on_reset: Option<Box<dyn Fn(&mut EventContext)>>,

    /// Specifies the edge from which the stack can be resized.
    direction: ResizeStackDirection,

    /// The mouse position on the active axis when dragging starts.
    drag_start: f32,

    /// The stack size when dragging starts, in logical pixels.
    start_size: f32,
}

impl ResizableStack {
    /// Creates a new `ResizableStack` view.
    /// The `size` parameter is a `Res<Units>` source for the stack size, updated when the stack is resized.
    /// The `direction` parameter specifies which edge of the stack is resizable.
    /// The `on_drag` callback is called with the new size when the stack is being resized.
    /// The `content` closure is called to build the content of the stack.
    pub fn new<F>(
        cx: &mut Context,
        size: impl Res<Units>,
        direction: ResizeStackDirection,
        on_drag: impl Fn(&mut EventContext, f32) + 'static,
        content: F,
    ) -> Handle<Self>
    where
        F: FnOnce(&mut Context),
    {
        let handle = Self {
            is_dragging: false,
            on_drag: Box::new(on_drag),
            on_reset: None,
            direction,
            drag_start: 0.0,
            start_size: 0.0,
        }
        .build(cx, |cx| {
            ResizeHandle::new(cx);
            (content)(cx);
        })
        .toggle_class("horizontal", direction.is_vertical())
        .toggle_class("vertical", direction.is_horizontal())
        .toggle_class("left", direction == ResizeStackDirection::Left)
        .toggle_class("right", direction == ResizeStackDirection::Right)
        .toggle_class("top", direction == ResizeStackDirection::Top)
        .toggle_class("bottom", direction == ResizeStackDirection::Bottom);

        if direction.is_horizontal() { handle.width(size) } else { handle.height(size) }
    }
}

/// Events emitted by the `ResizableStack` view to indicate changes in dragging state.
pub enum ResizableStackEvent {
    /// Emitted when the user starts dragging the resizable edge of the stack.
    /// This event is triggered when the user presses down on the resize handle.
    /// It enables dragging behavior and locks the cursor.
    StartDrag {
        cursor_x: f32, // The x-position of the mouse cursor when dragging starts.
        cursor_y: f32, // The y-position of the mouse cursor when dragging starts.
    },

    /// Emitted when the user stops dragging the resizable edge of the stack.
    /// This event is triggered when the user releases the mouse button after dragging.
    /// It disables dragging behavior and unlocks the cursor.
    StopDrag,

    /// Emitted when the user double-clicks the resize handle.
    Reset,
}

impl View for ResizableStack {
    fn element(&self) -> Option<&'static str> {
        Some("resizable-stack")
    }

    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|resizable_stack_event, event| match resizable_stack_event {
            ResizableStackEvent::StartDrag { cursor_x, cursor_y } => {
                self.is_dragging = true;
                cx.set_active(true);
                cx.capture();
                cx.lock_cursor_icon();
                self.start_size = if self.direction.is_horizontal() {
                    cx.bounds().w / cx.scale_factor()
                } else {
                    cx.bounds().h / cx.scale_factor()
                };

                // Disable pointer events for everything while dragging
                cx.with_current(Entity::root(), |cx| {
                    cx.set_pointer_events(false);
                });

                // Prevent propagation in case the resizable stack is within another resizable stack
                event.consume();

                if self.direction.is_horizontal() {
                    self.drag_start = *cursor_x;
                } else {
                    self.drag_start = *cursor_y;
                }
            }

            ResizableStackEvent::StopDrag => {
                self.is_dragging = false;
                cx.set_active(false);
                cx.release();
                cx.unlock_cursor_icon();

                // Re-enable pointer events
                cx.with_current(Entity::root(), |cx| {
                    cx.set_pointer_events(true);
                });

                event.consume()
            }

            ResizableStackEvent::Reset => {
                self.is_dragging = false;
                cx.set_active(false);
                cx.release();
                cx.unlock_cursor_icon();

                // Re-enable pointer events
                cx.with_current(Entity::root(), |cx| {
                    cx.set_pointer_events(true);
                });

                if let Some(on_reset) = &self.on_reset {
                    on_reset(cx);
                }

                event.consume()
            }
        });

        event.map(|window_event, _| match window_event {
            WindowEvent::MouseMove(x, y) => {
                let dpi = cx.scale_factor();
                if self.is_dragging {
                    let delta = if self.direction.is_horizontal() {
                        (*x - self.drag_start) / dpi
                    } else {
                        (*y - self.drag_start) / dpi
                    };

                    let new_size = if self.direction.resizes_from_leading_edge() {
                        self.start_size - delta
                    } else {
                        self.start_size + delta
                    };

                    if new_size.is_finite() && new_size > 5.0 {
                        (self.on_drag)(cx, new_size);
                    }
                }
            }

            WindowEvent::MouseUp(button) if *button == MouseButton::Left => {
                cx.emit(ResizableStackEvent::StopDrag);
            }

            _ => {}
        });
    }
}

impl Handle<'_, ResizableStack> {
    /// Sets a callback to be called when the stack is reset, i.e. when the resize handle is double-clicked.
    pub fn on_reset<F>(self, on_reset: F) -> Self
    where
        F: Fn(&mut EventContext) + 'static,
    {
        self.modify(|this| {
            this.on_reset = Some(Box::new(on_reset));
        })
    }
}

pub struct ResizeHandle;

impl ResizeHandle {
    pub fn new(cx: &mut Context) -> Handle<Self> {
        Self.build(cx, |_cx| {}).position_type(PositionType::Absolute).z_index(10)
    }
}

impl View for ResizeHandle {
    fn element(&self) -> Option<&'static str> {
        Some("resize-handle")
    }

    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|window_event, _| match window_event {
            WindowEvent::PressDown { mouse } if *mouse => {
                cx.emit(ResizableStackEvent::StartDrag {
                    cursor_x: cx.mouse.cursor_x,
                    cursor_y: cx.mouse.cursor_y,
                });
            }

            WindowEvent::MouseDoubleClick(button) if *button == MouseButton::Left => {
                cx.emit(ResizableStackEvent::Reset);
            }

            _ => {}
        });
    }
}
