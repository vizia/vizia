use vizia::*;

#[derive(Lens)]
pub struct CustomData {
    value: String,
}

impl CustomData {
    pub fn new() -> Self {
        Self { value: "Hello World".to_string() }
    }
}

impl Model for CustomData {}

fn main() {
    Application::new(WindowDescription::new().with_title("Binding"), |cx| {
        CustomData::new().build(cx);

        VStack::new(cx, |cx| {
            Binding::new(cx, CustomData::value, |cx, value| {
                Label::new(cx, value);
            });
        });
    })
    .run();
}
