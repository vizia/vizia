use vizia::prelude::*;

#[derive(Lens)]
pub struct AppData {
    text: String,
    value: f32,
    checked: bool,
}

#[derive(Debug)]
pub enum AppEvent {
    Toggle,
}

impl Model for AppData {
    fn event(&mut self, _cx: &mut EventContext, event: &mut Event) {
        event.map(|app_event, _| match app_event {
            AppEvent::Toggle => {
                self.checked ^= true;
            }
        });
    }
}

fn main() {
    Application::new(|cx| {
        AppData {
            text: String::from("As well as model data which implements ToString:"),
            value: 3.141592,
            checked: false,
        }
        .build(cx);

        VStack::new(cx, |cx| {
            Label::new(cx, "A label can display a static string of unicode 😂");

            Label::new(cx, AppData::text);

            Label::new(cx, AppData::value);

            Label::new(cx, "Text which is too long for the label will be wrapped.")
                .text_wrap(true)
                .width(Pixels(200.0));

            Label::new(cx, "Unless text wrapping is disabled.")
                .width(Pixels(200.0))
                .text_wrap(false)
                .font_style(FontStyle::Italic);

            HStack::new(cx, |cx| {
                Checkbox::new(cx, AppData::checked)
                    .on_toggle(|cx| cx.emit(AppEvent::Toggle))
                    .id("checkbox_1");

                Label::new(cx, "A label that is describing a form element also acts as a trigger")
                    .describing("checkbox_1");
            })
            .width(Auto)
            .col_between(Pixels(5.0));
        })
        .size(Pixels(600.0))
        .width(Stretch(1.0))
        .height(Stretch(1.0))
        .child_space(Stretch(1.0))
        .row_between(Pixels(20.0));
    })
    .title("Label")
    .run();
}
