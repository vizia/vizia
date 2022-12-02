use vizia::prelude::*;

#[derive(Lens)]
pub struct AppData {
    flag: bool,
}

pub enum AppEvent {
    ToggleFlag,
}

impl Model for AppData {
    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|app_event, _| match app_event {
            AppEvent::ToggleFlag => {
                self.flag ^= true;
            }
        });
    }
}

fn main() {
    Application::new(|cx| {
        AppData { flag: false }.build(cx);

        // Label::new(cx, "Hello");
        // Label::new(cx, "World");
        // Label::new(cx, "This vizia application is accessible thanks to Accesskit");
        Checkbox::new(cx, AppData::flag)
            .on_toggle(|cx| cx.emit(AppEvent::ToggleFlag))
            .name("Click me");
    })
    .title("AccessKit")
    .run();
}
