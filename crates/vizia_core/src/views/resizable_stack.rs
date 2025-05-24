use crate::prelude::*;

/// A direction for resizing a resizable stack, either horizontally (right) or vertically (bottom).
#[derive(PartialEq, Clone, Copy)]
pub enum ResizeStackDirection {
    Right,
    Bottom,
}

/// A view that can be resized by clicking and dragging from the right or bottom edge.
/// 
/// The `ResizableStack` struct allows users to create a resizable container in a user interface.
/// It supports resizing in either a horizontal (right) or vertical (bottom) direction, as specified
/// by the `direction` field. The resizing behavior is controlled via the `on_drag` callback, which
/// is triggered during a drag operation.
#[derive(Lens)]
pub struct ResizableStack {
    /// Tracks whether the edge of the view is currently being dragged.
    is_dragging: bool,

    /// A callback function that is triggered when the view is being dragged.
    /// The callback receives a mutable reference to the event context and the new size
    /// of the stack as a floating-point value.
    on_drag: Box<dyn Fn(&mut EventContext, f32)>,

    /// Specifies the direction in which the stack can be resized.
    /// This can be either `Right` for horizontal resizing or `Bottom` for vertical resizing.
    direction: ResizeStackDirection,
}

impl ResizableStack {
    /// Creates a new `ResizableStack` view.
    /// The `size` parameter is a lens to the size of the stack, which will be updated when the stack is resized.
    /// The `direction` parameter specifies whether the stack is resized horizontally (right) or vertically (bottom).
    /// The `on_drag` callback is called with the new size when the stack is being resized.
    /// The `content` closure is called to build the content of the stack.
    pub fn new<F>(
        cx: &mut Context,
        size: impl Lens<Target = Units>,
        direction: ResizeStackDirection,
        on_drag: impl Fn(&mut EventContext, f32) + 'static,
        content: F,
    ) -> Handle<Self>
    where
        F: FnOnce(&mut Context),
    {
        let handle = Self { is_dragging: false, on_drag: Box::new(on_drag), direction }
            .build(cx, |cx| {
                Element::new(cx)
                    .position_type(PositionType::Absolute)
                    .z_index(10)
                    .class("resize-handle")
                    .on_press_down(|cx| cx.emit(ResizableStackEvent::StartDrag));

                (content)(cx);
            })
            .toggle_class(
                "horizontal",
                ResizableStack::direction.map(|d| *d == ResizeStackDirection::Bottom),
            )
            .toggle_class(
                "vertical",
                ResizableStack::direction.map(|d| *d == ResizeStackDirection::Right),
            );

        if direction == ResizeStackDirection::Right {
            handle.width(size)
        } else {
            handle.height(size)
        }
    }
}

/// Events emitted by the `ResizableStack` view to indicate changes in dragging state.
pub enum ResizableStackEvent {
    /// Emitted when the user starts dragging the resizable edge of the stack.
    /// This event is triggered when the user presses down on the resize handle.
    /// It enables dragging behavior and locks the cursor.
    StartDrag,

    /// Emitted when the user stops dragging the resizable edge of the stack.
    /// This event is triggered when the user releases the mouse button after dragging.
    /// It disables dragging behavior and unlocks the cursor.
    StopDrag,
}

impl View for ResizableStack {
    fn element(&self) -> Option<&'static str> {
        Some("resizable-stack")
    }

    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|resizable_stack_event, event| match resizable_stack_event {
            ResizableStackEvent::StartDrag => {
                self.is_dragging = true;
                cx.set_active(true);
                cx.capture();
                cx.lock_cursor_icon();

                // Disable pointer events for everything while dragging
                cx.with_current(Entity::root(), |cx| {
                    cx.set_pointer_events(false);
                });

                // Prevent propagation in case the resizable stack is within another resizable stack
                event.consume();
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
        });

        event.map(|window_event, _| match window_event {
            WindowEvent::MouseMove(x, y) => {
                let dpi = cx.scale_factor();
                if self.is_dragging {
                    let new_size = if self.direction == ResizeStackDirection::Right {
                        let posx = cx.bounds().x;
                        (*x - posx) / dpi
                    } else {
                        let posy = cx.bounds().y;
                        (*y - posy) / dpi
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
