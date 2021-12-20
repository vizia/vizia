use vizia::Lens;
use vizia::*;

fn main() {
    Application::new(WindowDescription::new().with_title("List2"), |cx| {
        let list: Vec<u32> = (0..12u32).collect();
        Data { list }.build(cx);

        HStack::new(cx, |cx| {
            List::new(cx, Data::list, |cx, item| {
                if *item.value(cx) < 6 {
                    Binding::new(cx, ListData::selected, move |cx, selected| {
                        let item = item.clone();
                        HStack::new(cx, move |cx| {
                            Label::new(cx, "Hello");
                            Label::new(cx, "World");
                            Label::new(cx, &item.value(cx).to_string()).background_color(
                                if *item.value(cx) == 40 {
                                    Color::red()
                                } else {
                                    Color::rgba(0, 0, 0, 0)
                                },
                            );
                        })
                        .background_color(if item.index() == *selected.get(cx) {
                            Color::green()
                        } else {
                            Color::blue()
                        })
                        .on_press(cx, move |cx| cx.emit(ListEvent::SetSelected(item.index())));
                    });
                }
            });

            List::new(cx, Data::list, |cx, item| {
                Binding::new(cx, ListData::selected, move |cx, selected| {
                    let item = item.clone();
                    HStack::new(cx, move |cx| {
                        Label::new(cx, "Hello");
                        Label::new(cx, "World");
                        Label::new(cx, &item.value(cx).to_string()).background_color(
                            if *item.value(cx) == 40 {
                                Color::red()
                            } else {
                                Color::rgba(0, 0, 0, 0)
                            },
                        );
                        //Label::new(cx, &item.index().to_string());
                    })
                    .background_color(if item.index() == *selected.get(cx) {
                        Color::green()
                    } else {
                        Color::blue()
                    })
                    .on_press(cx, move |cx| cx.emit(ListEvent::SetSelected(item.index())));
                });
            });

            VStack::new(cx, |cx| {
                // Change item with index 5 to value of 40
                Button::new(
                    cx,
                    |cx| {
                        cx.emit(DataEvent::Update(5, 40));
                    },
                    |_| {},
                );
                // Set all items to value of 3
                Button::new(
                    cx,
                    |cx| {
                        cx.emit(DataEvent::All(3));
                    },
                    |_| {},
                );
                // Set all items value to their index
                Button::new(
                    cx,
                    |cx| {
                        cx.emit(DataEvent::Enumerate);
                    },
                    |_| {},
                );
            })
            .row_between(Pixels(10.0));
        });
    })
    .run();
}

#[derive(Lens)]
pub struct Data {
    list: Vec<u32>,
}

#[derive(Debug)]
pub enum DataEvent {
    Update(usize, u32),
    All(u32),
    Enumerate,
}

impl Model for Data {
    fn event(&mut self, cx: &mut Context, event: &mut Event) {
        if let Some(data_event) = event.message.downcast() {
            match data_event {
                DataEvent::Update(index, value) => {
                    self.list[*index] = *value;
                }

                DataEvent::All(value) => {
                    for item in self.list.iter_mut() {
                        *item = *value;
                    }
                }

                DataEvent::Enumerate => {
                    for (index, item) in self.list.iter_mut().enumerate() {
                        *item = index as u32;
                    }
                }
            }
        }
    }
}
