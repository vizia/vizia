use log::debug;
use vizia::prelude::*;

use crate::components::DemoRegion;

pub fn menu(cx: &mut Context) {
    let width_100 = cx.state(Pixels(100.0));

    VStack::new(cx, |cx| {
        Markdown::new(cx, "# Menu");

        Divider::new(cx);

        Markdown::new(cx, "### Basic menu");

        DemoRegion::new(
            cx,
            |cx| {
                Submenu::new(
                    cx,
                    |cx| Label::static_text(cx, "Menu"),
                    |cx| {
                        MenuButton::new(
                            cx,
                            |_| debug!("New"),
                            |cx| {
                                HStack::new(cx, |cx| {
                                    Label::static_text(cx, "New");
                                    Label::static_text(cx, "Ctrl + N").class("shortcut");
                                })
                            },
                        );
                        MenuButton::new(
                            cx,
                            |_| debug!("Open"),
                            |cx| {
                                HStack::new(cx, |cx| {
                                    Label::static_text(cx, "Open");
                                    Label::static_text(cx, "Ctrl + O").class("shortcut");
                                })
                            },
                        );
                        Submenu::new(
                            cx,
                            |cx| Label::static_text(cx, "Open Recent"),
                            |cx| {
                                MenuButton::new(
                                    cx,
                                    |_| debug!("Doc 1"),
                                    |cx| Label::static_text(cx, "Doc 1"),
                                );
                                Submenu::new(
                                    cx,
                                    |cx| Label::static_text(cx, "Doc 2"),
                                    |cx| {
                                        MenuButton::new(
                                            cx,
                                            |_| debug!("Version 1"),
                                            |cx| Label::static_text(cx, "Version 1"),
                                        );
                                        MenuButton::new(
                                            cx,
                                            |_| debug!("Version 2"),
                                            |cx| Label::static_text(cx, "Version 2"),
                                        );
                                        MenuButton::new(
                                            cx,
                                            |_| debug!("Version 3"),
                                            |cx| Label::static_text(cx, "Version 3"),
                                        );
                                    },
                                );
                                MenuButton::new(
                                    cx,
                                    |_| debug!("Doc 3"),
                                    |cx| Label::static_text(cx, "Doc 3"),
                                );
                            },
                        );
                        Divider::new(cx);
                        MenuButton::new(
                            cx,
                            |_| debug!("Save"),
                            |cx| Label::static_text(cx, "Save"),
                        );
                        MenuButton::new(
                            cx,
                            |_| debug!("Save As"),
                            |cx| Label::static_text(cx, "Save As"),
                        );
                        Divider::new(cx);
                        MenuButton::new(
                            cx,
                            |_| debug!("Quit"),
                            |cx| Label::static_text(cx, "Quit"),
                        );
                    },
                )
                .width(width_100);
            },
            r#"let width_100 = cx.state(Pixels(100.0));

Submenu::new(
    cx,
    |cx| Label::static_text(cx, "Menu"),
    |cx| {
        MenuButton::new(
            cx,
            |_| debug!("New"),
            |cx| {
                HStack::new(cx, |cx| {
                    Label::static_text(cx, "New");
                    Label::static_text(cx, "Ctrl + N").class("shortcut");
                })
            },
        );
        MenuButton::new(
            cx,
            |_| debug!("Open"),
            |cx| {
                HStack::new(cx, |cx| {
                    Label::static_text(cx, "Open");
                    Label::static_text(cx, "Ctrl + O").class("shortcut");
                })
            },
        );
        Submenu::new(
            cx,
            |cx| Label::static_text(cx, "Open Recent"),
            |cx| {
                MenuButton::new(
                    cx,
                    |_| debug!("Doc 1"),
                    |cx| Label::static_text(cx, "Doc 1"),
                );
                Submenu::new(
                    cx,
                    |cx| Label::static_text(cx, "Doc 2"),
                    |cx| {
                        MenuButton::new(
                            cx,
                            |_| debug!("Version 1"),
                            |cx| Label::static_text(cx, "Version 1"),
                        );
                        MenuButton::new(
                            cx,
                            |_| debug!("Version 2"),
                            |cx| Label::static_text(cx, "Version 2"),
                        );
                        MenuButton::new(
                            cx,
                            |_| debug!("Version 3"),
                            |cx| Label::static_text(cx, "Version 3"),
                        );
                    },
                );
                MenuButton::new(
                    cx,
                    |_| debug!("Doc 3"),
                    |cx| Label::static_text(cx, "Doc 3"),
                );
            },
        );
        MenuDivider::new(cx);
        MenuButton::new(cx, |_| debug!("Save"), |cx| Label::static_text(cx, "Save"));
        MenuButton::new(
            cx,
            |_| debug!("Save As"),
            |cx| Label::static_text(cx, "Save As"),
        );
        MenuDivider::new(cx);
        MenuButton::new(cx, |_| debug!("Quit"), |cx| Label::static_text(cx, "Quit"));
    },
)
.width(width_100);"#,
        );
    })
    .class("panel");
}
