use vizia::prelude::*;

const STYLE: &str = r#"
    * {
        border-width: 1px;
        border-color: red;
    }
"#;

#[derive(Lens)]
pub struct AppData {}

impl Model for AppData {}

fn main() {
    Application::new(|cx: &mut Context| {
        cx.add_theme(STYLE);

        AppData {}.build(cx);

        MenuBar::new(cx, |cx| {
            MenuButton::new(cx).width(Pixels(100.0)).height(Pixels(30.0));
            MenuButton::new(cx).width(Pixels(100.0)).height(Pixels(30.0));
            MenuButton::new(cx).width(Pixels(100.0)).height(Pixels(30.0));
            MenuButton::new(cx).width(Pixels(100.0)).height(Pixels(30.0));
        })
        .height(Pixels(30.0))
        .width(Stretch(1.0));

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
