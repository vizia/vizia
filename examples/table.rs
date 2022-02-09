use vizia::*;

fn main() {
    Application::new(WindowDescription::new().with_title("Table"), |cx| {
        TableData {
            people: vec![
                Person { first_name: "Peter".to_string(), last_name: "Pan".to_string(), age: 14 },
                Person {
                    first_name: "Mary".to_string(),
                    last_name: "Poppins".to_string(),
                    age: 32,
                },
            ],
        }
        .build(cx);

        // Table::new(cx, 10, TableData::table_data, |cx, _, item| {
        //     // VStack::new(cx, move |cx|{
        //     //     Label::new(cx, &format!("{}, {}", item.row(), item.col())).width(Stretch(1.0)).height(Stretch(1.0));
        //     // }).width(Stretch(1.0)).height(Stretch(1.0));
        //     Label::new(cx, &item.index().to_string())
        //         .width(Stretch(1.0))
        //         .height(Stretch(1.0))
        //         .background_color(Color::rgb(120, 120, 120));
        // })
        // .width(Pixels(300.0))
        // .height(Pixels(300.0))
        // .space(Stretch(1.0));

        // TableColumn::new(cx, TableData::people, Person::last_name, |cx|{}, |cx, index, last_name| {
        //     HStack::new(cx, move |cx| {
        //         // Binding::new(cx, person.then(Person::first_name), move |cx, first_name| {
        //         //     Label::new(cx, first_name);
        //         // });
        //         //Binding::new(cx, person.then(Person::last_name), move |cx, last_name| {
        //             //Label::new(cx, last_name);
        //             Textbox::new(cx, last_name)
        //             .on_edit(move |cx, text|{
        //                 // if let Ok(parsed_age) = text.parse::<i32>() {
        //                 //     cx.emit(AppEvent::SetAge(index, parsed_age));
        //                 // }
        //                 cx.emit(AppEvent::SetAge(index, text));
        //             })
        //             .width(Pixels(100.0))
        //             .height(Pixels(30.0));
        //         //});
        //     });
        // });

        Table::new(cx, TableData::people, |cx, list| {
            TableColumn::new(
                cx,
                list,
                Person::first_name,
                |cx| {
                    Label::new(cx, "First Name");
                },
                |cx, _, first_name| {
                    Label::new(cx, first_name).height(Pixels(30.0));
                },
            )
            .width(Pixels(200.0))
            .border_color(Color::black())
            .border_width(Pixels(1.0));

            TableColumn::new(
                cx,
                list,
                Person::last_name,
                |cx| {
                    Label::new(cx, "Last Name");
                },
                |cx, index, last_name| {
                    //Label::new(cx, last_name).height(Pixels(30.0));
                    Textbox::new(cx, last_name)
                        .on_edit(move |cx, text| {
                            // if let Ok(parsed_age) = text.parse::<i32>() {
                            //     cx.emit(AppEvent::SetAge(index, parsed_age));
                            // }
                            cx.emit(AppEvent::SetAge(index, text));
                        })
                        .width(Pixels(100.0))
                        .height(Pixels(30.0));
                },
            )
            .width(Pixels(200.0))
            .border_color(Color::black())
            .border_width(Pixels(1.0));

            TableColumn::new(
                cx,
                list,
                Person::age,
                |cx| {
                    Label::new(cx, "Age");
                },
                |cx, _, age| {
                    Label::new(cx, age).height(Pixels(30.0));
                },
            )
            .width(Pixels(200.0))
            .border_color(Color::black())
            .border_width(Pixels(1.0));
        })
        .space(Stretch(1.0));

        Button::new(cx, |cx| cx.emit(AppEvent::Print), |cx| Label::new(cx, "Print Data"));
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
}

pub enum AppEvent {
    SetAge(usize, String),
    Print,
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
            }
        }
    }
}
