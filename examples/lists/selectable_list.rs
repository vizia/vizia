use lazy_static::lazy_static;

use vizia::*;

const STYLE: &str = r#"
    
    label {
        background-color: white;
    }

    label:checked {
        background-color: blue;
    }
"#;

lazy_static! {
    pub static ref STATIC_LIST: Vec<u32> = {
        (20..24).collect()
    };
}

#[derive(Lens)]
pub struct AppData {
    list: Vec<u32>,
    selected: usize,
    selected_static: usize,
}

#[derive(Debug)]
pub enum AppEvent {
    SelectDynamic(usize),
    SelectStatic(usize),
}

impl Model for AppData {
    // Intercept list events from the list view to modify the selected index in the model
    fn event(&mut self, _: &mut Context, event: &mut Event) {
        if let Some(list_event) = event.message.downcast() {
            match list_event {
                AppEvent::SelectDynamic(index) => {
                    self.selected = *index;
                }
                AppEvent::SelectStatic(index) => {
                    self.selected_static = *index;
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
            selected_static: 0,
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
                        .on_press(move |cx| cx.emit(AppEvent::SelectDynamic(item_index)));
                });
            });
        })
            .row_between(Pixels(5.0))
            .space(Stretch(1.0));

        List::new(cx, StaticLens::new(STATIC_LIST.as_ref()), |cx, item|{
            let item_text = item.get(cx).to_string();
            let item_index = item.index();
            // This vstack shouldn't be necessary but because of how bindings work it's required
            VStack::new(cx, move |cx|{
                Binding::new(cx, AppData::selected_static, move |cx, selected|{
                    let selected = *selected.get(cx);

                    Label::new(cx, &item_text)
                        .width(Pixels(100.0))
                        .height(Pixels(30.0))
                        .border_color(Color::black())
                        .border_width(Pixels(1.0))
                        // Set the checked state based on whether this item is selected
                        .checked(if selected == item_index {true} else {false})
                        // Set the selected item to this one if pressed
                        .on_press(move |cx| cx.emit(AppEvent::SelectStatic(item_index)));
                });
            });
        })
            .row_between(Pixels(5.0))
            .space(Stretch(1.0));
    })
        .run();
}
