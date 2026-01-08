mod helpers;
use helpers::*;
use vizia::prelude::*;

struct LabelApp {
    checked: Signal<bool>,
}

impl App for LabelApp {
    fn new(cx: &mut Context) -> Self {
        Self {
            checked: cx.state(false),
        }
    }

    fn on_build(self, cx: &mut Context) -> Self {
        let checked = self.checked;

        ExamplePage::vertical(cx, |cx| {
            Label::new(cx, "A label can display a static string of unicode 😂")
                .background_color(Color::gray())
                .padding(Pixels(20.0));

            Label::new(cx, "As well as model data which implements ToString:");

            Label::new(cx, std::f32::consts::PI);

            Label::new(cx, "Text which is too long for the label will be wrapped.")
                .text_wrap(true)
                .width(Pixels(200.0));

            Label::new(cx, "Unless text wrapping is disabled.")
                .width(Pixels(200.0))
                .text_wrap(false)
                .font_slant(FontSlant::Italic);

            HStack::new(cx, |cx| {
                Checkbox::new(cx, checked).two_way().id("checkbox_1").top(Units::Pixels(2.0)).bottom(Units::Pixels(2.0));

                Label::new(cx, "A label that is describing a form element also acts as a trigger")
                    .describing("checkbox_1");
            })
            .width(Auto)
            .height(Auto)
            .alignment(Alignment::Center)
            .horizontal_gap(Pixels(8.0));
        });
        self
    }
}

fn main() -> Result<(), ApplicationError> {
    LabelApp::run()
}
