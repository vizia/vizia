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
        title: impl Into<String>,
        content: impl Fn(&mut Context) + 'static,
    ) -> Handle<'a, Self> {
        let open = Signal::new(false);

        Self { open }
            .build(cx, move |cx| {
                Label::new(cx, title.into()).class("region-title");
                HStack::new(cx, |cx| {
                    (content)(cx);
                })
                .class("region");
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
