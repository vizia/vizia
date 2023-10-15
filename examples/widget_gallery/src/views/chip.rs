use vizia::icons::ICON_CODE;
use vizia::prelude::*;

pub fn chip(cx: &mut Context) {
    VStack::new(cx, |cx| {
        Label::new(cx, "Chip").class("title");
        Label::new(cx, "A chip can be used to inform the user of the status of specific data.")
            .class("paragraph");

        Label::new(cx, "Chip").class("header");

        DemoRegion::new(
            cx,
            |cx| {
                Chip::new(cx, "Chip").background_color(Color::from("#ff004444"));
            },
            |cx| {
                Label::new(cx, r#"Chip::new(cx, "Chip");"#).class("code");
            },
        );
    })
    .class("panel");
}

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
        code: impl Fn(&mut Context),
    ) -> Handle<Self> {
        Self { open: false }.build(cx, |cx| {
            HStack::new(cx, |cx| {
                (content)(cx);
            })
            .class("region");
            // Element::new(cx).class("divider");
            HStack::new(cx, |cx| {
                (code)(cx);
            })
            .height(Auto)
            .display(DemoRegion::open);

            Button::new(cx, |ex| ex.emit(DemoRegionEvent::Toggle), |cx| Icon::new(cx, ICON_CODE))
                .space(Pixels(8.0))
                .left(Stretch(1.0))
                .position_type(PositionType::SelfDirected)
                .tooltip(|cx| {
                    Label::new(cx, "Toggle Dark/Light Mode");
                });
        })
    }
}

impl View for DemoRegion {
    fn element(&self) -> Option<&'static str> {
        Some("demo-region")
    }

    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|e, _| match e {
            DemoRegionEvent::Toggle => self.open ^= true,
        })
    }
}
