#[allow(unused_imports)]
use vizia::prelude::*;

fn main() {
    Application::new(|cx| {
        cx.emit(EnvironmentEvent::SetLocale("en-US".parse().unwrap()));

        HStack::new(cx, |cx| {
            Checkbox::new(cx, Environment::locale.map(|locale| *locale == "en-US"))
                .id("english")
                .role(Role::RadioButton)
                .on_toggle(|cx| cx.emit(EnvironmentEvent::SetLocale("en-US".parse().unwrap())));
            Label::new(cx, "English").describing("english").hidden(true);

            Checkbox::new(cx, Environment::locale.map(|locale| *locale == "fr"))
                .id("french")
                .role(Role::RadioButton)
                .on_toggle(|cx| cx.emit(EnvironmentEvent::SetLocale("fr".parse().unwrap())))
                .left(Pixels(10.0));
            Label::new(cx, "French").describing("french").hidden(true);
        })
        .role(Role::RadioGroup)
        .space(Pixels(10.0))
        .child_top(Stretch(1.0))
        .child_bottom(Stretch(1.0))
        .col_between(Pixels(5.0))
        .height(Auto);

        Binding::new(cx, Environment::locale, |cx, locale| {
            match locale.get(cx).to_string().as_ref() {
                "en-US" => {
                    Element::new(cx)
                        .background_color(Color::from("#006847"))
                        .role(Role::Alert)
                        .name("green")
                        .live(Live::Polite);
                }

                "fr" => {
                    Element::new(cx)
                        .background_color(Color::from("#004768"))
                        .role(Role::Alert)
                        .name("blue")
                        .live(Live::Polite);
                }

                _ => {}
            }
        });
    })
    .ignore_default_theme()
    .run();
}
