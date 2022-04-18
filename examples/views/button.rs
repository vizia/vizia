use vizia::*;

const ICON_PLUS: &str = "\u{2b}";

fn no_action(_: &mut Context) {}

fn main() {
    let window_description =
        WindowDescription::new().with_title("Button");

    Application::new(window_description, |cx| {

        Label::new(cx, "A button triggers an action when pressed and contains a view which describes its function, e.g. a Label.")
            .width(Stretch(1.0))
            .position_type(PositionType::SelfDirected)
            .space(Pixels(10.0));

        HStack::new(cx, |cx| {
            // Basic Button
            Button::new(cx, no_action, |cx| Label::new(cx, "Button"));
            // Accent button
            Button::new(cx, no_action, |cx| Label::new(cx, "Another Button")).class("accent");
            // Button with Icon
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
