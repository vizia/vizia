use std::collections::HashSet;

use vizia::*;

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
    fn event(&mut self, _: &mut Context, event: &mut Event) {
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
    Application::new(WindowDescription::new().with_title("List"), |cx| {
        cx.add_theme(include_str!("list_style.css"));

        let list: Vec<u32> = (10..14u32).collect();
        AppData { list, selected: HashSet::new() }.build(cx);

        List::new(cx, AppData::list, |cx, index, item| {
            // This vstack shouldn't be necessary but because of how bindings work it's required
            VStack::new(cx, move |cx| {
                Binding::new(cx, AppData::selected, move |cx, selected| {
                    let selected = selected.get(cx).contains(&index);

                    Label::new(cx, item)
                        // Set the checked state based on whether this item is selected
                        .checked(selected)
                        // Set the selected item to this one if pressed
                        .on_press(move |cx| cx.emit(AppEvent::Select(index)));
                });
            });
        })
        .space(Stretch(1.0))
        .on_clear(|cx| cx.emit(AppEvent::ClearSelection));
    })
    .run();
}
