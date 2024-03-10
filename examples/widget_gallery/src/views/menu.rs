use vizia::{
    icons::{ICON_CLOCK, ICON_COLUMN_INSERT_LEFT, ICON_USER},
    image,
    prelude::*,
};

use crate::components::DemoRegion;

pub fn menu(cx: &mut Context) {
    VStack::new(cx, |cx| {
        Label::new(cx, "Menu").class("title");
        Label::new(cx, "").class("paragraph");

        // Divider here
        Element::new(cx)
            .height(Pixels(1.0))
            .background_color(Color::rgb(210, 210, 210))
            .top(Pixels(12.0))
            .bottom(Pixels(12.0));

        Label::new(cx, "Menu").class("header");
        DemoRegion::new(
            cx,
            |cx| {
                // Menu::new(cx, |cx| {
                //     MenuButton::new(
                //         cx,
                //         |_| println!("New"),
                //         |cx| {
                //             HStack::new(cx, |cx| {
                //                 Label::new(cx, "New");
                //                 Label::new(cx, &format!("Ctrl + N")).class("shortcut");
                //             })
                //         },
                //     );
                //     MenuButton::new(
                //         cx,
                //         |_| println!("Open"),
                //         |cx| {
                //             HStack::new(cx, |cx| {
                //                 Label::new(cx, "Open");
                //                 Label::new(cx, &format!("Ctrl + O")).class("shortcut");
                //             })
                //         },
                //     );
                //     Submenu::new(
                //         cx,
                //         |cx| Label::new(cx, "Open Recent"),
                //         |cx| {
                //             MenuButton::new(
                //                 cx,
                //                 |_| println!("Doc 1"),
                //                 |cx| Label::new(cx, "Doc 1"),
                //             );
                //             Submenu::new(
                //                 cx,
                //                 |cx| Label::new(cx, "Doc 2"),
                //                 |cx| {
                //                     MenuButton::new(
                //                         cx,
                //                         |_| println!("Version 1"),
                //                         |cx| Label::new(cx, "Version 1"),
                //                     );
                //                     MenuButton::new(
                //                         cx,
                //                         |_| println!("Version 2"),
                //                         |cx| Label::new(cx, "Version 2"),
                //                     );
                //                     MenuButton::new(
                //                         cx,
                //                         |_| println!("Version 3"),
                //                         |cx| Label::new(cx, "Version 3"),
                //                     );
                //                 },
                //             );
                //             MenuButton::new(
                //                 cx,
                //                 |_| println!("Doc 3"),
                //                 |cx| Label::new(cx, "Doc 3"),
                //             );
                //         },
                //     );
                //     MenuDivider::new(cx);
                //     MenuButton::new(cx, |_| println!("Save"), |cx| Label::new(cx, "Save"));
                //     MenuButton::new(cx, |_| println!("Save As"), |cx| Label::new(cx, "Save As"));
                //     MenuDivider::new(cx);
                //     MenuButton::new(cx, |_| println!("Quit"), |cx| Label::new(cx, "Quit"));
                // });
            },
            r#"Avatar::new(cx, |cx|{
    Icon::new(cx, ICON_USER)
})"#,
        );
    })
    .class("panel");
}
