use vizia::prelude::*;

#[derive(Clone, Lens)]
struct AppState {
    options: Vec<&'static str>,
    selected_option: usize,
}

pub enum AppEvent {
    SetOption(usize),
}

impl Model for AppState {
    fn event(&mut self, _: &mut EventContext, event: &mut Event) {
        event.map(|app_event, _| match app_event {
            AppEvent::SetOption(index) => {
                self.selected_option = *index;
            }
        });
    }
}

#[allow(dead_code)]
const DARK_THEME: &str = "crates/vizia_core/resources/themes/dark_theme.css";
#[allow(dead_code)]
const LIGHT_THEME: &str = "crates/vizia_core/resources/themes/light_theme.css";

fn main() {
    Application::new(|cx| {
        AppState { options: vec!["One", "Two", "Three"], selected_option: 0 }.build(cx);

        cx.add_stylesheet(DARK_THEME).expect("Failed to find stylesheet");

        PickList::new(cx, AppState::options, AppState::selected_option, true)
            .on_select(|cx, index| cx.emit(AppEvent::SetOption(index)));
    })
    .ignore_default_theme()
    .title("Datepicker")
    .run();
}
