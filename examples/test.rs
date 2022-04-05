use vizia::*;

#[derive(Lens)]
pub struct ParentData {
    parent_data: String,
}

pub enum ParentEvent {
    UpdateData,
}

impl Model for ParentData {
    fn event(&mut self, cx: &mut Context, event: &mut Event) {
        if let Some(parent_event) = event.message.downcast() {
            match parent_event {
                ParentEvent::UpdateData => {
                    self.parent_data = String::from("Goodbye ");
                }
            }
        }
    }
}

#[derive(Lens)]
pub struct DerivedData {
    derived_data: String,
}

impl Model for DerivedData {}

fn main() {
    let mut window_description = WindowDescription::new();

    Application::new(window_description, |cx| {
        HStack::new(cx, |cx| {
            ParentData { parent_data: String::from("Hello ") }.build(cx);

            HStack::new(cx, |cx| {
                DerivedData { derived_data: String::from("TEST") }.build(cx).bind(
                    ParentData::parent_data,
                    |cx, derived, parent_data| {
                        derived.derived_data = parent_data.get(cx) + "World";
                    },
                );

                Label::new(cx, DerivedData::derived_data)
                    .space(Pixels(20.0))
                    .on_press(|cx| cx.emit(ParentEvent::UpdateData));
            });
        });
    })
    .run();
}
