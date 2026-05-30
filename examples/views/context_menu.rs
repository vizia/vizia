mod helpers;
use helpers::*;
use log::debug;
use vizia::prelude::*;

const STYLE: &str = r#"
    .context-target {
        width: 320px;
        height: 180px;
        border-width: 1px;
        border-color: black;
        corner-radius: 6px;
        alignment: center;
        cursor: context-menu;
    }

    .context-target > vstack {
        alignment: center;
        gap: 8px;
    }
"#;

fn main() -> Result<(), ApplicationError> {
    Application::new(|cx: &mut Context| {
        cx.add_stylesheet(STYLE).expect("Failed to add stylesheet");

        ExamplePage::new(cx, |cx| {
            VStack::new(cx, |cx| {
                Label::new(cx, "Right-click inside the box");
                Label::new(cx, "to open the context menu");
            })
            .class("context-target")
            .menu(|cx| {
                Menu::new(cx, Placement::Cursor, true, |cx| {
                    MenuButton::new(cx, |_| debug!("Cut"), |cx| Label::new(cx, "Cut"));
                    MenuButton::new(cx, |_| debug!("Copy"), |cx| Label::new(cx, "Copy"));
                    MenuButton::new(cx, |_| debug!("Paste"), |cx| Label::new(cx, "Paste"));
                    Divider::new(cx);
                    Submenu::new(
                        cx,
                        |cx| Label::new(cx, "Sort"),
                        |cx| {
                            MenuButton::new(
                                cx,
                                |_| debug!("Sort ascending"),
                                |cx| Label::new(cx, "Ascending"),
                            );
                            MenuButton::new(
                                cx,
                                |_| debug!("Sort descending"),
                                |cx| Label::new(cx, "Descending"),
                            );
                        },
                    );
                })
            });
        });
    })
    .title("Context Menu")
    .run()
}
