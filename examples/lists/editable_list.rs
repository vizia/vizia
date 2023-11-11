use vizia::prelude::*;

#[derive(Lens)]
pub struct AppData {
    list: Vec<u32>,
    selected: Option<usize>,
}

#[derive(Debug)]
pub enum AppEvent {
    Add(u32),
    RemoveSelected,
    Select(usize),
    IncrementSelection,
    DecrementSelection,
}

impl Model for AppData {
    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|app_event, _| match app_event {
            AppEvent::Add(value) => {
                self.list.push(*value);
                // self.selected = self.selected.clamp(0, self.list.len() - 1);
            }

            AppEvent::RemoveSelected => {
                if let Some(selected) = self.selected {
                    if !self.list.is_empty() {
                        self.list.remove(selected);
                    }
                    self.selected = None;
                }
            }

            AppEvent::Select(idx) => {
                self.selected = Some(*idx);
            }

            AppEvent::IncrementSelection => {
                if let Some(selected) = &mut self.selected {
                    *selected += 1;
                }
            }

            AppEvent::DecrementSelection => {
                if let Some(selected) = &mut self.selected {
                    *selected = selected.saturating_sub(1);
                }
            }
        });
    }
}

fn main() {
    Application::new(|cx| {
        cx.add_stylesheet(include_style!("examples/resources/themes/list_style.css"))
            .expect("Failed to add stylesheet");

        let list: Vec<u32> = (10..14u32).collect();
        AppData { list, selected: None }.build(cx);

        VStack::new(cx, |cx| {
            Button::new(cx, |cx| Label::new(cx, "Add").width(Stretch(1.0)))
                .on_press(|cx| cx.emit(AppEvent::Add(20)))
                .width(Stretch(1.0));

            Button::new(cx, |cx| Label::new(cx, "Remove").width(Stretch(1.0)))
                .on_press(|cx| cx.emit(AppEvent::RemoveSelected))
                .disabled(AppData::selected.map(|selected| selected.is_none()))
                .width(Stretch(1.0));

            List::new(cx, AppData::list, move |cx, index, item| {
                Label::new(cx, item)
                    .width(Pixels(100.0))
                    .height(Pixels(30.0))
                    .border_color(Color::black())
                    .border_width(Pixels(1.0))
                    // Set the checked state based on whether this item is selected
                    .checked(
                        AppData::selected
                            .map(move |selected| selected.map(|s| s == index).unwrap_or_default()),
                    )
                    // Set the selected item to this one if pressed
                    .on_press(move |cx| cx.emit(AppEvent::Select(index)));
            })
            .row_between(Pixels(5.0))
            .on_increment(move |cx| cx.emit(AppEvent::IncrementSelection))
            .on_decrement(move |cx| cx.emit(AppEvent::DecrementSelection));
        })
        .row_between(Pixels(5.0))
        .size(Auto)
        .width(Pixels(100.0))
        .space(Stretch(1.0));
    })
    .title("Editable List")
    .run();
}
