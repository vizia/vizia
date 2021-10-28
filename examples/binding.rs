use vizia::*;

fn main() {

    Application::new(|cx|{
        CustomData::new().build(cx);
        
        VStack::new(cx, |cx| {
            Binding::new(cx, CustomData::value, |cx, data|{
                Label::new(cx, &data.get(cx).to_string());
                //Button::new(cx, |cx| data.set(cx, |val| val = "two".to_string()), |_|{});
            });
        });
    }).run();
}


#[derive(Lens)]
pub struct CustomData {
    value: String,
}

impl CustomData {
    pub fn new() -> Self {
        Self {
            value: "two".to_string(),
        }
    }
}

impl Model for CustomData {

}
