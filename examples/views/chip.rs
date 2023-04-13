mod helpers;
use helpers::*;
use vizia::prelude::*;

#[derive(Clone, Lens)]
struct AppData {
    chip1: String,
    chip2: String,
}

impl Model for AppData {
    fn event(&mut self, _cx: &mut EventContext, event: &mut Event) {
        event.map(|app_event, _| match app_event {
            AppEvent::Close => {
                println!("Close");
            }
        })
    }
}

enum AppEvent {
    Close,
}

fn main() {
    Application::new(|cx| {
        AppData { chip1: "Chip".to_string(), chip2: "Another Chip".to_string() }.build(cx);

        ExamplePage::new(cx, |cx| {
            Chip::new(cx, AppData::chip1).background_color(Color::from("#00ffff44"));
            Chip::new(cx, AppData::chip2).background_color(Color::from("#ff004444"));
            Chip::new(cx, "red")
                .on_close(|cx| cx.emit(AppEvent::Close))
                .background_color(Color::from("#ff000044"));
        });
    })
    .title("Chip")
    .inner_size((400, 200))
    .run();
}
