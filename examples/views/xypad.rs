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

const CENTER_LAYOUT: &str = "crates/vizia_core/resources/themes/center_layout.css";
#[allow(dead_code)]
const DARK_THEME: &str = "crates/vizia_core/resources/themes/dark_theme.css";
#[allow(dead_code)]
const LIGHT_THEME: &str = "crates/vizia_core/resources/themes/light_theme.css";

fn main() {
    Application::new(|cx| {
        AppState { x: 0.2, y: 0.5 }.build(cx);

        cx.add_stylesheet(CENTER_LAYOUT).expect("Failed to find stylesheet");
        cx.add_stylesheet(DARK_THEME).expect("Failed to find stylesheet");

        XYPad::new(cx, AppState::root.map(|app_state| (app_state.x, app_state.y)))
            .on_change(|cx, x, y| cx.emit(AppEvent::SetValue(x, y)));
    })
    .ignore_default_theme()
    .title("Colorpicker")
    .run();
}
