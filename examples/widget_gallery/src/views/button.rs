use vizia::{
    icons::{ICON_CHECK, ICON_PENCIL, ICON_TRASH},
    prelude::*,
};

pub fn button(cx: &mut Context) {
    VStack::new(cx, |cx| {
        Label::new(cx, "Button").class("title").on_press(|cx| cx.emit(TabEvent::SetSelected(1)));
        HStack::new(cx, |cx| {
            // Basic Button
            Button::new(cx, |cx| Label::new(cx, "Button"));
            // Accent Button
            Button::new(cx, |cx| Label::new(cx, "Accent Button")).class("accent");
            // Outline Button
            Button::new(cx, |cx| Label::new(cx, "Outline Button")).class("outline");
            // Ghost Button
            Button::new(cx, |cx| Label::new(cx, "Ghost Button")).class("ghost");
            // Button with Icon
            Button::new(cx, |cx| {
                HStack::new(cx, |cx| {
                    Icon::new(cx, ICON_CHECK);
                    Label::new(cx, "Icon and Label");
                })
            });
            // Icon Button
            Button::new(cx, |cx| Icon::new(cx, ICON_CHECK));
        })
        .height(Auto)
        .col_between(Pixels(8.0));
    })
    .class("panel");
}

pub fn button2(cx: &mut Context) {
    VStack::new(cx, |cx| {
        Label::new(cx, "Button").class("title");
        Label::new(cx, "A button can be used to send an event when pressed. Typically they are used to trigger an action.")
        .class("paragraph");

        Label::new(cx, r#"Button::new(cx, |cx|{...}, |cx| Label::new(cx, "Press Me"));"#).class("code");

        Label::new(cx, "Basic buttons").class("header");
        HStack::new(cx, |cx| {
            Button::new(cx, |cx| Label::new(cx, "Button"));
            Button::new(cx, |cx| Label::new(cx, "Accent Button")).class("accent");
            Button::new(cx, |cx| Label::new(cx, "Outline Button")).class("outline");
            Button::new(cx, |cx| Label::new(cx, "Ghost Button")).class("ghost");
        })
        .class("region");

    Label::new(cx, "Buttons with icons and label").class("header");
    Label::new(cx, "An HStack can be used to add an icon as well as a label to a button. The icon can be positioned before or after the label by changing the order of the declarations.")
    .class("paragraph");
        HStack::new(cx, |cx| {
            Button::new(
                cx,
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
                |cx| {
                    HStack::new(cx, |cx| {
                        Label::new(cx, "Edit");
                        Icon::new(cx, ICON_PENCIL);
                    })
                },
            )
            .class("accent");

            // Icon Button
            Button::new(cx, |cx| Icon::new(cx, ICON_CHECK));
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
