mod helpers;
use helpers::*;
use log::debug;
use vizia::prelude::*;
use vizia_core::icons::{ICON_CLIPBOARD, ICON_COPY, ICON_CUT};

fn main() -> Result<(), ApplicationError> {
    Application::new(|cx: &mut Context| {
        ExamplePage::new(cx, |cx| {
            MenuBar::new(cx, |cx| {
                Submenu::new(
                    cx,
                    |cx| Label::new(cx, Localized::new("menubar-file")),
                    |cx| {
                        MenuButton::new(
                            cx,
                            |_| debug!("New"),
                            |cx| {
                                HStack::new(cx, |cx| {
                                    Label::new(cx, Localized::new("menu-new"));
                                    Label::new(cx, Localized::new("menu-shortcut-new"))
                                        .class("shortcut");
                                })
                                .gap(Stretch(1.0))
                            },
                        );
                        MenuButton::new(
                            cx,
                            |_| debug!("Open"),
                            |cx| {
                                HStack::new(cx, |cx| {
                                    Label::new(cx, Localized::new("menu-open"));
                                    Label::new(cx, Localized::new("menu-shortcut-open"))
                                        .class("shortcut");
                                })
                                .gap(Stretch(1.0))
                            },
                        );
                        Submenu::new(
                            cx,
                            |cx| Label::new(cx, Localized::new("menu-open-recent")),
                            |cx| {
                                MenuButton::new(
                                    cx,
                                    |_| debug!("Doc 1"),
                                    |cx| Label::new(cx, Localized::new("menu-doc-1")),
                                );
                                Submenu::new(
                                    cx,
                                    |cx| Label::new(cx, Localized::new("menu-doc-2")),
                                    |cx| {
                                        MenuButton::new(
                                            cx,
                                            |_| debug!("Version 1"),
                                            |cx| Label::new(cx, Localized::new("menu-version-1")),
                                        );
                                        MenuButton::new(
                                            cx,
                                            |_| debug!("Version 2"),
                                            |cx| Label::new(cx, Localized::new("menu-version-2")),
                                        );
                                        MenuButton::new(
                                            cx,
                                            |_| debug!("Version 3"),
                                            |cx| Label::new(cx, Localized::new("menu-version-3")),
                                        );
                                    },
                                );
                                MenuButton::new(
                                    cx,
                                    |_| debug!("Doc 3"),
                                    |cx| Label::new(cx, Localized::new("menu-doc-3")),
                                );
                            },
                        );
                        Spacer::new(cx).height(Pixels(4.0));
                        Divider::new(cx);
                        Spacer::new(cx).height(Pixels(4.0));
                        MenuButton::new(
                            cx,
                            |_| debug!("Save"),
                            |cx| Label::new(cx, Localized::new("menu-save")),
                        );
                        MenuButton::new(
                            cx,
                            |_| debug!("Save As"),
                            |cx| Label::new(cx, Localized::new("menu-save-as")),
                        );
                        Spacer::new(cx).height(Pixels(4.0));
                        Divider::new(cx);
                        Spacer::new(cx).height(Pixels(4.0));
                        MenuButton::new(
                            cx,
                            |_| debug!("Quit"),
                            |cx| Label::new(cx, Localized::new("menu-quit")),
                        );
                    },
                );
                Submenu::new(
                    cx,
                    |cx| Label::new(cx, Localized::new("menubar-edit")),
                    |cx| {
                        MenuButton::new(
                            cx,
                            |_| debug!("Cut"),
                            |cx| {
                                HStack::new(cx, |cx| {
                                    Svg::new(cx, ICON_CUT).class("icon");
                                    Label::new(cx, Localized::new("menubar-cut"));
                                })
                            },
                        );
                        MenuButton::new(
                            cx,
                            |_| debug!("Copy"),
                            |cx| {
                                HStack::new(cx, |cx| {
                                    Svg::new(cx, ICON_COPY).class("icon");
                                    Label::new(cx, Localized::new("menubar-copy"));
                                })
                            },
                        );
                        MenuButton::new(
                            cx,
                            |_| debug!("Paste"),
                            |cx| {
                                HStack::new(cx, |cx| {
                                    Svg::new(cx, ICON_CLIPBOARD).class("icon");
                                    Label::new(cx, Localized::new("menubar-paste"));
                                })
                            },
                        );
                    },
                );
                Submenu::new(
                    cx,
                    |cx| Label::new(cx, Localized::new("menubar-view")),
                    |cx| {
                        MenuButton::new(
                            cx,
                            |_| debug!("Zoom In"),
                            |cx| Label::new(cx, Localized::new("menubar-zoom-in")),
                        );
                        MenuButton::new(
                            cx,
                            |_| debug!("Zoom Out"),
                            |cx| Label::new(cx, Localized::new("menubar-zoom-out")),
                        );
                        Submenu::new(
                            cx,
                            |cx| Label::new(cx, Localized::new("menubar-zoom-level")),
                            |cx| {
                                MenuButton::new(
                                    cx,
                                    |_| debug!("10%"),
                                    |cx| Label::new(cx, Localized::new("menubar-zoom-10")),
                                );
                                MenuButton::new(
                                    cx,
                                    |_| debug!("20%"),
                                    |cx| Label::new(cx, Localized::new("menubar-zoom-20")),
                                );
                                MenuButton::new(
                                    cx,
                                    |_| debug!("50%"),
                                    |cx| Label::new(cx, Localized::new("menubar-zoom-50")),
                                );
                                MenuButton::new(
                                    cx,
                                    |_| debug!("100%"),
                                    |cx| Label::new(cx, Localized::new("menubar-zoom-100")),
                                );
                                MenuButton::new(
                                    cx,
                                    |_| debug!("150%"),
                                    |cx| Label::new(cx, Localized::new("menubar-zoom-150")),
                                );
                                MenuButton::new(
                                    cx,
                                    |_| debug!("200%"),
                                    |cx| Label::new(cx, Localized::new("menubar-zoom-200")),
                                );
                            },
                        );
                    },
                );
                Submenu::new(
                    cx,
                    |cx| Label::new(cx, Localized::new("menubar-help")),
                    |cx| {
                        MenuButton::new(
                            cx,
                            |_| debug!("Show License"),
                            |cx| Label::new(cx, Localized::new("menubar-show-license")),
                        );
                        MenuButton::new(
                            cx,
                            |_| debug!("About"),
                            |cx| Label::new(cx, Localized::new("menubar-about")),
                        );
                    },
                );
            });
        });
    })
    .title(Localized::new("view-title-menubar"))
    .run()
}
