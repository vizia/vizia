mod helpers;
use helpers::*;
use vizia::prelude::*;

#[derive(Lens)]
pub struct AppData {
    value: f32,
}

#[derive(Debug)]
pub enum AppEvent {
    SetValue(f32),
}

impl Model for AppData {
    fn event(&mut self, _: &mut EventContext, event: &mut Event) {
        event.map(|app_event, _| match app_event {
            AppEvent::SetValue(value) => {
                self.value = *value;
            }
        });
    }
}

fn main() {
    Application::new(|cx| {
        AppData { value: 0.2 }.build(cx);

        ExamplePage::new(cx, |cx| {
            Knob::new(cx, 0.5, AppData::value, false).on_changing(|cx, val| {
                cx.emit(AppEvent::SetValue(val));
            });
        });
    })
    .title("Knob")
    .inner_size((250, 250))
    .run();
}
