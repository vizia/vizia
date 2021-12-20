use vizia::*;

fn main() {
    Application::new(WindowDescription::new().with_title("Localization"), |cx| {
        HStack::new(cx, |cx| {
            Label::new(cx, "hello-world");
            Button::new(
                cx,
                |cx| cx.enviroment.set_locale("fr"),
                |cx| {
                    Label::new(cx, "fr");
                },
            );
            Button::new(
                cx,
                |cx| cx.enviroment.set_locale("en-US"),
                |cx| {
                    Label::new(cx, "en-US");
                },
            );
        });
    })
    .run();
}
