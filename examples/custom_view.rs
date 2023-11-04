use vizia::prelude::*;
use vizia::vg;

pub struct CustomView<C: 'static + Res<Color>> {
    // View local state (could be value or lens)
    color: C,
}

impl<C: 'static + Res<Color>> CustomView<C> {
    pub fn new(cx: &mut Context, color: C) -> Handle<Self> {
        Self { color }.build(cx, |cx| {
            Label::new(cx, "This is a custom view!");
        })
    }
}

impl<C: 'static + Res<Color>> View for CustomView<C> {
    fn draw(&self, cx: &mut DrawContext, canvas: &mut Canvas) {
        let col = self.color.get_ref(cx);
        let bounds = cx.bounds();
        let mut path = vg::Path::new();
        path.rect(bounds.x, bounds.y, bounds.w, bounds.h);
        canvas.fill_path(&path, &vg::Paint::color((*col).into()));
    }
}

#[derive(Lens)]
struct AppData {
    color: Color,
}

impl Model for AppData {}

fn main() {
    Application::new(|cx| {
        AppData { color: Color::red() }.build(cx);
        CustomView::new(cx, Color::blue());
        CustomView::new(cx, AppData::color);
    })
    .run();
}
