mod helpers;
use helpers::*;
use vizia::prelude::*;

fn main() -> Result<(), ApplicationError> {
    let (app, title) = Application::new_with_state(|cx| {
        let text = cx.state(String::from("As well as model data which implements ToString:"));
        let value = cx.state(std::f32::consts::PI);
        let checked = cx.state(false);
        let gray = cx.state(Color::gray());
        let padding_20 = cx.state(Pixels(20.0));
        let width_200 = cx.state(Pixels(200.0));
        let wrap_true = cx.state(true);
        let wrap_false = cx.state(false);
        let italic = cx.state(FontSlant::Italic);
        let top_2 = cx.state(Units::Pixels(2.0));
        let bottom_2 = cx.state(Units::Pixels(2.0));
        let auto = cx.state(Auto);
        let align_center = cx.state(Alignment::Center);
        let gap_8 = cx.state(Pixels(8.0));

        ExamplePage::vertical(cx, |cx| {
            Label::static_text(cx, "A label can display a static string of unicode 😂")
                .background_color(gray)
                .padding(padding_20);

            Label::new(cx, text);

            Label::new(cx, value);

            Label::static_text(cx, "Text which is too long for the label will be wrapped.")
                .text_wrap(wrap_true)
                .width(width_200);

            Label::static_text(cx, "Unless text wrapping is disabled.")
                .width(width_200)
                .text_wrap(wrap_false)
                .font_slant(italic);

            HStack::new(cx, |cx| {
                Checkbox::new(cx, checked)
                    .two_way()
                    .id("checkbox_1")
                    .top(top_2)
                    .bottom(bottom_2);

                Label::static_text(
                    cx,
                    "A label that is describing a form element also acts as a trigger",
                )
                .describing("checkbox_1");
            })
            .width(auto)
            .height(auto)
            .alignment(align_center)
            .horizontal_gap(gap_8);
        });
        cx.state("Label")
    });

    app.title(title).run()
}
