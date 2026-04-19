use vizia::prelude::*;

pub struct DemoRegion;

impl DemoRegion {
    pub fn new<'a>(
        cx: &'a mut Context,
        title: impl Into<String>,
        content: impl Fn(&mut Context) + 'static,
    ) -> Handle<'a, Self> {
        Self.build(cx, move |cx| {
            Label::new(cx, title.into()).class("region-title");
            HStack::new(cx, |cx| {
                (content)(cx);
            })
            .class("region");
        })
    }

    pub fn new_vertical<'a>(
        cx: &'a mut Context,
        title: impl Into<String>,
        content: impl Fn(&mut Context) + 'static,
    ) -> Handle<'a, Self> {
        Self.build(cx, move |cx| {
            Label::new(cx, title.into()).class("region-title");
            VStack::new(cx, |cx| {
                (content)(cx);
            })
            .class("region");
        })
    }
}

impl View for DemoRegion {
    fn element(&self) -> Option<&'static str> {
        Some("demo-region")
    }
}
