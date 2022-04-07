use vizia::*;

const ICON_PLUS: &str = "\u{2b}";

fn no_action(_: &mut Context) {}

fn main() {
    let window_description =
        WindowDescription::new().with_title("Button").with_inner_size(1280, 720);

    Application::new(window_description, |cx| {
        //cx.add_stylesheet("examples/test_style.css").unwrap();

        HStack::new(cx, |cx| {
            // Button
            Button::new(cx, no_action, |cx| Label::new(cx, "Button"));
            // Filled button
            Button::new(cx, no_action, |cx| Label::new(cx, "Another Button")).class("accent");
            // Button with icon
            Button::new(cx, no_action, |cx| {
                HStack::new(cx, |cx| {
                    Label::new(cx, ICON_PLUS).class("icon");
                    Label::new(cx, "Button");
                })
                .size(Auto)
                .child_space(Stretch(1.0))
                .col_between(Pixels(2.0))
            });
        })
        .child_space(Stretch(1.0))
        .col_between(Pixels(10.0));
    })
    .run();
}
