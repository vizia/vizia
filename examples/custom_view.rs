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
            .bind(color, |mut view, _| {
                view.needs_redraw();
            })
    }
}

impl View for CustomView {
    fn draw(&self, cx: &mut DrawContext, canvas: &Canvas) {
        let col = self.color.get();
        let bounds = cx.bounds();
        let rect: vg::Rect = bounds.into();
        let path = vg::Path::rect(rect, None);
        let mut paint = vg::Paint::default();
        paint.set_color(col);
        canvas.draw_path(&path, &paint);
    }
}

struct AppData {
    color: Signal<Color>,
}

impl Model for AppData {
    fn event(&mut self, _cx: &mut EventContext, event: &mut Event) {
        event.map(|app_event, _| match app_event {
            AppEvent::SetColor(col) => self.color.set(*col),
        })
    }
}

pub enum AppEvent {
    SetColor(Color),
}

fn main() -> Result<(), ApplicationError> {
    Application::new(|cx| {
        let color = Signal::new(Color::red());

        AppData { color }.build(cx);

        CustomView::new(cx, color).size(Pixels(200.0));

        Slider::new(cx, color.map(|col| col.r() as f32 / 255.0))
            .on_change(|cx, val| cx.emit(AppEvent::SetColor(Color::rgb((val * 255.0) as u8, 0, 0))))
            .width(Pixels(200.0))
            .space(Pixels(20.0));
    })
    .run()
}
