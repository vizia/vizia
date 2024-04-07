mod helpers;
use helpers::*;
use vizia::prelude::*;

fn main() -> Result<(), ApplicationError> {
    Application::new(|cx: &mut Context| {
        ExamplePage::new(cx, |cx| {
            Submenu::new(
                cx,
                |cx| Label::new(cx, "Menu"),
                |cx| {
                    MenuButton::new(
                        cx,
                        |_| println!("New"),
                        |cx| {
                            HStack::new(cx, |cx| {
                                Label::new(cx, "New");
                                Label::new(cx, "Ctrl + N").class("shortcut");
                            })
                        },
                    );
                    MenuButton::new(
                        cx,
                        |_| println!("Open"),
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
                },
            )
            .width(Pixels(100.0));
        });
    })
    .title("Menu")
    .run()
}
