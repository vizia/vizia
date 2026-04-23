mod helpers;
use helpers::*;
use vizia::prelude::*;

pub struct AppData {
    value: Signal<f32>,
}

#[derive(Debug)]
pub enum AppEvent {
    SetValue(f32),
}

impl Model for AppData {
    fn event(&mut self, _: &mut EventContext, event: &mut Event) {
        event.map(|app_event, _| match app_event {
            AppEvent::SetValue(value) => {
                self.value.set(*value);
            }
        });
    }
}

fn main() -> Result<(), ApplicationError> {
    Application::new(|cx| {
        let value = Signal::new(0.2);
        AppData { value }.build(cx);

        ExamplePage::new(cx, |cx| {
            Knob::new(cx, 0.5, value, false).on_change(|cx, val| {
                cx.emit(AppEvent::SetValue(val));
            });
        });
    })
    .title(Localized::new("view-title-knob"))
    .inner_size((300, 300))
    .run()
}
