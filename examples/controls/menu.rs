use vizia::*;
use vizia_glutin::application::Application;

#[derive(Lens)]
pub struct AppData {
    pub check1: bool,
    pub check2: bool,
}

impl Model for AppData {}

fn main() {
    Application::new(WindowDescription::new(), |cx| {
        AppData { check1: false, check2: true }.build(cx);
        MenuStack::new_horizontal(cx, |cx| {
            MenuButton::new_simple(cx, "option 1", |_| {});
            Menu::new(
                cx,
                |cx| Label::new(cx, "menu 1"),
                |cx| {
                    MenuButton::new_simple(cx, "option 1", |_| {});
                    MenuButton::new_simple(cx, "option 2 looooooooooooong", |_| {});
                    Menu::new(
                        cx,
                        |cx| Label::new(cx, "menu 2"),
                        |cx| {
                            MenuButton::new_check_simple(cx, "option 1", |_| {}, AppData::check1);
                            MenuButton::new_check_simple(cx, "option 2", |_| {}, AppData::check2);
                            MenuButton::new_simple(cx, "loooooooooooooooooooooooooong", |_| {});
                        },
                    );
                },
            );
            MenuButton::new_simple(cx, "option 3", |_| {});
        });
    })
    .run();
}
