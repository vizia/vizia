use vizia::prelude::*;

const STYLE: &str = r#"
    textbox {
        width: 1s;
    }

    hstack {
        height: auto;
        horizontal-gap: 10px;
        padding-top: 1s;
        padding-bottom: 1s;
    }

    vstack {
        height: 1s;
        vertical-gap: 10px;
    }

    button {
        width: 1s;
        padding: 1s;
    }

    list label {
        width: 1s;
        height: 32px;
        padding-left: 5px;
        padding-top: 1s;
        padding-bottom: 1s;
    }

    list label:checked {
        background-color: #5050AA40;
    }

    list {
        border-color: #d2d2d2;
        border-width: 1px;
        width: 1s;
        height: 1s;
    }
"#;

pub struct AppData {
    list: Signal<Vec<Signal<(String, String)>>>,
    selected: Signal<Option<usize>>,
    name: Signal<String>,
    surname: Signal<String>,
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
                self.selected.set(Some(*index));
                let (name, surname) = self.list.get()[*index].get();
                self.name.set(name);
                self.surname.set(surname);
            }

            AppEvent::SetName(name) => {
                self.name.set(name.clone());
            }

            AppEvent::SetSurname(surname) => {
                self.surname.set(surname.clone());
            }

            AppEvent::Create => {
                let name = self.name.get();
                let surname = self.surname.get();

                if !name.is_empty() && !surname.is_empty() {
                    self.list.update(|list| {
                        list.push(Signal::new((name.clone(), surname.clone())));
                    });
                    let len = self.list.get().len();
                    self.selected.set(Some(len - 1));
                }
            }

            AppEvent::Update => {
                if let Some(selected) = self.selected.get() {
                    let name = self.name.get();
                    let surname = self.surname.get();
                    self.list.get()[selected].set((name, surname));
                }
            }

            AppEvent::Delete => {
                if let Some(selected) = self.selected.get() {
                    self.list.update(|list| {
                        list.remove(selected);
                    });
                    if self.list.get().is_empty() {
                        self.selected.set(None);
                        self.name.set(String::new());
                        self.surname.set(String::new());
                    } else {
                        cx.emit(AppEvent::SetSelected(selected.saturating_sub(1)));
                    }
                }
            }
        });
    }
}

fn main() -> Result<(), ApplicationError> {
    Application::new(|cx| {
        cx.add_stylesheet(STYLE).expect("Failed to add stylesheet");

        let filter_prefix = Signal::new("".to_string());
        let list = Signal::new(vec![Signal::new(("John".to_string(), "Smith".to_string()))]);
        let selected = Signal::new(None::<usize>);
        let name = Signal::new("".to_string());
        let surname = Signal::new("".to_string());

        AppData { list, selected, name, surname }.build(cx);

        VStack::new(cx, |cx| {
            HStack::new(cx, |cx| {
                VStack::new(cx, |cx| {
                    HStack::new(cx, |cx| {
                        Label::new(cx, "Filter prefix:");
                        Textbox::new(cx, filter_prefix);
                    });

                    List::new(cx, list, move |cx, index, item| {
                        let label_text = Memo::new(move |_| {
                            let item = item.get();
                            format!("{}, {}", item.1, item.0)
                        });
                        let is_selected = Memo::new(move |_| selected.get() == Some(index));

                        Label::new(cx, label_text)
                            .on_press(move |cx| {
                                cx.emit(AppEvent::SetSelected(index));
                            })
                            .navigable(true)
                            .checked(is_selected);
                    });
                });

                VStack::new(cx, |cx| {
                    HStack::new(cx, |cx| {
                        Label::new(cx, "Name:").width(Pixels(80.0));

                        Textbox::new(cx, name).on_edit(move |cx, text| {
                            cx.emit(AppEvent::SetName(text.clone()));
                        });
                    });

                    HStack::new(cx, |cx| {
                        Label::new(cx, "Surname:").width(Pixels(80.0));

                        Textbox::new(cx, surname).on_edit(move |cx, text| {
                            cx.emit(AppEvent::SetSurname(text.clone()));
                        });
                    });
                });
            })
            .height(Stretch(1.0))
            .padding_top(Pixels(0.0))
            .padding_bottom(Pixels(0.0));

            HStack::new(cx, |cx| {
                Button::new(cx, |cx| Label::new(cx, "Create"))
                    .on_press(|cx| cx.emit(AppEvent::Create));
                Button::new(cx, |cx| Label::new(cx, "Update"))
                    .on_press(|cx| cx.emit(AppEvent::Update));
                Button::new(cx, |cx| Label::new(cx, "Delete"))
                    .on_press(|cx| cx.emit(AppEvent::Delete));
            })
            .horizontal_gap(Pixels(10.0));
        })
        .padding(Pixels(10.0));
    })
    .title("CRUD")
    .inner_size((450, 200))
    .run()
}
