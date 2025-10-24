mod helpers;
use helpers::*;
use vizia::prelude::*;

pub struct AppData {
    checked: Signal<bool>,
}
#[derive(Debug)]
pub enum AppEvent {
    Toggle,
}
impl Model for AppData {
    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|app_event, _| match app_event {
            AppEvent::Toggle => {
                self.checked.update(cx, |checked| *checked = !*checked);
            }
        });
    }
}
fn main() -> Result<(), ApplicationError> {
    Application::new(|cx| {
        let text = cx.state(String::from("As well as model data which implements ToString:"));
        let value = cx.state(std::f32::consts::PI);
        let checked = cx.state(false);

        AppData { checked }.build(cx);

        ExamplePage::vertical(cx, |cx| {
            Label::new(cx, "A label can display a static string of unicode 😂")
                .background_color(Color::gray())
                .padding(Pixels(20.0));

            Label::new(cx, text);

            Label::new(cx, value);

            Label::new(cx, "Text which is too long for the label will be wrapped.")
                .text_wrap(true)
                .width(Pixels(200.0));

            Label::new(cx, "Unless text wrapping is disabled.")
                .width(Pixels(200.0))
                .text_wrap(false)
                .font_slant(FontSlant::Italic);

            HStack::new(cx, |cx| {
                Checkbox::new(cx, checked)
                    .on_toggle(|cx| cx.emit(AppEvent::Toggle))
                    .id("checkbox_1")
                    .top(Units::Pixels(2.0))
                    .bottom(Units::Pixels(2.0));

                Label::new(cx, "A label that is describing a form element also acts as a trigger")
                    .describing("checkbox_1");
            })
            .width(Auto)
            .height(Auto)
            .alignment(Alignment::Center)
            .horizontal_gap(Pixels(8.0));
        });
    })
    .title("Label")
    .run()
}
