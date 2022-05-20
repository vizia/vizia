use vizia::prelude::*;

struct AppData {}

impl Model for AppData {
    fn event(&mut self, cx: &mut Context, event: &mut Event) {
        event.map(|window_event, _| {
            if let WindowEvent::Debug(txt) = window_event {
                println!("Debug: {}", txt);
            }
        })
    }
}

fn main() {
    Application::new(|cx| {
        AppData {}.build(cx);

        cx.spawn(|cx| {
            cx.emit(WindowEvent::Debug("Test".to_owned())).expect("Failed");
        });
    })
    .run();
}
