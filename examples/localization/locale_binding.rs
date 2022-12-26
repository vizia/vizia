use vizia::prelude::*;

#[allow(dead_code)]
const DARK_THEME: &str = "crates/vizia_core/resources/themes/dark_theme.css";
#[allow(dead_code)]
const LIGHT_THEME: &str = "crates/vizia_core/resources/themes/light_theme.css";

fn main() {
    Application::new(|cx| {
        cx.add_stylesheet(DARK_THEME).expect("Failed to find stylesheet");

        cx.emit(EnvironmentEvent::SetLocale("en-US".parse().unwrap()));

        HStack::new(cx, |cx| {
            Checkbox::new(cx, Environment::locale.map(|locale| locale.to_string() == "en-US"))
                .on_toggle(|cx| cx.emit(EnvironmentEvent::SetLocale("en-US".parse().unwrap())));
            Label::new(cx, "English");

            Checkbox::new(cx, Environment::locale.map(|locale| locale.to_string() == "fr"))
                .on_toggle(|cx| cx.emit(EnvironmentEvent::SetLocale("fr".parse().unwrap())))
                .left(Pixels(10.0));
            Label::new(cx, "French");
        })
        .space(Pixels(10.0))
        .child_top(Stretch(1.0))
        .child_bottom(Stretch(1.0))
        .col_between(Pixels(5.0))
        .height(Auto);

        Binding::new(cx, Environment::locale, |cx, locale| {
            match locale.get(cx).to_string().as_ref() {
                "en-US" => {
                    Element::new(cx).background_color(Color::from("#006847"));
                }

                "fr" => {
                    Element::new(cx).background_color(Color::from("#004768"));
                }

                _ => {}
            }
        });
    })
    .ignore_default_theme()
    .run();
}
