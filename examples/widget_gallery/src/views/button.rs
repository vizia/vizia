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

        DemoRegion::new(
            cx,
            "Basic Button",
            |cx| {
                Button::new(cx, |cx| Label::new(cx, "Button"));
            }
        );

        DemoRegion::new(
            cx,
            "Button Variants",
            |cx| {
                Button::new(cx, |cx| Label::new(cx, "Primary"));
                Button::new(cx, |cx| Label::new(cx, "Secondary")).variant(ButtonVariant::Secondary);
                Button::new(cx, |cx| Label::new(cx, "Outline")).variant(ButtonVariant::Outline);
                Button::new(cx, |cx| Label::new(cx, "Text")).variant(ButtonVariant::Text);
            });

            
        DemoRegion::new(
            cx,
            "Button with Icon and Label",
            |cx| {
                Button::new(cx, |cx| {
                    HStack::new(cx, |cx| {
                        Svg::new(cx, ICON_TRASH);
                        Label::new(cx, "Delete");
                    })
                })
                .class("outline");

                Button::new(cx, |cx| {
                    HStack::new(cx, |cx| {
                        Label::new(cx, "Edit");
                        Svg::new(cx, ICON_PENCIL);
                    })
                })
                .class("accent");
            });

    }).class("panel");
}
