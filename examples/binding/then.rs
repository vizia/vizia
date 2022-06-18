use vizia::prelude::*;

#[derive(Lens)]
pub struct AppData {
    pub subdata: SubData,
}

impl Model for AppData {}

#[derive(Lens, Clone, Data)]
pub struct SubData {
    pub name: String,
}

fn main() {
    Application::new(|cx| {
        AppData { subdata: SubData { name: String::from("test") } }.build(cx);

        Label::new(cx, AppData::subdata.then(SubData::name));
        Label::new(cx, AppData::subdata >> SubData::name);
    })
    .run();
}
