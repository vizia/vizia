use crate::prelude::*;

#[derive(PartialEq, Clone, Copy)]
pub enum ResizeStackDirection {
    Right,
    Bottom,
}

// A view which can be resized by clicking and dragging from the right/bottom edge of the view.
#[derive(Lens)]
pub struct ResizableStack {
    // State which tracks whether the edge of the view is being dragged.
    is_dragging: bool,
    // Callback which is triggered when the view is being dragged.
    on_drag: Box<dyn Fn(&mut EventContext, f32)>,

    direction: ResizeStackDirection,
}

impl ResizableStack {
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
        let handle =
            Self { is_dragging: false, on_drag: Box::new(on_drag), direction }.build(cx, |cx| {
                if direction == ResizeStackDirection::Right {
                    Element::new(cx)
                        .width(Pixels(6.0))
                        .left(Stretch(1.0))
                        .right(Pixels(-4.0))
                        .position_type(PositionType::Absolute)
                        .z_index(10)
                        .class("resize_handle")
                        .toggle_class("drag_handle", ResizableStack::is_dragging)
                        .cursor(CursorIcon::EwResize)
                        .on_press_down(|cx| cx.emit(ResizableStackEvent::StartDrag));
                } else {
                    Element::new(cx)
                        .height(Pixels(6.0))
                        .top(Stretch(1.0))
                        .bottom(Pixels(-4.0))
                        .position_type(PositionType::Absolute)
                        .z_index(10)
                        .class("resize_handle")
                        .toggle_class("drag_handle", ResizableStack::is_dragging)
                        .cursor(CursorIcon::NsResize)
                        .on_press_down(|cx| cx.emit(ResizableStackEvent::StartDrag));
                }

                (content)(cx);
            });

        if direction == ResizeStackDirection::Right {
            handle.width(size)
        } else {
            handle.height(size)
        }
    }
}

pub enum ResizableStackEvent {
    StartDrag,
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
