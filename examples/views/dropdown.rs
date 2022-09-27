use vizia::fonts::icons_names::DOWN;
use vizia::prelude::*;

#[derive(Lens, Model, Setter)]
pub struct AppData {
    list: Vec<String>,
    choice: String,
}

#[derive(Debug)]
pub enum AppEvent {
    SetChoice(String),
}

impl Model for AppData {
    fn event(&mut self, _: &mut EventContext, event: &mut Event) {
        event.map(|app_event, _| match app_event {
            AppEvent::SetChoice(choice) => {
                self.choice = choice.clone();
            }
        });
    }
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

        VStack::new(cx, |cx| {
            VStack::new(cx, |cx| {
                // Dropdown List
                Dropdown::new(
                    cx,
                    move |cx| {
                        // A Label and an Icon
                        HStack::new(cx, move |cx| {
                            Label::new(cx, AppData::choice);
                            Label::new(cx, DOWN).font("icons");
                        })
                        .child_left(Pixels(5.0))
                        .child_right(Pixels(5.0))
                        .col_between(Stretch(1.0))
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
                                    cx.emit(AppEvent::SetChoice(item.get(cx).clone()));
                                    cx.emit(PopupEvent::Close);
                                });
                        });
                    },
                )
                .width(Pixels(100.0));
            })
            .class("container");
        })
        .class("main");
    })
    .ignore_default_theme()
    .title("Dropdown")
    .run();
}
