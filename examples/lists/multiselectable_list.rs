use std::collections::HashSet;

use vizia::prelude::*;

#[derive(Lens)]
pub struct AppData {
    list: Vec<u32>,
    selected: HashSet<usize>,
}

#[derive(Debug)]
pub enum AppEvent {
    Select(usize),
    ClearSelection,
}

impl Model for AppData {
    fn event(&mut self, _: &mut EventContext, event: &mut Event) {
        event.map(|app_event, _| match app_event {
            AppEvent::Select(index) => {
                if !self.selected.insert(*index) {
                    self.selected.remove(index);
                }
            }

            AppEvent::ClearSelection => {
                self.selected.clear();
            }
        });
    }
}

fn main() {
    Application::new(|cx| {
        cx.add_stylesheet(include_style!("../resources/themes/list_style.css"))
            .expect("Failed to add stylesheet");

        let list: Vec<u32> = (10..14u32).collect();
        AppData { list, selected: HashSet::new() }.build(cx);

        List::new(cx, AppData::list, |cx, index, item| {
            Label::new(cx, item)
                // Set the checked state based on whether this item is selected
                .checked(AppData::selected.map(move |selected| selected.contains(&index)))
                // Set the selected item to this one if pressed
                .on_press(move |cx| cx.emit(AppEvent::Select(index)));
        })
        .space(Stretch(1.0))
        .on_clear(|cx| cx.emit(AppEvent::ClearSelection));
    })
    .title("Multiselectable List")
    .run();
}
