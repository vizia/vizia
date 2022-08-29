use vizia::prelude::*;

const COLORS: [Color; 3] = [Color::red(), Color::green(), Color::blue()];

#[allow(dead_code)]
const DARK_THEME: &str = "crates/vizia_core/resources/themes/dark_theme.css";
#[allow(dead_code)]
const LIGHT_THEME: &str = "crates/vizia_core/resources/themes/light_theme.css";

fn main() {
    Application::new(|cx| {
        cx.add_stylesheet(DARK_THEME).expect("Failed to find stylesheet");

        VStack::new(cx, |cx| {
            //

            HStack::new(cx, |cx| {
                for i in 0..3 {
                    Element::new(cx).size(Pixels(100.0)).background_color(COLORS[i]);
                }
            })
            .size(Auto)
            .row_between(Pixels(10.0))
            .space(Stretch(1.0));

            //
        })
        .class("main")
        .width(Units::Stretch(1.0))
        .height(Units::Stretch(1.0));
    })
    .ignore_default_theme()
    .title("HStack")
    .run();
}
