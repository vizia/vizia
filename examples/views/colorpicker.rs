mod helpers;
use helpers::*;
use vizia::prelude::*;

#[derive(Clone, Lens)]
struct AppState {
    color: Color,
}

pub enum AppEvent {
    SetColor(Color),
}

impl Model for AppState {
    fn event(&mut self, _: &mut EventContext, event: &mut Event) {
        event.map(|app_event, _| match app_event {
            AppEvent::SetColor(color) => {
                self.color = *color;
            }
        });
    }
}

fn main() {
    Application::new(|cx| {
        AppState { color: Color::rgb(200, 100, 50) }.build(cx);

        ExamplePage::new(cx, |cx| {
            ColorPicker::new(cx, AppState::color)
                .on_change(|cx, color| cx.emit(AppEvent::SetColor(color)));
        });
    })
    .title("Colorpicker")
    .run();
}
