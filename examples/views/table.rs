use vizia::fonts::icons_names::{DOWN, UP};
use vizia::prelude::*;

#[allow(dead_code)]
const DARK_THEME: &str = "crates/vizia_core/resources/themes/dark_theme.css";
#[allow(dead_code)]
const LIGHT_THEME: &str = "crates/vizia_core/resources/themes/light_theme.css";

fn column_header<L, M>(cx: &mut Context, text: &str, lens: L, on_press: M)
where
    L: Lens<Target = Sorted>,
    <L as Lens>::Target: Data,
    M: 'static + Send + Sync + Clone,
{
    VStack::new(cx, move |cx| {
        Element::new(cx).class("table-row-divisor");

        HStack::new(cx, |cx| {
            Label::new(cx, text).class("table-column-title");
            Binding::new(cx, lens, |cx, sorted| {
                let visible = match sorted.get(cx) {
                    Sorted::Forward | Sorted::Reverse => true,
                    Sorted::None => false,
                };
                let icon = if sorted.get(cx) == Sorted::Forward { DOWN } else { UP };
                Element::new(cx).class("table-column-icon").text(icon).visibility(visible);
            });
        })
        .on_press(move |cx| cx.emit(on_press.clone()));

        Element::new(cx).class("table-row-divisor");
    })
    .class("table-column-header");
}

fn table_element<'a, T>(cx: &mut Context, lens: impl Res<T>, checked: bool)
where
    T: ToString,
{
    Label::new(cx, lens).class(if checked { "table-element-colored" } else { "table-element" });
}

fn main() {
    Application::new(|cx| {
        let people = vec![
            Person {
                first_name: "Peter".to_string(),
                last_name: "Pan".to_string(),
                age: 14,
                email: "peterpan@email.com".to_string(),
            },
            Person {
                first_name: "Mary".to_string(),
                last_name: "Poppins".to_string(),
                age: 32,
                email: "marypoppins@email.com".to_string(),
            },
            Person {
                first_name: "John".to_string(),
                last_name: "Doe".to_string(),
                age: 54,
                email: "johndoe@email.com".to_string(),
            },
            Person {
                first_name: "Jane".to_string(),
                last_name: "Doe".to_string(),
                age: 47,
                email: "janedoe@email.com".to_string(),
            },
            Person {
                first_name: "Simon".to_string(),
                last_name: "Fields".to_string(),
                age: 19,
                email: "simonfields@email.com".to_string(),
            },
        ];

        TableData {
            people,
            first_name_sorted: Sorted::None,
            last_name_sorted: Sorted::None,
            age_sorted: Sorted::None,
            email_sorted: Sorted::None,
        }
        .build(cx);

        cx.add_stylesheet(DARK_THEME).expect("Failed to find stylesheet");

        VStack::new(cx, |cx| {
            VStack::new(cx, |cx| {
                Table::new(cx, TableData::people, |cx, list| {
                    TableColumn::new(
                        cx,
                        list,
                        Person::first_name,
                        |cx| {
                            column_header(
                                cx,
                                "First Name",
                                TableData::first_name_sorted,
                                AppEvent::ToggleSortFirstName,
                            )
                        },
                        |cx, _, first_name| table_element(cx, first_name, false),
                    );

                    TableColumn::new(
                        cx,
                        list,
                        Person::last_name,
                        |cx| {
                            column_header(
                                cx,
                                "Last Name",
                                TableData::last_name_sorted,
                                AppEvent::ToggleSortLastName,
                            )
                        },
                        |cx, _, last_name| table_element(cx, last_name, false),
                    );

                    TableColumn::new(
                        cx,
                        list,
                        Person::age,
                        |cx| {
                            column_header(cx, "Age", TableData::age_sorted, AppEvent::ToggleSortAge)
                        },
                        |cx, _, age| table_element(cx, age, false),
                    );

                    TableColumn::new(
                        cx,
                        list,
                        Person::email,
                        |cx| {
                            column_header(
                                cx,
                                "Email",
                                TableData::email_sorted,
                                AppEvent::ToggleSortEmail,
                            )
                        },
                        |cx, _, email| table_element(cx, email, true),
                    );
                });
            })
            .class("container");
        })
        .class("main");
    })
    .ignore_default_theme()
    .title("Table")
    .run();
}

#[derive(Debug, Clone, Lens, Data)]
pub struct Person {
    first_name: String,
    last_name: String,
    age: i32,
    email: String,
}

impl Model for Person {}

#[derive(Debug, Lens)]
pub struct TableData {
    people: Vec<Person>,
    first_name_sorted: Sorted,
    last_name_sorted: Sorted,
    age_sorted: Sorted,
    email_sorted: Sorted,
}

#[derive(Clone)]
pub enum AppEvent {
    SetAge(usize, String),
    Print,
    ToggleSortFirstName,
    ToggleSortLastName,
    ToggleSortAge,
    ToggleSortEmail,
}

#[derive(Debug, Clone, PartialEq, Data)]
pub enum Sorted {
    Forward,
    Reverse,
    None,
}

impl Model for TableData {
    fn event(&mut self, _: &mut EventContext, event: &mut Event) {
        event.map(|app_event, _| match app_event {
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
                self.email_sorted = Sorted::None;
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
                self.email_sorted = Sorted::None;
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
                self.email_sorted = Sorted::None;
            }

            AppEvent::ToggleSortEmail => {
                match self.email_sorted {
                    Sorted::Forward => {
                        self.email_sorted = {
                            self.people.sort_by_cached_key(|person| person.email.clone());
                            self.people.reverse();
                            Sorted::Reverse
                        }
                    }
                    Sorted::Reverse => {
                        self.email_sorted = {
                            self.people.sort_by_cached_key(|person| person.email.clone());
                            Sorted::Forward
                        }
                    }
                    Sorted::None => {
                        self.email_sorted = {
                            self.people.sort_by_cached_key(|person| person.email.clone());
                            Sorted::Forward
                        }
                    }
                }

                self.first_name_sorted = Sorted::None;
                self.last_name_sorted = Sorted::None;
                self.age_sorted = Sorted::None;
            }
        });
    }
}
