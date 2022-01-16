
use std::collections::HashSet;

use vizia::*;

const STYLE: &str = r#"
    
    label {
        background-color: white;
    }

    label:checked {
        background-color: blue;
    }
"#;

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
    // Intercept list events from the list view to modify the selected index in the model
    fn event(&mut self, _: &mut Context, event: &mut Event) {
        if let Some(list_event) = event.message.downcast() {
            match list_event {
                AppEvent::Select(index) => {

                    if !self.selected.insert(*index) {
                        self.selected.remove(index);
                    }
                }
                
                AppEvent::ClearSelection => {
                    self.selected.clear();
                }


                _=> {}
            }
        }
    }
}

fn main() {
    Application::new(WindowDescription::new().with_title("List"), |cx| {

        cx.add_theme(STYLE);

        let list: Vec<u32> = (10..14u32).collect();
        AppData { 
            list,
            selected: HashSet::new(),
        }.build(cx);

        List::new(cx, AppData::list, |cx, item|{
            let item_text = item.get(cx).to_string();
            let item_index = item.index();
            // This vstack shouldn't be necessary but because of how bindings work it's required
            VStack::new(cx, move |cx|{
                Binding::new(cx, AppData::selected, move |cx, selected|{
                    let selected = selected.get(cx).clone();
                    
                    Label::new(cx, &item_text)
                        .width(Pixels(100.0))
                        .height(Pixels(30.0))
                        .border_color(Color::black())
                        .border_width(Pixels(1.0))
                        // Set the checked state based on whether this item is selected
                        .checked(if selected.contains(&item_index) {true} else {false})
                        // Set the selected item to this one if pressed
                        .on_press(move |cx| cx.emit(AppEvent::Select(item_index)));
                });
            });
        })
            .row_between(Pixels(5.0))
            .space(Stretch(1.0))
            .on_clear(|cx| {
                cx.emit(AppEvent::ClearSelection)
            });
    })
    .run();
}
