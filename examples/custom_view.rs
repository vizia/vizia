use vizia::prelude::*;
use vizia::vg;

pub struct CustomView<L: Lens<Target = Color>> {
    color: L,
}

impl<L: Lens<Target = Color>> CustomView<L> {
    pub fn new(cx: &mut Context, color: L) -> Handle<'_, Self> {
        Self { color }
            .build(cx, |cx| {
                Label::new(cx, "This is a custom view!");
            })
            // Redraw when lensed data changes
            .bind(color, |mut handle, _| handle.needs_redraw())
    }
}

impl<L: Lens<Target = Color>> View for CustomView<L> {
    fn draw(&self, cx: &mut DrawContext, canvas: &Canvas) {
        let col = self.color.get(cx);
        let bounds = cx.bounds();
        let rect: vg::Rect = bounds.into();
        let mut path = vg::Path::new();
        path.add_rect(rect, None);
        let mut paint = vg::Paint::default();
        paint.set_color(col);
        canvas.draw_path(&path, &paint);
    }
}

#[derive(Lens)]
struct AppData {
    color: Color,
}

impl Model for AppData {
    fn event(&mut self, _cx: &mut EventContext, event: &mut Event) {
        event.map(|app_event, _| match app_event {
            AppEvent::SetColor(col) => self.color = *col,
        })
    }
}

pub enum AppEvent {
    SetColor(Color),
}

fn main() -> Result<(), ApplicationError> {
    Application::new(|cx| {
        AppData { color: Color::red() }.build(cx);
        CustomView::new(cx, AppData::color).size(Pixels(200.0));
        Slider::new(cx, AppData::color.map(|c| c.r() as f32 / 255.0))
            .on_change(|cx, val| cx.emit(AppEvent::SetColor(Color::rgb((val * 255.0) as u8, 0, 0))))
            .width(Pixels(200.0))
            .space(Pixels(20.0));
    })
    .run()
}
