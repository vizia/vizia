use lazy_static::lazy_static;

use vizia::prelude::*;
use vizia_core::binding::StaticLens;

lazy_static! {
    pub static ref STATIC_LIST: Vec<u32> = (20..24).collect();
}

#[derive(Lens)]
pub struct AppData {
    selected: usize,
}

#[derive(Debug)]
pub enum AppEvent {
    Select(usize),
    IncrementSelection,
    DecrementSelection,
}

impl Model for AppData {
    // Intercept list events from the list view to modify the selected index in the model
    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|app_event, _| match app_event {
            AppEvent::Select(index) => {
                self.selected = *index;
            }

            AppEvent::IncrementSelection => {
                cx.emit(AppEvent::Select((self.selected + 1).min(STATIC_LIST.len() - 1)));
            }

            AppEvent::DecrementSelection => {
                cx.emit(AppEvent::Select(self.selected.saturating_sub(1)));
            }
        });
    }
}

fn main() {
    Application::new(|cx| {
        cx.add_stylesheet(include_style!("../resources/themes/list_style.css"))
            .expect("Failed to add stylesheet");

        AppData { selected: 0 }.build(cx);

        VStack::new(cx, move |cx| {
            List::new(cx, StaticLens::new(STATIC_LIST.as_ref()), move |cx, index, item| {
                VStack::new(cx, move |cx| {
                    Label::new(cx, item)
                        .class("list_item")
                        // Set the checked state based on whether this item is selected
                        .checked(AppData::selected.map(move |selected| *selected == index))
                        // Set the selected item to this one if pressed
                        .on_press(move |cx| cx.emit(AppEvent::Select(index)));
                });
            })
            .on_increment(move |cx| cx.emit(AppEvent::IncrementSelection))
            .on_decrement(move |cx| cx.emit(AppEvent::DecrementSelection));

            Binding::new(cx, AppData::selected, move |cx, selected_item| {
                Label::new(cx, &format!("You have selected: {}", selected_item.get(cx),));
            });
        })
        .class("container");
    })
    .title("Static List")
    .run();
}
