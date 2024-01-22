use vizia::{
    icons::{ICON_CHECK, ICON_PENCIL, ICON_TRASH},
    prelude::*,
};

use crate::{
    app_data::AppData,
    components::{DemoRegion, OneOf, OneOfModifiers, PageView},
};

pub fn button(cx: &mut Context) {
    VStack::new(cx, |cx|{

        Label::new(cx, "Button").class("title");
        Label::new(cx, "A button can be used to send an event when pressed. Typically they are used to trigger an action.")
            .class("paragraph");

        Label::new(cx, "Basic button").class("header");
        DemoRegion::new(
            cx,
            |cx| {
                Button::new(cx, |cx| Label::new(cx, "Button"));
            },
            |cx| {
                Label::new(cx, r#"Button::new(cx, |cx| Label::new(cx, "Button"));"#).class("code");
            },
        );

        Label::new(cx, "Button variants").class("header");
        DemoRegion::new(
            cx,
            |cx| {
                Button::new(cx, |cx| Label::new(cx, "Normal"));
                Button::new(cx, |cx| Label::new(cx, "Accent")).variant(ButtonVariant::Accent);
                Button::new(cx, |cx| Label::new(cx, "Outline")).variant(ButtonVariant::Outline);
                Button::new(cx, |cx| Label::new(cx, "Text")).variant(ButtonVariant::Text);
            },
            |cx| {
                Label::new(
                    cx,
                    r#"Button::new(cx, |cx| Label::new(cx, "Normal"));
Button::new(cx, |cx| Label::new(cx, "Accent"))
    .variant(ButtonVariant::Accent);
Button::new(cx, |cx| Label::new(cx, "Outline"))
    .variant(ButtonVariant::Outline);
Button::new(cx, |cx| Label::new(cx, "Text"))
    .variant(ButtonVariant::Text);"#,
                )
                .class("code");
            },
        );

        Label::new(cx, "Button with icon and label").class("header");
        Label::new(cx, "An HStack can be used to add an icon as well as a label to a button. The icon can be positioned before or after the label by changing the order of the declarations.")
            .class("paragraph");

        DemoRegion::new(
            cx,
            |cx| {
                Button::new(cx, |cx| {
                    HStack::new(cx, |cx| {
                        Icon::new(cx, ICON_TRASH);
                        Label::new(cx, "Delete");
                    })
                })
                .class("outline");

                Button::new(cx, |cx| {
                    HStack::new(cx, |cx| {
                        Label::new(cx, "Edit");
                        Icon::new(cx, ICON_PENCIL);
                    })
                })
                .class("accent");
            },
            |cx| {
                Label::new(
                    cx,
                    r#"Button::new(cx, |cx| {
    HStack::new(cx, |cx| {
        Icon::new(cx, ICON_TRASH);
        Label::new(cx, "Delete");
    })
})
.class("outline");

Button::new(cx, |cx| {
    HStack::new(cx, |cx| {
        Label::new(cx, "Edit");
        Icon::new(cx, ICON_PENCIL);
    })
})
.class("accent");"#,
                )
                .class("code");
            },
        );

        Label::new(cx, "Handling clicks").class("header");
        HStack::new(
            cx,
            |cx|{
            Label::new(cx, r#"Button::new(cx, |cx| Label::new(cx, "Button"))
    .on_press(|cx| {...});"#).class("code");    
        }).height(Auto).class("code");

    }).class("panel");
}

#[derive(Lens)]
pub struct Collapse {
    is_collapsed: bool,
}

impl Collapse {
    pub fn new<H: 'static + Fn(&mut Context), F: 'static + Fn(&mut Context)>(
        cx: &mut Context,
        header: H,
        content: F,
    ) -> Handle<Self> {
        Self { is_collapsed: false }
            .build(cx, |cx| {
                (header)(cx);
                Binding::new(cx, Collapse::is_collapsed, move |cx, is_collapsed| {
                    if !is_collapsed.get(cx) {
                        (content)(cx);
                    }
                });
            })
            .height(Auto)
    }
}

impl View for Collapse {
    fn element(&self) -> Option<&'static str> {
        Some("collapse")
    }
}
