


use vizia::*;

fn main() {
    let window_description = WindowDescription::new().with_title("Textbox");
    Application::new(window_description, |cx|{
        HStack::new(cx, |cx|{
            Textbox::new(cx, "This is somé téxt")
                .width(Pixels(200.0))
                .child_left(Pixels(5.0));

            Textbox::new(cx, "Another textbox!")
                .width(Pixels(200.0))
                .child_left(Pixels(5.0));
        }).col_between(Pixels(10.0)).space(Stretch(1.0));

        // Button::new(cx, |_|{}, |cx|{
        //     Label::new(cx, "This is some text").position_type(PositionType::SelfDirected)
        // }).size(Auto).background_color(Color::red()).space(Stretch(1.0));
    }).run();

}