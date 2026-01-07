use vizia::prelude::*;

struct RichTextApp {
    text: Signal<&'static str>,
    pointer_auto: Signal<PointerEvents>,
    cursor_hand: Signal<CursorIcon>,
    hello: Signal<&'static str>,
    rich: Signal<&'static str>,
    text_ex: Signal<&'static str>,
}

impl App for RichTextApp {
    fn app_name() -> &'static str {
        "Rich Text"
    }

    fn new(cx: &mut Context) -> Self {
        Self {
            text: cx.state(""),
            pointer_auto: cx.state(PointerEvents::Auto),
            cursor_hand: cx.state(CursorIcon::Hand),
            hello: cx.state("Hello"),
            rich: cx.state(" Rich"),
            text_ex: cx.state(" Text!"),
        }
    }

    fn on_build(self, cx: &mut Context) -> Self {
        let text = self.text;
        let pointer_auto = self.pointer_auto;
        let cursor_hand = self.cursor_hand;
        let hello = self.hello;
        let rich = self.rich;
        let text_ex = self.text_ex;

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
        self
    }

    fn window_config(&self) -> WindowConfig {
        window(|app| app)
    }
}

fn main() -> Result<(), ApplicationError> {
    RichTextApp::run()
}
