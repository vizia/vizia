use vizia::prelude::*;

fn main() -> Result<(), ApplicationError> {
    Application::new(|cx| {
        let text = cx.state("");
        let pointer_auto = cx.state(PointerEvents::Auto);
        let cursor_hand = cx.state(CursorIcon::Hand);
        let hello = cx.state("Hello");
        let rich = cx.state(" Rich");
        let text_ex = cx.state(" Text!");
        Label::rich(cx, text, |cx| {
            TextSpan::new(cx, hello, |_| {})
                .pointer_events(pointer_auto)
                .cursor(cursor_hand)
                .class("span");
            TextSpan::new(cx, rich, |_| {})
                .pointer_events(pointer_auto)
                .cursor(cursor_hand)
                .class("span");

            TextSpan::new(cx, text_ex, |_| {})
                .pointer_events(pointer_auto)
                .cursor(cursor_hand)
                .class("span");
        })
        .class("testy");
    })
    .run()
}
