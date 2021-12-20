use vizia::*;

fn main() {
    Application::new(WindowDescription::new().with_title("Binding in View"), |cx| {
        Data { something: 55 }.build(cx);

        CustomView::new(cx);
    })
    .run();
}

#[derive(Lens)]
pub struct Data {
    something: i32,
}

impl Model for Data {}

pub struct CustomView {}

impl CustomView {
    pub fn new(cx: &mut Context) -> Handle<Self> {
        Self {}.build(cx)
    }
}

impl View for CustomView {
    fn body(&mut self, cx: &mut Context) {
        Binding::new(cx, Data::something, |cx, something| {
            Label::new(cx, &something.get(cx).to_string());
        });
    }
}
