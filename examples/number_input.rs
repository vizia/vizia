// TODO: Make this a built-in view

use vizia::prelude::*;

const STYLE: &str = r#"
    textbox:invalid {
        border-color: #ff0000;
    }
"#;

#[derive(Lens)]
pub struct AppData {
    number: i32,
    invalid: bool,
}

#[derive(Debug)]
pub enum AppEvent {
    SetNumber(i32),
    SetInvalid,
}

impl Model for AppData {
    fn event(&mut self, _: &mut EventContext, event: &mut Event) {
        event.map(|app_event, _| match app_event {
            AppEvent::SetNumber(num) => {
                self.number = *num;
                self.invalid = false;
            }
            AppEvent::SetInvalid => {
                self.invalid = true;
            }
        });
    }
}

fn main() {
    Application::new(|cx| {
        cx.add_stylesheet(STYLE).expect("Failed to add stylesheet");
        AppData { number: 5, invalid: false }.build(cx);

        HStack::new(cx, |cx| {
            Textbox::new(cx, AppData::number)
                .validate(|text| text.parse::<f32>().is_ok())
                .on_submit(|cx, text, _| {
                    if let Ok(valid_number) = text.parse::<i32>() {
                        cx.emit(AppEvent::SetNumber(valid_number));
                    }
                })
                .width(Pixels(200.0))
                .child_left(Pixels(5.0));

            Label::new(cx, AppData::number)
                .width(Pixels(200.0))
                .height(Pixels(32.0))
                .child_top(Stretch(1.0))
                .child_bottom(Stretch(1.0))
                .background_color(Color::gray())
                .child_left(Pixels(5.0));
        })
        .background_color(Color::red())
        .space(Stretch(1.0))
        .child_space(Stretch(1.0))
        .col_between(Pixels(10.0));
    })
    .title("Number Input")
    .run();
}
