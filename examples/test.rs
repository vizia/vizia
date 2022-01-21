use vizia::*;

const STYLE: &str = r#"

"#;

#[derive(Default, Lens)]
pub struct AppData {
    value: bool,
}

impl Model for AppData {
    fn event(&mut self, _: &mut Context, event: &mut Event) {
        if let Some(app_event) = event.message.downcast() {
            match app_event {
                AppEvent::ToggleValue => {
                    self.value ^= true;
                }
            }
        }
    }
}

#[derive(Debug)]
pub enum AppEvent {
    ToggleValue,
}

fn main() {
    Application::new(WindowDescription::new().with_title("Test"), |cx| {
        cx.add_theme(STYLE);

        AppData::default().build(cx);
        HStack::new(cx, |cx| {
            Label::new(cx, "\u{e88a}");
        })
        .font_size(50.0)
        .font("material");
    })
    .run();
}

// .border_shape_top_left(BorderCornerShape::Bevel)
