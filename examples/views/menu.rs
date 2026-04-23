mod helpers;
use helpers::*;
use log::debug;
use vizia::prelude::*;

fn main() -> Result<(), ApplicationError> {
    Application::new(|cx: &mut Context| {
        ExamplePage::new(cx, |cx| {
            Submenu::new(
                cx,
                |cx| Label::new(cx, Localized::new("menu-root")),
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
                    Divider::new(cx);
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
                    Divider::new(cx);
                    MenuButton::new(
                        cx,
                        |_| debug!("Quit"),
                        |cx| Label::new(cx, Localized::new("menu-quit")),
                    );
                },
            )
            .width(Pixels(100.0));
        });
    })
    .title(Localized::new("view-title-menu"))
    .run()
}
