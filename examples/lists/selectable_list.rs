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
    selected: usize,
}

impl Model for AppData {
    // Intercept list events from the list view to modify the selected index in the model
    fn event(&mut self, _: &mut Context, event: &mut Event) {
        if let Some(list_event) = event.message.downcast() {
            match list_event {
                ListEvent::SetSelected(index) => {
                    self.selected = *index;
                }

                ListEvent::IncrementSelection => {
                    self.selected = self.selected.saturating_add(1).clamp(0, self.list.len()-1);
                }

                ListEvent::DecrementSelection => {
                    self.selected = self.selected.saturating_sub(1).clamp(0, self.list.len()-1);
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
            selected: 0,
        }.build(cx);

        List::new(cx, AppData::list, |cx, item|{
            let item_text = item.get(cx).to_string();
            let item_index = item.index();
            // This vstack shouldn't be necessary but because of how bindings work it's required
            VStack::new(cx, move |cx|{
                Binding::new(cx, AppData::selected, move |cx, selected|{
                    let selected = *selected.get(cx);
                    
                    Label::new(cx, &item_text)
                        .width(Pixels(100.0))
                        .height(Pixels(30.0))
                        .border_color(Color::black())
                        .border_width(Pixels(1.0))
                        // Set the checked state based on whether this item is selected
                        .checked(if selected == item_index {true} else {false})
                        // Set the selected item to this one if pressed
                        .on_press(move |cx| cx.emit(ListEvent::SetSelected(item_index)));
                });
            });
        })
        .row_between(Pixels(5.0))
        .space(Stretch(1.0));
    })
    .run();
}


