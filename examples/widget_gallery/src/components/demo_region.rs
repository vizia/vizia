use vizia::icons::ICON_CODE;
use vizia::prelude::*;

#[derive(Lens)]
pub struct DemoRegion {
    open: bool,
}

pub enum DemoRegionEvent {
    Toggle,
}

impl DemoRegion {
    pub fn new(
        cx: &mut Context,
        content: impl Fn(&mut Context),
        code: impl Fn(&mut Context) + 'static,
    ) -> Handle<Self> {
        Self { open: true }
            .build(cx, |cx| {
                HStack::new(cx, |cx| {
                    (content)(cx);
                })
                .class("region");
                Divider::horizontal(cx);
                HStack::new(cx, |cx| {
                    ToggleButton::new(cx, DemoRegion::open, |cx| Icon::new(cx, ICON_CODE))
                        .on_press(|ex| ex.emit(DemoRegionEvent::Toggle))
                        .space(Pixels(8.0))
                        .left(Stretch(1.0))
                        .tooltip(|cx| {
                            Tooltip::new(cx, |cx| {
                                Label::new(cx, "Toggle Code");
                            })
                        });
                })
                .class("controls");
                // Element::new(cx).class("divider");
                HStack::new(cx, |cx| {
                    ScrollView::new(cx, 0.0, 0.0, true, true, move |cx| {
                        (code)(cx);
                    })
                    .height(Auto);
                })
                .class("code")
                .height(Auto)
                .display(DemoRegion::open);
            })
            .toggle_class("open", DemoRegion::open)
    }
}

impl View for DemoRegion {
    fn element(&self) -> Option<&'static str> {
        Some("demo-region")
    }

    fn event(&mut self, _cx: &mut EventContext, event: &mut Event) {
        event.map(|e, _| match e {
            DemoRegionEvent::Toggle => self.open ^= true,
        })
    }
}
