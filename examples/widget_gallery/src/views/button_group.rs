use vizia::icons::{ICON_CHECK, ICON_CODE, ICON_PENCIL, ICON_TRASH};
use vizia::prelude::*;

pub fn button_group(cx: &mut Context) {
    VStack::new(cx, |cx| {
        Label::new(cx, "Button Group").class("title");
        Label::new(cx, "Buttons can be grouped by wrapping them in a ButtonGroup view.")
        .class("paragraph");


        Label::new(cx, "Button Group").class("header");
        DemoRegion::new(cx, |cx|{
            ButtonGroup::new(cx, |cx|{
                Button::new(cx, |_| {}, |cx| Label::new(cx, "One"));
                Button::new(cx, |_| {}, |cx| Label::new(cx, "Two"));
                Button::new(cx, |_| {}, |cx| Label::new(cx, "Three"));
            });
        }, |cx|{
            Label::new(cx, r#"ButtonGroup::new(cx, |cx|{
    Button::new(cx, |_| {}, |cx| Label::new(cx, "One"));
    Button::new(cx, |_| {}, |cx| Label::new(cx, "Two"));
    Button::new(cx, |_| {}, |cx| Label::new(cx, "Three"));
});"#).class("code");
        });

        Label::new(cx, "Button Group Vertical").class("header");
        DemoRegion::new(cx, |cx|{
            ButtonGroup::new(cx, |cx|{
                Button::new(cx, |_| {}, |cx| Label::new(cx, "One"));
                Button::new(cx, |_| {}, |cx| Label::new(cx, "Two"));
                Button::new(cx, |_| {}, |cx| Label::new(cx, "Three"));
            }).vertical();
        }, |cx|{
            Label::new(cx, r#"ButtonGroup::new(cx, |cx|{
    Button::new(cx, |_| {}, |cx| Label::new(cx, "One"));
    Button::new(cx, |_| {}, |cx| Label::new(cx, "Two"));
    Button::new(cx, |_| {}, |cx| Label::new(cx, "Three"));
});"#).class("code");
        });


    Label::new(cx, "Buttons with icons and label").class("header");
    Label::new(cx, "An HStack can be used to add an icon as well as a label to a button. The icon can be positioned before or after the label by changing the order of the declarations.")
    .class("paragraph");
        HStack::new(cx, |cx| {
            Button::new(
                cx,
                |_| {},
                |cx| {
                    HStack::new(cx, |cx| {
                        Icon::new(cx, ICON_TRASH);
                        Label::new(cx, "Delete");
                    })
                },
            )
            .class("outline");

            Button::new(
                cx,
                |_| {},
                |cx| {
                    HStack::new(cx, |cx| {
                        Label::new(cx, "Edit");
                        Icon::new(cx, ICON_PENCIL);
                    })
                },
            )
            .class("accent");

            // Icon Button
            Button::new(cx, |_| {}, |cx| Icon::new(cx, ICON_CHECK));
        })
        .class("region");

        Label::new(cx, r#"Button::new(
    cx,
    |_| {},
    |cx| {
        HStack::new(cx, |cx| {
            Label::new(cx, "Edit");
            Icon::new(cx, ICON_PENCIL);
        })
    },
)
.class("accent");"#).class("code");
        Label::new(cx, "Icon Buttons").class("header");
        // TODO
        // HStack::new(cx, |cx| {
        //     IconButton::new(cx, |_| {}, ICON_TRASH);
        // })
        // .height(Auto)
        // .col_between(Pixels(8.0));
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
