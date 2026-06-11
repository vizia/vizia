use crate::prelude::*;

/// A view which allows the user to manipulate 2 floating point values simultaneously on a two dimensional pane.
pub struct XYPad {
    is_dragging: bool,

    on_change: Option<Box<dyn Fn(&mut EventContext, f32, f32)>>,
}

impl XYPad {
    fn normalized_from_cursor(cx: &EventContext, current: Entity, x: f32, y: f32) -> (f32, f32) {
        let bounds = cx.transformed_bounds(current);
        let width = bounds.width().max(f32::EPSILON);
        let height = bounds.height().max(f32::EPSILON);

        let dx = ((x - bounds.left()) / width).clamp(0.0, 1.0);
        let dy = ((y - bounds.top()) / height).clamp(0.0, 1.0);

        (dx, dy)
    }

    /// creates a new [XYPad] view.
    pub fn new<R: Res<(f32, f32)> + 'static>(cx: &mut Context, value: R) -> Handle<Self> {
        let value_state = value.to_signal(cx);
        let left = Memo::new(move |_| Percentage(value_state.get().0 * 100.0));
        let top = Memo::new(move |_| Percentage((1.0 - value_state.get().1) * 100.0));

        Self { is_dragging: false, on_change: None }
            .build(cx, |cx| {
                // Thumb
                Element::new(cx)
                    .position_type(PositionType::Absolute)
                    .left(left)
                    .top(top)
                    .translate(Translate::new(
                        Length::Value(LengthValue::Px(-6.0)),
                        Length::Value(LengthValue::Px(-6.0)),
                    ))
                    .size(Pixels(10.0))
                    .corner_radius(Percentage(50.0))
                    .border_width(Pixels(2.0))
                    .hoverable(false)
                    .class("thumb");
            })
            .overflow(Overflow::Hidden)
            .border_width(Pixels(1.0))
            .size(Pixels(200.0))
    }
}

impl View for XYPad {
    fn element(&self) -> Option<&'static str> {
        Some("xypad")
    }

    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|window_event, meta| match window_event {
            WindowEvent::MouseDown(button) if *button == MouseButton::Left => {
                if cx.is_disabled() {
                    return;
                }
                let current = cx.current();
                let (mouse_x, mouse_y) = cx.mouse.left.pos_down;
                if meta.target == current || meta.target.is_descendant_of(cx.tree, current) {
                    cx.capture();
                    let (dx, dy) = Self::normalized_from_cursor(cx, current, mouse_x, mouse_y);

                    self.is_dragging = true;

                    if let Some(callback) = &self.on_change {
                        (callback)(cx, dx, 1.0 - dy);
                    }
                }
            }

            WindowEvent::MouseUp(button) if *button == MouseButton::Left => {
                cx.set_active(false);
                cx.release();
                self.is_dragging = false;
            }

            WindowEvent::MouseMove(x, y) => {
                if self.is_dragging {
                    let current = cx.current();
                    let (dx, dy) = Self::normalized_from_cursor(cx, current, *x, *y);

                    if let Some(callback) = &self.on_change {
                        (callback)(cx, dx, 1.0 - dy);
                    }
                }
            }

            _ => {}
        });
    }
}

impl Handle<'_, XYPad> {
    /// Set the callback which will be triggered when the XYPad is manipulated.
    pub fn on_change<F: Fn(&mut EventContext, f32, f32) + 'static>(self, callback: F) -> Self {
        self.modify(|xypad| xypad.on_change = Some(Box::new(callback)))
    }
}
