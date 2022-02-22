use vizia::*;

const STYLE: &str = r#"

"#;

pub struct BranchingView {
}

impl View for BranchingView { }

impl BranchingView {
    pub fn new(cx: &mut Context, depth: usize) -> Handle<'_, Self> {
        Self {}.build2(cx, move |cx| {
            if depth > 0 {
                Self::new(cx, depth - 1);
                Self::new(cx, depth - 1);
            }
        })
        //.border_color(Color::red()).border_width(Pixels(1.0))
    }
}

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
        Binding::new(cx, AppData::value, move |cx, _| {
            let time_start = std::time::Instant::now();
            BranchingView::new(cx, 15);
            let time_end = std::time::Instant::now();
            let duration = time_end - time_start;
            println!("Build took {}ms", duration.as_millis());
        });

        Button::new(cx, move |cx| {
            cx.emit(AppEvent::ToggleValue);
        }, move |cx| {
            Label::new(cx, "Rebuild")
        });
    })
    .run();
}

// .border_shape_top_left(BorderCornerShape::Bevel)
