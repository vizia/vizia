mod helpers;
pub use helpers::*;
use vizia::prelude::*;

#[derive(Clone, Lens)]
struct AppState {
    x: f32,
    y: f32,
}

pub enum AppEvent {
    SetValue(f32, f32),
}

impl Model for AppState {
    fn event(&mut self, _: &mut EventContext, event: &mut Event) {
        event.map(|app_event, _| match app_event {
            AppEvent::SetValue(x, y) => {
                self.x = *x;
                self.y = *y;
            }
        });
    }
}

fn main() {
    Application::new(|cx| {
        AppState { x: 0.2, y: 0.5 }.build(cx);

        ExamplePage::new(cx, |cx| {
            XYPad::new(cx, AppState::root.map(|app_state| (app_state.x, app_state.y)))
                .on_change(|cx, x, y| cx.emit(AppEvent::SetValue(x, y)));
        });
    })
    .title("XYPad")
    .run();
}
