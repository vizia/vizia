use vizia::vg::{Paint, Path};
use vizia::{prelude::*, style::PropType};

const STYLE: &str = r#"
    .foo {
        --custom-color: red;
        transition: --custom-color 1.0 0.0;
    }

    .foo:hover {
        --custom-color: green;
        transition: --custom-color 1.0 0.0;
    }
"#;

pub struct CustomView {}

impl CustomView {
    pub fn new(cx: &mut Context) -> Handle<Self> {
        Self {}.build(cx, |_| {})
    }
}

impl View for CustomView {
    fn draw(&self, cx: &mut DrawContext, canvas: &mut Canvas) {
        if let Some(custom_color) = cx.get_property::<Color>("custom-color").cloned() {
            let bounds = cx.bounds();

            let mut path = Path::new();

            path.rect(bounds.x, bounds.y, bounds.w, bounds.h);

            canvas.fill_path(&mut path, Paint::color(custom_color.into()));
        }
    }
}

pub trait CustomProp {
    fn custom_color(self, color: Color) -> Self;
}

impl<'a, V: View> CustomProp for Handle<'a, V> {
    fn custom_color(mut self, color: Color) -> Self {
        self.set_property("custom-color", color);

        self
    }
}

fn main() {
    Application::new(|cx| {
        cx.add_property("custom-color", PropType::Color(Color::default()));

        cx.add_theme(STYLE);

        // Uses CSS styling
        CustomView::new(cx).size(Pixels(100.0)).class("foo");

        // Uses inline styling
        CustomView::new(cx).size(Pixels(100.0)).custom_color(Color::blue());
    })
    .run();
}
