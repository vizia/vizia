use vizia::prelude::*;

#[derive(Lens)]
pub struct AppData {
    pub check1: bool,
    pub check2: bool,
}

impl Model for AppData {}

#[allow(dead_code)]
const DARK_THEME: &str = "crates/vizia_core/resources/themes/dark_theme.css";
#[allow(dead_code)]
const LIGHT_THEME: &str = "crates/vizia_core/resources/themes/light_theme.css";

fn main() {
    Application::new(|cx| {
        cx.add_stylesheet(DARK_THEME).expect("Failed to find stylesheet");

        AppData { check1: false, check2: true }.build(cx);

        //MenuBar::new(cx);

        /*
        MenuController::new(cx, false, |cx| {
            MenuStack::new_horizontal(cx, |cx| {
                Menu::new(
                    cx,
                    |cx| Label::new(cx, "menu 1"),
                    |cx| {
                        MenuButton::new_simple(cx, "option 1", |_| {});
                        MenuButton::new_simple(cx, "option 2 looooooooooooong", |_| {});
                        Menu::new(
                            cx,
                            |cx| Label::new(cx, "menu 1a"),
                            |cx| {
                                MenuButton::new_check_simple(
                                    cx,
                                    "option 1",
                                    |_| {},
                                    AppData::check1,
                                );
                                MenuButton::new_check_simple(
                                    cx,
                                    "option 2",
                                    |_| {},
                                    AppData::check2,
                                );
                                MenuButton::new_simple(cx, "loooooooooooooooooooooooooong", |_| {});
                            },
                        );
                    },
                );
                Menu::new(
                    cx,
                    |cx| Label::new(cx, "menu 2"),
                    |cx| {
                        MenuButton::new_simple(cx, "option 1", |_| {});
                        MenuButton::new_simple(cx, "option 2 looooooooooooong", |_| {});
                        Menu::new(
                            cx,
                            |cx| Label::new(cx, "menu 2a"),
                            |cx| {
                                MenuButton::new_check_simple(
                                    cx,
                                    "option 1",
                                    |_| {},
                                    AppData::check1,
                                );
                                MenuButton::new_check_simple(
                                    cx,
                                    "option 2",
                                    |_| {},
                                    AppData::check2,
                                );
                                MenuButton::new_simple(cx, "loooooooooooooooooooooooooong", |_| {});
                            },
                        );
                    },
                );
            });
        });
        */
    })
    .ignore_default_theme()
    .title("Menu")
    .run();
}
