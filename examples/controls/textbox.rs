


use vizia::*;

fn main() {
    let window_description = WindowDescription::new().with_title("Textbox");
    Application::new(window_description, |cx|{
        Textbox::new(cx, "This is some text").space(Stretch(1.0)).width(Pixels(100.0));
        // Button::new(cx, |_|{}, |cx|{
        //     Label::new(cx, "This is some text").position_type(PositionType::SelfDirected)
        // }).size(Auto).background_color(Color::red()).space(Stretch(1.0));
    }).run();

}