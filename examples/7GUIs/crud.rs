use vizia::*;

const STYLE: &str = r#"
    textbox {
        width: 1s;
        height: 30px;
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
        width: 100px;
    }

    label {
        width: 1s;
        height: 30px;
        child-left: 5px;
    }

    label:checked {
        background-color: blue;
        color: white;
    }

    list {
        border-color: black;
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
    fn event(&mut self, cx: &mut Context, event: &mut Event) {
        if let Some(app_event) = event.try_mut() {
            match app_event {
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
            }
        }
    }
}

fn main() {
    let window_description = WindowDescription::new().with_title("CRUD").with_inner_size(450, 200);
    Application::new(window_description, |cx| {
        cx.add_theme(STYLE);

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
                        Textbox::new(cx, AppData::filter_prefix).width(Pixels(80.0));
                    });

                    List::new(cx, AppData::list, |cx, index, item| {
                        Binding::new(cx, AppData::selected, move |cx, selected| {
                            let is_selected = if let Some(selected) = *selected.get(cx) {
                                selected == index
                            } else {
                                false
                            };
                            Binding::new(cx, item, move |cx, item| {
                                let (name, surname) = item.get(cx).clone();
                                Label::new(cx, &format!("{}, {}", surname, name))
                                    .on_press(move |cx| {
                                        cx.emit(AppEvent::SetSelected(index));
                                    })
                                    .checked(is_selected);
                            });
                        });
                    });
                });

                VStack::new(cx, |cx| {
                    HStack::new(cx, |cx| {
                        Label::new(cx, "Name:");

                        Textbox::new(cx, AppData::name)
                            .on_edit(move |cx, text| {
                                cx.emit(AppEvent::SetName(text.clone()));
                            })
                            .width(Pixels(120.0));
                    });

                    HStack::new(cx, |cx| {
                        Label::new(cx, "Surname:");

                        Textbox::new(cx, AppData::surname)
                            .on_edit(move |cx, text| {
                                cx.emit(AppEvent::SetSurname(text.clone()));
                            })
                            .width(Pixels(120.0));
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
    .run();
}
