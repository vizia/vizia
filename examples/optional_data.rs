

use vizia::*;

#[derive(Lens)]
pub struct AppData {
    data: Option<String>,
}

#[derive(Debug)]
pub enum AppEvent {
    AddData,
    ClearData,
}

impl Model for AppData {
    fn event(&mut self, cx: &mut Context, event: &mut Event) {
        if let Some(app_event) = event.message.downcast() {
            match app_event {
                AppEvent::AddData => {
                    self.data = Some("Hello World".to_string());
                }

                AppEvent::ClearData => {
                    println!("Clear");
                    self.data = None;
                }
            }
        }
    }
}

fn main() {
    let window_description = WindowDescription::new();
    Application::new(window_description, |cx|{

        AppData {
            data: Some("Hello World".to_string()),
        }.build(cx);

        Binding::new_fallible(cx, AppData::data, |cx, text|{
            println!("Something");
            if let Some(text) = text.get(cx).clone() {
                Label::new(cx, &text);
            }
        }, |cx|{
            println!("Nothing");
        });

        Button::new(cx, |cx| cx.emit(AppEvent::ClearData), |cx|{
            Label::new(cx, "Clear")
        });
    }).run();
}