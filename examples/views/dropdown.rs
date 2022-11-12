use vizia::fonts::icons_names::DOWN;
use vizia::prelude::*;

#[derive(Lens, Model, Setter)]
pub struct AppData {
    list: Vec<String>,
    choice: String,
}

#[allow(dead_code)]
const DARK_THEME: &str = "crates/vizia_core/resources/themes/dark_theme.css";
#[allow(dead_code)]
const LIGHT_THEME: &str = "crates/vizia_core/resources/themes/light_theme.css";

fn main() {
    Application::new(|cx| {
        AppData {
            list: vec!["Red".to_string(), "Green".to_string(), "Blue".to_string()],
            choice: "Red".to_string(),
        }
        .build(cx);

        cx.add_stylesheet(DARK_THEME).expect("Failed to find stylesheet");

        // Dropdown List
        Dropdown::new(
            cx,
            move |cx| {
                // A Label and an Icon
                Label::new(cx, AppData::choice)
            },
            move |cx| {
                List::new(cx, AppData::list, |cx, _, item| {
                    Label::new(cx, item)
                        .width(Stretch(1.0))
                        .child_top(Stretch(1.0))
                        .child_bottom(Stretch(1.0))
                        .border_radius(Units::Pixels(4.0))
                        .bind(AppData::choice, move |handle, selected| {
                            if item.get(handle.cx) == selected.get(handle.cx) {
                                handle.checked(true);
                            }
                        })
                        .on_press(move |cx| {
                            cx.emit(AppDataSetter::Choice(item.get(cx).clone()));
                            cx.emit(PopupEvent::Close);
                        });
                });
            },
        )
        .width(Pixels(100.0));
    })
    .ignore_default_theme()
    .title("Dropdown")
    .run();
}
