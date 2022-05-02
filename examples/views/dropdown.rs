use vizia::fonts::icons_names::DOWN;
use vizia::prelude::*;

#[derive(Lens)]
pub struct AppData {
    list: Vec<String>,
    choice: String,
}

#[derive(Debug)]
pub enum AppEvent {
    SetChoice(String),
}

impl Model for AppData {
    fn event(&mut self, _: &mut Context, event: &mut Event) {
        event.map(|app_event, _| match app_event {
            AppEvent::SetChoice(choice) => {
                self.choice = choice.clone();
            }
        });
    }
}

fn main() {
    Application::new(|cx| {
        AppData {
            list: vec!["Red".to_string(), "Green".to_string(), "Blue".to_string()],
            choice: "Red".to_string(),
        }
        .build(cx);

        HStack::new(cx, move |cx| {
            // Dropdown List
            Dropdown::new(
                cx,
                move |cx|
                // A Label and an Icon
                HStack::new(cx, move |cx|{
                    Label::new(cx, AppData::choice);
                    Label::new(cx, DOWN).font("icons");
                })
                .child_left(Pixels(5.0))
                .child_right(Pixels(5.0))
                .col_between(Stretch(1.0)),
                move |cx| {
                    List::new(cx, AppData::list, |cx, _, item| {
                        Label::new(cx, item)
                            .width(Stretch(1.0))
                            .child_top(Stretch(1.0))
                            .child_bottom(Stretch(1.0))
                            .bind(AppData::choice, move |handle, selected| {
                                if item.get(handle.cx) == selected.get(handle.cx) {
                                    handle.background_color(Color::from("#f8ac14"));
                                } else {
                                    handle.background_color(Color::white());
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
        .child_space(Stretch(1.0));
    })
    .title("Dropdown")
    .run();
}
