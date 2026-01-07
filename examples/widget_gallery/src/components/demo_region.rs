use vizia::icons::ICON_CODE;
use vizia::prelude::*;

pub struct DemoRegion {
    open: Signal<bool>,
}

impl DemoRegion {
    pub fn new<'a>(
        cx: &'a mut Context,
        content: impl Fn(&mut Context) + 'static,
        code: &'static str,
    ) -> Handle<'a, Self> {
        let code = code.to_string();
        let open = cx.state(false);
        let auto = cx.state(Auto);
        Self { open }
            .build(cx, move |cx| {
                HStack::new(cx, |cx| {
                    (content)(cx);
                })
                .class("region");
                Divider::horizontal(cx);
                HStack::new(cx, |cx| {
                    ToggleButton::new(cx, open, |cx| Svg::new(cx, ICON_CODE))
                        .on_press(move |ex| open.upd(ex, |state| *state = !*state))
                        .tooltip(|cx| {
                            Tooltip::new(cx, |cx| {
                                Label::new(cx, "Toggle Code");
                            })
                        });
                })
                .class("controls");
                // Element::new(cx).class("divider");
                HStack::new(cx, move |cx| {
                    ScrollView::new(cx, move |cx| {
                        let code_signal = cx.state(code);
                        Label::new(cx, code_signal).class("code");
                    })
                    .height(auto);
                })
                .class("code")
                .height(auto)
                .display(open);
            })
            .toggle_class("open", open)
    }
}

impl View for DemoRegion {
    fn element(&self) -> Option<&'static str> {
        Some("demo-region")
    }

    fn event(&mut self, _cx: &mut EventContext, _event: &mut Event) {}
}
