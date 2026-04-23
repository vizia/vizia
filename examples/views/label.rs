mod helpers;
use helpers::*;
use vizia::prelude::*;

pub struct AppData {
    _text: Signal<String>,
    _value: Signal<f32>,
    checked: Signal<bool>,
}

#[derive(Debug)]
pub enum AppEvent {
    Toggle,
}

impl Model for AppData {
    fn event(&mut self, _cx: &mut EventContext, event: &mut Event) {
        event.map(|app_event, _| match app_event {
            AppEvent::Toggle => {
                self.checked.update(|checked| *checked ^= true);
            }
        });
    }
}

fn main() -> Result<(), ApplicationError> {
    Application::new(|cx| {
        let text = Signal::new(String::from("As well as model data which implements ToString:"));
        let value = Signal::new(std::f32::consts::PI);
        let checked = Signal::new(false);

        AppData { _text: text, _value: value, checked }.build(cx);

        ExamplePage::vertical(cx, |cx| {
            Label::new(cx, Localized::new("label-static-unicode"))
                .background_color(Color::gray())
                .padding(Pixels(20.0));

            Label::new(cx, text);

            Label::new(cx, value);

            Label::new(cx, Localized::new("label-wrap-enabled")).width(Pixels(200.0));

            Label::new(cx, Localized::new("label-wrap-disabled"))
                .width(Auto)
                .text_wrap(false)
                .font_slant(FontSlant::Italic);

            HStack::new(cx, |cx| {
                Checkbox::new(cx, checked)
                    .on_toggle(|cx| cx.emit(AppEvent::Toggle))
                    .id("checkbox_1");

                Label::new(cx, Localized::new("label-describing-trigger")).describing("checkbox_1");
            })
            .width(Auto)
            .height(Auto)
            .alignment(Alignment::Center)
            .horizontal_gap(Pixels(8.0));
        });
    })
    .title(Localized::new("view-title-label"))
    .run()
}
