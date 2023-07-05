mod helpers;
use helpers::*;
use vizia::prelude::*;

const STYLE: &str = r#"
    .progress-layout {
        col-between: 10px;
    }

    .progress-with-label {
        col-between: 5px;
    }

    .horizontal-progress-layout {
        row-between: 10px;
    }

    .progress-label {
        width: 30px;
        top: 1s;
        bottom: 1s;
    }

    /* Styled progressbar */

    .styled-progress, .styled-progress .progressbar-bar {
        border-radius: 20%;
    }

    .styled-progress .progressbar-bar {
        background-image: linear-gradient(to right, #51afef, green);
    }
"#;

#[derive(Lens)]
pub struct AppData {
    progress: f32,
    color: Color,
}

#[derive(Debug)]
pub enum AppEvent {
    SetProgress(f32),
    ChangeColor(Color),
}

impl Model for AppData {
    fn event(&mut self, _: &mut EventContext, event: &mut Event) {
        event.map(|app_event, _| match app_event {
            AppEvent::SetProgress(value) => {
                self.progress = *value;
            }
            AppEvent::ChangeColor(c) => self.color = *c,
        });
    }
}

fn main() {
    Application::new(|cx: &mut Context| {
        AppData { progress: 0.5, color: "#51afef".into() }.build(cx);

        cx.add_stylesheet(STYLE).expect("Failed to add stylesheet");

        ExamplePage::vertical(cx, |cx| {
            HStack::new(cx, |cx| {
                // vertical progressbar
                ProgressBar::vertical(cx, AppData::progress).bar_color(AppData::color);
                VStack::new(cx, |cx| {
                    // horizontal progressbar without bar color lens
                    ProgressBar::horizontal(cx, AppData::progress);
                    // horizontal progressbar with a percentage lebel beside it
                    HStack::new(cx, |cx| {
                        ProgressBar::horizontal(cx, AppData::progress).bar_color(AppData::color);
                        Label::new(cx, AppData::progress.map(|v| format!("{:.0}%", v * 100.0)))
                            .class("progress-label");
                    })
                    .class("progress-with-label");
                    // a styled progressbar
                    ProgressBar::horizontal(cx, AppData::progress).class("styled-progress");
                })
                .class("horizontal-progress-layout");
            })
            .class("progress-layout");

            Slider::new(cx, AppData::progress).on_changing(|cx, val| {
                cx.emit(AppEvent::SetProgress(val));
            });

            HStack::new(cx, |cx| {
                Button::new(
                    cx,
                    |cx| cx.emit(AppEvent::ChangeColor(Color::red())),
                    |cx| Label::new(cx, "Red"),
                );
                Button::new(
                    cx,
                    |cx| cx.emit(AppEvent::ChangeColor(Color::green())),
                    |cx| Label::new(cx, "Green"),
                );
                Button::new(
                    cx,
                    |cx| cx.emit(AppEvent::ChangeColor(Color::blue())),
                    |cx| Label::new(cx, "Blue"),
                );
                Button::new(
                    cx,
                    |cx| cx.emit(AppEvent::ChangeColor(Color::yellow())),
                    |cx| Label::new(cx, "Yellow"),
                );
            })
            .child_space(Stretch(1.0))
            .col_between(Pixels(5.0));
        });
    })
    .title("ProgressBar")
    .inner_size((750, 550))
    .run();
}
