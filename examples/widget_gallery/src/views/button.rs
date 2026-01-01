use vizia::{
    icons::{ICON_PENCIL, ICON_TRASH},
    prelude::*,
};

use crate::components::DemoRegion;

pub fn button(cx: &mut Context) {
    VStack::new(cx, |cx|{

        Markdown::new(cx, "# Button
A button can be used to send an event when pressed. Typically they are used to trigger an action.
        ");

        Divider::new(cx);

        Markdown::new(cx, "### Basic button");

        DemoRegion::new(
            cx,
            |cx| {
                Button::new(cx, |cx| Label::static_text(cx, "Button"));
            },
            r#"Button::new(cx, |cx| Label::static_text(cx, "Button"));"#
        );

        Markdown::new(cx, "### Button variants");

        DemoRegion::new(
            cx,
            |cx| {
                let accent = cx.state(ButtonVariant::Accent);
                let outline = cx.state(ButtonVariant::Outline);
                let text = cx.state(ButtonVariant::Text);
                Button::new(cx, |cx| Label::static_text(cx, "Normal"));
                Button::new(cx, |cx| Label::static_text(cx, "Accent")).variant(accent);
                Button::new(cx, |cx| Label::static_text(cx, "Outline")).variant(outline);
                Button::new(cx, |cx| Label::static_text(cx, "Text")).variant(text);
            },
            r#"let accent = cx.state(ButtonVariant::Accent);
let outline = cx.state(ButtonVariant::Outline);
let text = cx.state(ButtonVariant::Text);

Button::new(cx, |cx| Label::static_text(cx, "Normal"));
Button::new(cx, |cx| Label::static_text(cx, "Accent")).variant(accent);
Button::new(cx, |cx| Label::static_text(cx, "Outline")).variant(outline);
Button::new(cx, |cx| Label::static_text(cx, "Text")).variant(text);"#
        );

        Markdown::new(cx, "### Button with icon and label
An HStack can be used to add an icon as well as a label to a button. The icon can be positioned before or after the label by changing the order of the declarations.");

        DemoRegion::new(
            cx,
            |cx| {
                Button::new(cx, |cx| {
                    HStack::new(cx, |cx| {
                        Svg::new(cx, ICON_TRASH);
                        Label::static_text(cx, "Delete");
                    })
                })
                .class("outline");

                Button::new(cx, |cx| {
                    HStack::new(cx, |cx| {
                        Label::static_text(cx, "Edit");
                        Svg::new(cx, ICON_PENCIL);
                    })
                })
                .class("accent");
            },
            r#"Button::new(cx, |cx| {
    HStack::new(cx, |cx| {
        Svg::new(cx, ICON_TRASH);
        Label::static_text(cx, "Delete");
    })
})
.class("outline");

Button::new(cx, |cx| {
    HStack::new(cx, |cx| {
        Label::static_text(cx, "Edit");
        Svg::new(cx, ICON_PENCIL);
    })
})
.class("accent");"#
        );

    }).class("panel");
}
