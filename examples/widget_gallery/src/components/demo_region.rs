use vizia::prelude::*;

pub struct DemoRegion;

impl DemoRegion {
    pub fn new<'a, T: ToStringLocalized + Clone + 'static>(
        cx: &'a mut Context,
        title: impl Res<T> + 'static + Clone,
        content: impl Fn(&mut Context) + 'static,
    ) -> Handle<'a, Self> {
        Self.build(cx, move |cx| {
            Label::new(cx, title).class("region-title");
            HStack::new(cx, |cx| {
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
