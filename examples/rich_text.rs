use vizia::prelude::*;

fn main() -> Result<(), ApplicationError> {
    Application::new(|cx| {
        Label::rich(cx, "", |cx| {
            TextSpan::new(cx, "Hello", |_| {})
                .pointer_events(PointerEvents::Auto)
                .cursor(CursorIcon::Hand)
                .class("span");
            TextSpan::new(cx, " Rich", |_| {})
                .pointer_events(PointerEvents::Auto)
                .cursor(CursorIcon::Hand)
                .class("span");

            TextSpan::new(cx, " Text!", |_| {})
                .pointer_events(PointerEvents::Auto)
                .cursor(CursorIcon::Hand)
                .class("span");
        })
        .class("testy");
    })
    .run()
}
