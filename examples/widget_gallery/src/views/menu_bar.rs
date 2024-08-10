use log::debug;
use vizia::prelude::*;

use vizia::icons::{ICON_CLIPBOARD, ICON_COPY, ICON_CUT};

use crate::DemoRegion;

pub fn menu_bar(cx: &mut Context) {
    VStack::new(cx, |cx| {
        Label::new(cx, "MenuBar").class("title");
        Label::new(cx, "A label can be used to display a string of text.").class("paragraph");

        Divider::new(cx).top(Pixels(12.0)).bottom(Pixels(12.0));

        Label::new(cx, "Basic menu bar").class("header");
        DemoRegion::new(
            cx,
            |cx| {
                MenuBar::new(cx, |cx| {
                    Submenu::new(
                        cx,
                        |cx| Label::new(cx, "File"),
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
                            MenuButton::new(
                                cx,
                                |_| debug!("Save As"),
                                |cx| Label::new(cx, "Save As"),
                            );
                            Divider::new(cx);
                            MenuButton::new(cx, |_| debug!("Quit"), |cx| Label::new(cx, "Quit"));
                        },
                    );
                    Submenu::new(
                        cx,
                        |cx| Label::new(cx, "Edit"),
                        |cx| {
                            MenuButton::new(
                                cx,
                                |_| debug!("Cut"),
                                |cx| {
                                    HStack::new(cx, |cx| {
                                        Svg::new(cx, ICON_CUT).class("icon");
                                        Label::new(cx, "Cut");
                                    })
                                },
                            );
                            MenuButton::new(
                                cx,
                                |_| debug!("Copy"),
                                |cx| {
                                    HStack::new(cx, |cx| {
                                        Svg::new(cx, ICON_COPY).class("icon");
                                        Label::new(cx, "Copy");
                                    })
                                },
                            );
                            MenuButton::new(
                                cx,
                                |_| debug!("Paste"),
                                |cx| {
                                    HStack::new(cx, |cx| {
                                        Svg::new(cx, ICON_CLIPBOARD).class("icon");
                                        Label::new(cx, "Paste");
                                    })
                                },
                            );
                        },
                    );
                    Submenu::new(
                        cx,
                        |cx| Label::new(cx, "View"),
                        |cx| {
                            MenuButton::new(
                                cx,
                                |_| debug!("Zoom In"),
                                |cx| Label::new(cx, "Zoom In"),
                            );
                            MenuButton::new(
                                cx,
                                |_| debug!("Zoom Out"),
                                |cx| Label::new(cx, "Zoom Out"),
                            );
                            Submenu::new(
                                cx,
                                |cx| Label::new(cx, "Zoom Level"),
                                |cx| {
                                    MenuButton::new(
                                        cx,
                                        |_| debug!("10%"),
                                        |cx| Label::new(cx, "10%"),
                                    );
                                    MenuButton::new(
                                        cx,
                                        |_| debug!("20%"),
                                        |cx| Label::new(cx, "20%"),
                                    );
                                    MenuButton::new(
                                        cx,
                                        |_| debug!("50%"),
                                        |cx| Label::new(cx, "50%"),
                                    );
                                    MenuButton::new(
                                        cx,
                                        |_| debug!("100%"),
                                        |cx| Label::new(cx, "100%"),
                                    );
                                    MenuButton::new(
                                        cx,
                                        |_| debug!("150%"),
                                        |cx| Label::new(cx, "150%"),
                                    );
                                    MenuButton::new(
                                        cx,
                                        |_| debug!("200%"),
                                        |cx| Label::new(cx, "200%"),
                                    );
                                },
                            );
                        },
                    );
                    Submenu::new(
                        cx,
                        |cx| Label::new(cx, "Help"),
                        |cx| {
                            MenuButton::new(
                                cx,
                                |_| debug!("Show License"),
                                |cx| Label::new(cx, "Show License"),
                            );
                            MenuButton::new(cx, |_| debug!("About"), |cx| Label::new(cx, "About"));
                        },
                    );
                });
            },
            r#"MenuBar::new(cx, |cx| {
    Submenu::new(
        cx,
        |cx| Label::new(cx, "File"),
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
            MenuButton::new(cx, |_| debug!("Save As"), |cx| Label::new(cx, "Save As"));
            MenuDivider::new(cx);
            MenuButton::new(cx, |_| debug!("Quit"), |cx| Label::new(cx, "Quit"));
        },
    );
    Submenu::new(
        cx,
        |cx| Label::new(cx, "Edit"),
        |cx| {
            MenuButton::new(
                cx,
                |_| debug!("Cut"),
                |cx| {
                    HStack::new(cx, |cx| {
                        Svg::new(cx, ICON_CUT).class("icon");
                        Label::new(cx, "Cut");
                    })
                },
            );
            MenuButton::new(
                cx,
                |_| debug!("Copy"),
                |cx| {
                    HStack::new(cx, |cx| {
                        Svg::new(cx, ICON_COPY).class("icon");
                        Label::new(cx, "Copy");
                    })
                },
            );
            MenuButton::new(
                cx,
                |_| debug!("Paste"),
                |cx| {
                    HStack::new(cx, |cx| {
                        Svg::new(cx, ICON_CLIPBOARD).class("icon");
                        Label::new(cx, "Paste");
                    })
                },
            );
        },
    );
    Submenu::new(
        cx,
        |cx| Label::new(cx, "View"),
        |cx| {
            MenuButton::new(cx, |_| debug!("Zoom In"), |cx| Label::new(cx, "Zoom In"));
            MenuButton::new(cx, |_| debug!("Zoom Out"), |cx| Label::new(cx, "Zoom Out"));
            Submenu::new(
                cx,
                |cx| Label::new(cx, "Zoom Level"),
                |cx| {
                    MenuButton::new(cx, |_| debug!("10%"), |cx| Label::new(cx, "10%"));
                    MenuButton::new(cx, |_| debug!("20%"), |cx| Label::new(cx, "20%"));
                    MenuButton::new(cx, |_| debug!("50%"), |cx| Label::new(cx, "50%"));
                    MenuButton::new(cx, |_| debug!("100%"), |cx| Label::new(cx, "100%"));
                    MenuButton::new(cx, |_| debug!("150%"), |cx| Label::new(cx, "150%"));
                    MenuButton::new(cx, |_| debug!("200%"), |cx| Label::new(cx, "200%"));
                },
            );
        },
    );
    Submenu::new(
        cx,
        |cx| Label::new(cx, "Help"),
        |cx| {
            MenuButton::new(
                cx,
                |_| debug!("Show License"),
                |cx| Label::new(cx, "Show License"),
            );
            MenuButton::new(cx, |_| debug!("About"), |cx| Label::new(cx, "About"));
        },
    );
});"#,
        );
    })
    .class("panel");
}
