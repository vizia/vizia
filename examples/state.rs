use vizia::*;

// Example of using state types to store view-local mutable state

#[derive(Lens)]
pub struct Data {
    something: String,
    other: i32,
}

impl Model for Data {
    fn event(&mut self, _: &mut Context, event: &mut Event) {
        if let Some(data_event) = event.message.downcast() {
            match data_event {
                DataEvent::ChangeSomething => {
                    self.something = "Test".to_string();
                }

                DataEvent::ChangeOther => {
                    self.other = 44;
                }
            }
        }
    }
}

// Lens provides a way to filter the model for some specific data
// When sending updates, store the lenses to a particular model and then iterate on them to get the `State<>`s
// Then send the updates to the bound entities

fn main() {
    Application::new(WindowDescription::new().with_title("State"), |cx| {
        Data { something: "Something".to_string(), other: 32 }.build(cx);

        Binding::new(cx, Data::something, |cx, something| {
            println!("Rebuild something");
            Label::new(cx, something);
        });

        Binding::new(cx, Data::other, |cx, other| {
            println!("Rebuild other");
            Label::new(cx, other);
        });

        Button::new(
            cx,
            |cx| cx.emit(DataEvent::ChangeSomething),
            |cx| Label::new(cx, "Change Something"),
        );
        Button::new(cx, |cx| cx.emit(DataEvent::ChangeOther), |cx| Label::new(cx, "Change Other"));
    })
    .run();
}

#[derive(Debug)]
pub enum DataEvent {
    ChangeSomething,
    ChangeOther,
}
