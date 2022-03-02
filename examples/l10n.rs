use vizia::*;

#[derive(Lens)]
pub struct AppData {
    name: String,
}

pub enum AppEvent {
    SetName(String),
}

impl Model for AppData {
    fn event(&mut self, _cx: &mut Context, event: &mut Event) {
        if let Some(msg) = event.message.downcast() {
            match msg {
                AppEvent::SetName(s) => self.name = s.clone(),
            }
        }
    }
}

fn main() {
    Application::new(WindowDescription::new(), |cx| {
        cx.add_translation("en-US".parse().unwrap(), include_str!("resources/en-US/hello.ftl").to_owned());
        cx.add_translation("fr".parse().unwrap(), include_str!("resources/fr/hello.ftl").to_owned());

        AppData { name: "Audrey".to_owned() }.build(cx);

        Label::new(cx, Localized::new("hello-world"));
        HStack::new(cx, |cx| {
            Label::new(cx, Localized::new("enter-name"));
            Textbox::new(cx, AppData::name)
                .width(Units::Pixels(300.0))
                .on_edit(|cx, text| {
                    cx.emit(AppEvent::SetName(text));
                });
        });
        Label::new(cx, Localized::new("intro").arg("name", AppData::name));
    }).run()
}
