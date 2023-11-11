use vizia::prelude::*;
use vizia_core::{
    icons::{ICON_CLIPBOARD, ICON_COPY, ICON_CUT},
    modifiers::ModalEvent,
};

mod helpers;
use helpers::*;

#[derive(Lens)]
pub struct AppData {}

impl Model for AppData {}

fn main() {
    Application::new(|cx: &mut Context| {
        // cx.add_stylesheet(STYLE);

        // setup_logging();

        AppData {}.build(cx);

        Button::new(cx, |cx| Label::new(cx, "Show Menu"))
            .on_press(|cx| cx.emit(ModalEvent::ShowMenu))
            .menu(|cx| {
                Menu::new(cx, |cx| {
                    MenuButton::new(
                        cx,
                        |_| println!("New"),
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
                    MenuButton::new(cx, |_| println!("Save As"), |cx| Label::new(cx, "Save As"));
                    MenuDivider::new(cx);
                    MenuButton::new(cx, |_| println!("Quit"), |cx| Label::new(cx, "Quit"));
                })
            });

        // Submenu::new(
        //     cx,
        //     |cx| Label::new(cx, "File"),
        //     |cx| {
        //         MenuButton::new(
        //             cx,
        //             |_| println!("New"),
        //             |cx| {
        //                 HStack::new(cx, |cx| {
        //                     Label::new(cx, "New");
        //                     Label::new(cx, &format!("Ctrl + N")).class("shortcut");
        //                 })
        //             },
        //         );
        //         MenuButton::new(
        //             cx,
        //             |_| println!("Open"),
        //             |cx| {
        //                 HStack::new(cx, |cx| {
        //                     Label::new(cx, "Open");
        //                     Label::new(cx, &format!("Ctrl + O")).class("shortcut");
        //                 })
        //             },
        //         );
        //         Submenu::new(
        //             cx,
        //             |cx| Label::new(cx, "Open Recent"),
        //             |cx| {
        //                 MenuButton::new(cx, |_| println!("Doc 1"), |cx| Label::new(cx, "Doc 1"));
        //                 Submenu::new(
        //                     cx,
        //                     |cx| Label::new(cx, "Doc 2"),
        //                     |cx| {
        //                         MenuButton::new(
        //                             cx,
        //                             |_| println!("Version 1"),
        //                             |cx| Label::new(cx, "Version 1"),
        //                         );
        //                         MenuButton::new(
        //                             cx,
        //                             |_| println!("Version 2"),
        //                             |cx| Label::new(cx, "Version 2"),
        //                         );
        //                         MenuButton::new(
        //                             cx,
        //                             |_| println!("Version 3"),
        //                             |cx| Label::new(cx, "Version 3"),
        //                         );
        //                     },
        //                 );
        //                 MenuButton::new(cx, |_| println!("Doc 3"), |cx| Label::new(cx, "Doc 3"));
        //             },
        //         );
        //         MenuDivider::new(cx);
        //         MenuButton::new(cx, |_| println!("Save"), |cx| Label::new(cx, "Save"));
        //         MenuButton::new(cx, |_| println!("Save As"), |cx| Label::new(cx, "Save As"));
        //         MenuDivider::new(cx);
        //         MenuButton::new(cx, |_| println!("Quit"), |cx| Label::new(cx, "Quit"));
        //     },
        // );

        // Submenu::new(
        //     cx,
        //     |cx| Label::new(cx, "Options"),
        //     |cx| {
        //         MenuButton::new(
        //             cx,
        //             |cx| {},
        //             |cx| {
        //                 HStack::new(cx, |cx| {
        //                     Label::new(cx, ICON_SHARE_3).class("icon");
        //                     Label::new(cx, "Share");
        //                 })
        //             },
        //         );
        //         MenuButton::new(
        //             cx,
        //             |cx| {},
        //             |cx| {
        //                 HStack::new(cx, |cx| {
        //                     Label::new(cx, ICON_CURSOR_TEXT).class("icon");
        //                     Label::new(cx, "Rename");
        //                 })
        //             },
        //         );
        //         MenuButton::new(
        //             cx,
        //             |cx| {},
        //             |cx| {
        //                 HStack::new(cx, |cx| {
        //                     Label::new(cx, ICON_TRASH).class("icon");
        //                     Label::new(cx, "Delete");
        //                 })
        //             },
        //         );
        //     },
        // )
        // .space(Stretch(1.0));

        // MenuController { open_menu: None }.build(cx);

        // MenuButton::new(cx, |cx| {
        //     Label::new(cx, "first").width(Pixels(100.0)).height(Pixels(30.0));
        //     MenuButton::new(cx, |cx| {
        //         Label::new(cx, "first").width(Pixels(100.0)).height(Pixels(30.0));
        //         Label::new(cx, "first").width(Pixels(100.0)).height(Pixels(30.0));
        //         Label::new(cx, "first").width(Pixels(100.0)).height(Pixels(30.0));
        //         Label::new(cx, "first").width(Pixels(100.0)).height(Pixels(30.0));
        //     })
        //     .width(Pixels(100.0))
        //     .height(Pixels(30.0));
        //     Label::new(cx, "first").width(Pixels(100.0)).height(Pixels(30.0));
        //     Label::new(cx, "first").width(Pixels(100.0)).height(Pixels(30.0));
        // })
        // .width(Pixels(100.0))
        // .height(Pixels(30.0));

        // MenuController::new(cx, false, |cx| {
        //     MenuStack::new_horizontal(cx, |cx| {
        //         Menu::new(
        //             cx,
        //             |cx| Label::new(cx, "menu 1"),
        //             |cx| {
        //                 MenuButton::new_simple(cx, "option 1", |_| {});
        //                 MenuButton::new_simple(cx, "option 2 looooooooooooong", |_| {});
        //                 Menu::new(
        //                     cx,
        //                     |cx| Label::new(cx, "menu 1a"),
        //                     |cx| {
        //                         MenuButton::new_check_simple(
        //                             cx,
        //                             "option 1",
        //                             |_| {},
        //                             AppData::check1,
        //                         );
        //                         MenuButton::new_check_simple(
        //                             cx,
        //                             "option 2",
        //                             |_| {},
        //                             AppData::check2,
        //                         );
        //                         MenuButton::new_simple(cx, "loooooooooooooooooooooooooong", |_| {});
        //                     },
        //                 );
        //             },
        //         );
        //         Menu::new(
        //             cx,
        //             |cx| Label::new(cx, "menu 2"),
        //             |cx| {
        //                 MenuButton::new_simple(cx, "option 1", |_| {});
        //                 MenuButton::new_simple(cx, "option 2 looooooooooooong", |_| {});
        //                 Menu::new(
        //                     cx,
        //                     |cx| Label::new(cx, "menu 2a"),
        //                     |cx| {
        //                         MenuButton::new_check_simple(
        //                             cx,
        //                             "option 1",
        //                             |_| {},
        //                             AppData::check1,
        //                         );
        //                         MenuButton::new_check_simple(
        //                             cx,
        //                             "option 2",
        //                             |_| {},
        //                             AppData::check2,
        //                         );
        //                         MenuButton::new_simple(cx, "loooooooooooooooooooooooooong", |_| {});
        //                     },
        //                 );
        //             },
        //         );
        //     });
        // });
    })
    .title("Menu")
    .run();
}
