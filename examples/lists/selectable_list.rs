use lazy_static::lazy_static;

use vizia::*;

const STYLE: &str = r#"
    .list_item {
        width: 100px;
        height: 30px;
        border-color: black;
        border-width: 1px;
        background-color: white;
    }

    .list_item:checked {
        background-color: blue;
    }

    list {
        row-between: 5px;
        space: 5px;
    }

    vstack {
        space: 1s;
    }
"#;

lazy_static! {
    pub static ref STATIC_LIST: Vec<u32> = { (20..24).collect() };
}

#[derive(Lens)]
pub struct AppData {
    list: Vec<u32>,
    selected: usize,
    selected_static: usize,
}

#[derive(Debug)]
pub enum AppEvent {
    SelectDynamic(usize),
    IncrementDynamic,
    DecrementDynamic,
    SelectStatic(usize),
    IncrementStatic,
    DecrementStatic,
}

impl Model for AppData {
    // Intercept list events from the list view to modify the selected index in the model
    fn event(&mut self, cx: &mut Context, event: &mut Event) {
        if let Some(list_event) = event.message.downcast() {
            match list_event {
                AppEvent::SelectDynamic(index) => {
                    self.selected = *index;
                }
                AppEvent::SelectStatic(index) => {
                    self.selected_static = *index;
                }
                AppEvent::IncrementDynamic => {
                    cx.emit(AppEvent::SelectDynamic((self.selected + 1).min(self.list.len() - 1)))
                }
                AppEvent::DecrementDynamic => {
                    cx.emit(AppEvent::SelectDynamic(self.selected.saturating_sub(1)))
                }
                AppEvent::IncrementStatic => cx.emit(AppEvent::SelectStatic(
                    (self.selected_static + 1).min(STATIC_LIST.len() - 1),
                )),
                AppEvent::DecrementStatic => {
                    cx.emit(AppEvent::SelectStatic(self.selected_static.saturating_sub(1)))
                }
            }
        }
    }
}

fn main() {
    Application::new(WindowDescription::new().with_title("List"), |cx| {
        cx.add_theme(STYLE);

        let list: Vec<u32> = (10..14u32).collect();
        AppData { list, selected: 0, selected_static: 0 }.build(cx);

        VStack::new(cx, move |cx| {
            HStack::new(cx, |cx| {
                VStack::new(cx, |cx| {
                    Label::new(cx, "Model-owned list");
                    List::new(cx, AppData::list, move |cx, item| {
                        let item_text = item.get(cx).to_string();
                        let item_index = item.index();
                        Binding::new(cx, AppData::selected, move |cx, selected| {
                            let selected = *selected.get(cx);
                            Label::new(cx, &item_text)
                                .class("list_item")
                                // Set the checked state based on whether this item is selected
                                .checked(if selected == item_index { true } else { false })
                                // Set the selected item to this one if pressed
                                .on_press(move |cx| cx.emit(AppEvent::SelectDynamic(item_index)));
                        });
                    })
                    .on_increment(move |cx| cx.emit(AppEvent::IncrementDynamic))
                    .on_decrement(move |cx| cx.emit(AppEvent::DecrementDynamic));
                });

                VStack::new(cx, |cx| {
                    Label::new(cx, "Static list");
                    List::new(cx, StaticLens::new(STATIC_LIST.as_ref()), move |cx, item| {
                        let item_text = item.get(cx).to_string();
                        let item_index = item.index();
                        Binding::new(cx, AppData::selected_static, move |cx, selected| {
                            let selected = *selected.get(cx);
                            Label::new(cx, &item_text)
                                .class("list_item")
                                // Set the checked state based on whether this item is selected
                                .checked(if selected == item_index { true } else { false })
                                // Set the selected item to this one if pressed
                                .on_press(move |cx| cx.emit(AppEvent::SelectStatic(item_index)));
                        });
                    })
                    .on_increment(move |cx| cx.emit(AppEvent::IncrementStatic))
                    .on_decrement(move |cx| cx.emit(AppEvent::DecrementStatic));
                });
            });
            Binding::new(cx, AppData::selected, move |cx, selected_item| {
                Binding::new(cx, AppData::selected_static, move |cx, selected_static_item| {
                    Label::new(
                        cx,
                        &format!(
                            "You selected {} and {}",
                            selected_item.get(cx),
                            selected_static_item.get(cx)
                        ),
                    );
                });
            });
        });
    })
    .run();
}
