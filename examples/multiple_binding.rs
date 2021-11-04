use vizia::*;

// Example of binding to two pieces of data

fn main() {

    Application::new(|cx|{
        CustomData::new().build(cx);
        OtherData::new().build(cx);
        
        VStack::new(cx, |cx| {
            Binding::new(cx, CustomData::value, |cx, data|{
                Binding::new(cx, OtherData::value, move |cx, other|{
                    Label::new(cx, &format!("{} {}", data.get(cx), other.get(cx)));
                });
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
            value: "Hello".to_string(),
        }
    }
}

impl Model for CustomData {

}


#[derive(Lens)]
pub struct OtherData {
    value: String,
}

impl OtherData {
    pub fn new() -> Self {
        Self {
            value: "World".to_string(),
        }
    }
}

impl Model for OtherData {

}

