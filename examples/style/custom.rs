use vizia::vg::{Paint, Path};
use vizia::{prelude::*, style::PropType};

const STYLE: &str = r#"
    .foo {
        --custom-color: red;
    }

    .foo:hover {
        --custom-color: green;
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

fn main() {
    Application::new(|cx| {
        cx.add_property("custom-color", PropType::Color(Color::default()));

        cx.add_theme(STYLE);

        CustomView::new(cx).size(Pixels(100.0)).class("foo");
    })
    .run();
}
