use vizia::prelude::*;

#[allow(dead_code)]
const DARK_THEME: &str = "crates/vizia_core/resources/themes/dark_theme.css";
#[allow(dead_code)]
const LIGHT_THEME: &str = "crates/vizia_core/resources/themes/light_theme.css";

#[derive(Lens)]
pub struct AppData {
    count: i32,
}

pub enum AppEvent {
    Increment,
}

impl Model for AppData {
    fn event(&mut self, _: &mut EventContext, event: &mut Event) {
        event.map(|app_event, _| match app_event {
            AppEvent::Increment => {
                self.count += 1;
            }
        });
    }
}

fn main() {
    Application::new(|cx| {
        cx.add_stylesheet(DARK_THEME).expect("Failed to find stylesheet");

        AppData { count: 0 }.build(cx);

        HStack::new(cx, |cx| {
            Button::new(cx, |cx| cx.emit(AppEvent::Increment), |cx| Label::new(cx, "Increment"));

            Label::new(cx, AppData::count).width(Pixels(50.0));
        })
        .child_space(Stretch(1.0))
        .col_between(Pixels(50.0));
    })
    .title("Counter")
    .inner_size((400, 100))
    .ignore_default_theme()
    .run();
}
