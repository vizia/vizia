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

#[derive(Debug)]
pub enum AppEvent {
    Add(u32),
    RemoveSelected,
}

impl Model for AppData {
    fn event(&mut self, cx: &mut Context, event: &mut Event) {
        // Intercept list events from the list view to modify the selected index in the model
        if let Some(list_event) = event.message.downcast() {
            match list_event {
                ListEvent::SetSelected(index) => {
                    self.selected = *index;
                }

                ListEvent::IncrementSelection => {
                    if !self.list.is_empty() {
                        self.selected = self.selected.saturating_add(1).clamp(0, self.list.len()-1);
                    }
                }

                ListEvent::DecrementSelection => {
                    if !self.list.is_empty() {
                        self.selected = self.selected.saturating_sub(1).clamp(0, self.list.len()-1);
                    }
                }

                _=> {}
            }
        }

        if let Some(app_event) = event.message.downcast() {
            match app_event {
                AppEvent::Add(value) => {
                    self.list.push(*value);
                    self.selected = self.selected.clamp(0, self.list.len()-1);
                }

                AppEvent::RemoveSelected => {
                    if !self.list.is_empty() {
                        self.list.remove(self.selected);
                    }
                    if !self.list.is_empty() {
                        self.selected = self.selected.clamp(0, self.list.len()-1);
                    }
                }
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

        VStack::new(cx, |cx|{
            Button::new(cx, |cx| cx.emit(AppEvent::Add(20)), |cx|{
                Label::new(cx, "Add")
                    .width(Stretch(1.0))
            }).width(Percentage(100.0));
    
            Button::new(cx, |cx| cx.emit(AppEvent::RemoveSelected), |cx|{
                Label::new(cx, "Remove Selected")
            });

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
            .row_between(Pixels(5.0));

        }).row_between(Pixels(5.0)).size(Auto).space(Stretch(1.0)).top(Pixels(100.0)).child_space(Stretch(1.0));


    })
    .run();
}


