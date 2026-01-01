use log::debug;
use vizia::prelude::*;

use vizia::icons::{ICON_CLIPBOARD, ICON_COPY, ICON_CUT};

use crate::DemoRegion;

pub fn menu_bar(cx: &mut Context) {
    VStack::new(cx, |cx| {
        Markdown::new(cx, "# MenuBar");

        Divider::new(cx);

        Markdown::new(cx, "### Basic menu bar");

        DemoRegion::new(
            cx,
            |cx| {
                MenuBar::new(cx, |cx| {
                    Submenu::new(
                        cx,
                        |cx| Label::static_text(cx, "File"),
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
                    );
                    Submenu::new(
                        cx,
                        |cx| Label::static_text(cx, "Edit"),
                        |cx| {
                            MenuButton::new(
                                cx,
                                |_| debug!("Cut"),
                                |cx| {
                                    HStack::new(cx, |cx| {
                                        Svg::new(cx, ICON_CUT).class("icon");
                                        Label::static_text(cx, "Cut");
                                    })
                                },
                            );
                            MenuButton::new(
                                cx,
                                |_| debug!("Copy"),
                                |cx| {
                                    HStack::new(cx, |cx| {
                                        Svg::new(cx, ICON_COPY).class("icon");
                                        Label::static_text(cx, "Copy");
                                    })
                                },
                            );
                            MenuButton::new(
                                cx,
                                |_| debug!("Paste"),
                                |cx| {
                                    HStack::new(cx, |cx| {
                                        Svg::new(cx, ICON_CLIPBOARD).class("icon");
                                        Label::static_text(cx, "Paste");
                                    })
                                },
                            );
                        },
                    );
                    Submenu::new(
                        cx,
                        |cx| Label::static_text(cx, "View"),
                        |cx| {
                            MenuButton::new(
                                cx,
                                |_| debug!("Zoom In"),
                                |cx| Label::static_text(cx, "Zoom In"),
                            );
                            MenuButton::new(
                                cx,
                                |_| debug!("Zoom Out"),
                                |cx| Label::static_text(cx, "Zoom Out"),
                            );
                            Submenu::new(
                                cx,
                                |cx| Label::static_text(cx, "Zoom Level"),
                                |cx| {
                                    MenuButton::new(
                                        cx,
                                        |_| debug!("10%"),
                                        |cx| Label::static_text(cx, "10%"),
                                    );
                                    MenuButton::new(
                                        cx,
                                        |_| debug!("20%"),
                                        |cx| Label::static_text(cx, "20%"),
                                    );
                                    MenuButton::new(
                                        cx,
                                        |_| debug!("50%"),
                                        |cx| Label::static_text(cx, "50%"),
                                    );
                                    MenuButton::new(
                                        cx,
                                        |_| debug!("100%"),
                                        |cx| Label::static_text(cx, "100%"),
                                    );
                                    MenuButton::new(
                                        cx,
                                        |_| debug!("150%"),
                                        |cx| Label::static_text(cx, "150%"),
                                    );
                                    MenuButton::new(
                                        cx,
                                        |_| debug!("200%"),
                                        |cx| Label::static_text(cx, "200%"),
                                    );
                                },
                            );
                        },
                    );
                    Submenu::new(
                        cx,
                        |cx| Label::static_text(cx, "Help"),
                        |cx| {
                            MenuButton::new(
                                cx,
                                |_| debug!("Show License"),
                                |cx| Label::static_text(cx, "Show License"),
                            );
                            MenuButton::new(
                                cx,
                                |_| debug!("About"),
                                |cx| Label::static_text(cx, "About"),
                            );
                        },
                    );
                });
            },
            r#"MenuBar::new(cx, |cx| {
    Submenu::new(
        cx,
        |cx| Label::static_text(cx, "File"),
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
            MenuButton::new(cx, |_| debug!("Save As"), |cx| Label::static_text(cx, "Save As"));
            MenuDivider::new(cx);
            MenuButton::new(cx, |_| debug!("Quit"), |cx| Label::static_text(cx, "Quit"));
        },
    );
    Submenu::new(
        cx,
        |cx| Label::static_text(cx, "Edit"),
        |cx| {
            MenuButton::new(
                cx,
                |_| debug!("Cut"),
                |cx| {
                    HStack::new(cx, |cx| {
                        Svg::new(cx, ICON_CUT).class("icon");
                        Label::static_text(cx, "Cut");
                    })
                },
            );
            MenuButton::new(
                cx,
                |_| debug!("Copy"),
                |cx| {
                    HStack::new(cx, |cx| {
                        Svg::new(cx, ICON_COPY).class("icon");
                        Label::static_text(cx, "Copy");
                    })
                },
            );
            MenuButton::new(
                cx,
                |_| debug!("Paste"),
                |cx| {
                    HStack::new(cx, |cx| {
                        Svg::new(cx, ICON_CLIPBOARD).class("icon");
                        Label::static_text(cx, "Paste");
                    })
                },
            );
        },
    );
    Submenu::new(
        cx,
        |cx| Label::static_text(cx, "View"),
        |cx| {
            MenuButton::new(cx, |_| debug!("Zoom In"), |cx| Label::static_text(cx, "Zoom In"));
            MenuButton::new(cx, |_| debug!("Zoom Out"), |cx| Label::static_text(cx, "Zoom Out"));
            Submenu::new(
                cx,
                |cx| Label::static_text(cx, "Zoom Level"),
                |cx| {
                    MenuButton::new(cx, |_| debug!("10%"), |cx| Label::static_text(cx, "10%"));
                    MenuButton::new(cx, |_| debug!("20%"), |cx| Label::static_text(cx, "20%"));
                    MenuButton::new(cx, |_| debug!("50%"), |cx| Label::static_text(cx, "50%"));
                    MenuButton::new(cx, |_| debug!("100%"), |cx| Label::static_text(cx, "100%"));
                    MenuButton::new(cx, |_| debug!("150%"), |cx| Label::static_text(cx, "150%"));
                    MenuButton::new(cx, |_| debug!("200%"), |cx| Label::static_text(cx, "200%"));
                },
            );
        },
    );
    Submenu::new(
        cx,
        |cx| Label::static_text(cx, "Help"),
        |cx| {
            MenuButton::new(
                cx,
                |_| debug!("Show License"),
                |cx| Label::static_text(cx, "Show License"),
            );
            MenuButton::new(cx, |_| debug!("About"), |cx| Label::static_text(cx, "About"));
        },
    );
});"#,
        );
    })
    .class("panel");
}
