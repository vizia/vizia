use vizia::icons::ICON_CODE;
use vizia::prelude::*;

pub struct DemoRegion {
    open: Signal<bool>,
}

pub enum DemoRegionEvent {
    Toggle,
}

impl DemoRegion {
    pub fn new<'a>(
        cx: &'a mut Context,
        content: impl Fn(&mut Context) + 'static,
        code: &'static str,
    ) -> Handle<'a, Self> {
        let code = code.to_string();
        let open = Signal::new(false);

        Self { open }
            .build(cx, move |cx| {
                HStack::new(cx, |cx| {
                    (content)(cx);
                })
                .class("region");
                Divider::horizontal(cx);
                HStack::new(cx, |cx| {
                    ToggleButton::new(cx, open, |cx| Svg::new(cx, ICON_CODE))
                        .on_press(|ex| ex.emit(DemoRegionEvent::Toggle))
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
                        Label::new(cx, code).class("code");
                    })
                    .height(Auto);
                })
                .class("code")
                .height(Auto)
                .display(open);
            })
            .toggle_class("open", open)
    }
}

impl View for DemoRegion {
    fn element(&self) -> Option<&'static str> {
        Some("demo-region")
    }

    fn event(&mut self, _cx: &mut EventContext, event: &mut Event) {
        event.map(|e, _| match e {
            DemoRegionEvent::Toggle => self.open.update(|open| *open ^= true),
        })
    }
}
