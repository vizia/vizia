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
    CustomViewApp::run()
}

struct CustomViewApp {
    color: Signal<Color>,
    red_value: Signal<f32>,
    size_200: Signal<Units>,
    width_200: Signal<Units>,
    space_20: Signal<Units>,
}

impl App for CustomViewApp {
    fn new(cx: &mut Context) -> Self {
        Self {
            color: cx.state(Color::red()),
            red_value: cx.state(1.0f32),
            size_200: cx.state(Pixels(200.0)),
            width_200: cx.state(Pixels(200.0)),
            space_20: cx.state(Pixels(20.0)),
        }
    }

    fn on_build(self, cx: &mut Context) -> Self {
        let color = self.color;
        let red_value = self.red_value;
        let size_200 = self.size_200;
        let width_200 = self.width_200;
        let space_20 = self.space_20;

        CustomView::new(cx, color).size(size_200);
        Slider::new(cx, red_value)
            .on_change(move |cx, val| {
                red_value.set(cx, val);
                color.set(cx, Color::rgb((val * 255.0) as u8, 0, 0));
            })
            .width(width_200)
            .space(space_20);
        self
    }

    fn window_config(&self) -> WindowConfig {
        window(|app| app.title("Custom View"))
    }
}
