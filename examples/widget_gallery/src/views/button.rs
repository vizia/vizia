use vizia::{
    icons::{ICON_PENCIL, ICON_TRASH},
    prelude::*,
};

use crate::components::DemoRegion;

pub fn button(cx: &mut Context) {
    VStack::new(cx, |cx|{

        Label::new(cx, "Button").class("title");
        Label::new(cx, "A button can be used to send an event when pressed. Typically they are used to trigger an action.")
            .class("paragraph");

        Divider::new(cx).top(Pixels(12.0)).bottom(Pixels(12.0));

        Label::new(cx, "Basic button").class("header");
        DemoRegion::new(
            cx,
            |cx| {
                Button::new(cx, |cx| Label::new(cx, "Button"));
            }, r#"Button::new(cx, |cx| Label::new(cx, "Button"));"#
        );

        Label::new(cx, "Button variants").class("header");
        DemoRegion::new(
            cx,
            |cx| {
                Button::new(cx, |cx| Label::new(cx, "Normal"));
                Button::new(cx, |cx| Label::new(cx, "Accent")).variant(ButtonVariant::Accent);
                Button::new(cx, |cx| Label::new(cx, "Outline")).variant(ButtonVariant::Outline);
                Button::new(cx, |cx| Label::new(cx, "Text")).variant(ButtonVariant::Text);
            }, r#"Button::new(cx, |cx| Label::new(cx, "Normal"));
Button::new(cx, |cx| Label::new(cx, "Accent"))
    .variant(ButtonVariant::Accent);
Button::new(cx, |cx| Label::new(cx, "Outline"))
    .variant(ButtonVariant::Outline);
Button::new(cx, |cx| Label::new(cx, "Text"))
    .variant(ButtonVariant::Text);"#
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
            }, r#"Button::new(cx, |cx| {
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
.class("accent");"#
        );

    }).class("panel");
}
