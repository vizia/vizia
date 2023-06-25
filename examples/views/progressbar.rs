mod helpers;
use helpers::*;
use vizia::prelude::*;

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
        AppData { progress: 50.0, color: Color::cyan() }.build(cx);

        ExamplePage::vertical(cx, |cx| {
            ProgressBar::horizontal(cx, AppData::progress).bar_color(AppData::color);
            Slider::new(cx, AppData::progress).range(0.0..100.0).on_changing(|cx, val| {
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
    .inner_size((350, 250))
    .run();
}
