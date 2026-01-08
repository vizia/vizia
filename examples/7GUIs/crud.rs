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

    .crud-list label {
        width: 1s;
        height: 32px;
        padding-left: 5px;
        padding-top: 1s;
        padding-bottom: 1s;
    }

    .crud-list label:checked {
        background-color: #5050AA40;
    }

    .crud-list {
        border-color: #d2d2d2;
        border-width: 1px;
        width: 1s;
        height: 1s;
    }
"#;

enum CRUDEvent {
    SetSelected(usize),
    Create,
    Update,
    Delete,
}

struct CRUDApp {
    filter_prefix: Signal<String>,
    list: Signal<Vec<(String, String)>>,
    selected: Signal<Option<usize>>,
    name: Signal<String>,
    surname: Signal<String>,
}

impl App for CRUDApp {
    fn new(cx: &mut Context) -> Self {
        Self {
            filter_prefix: cx.state(String::new()),
            list: cx.state(vec![("John".to_string(), "Smith".to_string())]),
            selected: cx.state(None::<usize>),
            name: cx.state(String::new()),
            surname: cx.state(String::new()),
        }
    }

    fn on_build(self, cx: &mut Context) -> Self {
        cx.add_stylesheet(STYLE).expect("Failed to add stylesheet");

        let filter_prefix = self.filter_prefix;
        let list = self.list;
        let selected = self.selected;
        let name = self.name;
        let surname = self.surname;

        VStack::new(cx, move |cx| {
            HStack::new(cx, move |cx| {
                VStack::new(cx, move |cx| {
                    HStack::new(cx, move |cx| {
                        Label::new(cx, "Filter prefix:");
                        Textbox::new(cx, filter_prefix).on_edit(move |cx, text| {
                            filter_prefix.set(cx, text);
                        });
                    });

                    // Custom list using Binding to rebuild on data changes
                    ScrollView::new(cx, move |cx| {
                        VStack::new(cx, move |cx| {
                            Binding::new(cx, list, move |cx| {
                                let items = list.get(cx).clone();
                                let filter = filter_prefix.get(cx).to_lowercase();
                                for (index, (first, last)) in items.iter().enumerate() {
                                    let display = format!("{}, {}", last, first);
                                    if filter.is_empty() || last.to_lowercase().starts_with(&filter) {
                                        let display_signal = cx.state(display);
                                        let is_selected =
                                            selected.drv(cx, move |v, _| *v == Some(index));
                                        Label::new(cx, display_signal)
                                            .on_press(move |cx| {
                                                cx.emit(CRUDEvent::SetSelected(index));
                                            })
                                            .navigable(true)
                                            .checked(is_selected);
                                    }
                                }
                            });
                        })
                        .height(Auto);
                    })
                    .class("crud-list");
                });

                VStack::new(cx, move |cx| {
                    HStack::new(cx, move |cx| {
                        Label::new(cx, "Name:").width(Pixels(80.0));
                        Textbox::new(cx, name).on_edit(move |cx, text| {
                            name.set(cx, text);
                        });
                    });

                    HStack::new(cx, move |cx| {
                        Label::new(cx, "Surname:").width(Pixels(80.0));
                        Textbox::new(cx, surname).on_edit(move |cx, text| {
                            surname.set(cx, text);
                        });
                    });
                });
            })
            .height(Stretch(1.0))
            .padding_top(Pixels(0.0))
            .padding_bottom(Pixels(0.0));

            HStack::new(cx, move |cx| {
                Button::new(cx, |cx| Label::new(cx, "Create"))
                    .on_press(move |cx| cx.emit(CRUDEvent::Create));
                Button::new(cx, |cx| Label::new(cx, "Update"))
                    .on_press(move |cx| cx.emit(CRUDEvent::Update));
                Button::new(cx, |cx| Label::new(cx, "Delete"))
                    .on_press(move |cx| cx.emit(CRUDEvent::Delete));
            })
            .horizontal_gap(Pixels(10.0));
        })
        .padding(Pixels(10.0));

        self
    }

    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|crud_event, _| match crud_event {
            CRUDEvent::SetSelected(index) => {
                self.selected.set(cx, Some(*index));
                let items = self.list.get(cx);
                let name_surname = if *index < items.len() {
                    Some((items[*index].0.clone(), items[*index].1.clone()))
                } else {
                    None
                };
                if let Some((first, last)) = name_surname {
                    self.name.set(cx, first);
                    self.surname.set(cx, last);
                }
            }

            CRUDEvent::Create => {
                let name_val = self.name.get(cx).clone();
                let surname_val = self.surname.get(cx).clone();
                if !name_val.is_empty() && !surname_val.is_empty() {
                    self.list.upd(cx, |items| {
                        items.push((name_val, surname_val));
                    });
                    let new_index = self.list.get(cx).len() - 1;
                    self.selected.set(cx, Some(new_index));
                }
            }

            CRUDEvent::Update => {
                if let Some(sel) = *self.selected.get(cx) {
                    let name_val = self.name.get(cx).clone();
                    let surname_val = self.surname.get(cx).clone();
                    self.list.upd(cx, |items| {
                        if let Some(item) = items.get_mut(sel) {
                            item.0 = name_val;
                            item.1 = surname_val;
                        }
                    });
                }
            }

            CRUDEvent::Delete => {
                if let Some(sel) = *self.selected.get(cx) {
                    self.list.upd(cx, |items| {
                        if sel < items.len() {
                            items.remove(sel);
                        }
                    });

                    let list_len = self.list.get(cx).len();
                    if list_len == 0 {
                        self.selected.set(cx, None);
                        self.name.set(cx, String::new());
                        self.surname.set(cx, String::new());
                    } else {
                        let new_sel = sel.saturating_sub(1).min(list_len - 1);
                        cx.emit(CRUDEvent::SetSelected(new_sel));
                    }
                }
            }
        });
    }

    fn window_config(&self) -> WindowConfig {
        window(|app| app.inner_size((450, 200)))
    }
}

fn main() -> Result<(), ApplicationError> {
    CRUDApp::run()
}
