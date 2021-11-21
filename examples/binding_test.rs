use vizia::*;

fn main() {

    Application::new(|cx|{
        CustomData::new().build(cx);
    
        Binding::new(cx, CustomData::value, |cx, data|{
            Label::new(cx, &data.get(cx).to_string()).background_color(Color::red());
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
            value: "Hello World".to_string(),
        }
    }
}

impl Model for CustomData {

}
