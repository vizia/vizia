use vizia::prelude::*;
use vizia::vg;

pub struct CustomView {
    color: Signal<Color>,
}

impl CustomView {
    pub fn new(cx: &mut Context, color: Signal<Color>) -> Handle<'_, Self> {
        Self { color }
            .build(cx, |cx| {
                Label::new(cx, "This is a custom view!");
            })
            // Redraw when signal changes
            .bind(color, |mut handle, _| handle.needs_redraw())
    }
}

impl View for CustomView {
    fn draw(&self, cx: &mut DrawContext, canvas: &Canvas) {
        let col = *self.color.get(cx);
        let bounds = cx.bounds();
        let rect: vg::Rect = bounds.into();
        let mut path = vg::Path::new();
        path.add_rect(rect, None);
        let mut paint = vg::Paint::default();
        paint.set_color(col);
        canvas.draw_path(&path, &paint);
    }
}

fn main() -> Result<(), ApplicationError> {
    Application::new(|cx| {
        let color = cx.state(Color::red());
        let red_value = cx.state(1.0f32);

        CustomView::new(cx, color).size(Pixels(200.0));
        Slider::new(cx, red_value)
            .on_change(move |cx, val| {
                red_value.set(cx, val);
                color.set(cx, Color::rgb((val * 255.0) as u8, 0, 0));
            })
            .width(Pixels(200.0))
            .space(Pixels(20.0));
    })
    .run()
}
