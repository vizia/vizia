use vizia::prelude::*;

#[derive(Lens)]
pub struct AppData {
    list: Vec<u32>,
    selected: usize,
}

#[derive(Debug)]
pub enum AppEvent {
    Select(usize),
    IncrementSelection,
    DecrementSelection,
}

impl Model for AppData {
    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|app_event, _| match app_event {
            AppEvent::Select(index) => {
                self.selected = *index;
            }

            AppEvent::IncrementSelection => {
                cx.emit(AppEvent::Select((self.selected + 1).min(self.list.len() - 1)));
            }

            AppEvent::DecrementSelection => {
                cx.emit(AppEvent::Select(self.selected.saturating_sub(1)));
            }
        });
    }
}

fn main() {
    Application::new(|cx| {
        cx.add_stylesheet(include_style!("examples/resources/themes/list_style.css"))
            .expect("Failed to add stylesheet");

        let list: Vec<u32> = (0..4u32).collect();
        AppData { list, selected: 0 }.build(cx);

        VStack::new(cx, move |cx| {
            List::new(cx, AppData::list, move |cx, index, item| {
                Label::new(cx, item)
                    // Set the checked state based on whether this item is selected
                    .checked(AppData::selected.map(move |selected| *selected == index))
                    // Set the selected item to this one if pressed
                    .on_press(move |cx| cx.emit(AppEvent::Select(index)));
            })
            .on_increment(move |cx| cx.emit(AppEvent::IncrementSelection))
            .on_decrement(move |cx| cx.emit(AppEvent::DecrementSelection));

            Label::new(
                cx,
                AppData::selected.map(|selected| format!("You have selected: {}", selected)),
            );
        })
        .class("container");
    })
    .title("Selectable List")
    .run();
}
