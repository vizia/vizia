use vizia::*;

fn main() {
    Application::new(WindowDescription::new().with_title("Table"), |cx| {
        let people = vec![
            Person { first_name: "Peter".to_string(), last_name: "Pan".to_string(), age: 14 },
            Person { first_name: "Mary".to_string(), last_name: "Poppins".to_string(), age: 32 },
            Person { first_name: "John".to_string(), last_name: "Doe".to_string(), age: 54 },
            Person { first_name: "Jane".to_string(), last_name: "Doe".to_string(), age: 47 },
            Person { first_name: "Simon".to_string(), last_name: "Fields".to_string(), age: 19 },
        ];

        TableData {
            people,
            first_name_sorted: Sorted::None,
            last_name_sorted: Sorted::None,
            age_sorted: Sorted::None,
        }
        .build(cx);

        Table::new(cx, TableData::people, |cx, list| {
            TableColumn::new(
                cx,
                list,
                Person::first_name,
                |cx| {
                    VStack::new(cx, move |cx| {
                        HStack::new(cx, |cx| {
                            Label::new(cx, "First Name")
                                .child_left(Pixels(5.0))
                                .width(Stretch(1.0))
                                .height(Stretch(1.0))
                                .font("roboto-bold");
                            Binding::new(cx, TableData::first_name_sorted, |cx, sorted| {
                                let visible = match *sorted.get(cx) {
                                    Sorted::Forward | Sorted::Reverse => true,
                                    Sorted::None => false,
                                };
                                let icon = if *sorted.get(cx) == Sorted::Forward {
                                    "\u{e75c}"
                                } else {
                                    "\u{e75f}"
                                };
                                Element::new(cx)
                                    .width(Pixels(30.0))
                                    .child_space(Stretch(1.0))
                                    .text(icon)
                                    .font("icons")
                                    .visibility(visible);
                            });

                            Element::new(cx)
                                .width(Pixels(1.0))
                                .height(Percentage(80.0))
                                .top(Stretch(1.0))
                                .bottom(Stretch(1.0))
                                .background_color(Color::rgb(150, 150, 150));
                        })
                        .on_press(|cx| cx.emit(AppEvent::ToggleSortFirstName));

                        Element::new(cx)
                            .height(Pixels(1.0))
                            .background_color(Color::rgb(150, 150, 150));
                    })
                    .height(Pixels(30.0));
                },
                |cx, index, first_name| {
                    Label::new(cx, first_name)
                        .child_left(Pixels(5.0))
                        .width(Stretch(1.0))
                        .height(Pixels(30.0))
                        .border_radius_top_left(Pixels(3.0))
                        .border_radius_bottom_left(Pixels(3.0))
                        .background_color(if index % 2 != 0 {
                            Color::rgb(230, 230, 230)
                        } else {
                            Color::white()
                        });
                },
            )
            .width(Pixels(200.0));

            TableColumn::new(
                cx,
                list,
                Person::last_name,
                |cx| {
                    VStack::new(cx, move |cx| {
                        HStack::new(cx, |cx| {
                            Label::new(cx, "Last Name")
                                .child_left(Pixels(5.0))
                                .width(Stretch(1.0))
                                .height(Stretch(1.0))
                                .font("roboto-bold");
                            Binding::new(cx, TableData::last_name_sorted, |cx, sorted| {
                                let visible = match *sorted.get(cx) {
                                    Sorted::Forward | Sorted::Reverse => true,
                                    Sorted::None => false,
                                };
                                let icon = if *sorted.get(cx) == Sorted::Forward {
                                    "\u{e75c}"
                                } else {
                                    "\u{e75f}"
                                };
                                Element::new(cx)
                                    .width(Pixels(30.0))
                                    .child_space(Stretch(1.0))
                                    .text(icon)
                                    .font("icons")
                                    .visibility(visible);
                            });
                            Element::new(cx)
                                .width(Pixels(1.0))
                                .height(Percentage(80.0))
                                .top(Stretch(1.0))
                                .bottom(Stretch(1.0))
                                .background_color(Color::rgb(150, 150, 150));
                        })
                        .on_press(|cx| cx.emit(AppEvent::ToggleSortLastName));

                        Element::new(cx)
                            .height(Pixels(1.0))
                            .background_color(Color::rgb(150, 150, 150));
                    })
                    .height(Pixels(30.0));
                },
                |cx, index, last_name| {
                    Label::new(cx, last_name)
                        .child_left(Pixels(5.0))
                        .width(Stretch(1.0))
                        .height(Pixels(30.0))
                        .background_color(if index % 2 != 0 {
                            Color::rgb(230, 230, 230)
                        } else {
                            Color::white()
                        });
                },
            )
            .width(Pixels(200.0));

            TableColumn::new(
                cx,
                list,
                Person::age,
                |cx| {
                    VStack::new(cx, move |cx| {
                        HStack::new(cx, |cx| {
                            Label::new(cx, "Age")
                                .child_left(Pixels(5.0))
                                .width(Stretch(1.0))
                                .height(Stretch(1.0))
                                .font("roboto-bold");

                            Binding::new(cx, TableData::age_sorted, |cx, sorted| {
                                let visible = match *sorted.get(cx) {
                                    Sorted::Forward | Sorted::Reverse => true,
                                    Sorted::None => false,
                                };
                                let icon = if *sorted.get(cx) == Sorted::Forward {
                                    "\u{e75c}"
                                } else {
                                    "\u{e75f}"
                                };
                                Element::new(cx)
                                    .width(Pixels(30.0))
                                    .child_space(Stretch(1.0))
                                    .text(icon)
                                    .font("icons")
                                    .visibility(visible);
                            });
                        })
                        .on_press(|cx| cx.emit(AppEvent::ToggleSortAge));

                        Element::new(cx)
                            .height(Pixels(1.0))
                            .background_color(Color::rgb(150, 150, 150));
                    })
                    .height(Pixels(30.0));
                },
                |cx, index, age| {
                    Label::new(cx, age)
                        .child_left(Pixels(5.0))
                        .width(Stretch(1.0))
                        .height(Pixels(30.0))
                        .border_radius_top_right(Pixels(3.0))
                        .border_radius_bottom_right(Pixels(3.0))
                        .background_color(if index % 2 != 0 {
                            Color::rgb(230, 230, 230)
                        } else {
                            Color::white()
                        });
                },
            )
            .width(Pixels(200.0));
        })
        .space(Stretch(1.0));
    })
    .run();
}

#[derive(Debug, Clone, Lens, Data)]
pub struct Person {
    first_name: String,
    last_name: String,
    age: i32,
}

impl Model for Person {}

#[derive(Debug, Lens)]
pub struct TableData {
    people: Vec<Person>,
    first_name_sorted: Sorted,
    last_name_sorted: Sorted,
    age_sorted: Sorted,
}

pub enum AppEvent {
    SetAge(usize, String),
    Print,
    ToggleSortFirstName,
    ToggleSortLastName,
    ToggleSortAge,
}

#[derive(Debug, Clone, PartialEq, Data)]
pub enum Sorted {
    Forward,
    Reverse,
    None,
}

impl Model for TableData {
    fn event(&mut self, _: &mut Context, event: &mut Event) {
        if let Some(app_event) = event.message.downcast() {
            match app_event {
                AppEvent::SetAge(index, age) => {
                    self.people[*index].last_name = age.clone();
                }

                AppEvent::Print => {
                    println!("{:?}", self.people);
                }

                AppEvent::ToggleSortFirstName => {
                    match self.first_name_sorted {
                        Sorted::Forward => {
                            self.first_name_sorted = {
                                self.people.sort_by_cached_key(|person| person.first_name.clone());
                                self.people.reverse();
                                Sorted::Reverse
                            }
                        }
                        Sorted::Reverse => {
                            self.first_name_sorted = {
                                self.people.sort_by_cached_key(|person| person.first_name.clone());
                                Sorted::Forward
                            }
                        }
                        Sorted::None => {
                            self.first_name_sorted = {
                                self.people.sort_by_cached_key(|person| person.first_name.clone());
                                Sorted::Forward
                            }
                        }
                    }

                    self.last_name_sorted = Sorted::None;
                    self.age_sorted = Sorted::None;
                }

                AppEvent::ToggleSortLastName => {
                    match self.last_name_sorted {
                        Sorted::Forward => {
                            self.last_name_sorted = {
                                self.people.sort_by_cached_key(|person| person.last_name.clone());
                                self.people.reverse();
                                Sorted::Reverse
                            }
                        }
                        Sorted::Reverse => {
                            self.last_name_sorted = {
                                self.people.sort_by_cached_key(|person| person.last_name.clone());
                                Sorted::Forward
                            }
                        }
                        Sorted::None => {
                            self.last_name_sorted = {
                                self.people.sort_by_cached_key(|person| person.last_name.clone());
                                Sorted::Forward
                            }
                        }
                    }

                    self.first_name_sorted = Sorted::None;
                    self.age_sorted = Sorted::None;
                }

                AppEvent::ToggleSortAge => {
                    match self.age_sorted {
                        Sorted::Forward => {
                            self.age_sorted = {
                                self.people.sort_by_cached_key(|person| person.age.clone());
                                self.people.reverse();
                                Sorted::Reverse
                            }
                        }
                        Sorted::Reverse => {
                            self.age_sorted = {
                                self.people.sort_by_cached_key(|person| person.age.clone());
                                Sorted::Forward
                            }
                        }
                        Sorted::None => {
                            self.age_sorted = {
                                self.people.sort_by_cached_key(|person| person.age.clone());
                                Sorted::Forward
                            }
                        }
                    }

                    self.first_name_sorted = Sorted::None;
                    self.last_name_sorted = Sorted::None;
                }
            }
        }
    }
}
