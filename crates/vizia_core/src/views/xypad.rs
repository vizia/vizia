use crate::prelude::*;

/// A view which allows the user to manipulate 2 floating point values simultaneously on a two dimensional pane.
///
/// # Examples
///
/// ## Basic XYPad
/// ```
/// # use vizia_core::prelude::*;
/// # let mut cx = &mut Context::default();
/// # let xy = cx.state((0.5f32, 0.5f32));
/// XYPad::new(cx, xy)
///     .on_change(move |cx, x, y| {
///         xy.set(cx, (x, y));
///     });
/// ```
pub struct XYPad {
    value: Signal<(f32, f32)>,
    is_dragging: bool,
    on_change: Option<Box<dyn Fn(&mut EventContext, f32, f32)>>,
}

impl XYPad {
    /// Creates a new [XYPad] view bound to the given value.
    ///
    /// Accepts either a plain tuple or a `Signal<(f32, f32)>` for reactive state.
    /// Use `.two_way()` for automatic signal updates, or `.on_change()` for custom handling.
    ///
    /// # Examples
    ///
    /// ```
    /// # use vizia_core::prelude::*;
    /// # let mut cx = &mut Context::default();
    /// // Static value
    /// XYPad::new(cx, (0.5f32, 0.5f32));
    ///
    /// // Reactive with two-way binding
    /// # let xy = cx.state((0.5f32, 0.5f32));
    /// XYPad::new(cx, xy).two_way();
    /// ```
    pub fn new(cx: &mut Context, value: impl Res<(f32, f32)> + 'static) -> Handle<Self> {
        let value = value.into_signal(cx);
        let position_absolute = cx.state(PositionType::Absolute);
        let thumb_size = cx.state(Pixels(10.0));
        let pad_size = cx.state(Pixels(200.0));
        let thumb_translate = cx.state(Translate::new(
            Length::Value(LengthValue::Px(-6.0)),
            Length::Value(LengthValue::Px(-6.0)),
        ));
        let thumb_radius = cx.state(Percentage(50.0));
        let thumb_border = cx.state(Pixels(2.0));
        let pad_border = cx.state(Pixels(1.0));
        let false_signal = cx.state(false);
        let overflow_hidden = cx.state(Overflow::Hidden);
        let left = cx.derived({
            let value = value;
            move |store| Percentage(value.get(store).0 * 100.0)
        });
        let top = cx.derived({
            let value = value;
            move |store| Percentage((1.0 - value.get(store).1) * 100.0)
        });
        Self { value, is_dragging: false, on_change: None }
            .build(cx, move |cx| {
                // Thumb
                Element::new(cx)
                    .position_type(position_absolute)
                    .left(left)
                    .top(top)
                    .translate(thumb_translate)
                    .size(thumb_size)
                    .corner_radius(thumb_radius)
                    .border_width(thumb_border)
                    .hoverable(false_signal)
                    .class("thumb");
            })
            .overflow(overflow_hidden)
            .border_width(pad_border)
            .size(pad_size)
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
                if meta.target == current || meta.target.is_descendant_of(cx.tree, current) {
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

            WindowEvent::MouseMove(x, y) => {
                if self.is_dragging {
                    let current = cx.current();
                    let mut dx = (*x - cx.cache.get_posx(current)) / cx.cache.get_width(current);
                    let mut dy = (*y - cx.cache.get_posy(current)) / cx.cache.get_height(current);

                    dx = dx.clamp(0.0, 1.0);
                    dy = dy.clamp(0.0, 1.0);

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

    /// Enables two-way binding: XYPad changes automatically update the bound signal.
    ///
    /// This is a convenience method equivalent to:
    /// ```ignore
    /// .on_change(move |cx, x, y| signal.set(cx, (x, y)))
    /// ```
    pub fn two_way(self) -> Self {
        self.modify(|xypad| {
            let signal = xypad.value;
            xypad.on_change = Some(Box::new(move |cx, x, y| signal.set(cx, (x, y))));
        })
    }
}
