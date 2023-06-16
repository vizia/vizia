mod helpers;
use helpers::*;
use vizia::prelude::*;
use vizia_core::icons::{ICON_CLIPBOARD, ICON_COPY, ICON_CUT};

#[derive(Lens)]
pub struct AppData {}

impl Model for AppData {}

fn main() {
    Application::new(|cx: &mut Context| {
        // cx.add_stylesheet(STYLE);

        AppData {}.build(cx);

        ExamplePage::new(cx, |cx| {
            MenuBar::new(cx, |cx| {
                Submenu::new(
                    cx,
                    |cx| Label::new(cx, "File"),
                    |cx| {
                        MenuButton::new(
                            cx,
                            |_| println!("File"),
                            |cx| {
                                HStack::new(cx, |cx| {
                                    Label::new(cx, "New");
                                    Label::new(cx, &format!("Ctrl + N")).class("shortcut");
                                })
                            },
                        );
                        MenuButton::new(
                            cx,
                            |_| println!("Open"),
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
                                    |_| println!("Doc 1"),
                                    |cx| Label::new(cx, "Doc 1"),
                                );
                                Submenu::new(
                                    cx,
                                    |cx| Label::new(cx, "Doc 2"),
                                    |cx| {
                                        MenuButton::new(
                                            cx,
                                            |_| println!("Version 1"),
                                            |cx| Label::new(cx, "Version 1"),
                                        );
                                        MenuButton::new(
                                            cx,
                                            |_| println!("Version 2"),
                                            |cx| Label::new(cx, "Version 2"),
                                        );
                                        MenuButton::new(
                                            cx,
                                            |_| println!("Version 3"),
                                            |cx| Label::new(cx, "Version 3"),
                                        );
                                    },
                                );
                                MenuButton::new(
                                    cx,
                                    |_| println!("Doc 3"),
                                    |cx| Label::new(cx, "Doc 3"),
                                );
                            },
                        );
                        MenuDivider::new(cx);
                        MenuButton::new(cx, |_| println!("Save"), |cx| Label::new(cx, "Save"));
                        MenuButton::new(
                            cx,
                            |_| println!("Save As"),
                            |cx| Label::new(cx, "Save As"),
                        );
                        MenuDivider::new(cx);
                        MenuButton::new(cx, |_| println!("Quit"), |cx| Label::new(cx, "Quit"));
                    },
                );
                Submenu::new(
                    cx,
                    |cx| Label::new(cx, "Edit"),
                    |cx| {
                        MenuButton::new(
                            cx,
                            |_| println!("Cut"),
                            |cx| {
                                HStack::new(cx, |cx| {
                                    Label::new(cx, ICON_CUT).class("icon");
                                    Label::new(cx, "Cut");
                                })
                            },
                        );
                        MenuButton::new(
                            cx,
                            |_| println!("Copy"),
                            |cx| {
                                HStack::new(cx, |cx| {
                                    Label::new(cx, ICON_COPY).class("icon");
                                    Label::new(cx, "Copy");
                                })
                            },
                        );
                        MenuButton::new(
                            cx,
                            |_| println!("Paste"),
                            |cx| {
                                HStack::new(cx, |cx| {
                                    Label::new(cx, ICON_CLIPBOARD).class("icon");
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
                            |_| println!("Zoom In"),
                            |cx| Label::new(cx, "Zoom In"),
                        );
                        MenuButton::new(
                            cx,
                            |_| println!("Zoom Out"),
                            |cx| Label::new(cx, "Zoom Out"),
                        );
                        Submenu::new(
                            cx,
                            |cx| Label::new(cx, "Zoom Level"),
                            |cx| {
                                MenuButton::new(
                                    cx,
                                    |_| println!("10%"),
                                    |cx| Label::new(cx, "10%"),
                                );
                                MenuButton::new(
                                    cx,
                                    |_| println!("20%"),
                                    |cx| Label::new(cx, "20%"),
                                );
                                MenuButton::new(
                                    cx,
                                    |_| println!("50%"),
                                    |cx| Label::new(cx, "50%"),
                                );
                                MenuButton::new(
                                    cx,
                                    |_| println!("100%"),
                                    |cx| Label::new(cx, "100%"),
                                );
                                MenuButton::new(
                                    cx,
                                    |_| println!("150%"),
                                    |cx| Label::new(cx, "150%"),
                                );
                                MenuButton::new(
                                    cx,
                                    |_| println!("200%"),
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
                            |_| println!("Show License"),
                            |cx| Label::new(cx, "Show License"),
                        );
                        MenuButton::new(cx, |_| println!("About"), |cx| Label::new(cx, "About"));
                    },
                );
            })
            .top(Pixels(0.0));
        });
    })
    .title("Menu")
    .run();
}
