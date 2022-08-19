#[allow(unused_imports)]
use vizia::prelude::*;

#[cfg(not(feature = "localization"))]
fn main() {
    panic!("This example requires the 'localization' feature!");
}

#[cfg(feature = "localization")]
fn main() {
    Application::new(|cx| {
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
    .run();
}
