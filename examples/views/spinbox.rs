mod helpers;
use helpers::*;

use vizia::prelude::*;

struct AppState {
    spinbox_value: Signal<f64>,
}

#[derive(Clone)]
enum AppEvent {
    SetValue(f64),
}

fn main() -> Result<(), ApplicationError> {
    Application::new(|cx| {
        let spinbox_value = Signal::new(99f64);

        AppState { spinbox_value }.build(cx);

        ExamplePage::new(cx, |cx| {
            Spinbox::new(cx, spinbox_value)
                .icons(SpinboxIcons::PlusMinus)
                .on_change(|ex, v| ex.emit(AppEvent::SetValue(v)));
        });
    })
    .title("Spinbox")
    .run()
}

impl Model for AppState {
    fn event(&mut self, _cx: &mut EventContext, event: &mut Event) {
        event.map(|e, _| match e {
            AppEvent::SetValue(v) => self.spinbox_value.set(*v),
        })
    }
}
