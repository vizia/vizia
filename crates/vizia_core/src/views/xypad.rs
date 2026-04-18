use crate::prelude::*;

/// A view which allows the user to manipulate 2 floating point values simultaneously on a two dimensional pane.
pub struct XYPad {
    is_dragging: bool,

    on_change: Option<Box<dyn Fn(&mut EventContext, f32, f32)>>,
}

impl XYPad {
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
                cx.capture();
                let mouse = cx.mouse();
                if meta.target == current {
                    let mut dx = (mouse.left.pos_down.0 - cx.cache.get_posx(current))
                        / cx.cache.get_width(current);
                    let mut dy = (mouse.left.pos_down.1 - cx.cache.get_posy(current))
                        / cx.cache.get_height(current);

                    dx = dx.clamp(0.0, 1.0);
                    dy = dy.clamp(0.0, 1.0);

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
                if meta.target == cx.current() {
                    cx.release();
                }
            }

            WindowEvent::MouseMove(x, y) if self.is_dragging => {
                let current = cx.current();
                let mut dx = (*x - cx.cache.get_posx(current)) / cx.cache.get_width(current);
                let mut dy = (*y - cx.cache.get_posy(current)) / cx.cache.get_height(current);

                dx = dx.clamp(0.0, 1.0);
                dy = dy.clamp(0.0, 1.0);

                if let Some(callback) = &self.on_change {
                    (callback)(cx, dx, 1.0 - dy);
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
