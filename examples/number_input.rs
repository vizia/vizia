// TODO: Make this a built-in view

use vizia::prelude::*;

const STYLE: &str = r#"
textbox:invalid {
    border-color: #ff0000;
}

.validation_error_label {
    display: none;
    color: red;
}

.validation_error_label.validation_error {
    display: flex;
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
                .validate(|val| *val < 50)
                .on_submit(|cx, val, _| {
                    cx.emit(AppEvent::SetNumber(val));
                })
                .width(Pixels(200.0))
                .child_left(Pixels(5.0));

            // Label::new(cx, "Please enter a number less than 50")
            //     .class("validation_error_label")
            //     .toggle_class("validation_error", AppData::invalid);

            Label::new(cx, AppData::number)
                .width(Pixels(200.0))
                .height(Pixels(32.0))
                .child_top(Stretch(1.0))
                .child_bottom(Stretch(1.0))
                .child_left(Pixels(5.0));
        })
        .child_top(Stretch(1.0))
        .child_bottom(Stretch(1.0))
        .height(Auto)
        .space(Stretch(1.0))
        .col_between(Pixels(10.0));
    })
    .title("Number Input")
    .run();
}
