use log::debug;
use vizia::prelude::*;

use crate::components::DemoRegion;

pub fn menu(cx: &mut Context) {
    VStack::new(cx, |cx| {
        Markdown::new(cx, "# Menu");

        Divider::new(cx);

        Markdown::new(cx, "### Basic menu");

        DemoRegion::new(
            cx,
            |cx| {
                Submenu::new(
                    cx,
                    |cx| Label::new(cx, "Menu"),
                    |cx| {
                        MenuButton::new(
                            cx,
                            |_| debug!("New"),
                            |cx| {
                                HStack::new(cx, |cx| {
                                    Label::new(cx, "New");
                                    Label::new(cx, "Ctrl + N").class("shortcut");
                                })
                            },
                        );
                        MenuButton::new(
                            cx,
                            |_| debug!("Open"),
                            |cx| {
                                HStack::new(cx, |cx| {
                                    Label::new(cx, "Open");
                                    Label::new(cx, "Ctrl + O").class("shortcut");
                                })
                            },
                        );
                        Submenu::new(
                            cx,
                            |cx| Label::new(cx, "Open Recent"),
                            |cx| {
                                MenuButton::new(
                                    cx,
                                    |_| debug!("Doc 1"),
                                    |cx| Label::new(cx, "Doc 1"),
                                );
                                Submenu::new(
                                    cx,
                                    |cx| Label::new(cx, "Doc 2"),
                                    |cx| {
                                        MenuButton::new(
                                            cx,
                                            |_| debug!("Version 1"),
                                            |cx| Label::new(cx, "Version 1"),
                                        );
                                        MenuButton::new(
                                            cx,
                                            |_| debug!("Version 2"),
                                            |cx| Label::new(cx, "Version 2"),
                                        );
                                        MenuButton::new(
                                            cx,
                                            |_| debug!("Version 3"),
                                            |cx| Label::new(cx, "Version 3"),
                                        );
                                    },
                                );
                                MenuButton::new(
                                    cx,
                                    |_| debug!("Doc 3"),
                                    |cx| Label::new(cx, "Doc 3"),
                                );
                            },
                        );
                        Divider::new(cx);
                        MenuButton::new(cx, |_| debug!("Save"), |cx| Label::new(cx, "Save"));
                        MenuButton::new(cx, |_| debug!("Save As"), |cx| Label::new(cx, "Save As"));
                        Divider::new(cx);
                        MenuButton::new(cx, |_| debug!("Quit"), |cx| Label::new(cx, "Quit"));
                    },
                )
                .width(Pixels(100.0));
            },
            r#"Submenu::new(
    cx,
    |cx| Label::new(cx, "Menu"),
    |cx| {
        MenuButton::new(
            cx,
            |_| debug!("New"),
            |cx| {
                HStack::new(cx, |cx| {
                    Label::new(cx, "New");
                    Label::new(cx, &format!("Ctrl + N")).class("shortcut");
                })
            },
        );
        MenuButton::new(
            cx,
            |_| debug!("Open"),
            |cx| {
                HStack::new(cx, |cx| {
                    Label::new(cx, "Open");
                    Label::new(cx, &format!("Ctrl + O")).class("shortcut");
                })
            },
        );
        Submenu::new(
            cx,
            |cx| Label::new(cx, "Open Recent"),
            |cx| {
                MenuButton::new(
                    cx,
                    |_| debug!("Doc 1"),
                    |cx| Label::new(cx, "Doc 1"),
                );
                Submenu::new(
                    cx,
                    |cx| Label::new(cx, "Doc 2"),
                    |cx| {
                        MenuButton::new(
                            cx,
                            |_| debug!("Version 1"),
                            |cx| Label::new(cx, "Version 1"),
                        );
                        MenuButton::new(
                            cx,
                            |_| debug!("Version 2"),
                            |cx| Label::new(cx, "Version 2"),
                        );
                        MenuButton::new(
                            cx,
                            |_| debug!("Version 3"),
                            |cx| Label::new(cx, "Version 3"),
                        );
                    },
                );
                MenuButton::new(
                    cx,
                    |_| debug!("Doc 3"),
                    |cx| Label::new(cx, "Doc 3"),
                );
            },
        );
        MenuDivider::new(cx);
        MenuButton::new(cx, |_| debug!("Save"), |cx| Label::new(cx, "Save"));
        MenuButton::new(
            cx,
            |_| debug!("Save As"),
            |cx| Label::new(cx, "Save As"),
        );
        MenuDivider::new(cx);
        MenuButton::new(cx, |_| debug!("Quit"), |cx| Label::new(cx, "Quit"));
    },
)
.width(Pixels(100.0));"#,
        );
    })
    .class("panel");
}
