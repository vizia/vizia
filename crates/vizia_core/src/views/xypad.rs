use crate::prelude::*;

pub struct XYPad {
    is_dragging: bool,

    on_change: Option<Box<dyn Fn(&mut EventContext, f32, f32)>>,
}

impl XYPad {
    pub fn new<L: Lens<Target = (f32, f32)>>(cx: &mut Context, lens: L) -> Handle<Self> {
        Self { is_dragging: false, on_change: None }
            .build(cx, |cx| {
                Element::new(cx)
                    .position_type(PositionType::SelfDirected)
                    .left(lens.clone().map(|(x, _)| Percentage(*x * 100.0)))
                    .top(lens.clone().map(|(_, y)| Percentage((1.0 - *y) * 100.0)))
                    .translate((Pixels(-5.0), Pixels(-5.0)))
                    .size(Pixels(10.0))
                    .border_radius(Percentage(50.0))
                    .border_width(Pixels(2.0))
                    .border_color(Color::white())
                    .hoverable(false);
            })
            .overflow(Overflow::Hidden)
            .size(Pixels(200.0))
            .background_color(Color::rgb(40, 40, 40))
    }
}

impl View for XYPad {
    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|window_event, meta| match window_event {
            WindowEvent::MouseDown(button) if *button == MouseButton::Left => {
                let current = cx.current();
                if meta.target == current {
                    cx.capture();
                    // let width = cx.cache.get_width(current);
                    // let height = cx.cache.get_height(current);

                    let mut dx = (cx.mouse.left.pos_down.0 - cx.cache.get_posx(current))
                        / cx.cache.get_width(current);
                    let mut dy = (cx.mouse.left.pos_down.1 - cx.cache.get_posy(current))
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
    pub fn on_change<F>(self, callback: F) -> Self
    where
        F: 'static + Fn(&mut EventContext, f32, f32),
    {
        self.modify(|xypad: &mut XYPad| xypad.on_change = Some(Box::new(callback)))
    }
}
