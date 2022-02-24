use vizia::*;

#[derive(Lens)]
pub struct AppData {
    list: Vec<u32>,
    selected: usize,

    visible: bool,
}

#[derive(Debug)]
pub enum AppEvent {
    Select(usize),
    IncrementSelection,
    DecrementSelection,
    ToggleVisible,
}

impl Model for AppData {
    // Intercept list events from the list view to modify the selected index in the model
    fn event(&mut self, cx: &mut Context, event: &mut Event) {
        if let Some(list_event) = event.message.downcast() {
            match list_event {
                AppEvent::Select(index) => {
                    self.selected = *index;
                }

                AppEvent::IncrementSelection => {
                    cx.emit(AppEvent::Select((self.selected + 1).min(self.list.len() - 1)));
                }

                AppEvent::DecrementSelection => {
                    cx.emit(AppEvent::Select(self.selected.saturating_sub(1)));
                }

                AppEvent::ToggleVisible => {
                    self.visible ^= true;
                }
            }
        }
    }
}

fn main() {
    Application::new(WindowDescription::new().with_title("List"), |cx| {
        cx.add_stylesheet("examples/lists/list_style.css").unwrap();

        let list: Vec<u32> = (1..10000u32).collect();
        AppData { list, selected: 0, visible: true }.build(cx);

        VStack::new(cx, move |cx| {
            Binding::new(cx, AppData::visible, |cx, visible| {
                Checkbox::new(cx, *visible.get(cx))
                    .on_toggle(|cx| cx.emit(AppEvent::ToggleVisible));

                List::new(cx, AppData::list, move |cx, index, item| {
                    //println!("Do This");
                    let item_text = item.get(cx).to_string();
                    VStack::new(cx, move |cx| {
                        Binding::new(cx, AppData::selected, move |cx, selected| {
                            //println!("Select");
                            let selected = *selected.get(cx);
                            Label::new(cx, &item_text)
                                // Set the checked state based on whether this item is selected
                                .checked(if selected == index { true } else { false })
                                // Set the selected item to this one if pressed
                                .on_press(move |cx| cx.emit(AppEvent::Select(index)));
                        });
                    });
                })
                .on_increment(move |cx| cx.emit(AppEvent::IncrementSelection))
                .on_decrement(move |cx| cx.emit(AppEvent::DecrementSelection))
                .display(visible);
            });

            Binding::new(cx, AppData::selected, move |cx, selected_item| {
                Label::new(cx, &format!("You have selected: {}", *selected_item.get(cx)));
            });
        })
        .class("container");
    })
    .run();
}
