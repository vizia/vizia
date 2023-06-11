use vizia::prelude::*;

const STYLE: &str = r#"
    textbox {
        width: 1s;
    }

    hstack {
        height: auto;
        col-between: 10px;
        child-top: 1s;
        child-bottom: 1s;
    }

    vstack {
        height: 1s;
        row-between: 10px;
    }

    button {
        width: 1s;
        child-space: 1s;
    }

    list label {
        width: 1s;
        height: 32px;
        child-left: 5px;
        child-top: 1s;
        child-bottom: 1s;
    }

    list label:checked {
        background-color: #5050AA40;
    }

    list {
        border-color: white;
        border-width: 1px;
        width: 1s;
        height: 1s;
    }
"#;

#[derive(Lens)]
pub struct AppData {
    filter_prefix: String,
    list: Vec<(String, String)>,
    selected: Option<usize>,
    name: String,
    surname: String,
}

pub enum AppEvent {
    SetSelected(usize),
    SetName(String),
    SetSurname(String),
    Create,
    Update,
    Delete,
}

impl Model for AppData {
    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|app_event, _| match app_event {
            AppEvent::SetSelected(index) => {
                self.selected = Some(*index);
                self.name = self.list[*index].0.clone();
                self.surname = self.list[*index].1.clone();
            }

            AppEvent::SetName(name) => {
                self.name = name.clone();
            }

            AppEvent::SetSurname(surname) => {
                self.surname = surname.clone();
            }

            AppEvent::Create => {
                if !self.name.is_empty() && !self.surname.is_empty() {
                    self.list.push((self.name.clone(), self.surname.clone()));
                    self.selected = Some(self.list.len() - 1);
                }
            }

            AppEvent::Update => {
                if let Some(selected) = self.selected {
                    self.list[selected].0 = self.name.clone();
                    self.list[selected].1 = self.surname.clone();
                }
            }

            AppEvent::Delete => {
                if let Some(selected) = self.selected {
                    self.list.remove(selected);
                    if self.list.is_empty() {
                        self.selected = None;
                        self.name = String::new();
                        self.surname = String::new();
                    } else {
                        cx.emit(AppEvent::SetSelected(selected.saturating_sub(1)));
                    }
                }
            }
        });
    }
}

fn main() {
    Application::new(|cx| {
        cx.add_stylesheet(STYLE).expect("Failed to add stylesheet");

        AppData {
            filter_prefix: "".to_string(),
            list: vec![("John".to_string(), "Smith".to_string())],
            selected: None,
            name: "".to_string(),
            surname: "".to_string(),
        }
        .build(cx);

        VStack::new(cx, |cx| {
            HStack::new(cx, |cx| {
                VStack::new(cx, |cx| {
                    HStack::new(cx, |cx| {
                        Label::new(cx, "Filter prefix:");
                        Textbox::new(cx, AppData::filter_prefix);
                    });

                    List::new(cx, AppData::list, |cx, index, item| {
                        Label::new(
                            cx,
                            item.map(|(name, surname)| format!("{}, {}", surname, name)),
                        )
                        .on_press(move |cx| {
                            cx.emit(AppEvent::SetSelected(index));
                        })
                        .checked(AppData::selected.map(move |selected| *selected == Some(index)));
                    });
                });

                VStack::new(cx, |cx| {
                    HStack::new(cx, |cx| {
                        Label::new(cx, "Name:").width(Pixels(80.0));

                        Textbox::new(cx, AppData::name).on_edit(move |cx, text| {
                            cx.emit(AppEvent::SetName(text.clone()));
                        });
                    });

                    HStack::new(cx, |cx| {
                        Label::new(cx, "Surname:").width(Pixels(80.0));

                        Textbox::new(cx, AppData::surname).on_edit(move |cx, text| {
                            cx.emit(AppEvent::SetSurname(text.clone()));
                        });
                    });
                });
            })
            .height(Stretch(1.0))
            .child_top(Pixels(0.0))
            .child_bottom(Pixels(0.0));

            HStack::new(cx, |cx| {
                Button::new(cx, |cx| cx.emit(AppEvent::Create), |cx| Label::new(cx, "Create"));
                Button::new(cx, |cx| cx.emit(AppEvent::Update), |cx| Label::new(cx, "Update"));
                Button::new(cx, |cx| cx.emit(AppEvent::Delete), |cx| Label::new(cx, "Delete"));
            })
            .col_between(Pixels(10.0));
        })
        .child_space(Pixels(10.0));
    })
    .title("CRUD")
    .inner_size((450, 200))
    .run();
}
