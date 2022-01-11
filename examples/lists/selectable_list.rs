use vizia::*;

const STYLE: &str = r#"
    
    .list_item {
        background-color: white;
        width: 100px;
        height: 30px;
        border-color: black;
        border-width: 1px;
    }

    .list_item:checked {
        background-color: blue;
    }

    hstack {
        child-space: 3px;
    }
    vstack {
        child-space: 3px;
    }
"#;

#[derive(Lens)]
pub struct AppData {
    list1: Vec<u32>,
    list2: Vec<u32>,
    selected1: usize,
    selected2: usize,
}

#[derive(Debug)]
pub enum AppEvent {
    SetSelected1(usize),
    SetSelected2(usize),
}

impl Model for AppData {
    fn event(&mut self, cx: &mut Context, event: &mut Event) {
        match event.message.downcast() {
            Some(AppEvent::SetSelected1(idx)) => self.selected1 = *idx,
            Some(AppEvent::SetSelected2(idx)) => self.selected2 = *idx,
            _ => {}
        }
    }
}

fn main() {
    Application::new(WindowDescription::new().with_title("List"), |cx| {

        cx.add_theme(STYLE);

        let list1: Vec<u32> = (10..14u32).collect();
        let list2: Vec<u32> = (20..24u32).collect();
        AppData {
            list1,
            list2,
            selected1: 0,
            selected2: 0,
        }.build(cx);

        VStack::new(cx, |cx| {
            HStack::new(cx, |cx| {
                VStack::new(cx, |cx| {
                    Label::new(cx, "List 1");
                    List::new(cx, AppData::list1, |cx, item|{
                        let item_text = item.get(cx).to_string();
                        let item_index = item.index();
                        // This vstack shouldn't be necessary but because of how bindings work it's required
                        VStack::new(cx, move |cx|{
                            Binding::new(cx, AppData::selected1, move |cx, selected|{
                                let selected = *selected.get(cx);

                                Label::new(cx, &item_text)
                                    .class("list_item")
                                    // Set the checked state based on whether this item is selected
                                    .checked(if selected == item_index {true} else {false})
                                    // Set the selected item to this one if pressed
                                    .on_press(move |cx| cx.emit(ListEvent::SetSelected(item_index)));
                            });
                        });
                    })
                        .row_between(Pixels(5.0))
                        .space(Stretch(1.0))
                        .on_index_selected(|cx, idx| cx.emit(AppEvent::SetSelected1(idx)));
                });

                VStack::new(cx, |cx| {
                    Label::new(cx, "List 2");
                    List::new(cx, AppData::list2, |cx, item|{
                        let item_text = item.get(cx).to_string();
                        let item_index = item.index();
                        // This vstack shouldn't be necessary but because of how bindings work it's required
                        VStack::new(cx, move |cx|{
                            Binding::new(cx, AppData::selected2, move |cx, selected|{
                                let selected = *selected.get(cx);

                                Label::new(cx, &item_text)
                                    .class("list_item")
                                    // Set the checked state based on whether this item is selected
                                    .checked(if selected == item_index {true} else {false})
                                    // Set the selected item to this one if pressed
                                    .on_press(move |cx| cx.emit(ListEvent::SetSelected(item_index)));
                            });
                        });
                    })
                        .row_between(Pixels(5.0))
                        .space(Stretch(1.0))
                        .on_index_selected(|cx, idx| cx.emit(AppEvent::SetSelected2(idx)));
                });
            });

            Binding::new(cx, AppData::selected1, move |cx, selected1| {
                Binding::new(cx, AppData::selected2, move |cx, selected2| {
                    Label::new(cx, &format!("You selected items {} and {}", selected1.get(cx), selected2.get(cx)));
                });
            });
        });
    })
        .run();
}
