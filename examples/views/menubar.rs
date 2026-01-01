mod helpers;
use helpers::*;
use log::debug;
use vizia::prelude::*;
use vizia_core::icons::{ICON_CLIPBOARD, ICON_COPY, ICON_CUT};

fn main() -> Result<(), ApplicationError> {
    let (app, title) = Application::new_with_state(|cx: &mut Context| {
        let gap_stretch = cx.state(Stretch(1.0));
        let spacer_height = cx.state(Pixels(4.0));
        let icon_cut = cx.state(ICON_CUT);
        let icon_copy = cx.state(ICON_COPY);
        let icon_clipboard = cx.state(ICON_CLIPBOARD);
        ExamplePage::new(cx, move |cx| {
            MenuBar::new(cx, move |cx| {
                Submenu::new(
                    cx,
                    |cx| Label::static_text(cx, "File"),
                    move |cx| {
                        MenuButton::new(
                            cx,
                            |_| debug!("New"),
                            move |cx| {
                                HStack::new(cx, |cx| {
                                    Label::static_text(cx, "New");
                                    Label::static_text(cx, "Ctrl + N").class("shortcut");
                                })
                                .gap(gap_stretch)
                            },
                        );
                        MenuButton::new(
                            cx,
                            |_| debug!("Open"),
                            move |cx| {
                                HStack::new(cx, |cx| {
                                    Label::static_text(cx, "Open");
                                    Label::static_text(cx, "Ctrl + O").class("shortcut");
                                })
                                .gap(gap_stretch)
                            },
                        );
                        Submenu::new(
                            cx,
                            |cx| Label::static_text(cx, "Open Recent"),
                            move |cx| {
                                MenuButton::new(
                                    cx,
                                    |_| debug!("Doc 1"),
                                    |cx| Label::static_text(cx, "Doc 1"),
                                );
                                Submenu::new(
                                    cx,
                                    |cx| Label::static_text(cx, "Doc 2"),
                                    move |cx| {
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
                        Spacer::new(cx).height(spacer_height);
                        Divider::new(cx);
                        Spacer::new(cx).height(spacer_height);
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
                        Spacer::new(cx).height(spacer_height);
                        Divider::new(cx);
                        Spacer::new(cx).height(spacer_height);
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
                    move |cx| {
                        MenuButton::new(
                            cx,
                            |_| debug!("Cut"),
                            move |cx| {
                                HStack::new(cx, |cx| {
                                    Svg::new(cx, icon_cut).class("icon");
                                    Label::static_text(cx, "Cut");
                                })
                            },
                        );
                        MenuButton::new(
                            cx,
                            |_| debug!("Copy"),
                            move |cx| {
                                HStack::new(cx, |cx| {
                                    Svg::new(cx, icon_copy).class("icon");
                                    Label::static_text(cx, "Copy");
                                })
                            },
                        );
                        MenuButton::new(
                            cx,
                            |_| debug!("Paste"),
                            move |cx| {
                                HStack::new(cx, |cx| {
                                    Svg::new(cx, icon_clipboard).class("icon");
                                    Label::static_text(cx, "Paste");
                                })
                            },
                        );
                    },
                );
                Submenu::new(
                    cx,
                    |cx| Label::static_text(cx, "View"),
                    move |cx| {
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
                            move |cx| {
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
                    move |cx| {
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
        });
        cx.state("Menu Bar")
    });

    app.title(title).run()
}
