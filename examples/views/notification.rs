mod helpers;
use helpers::*;
use vizia::prelude::*;

fn main() {
    Application::new(|cx| {
        view_controls(cx);

        VStack::new(cx, |cx| {
            Notification::new(
                cx,
                "Notification Title".to_string(),
                Some("This is some information about the notification you just got!".to_string()),
            );
        })
        .disabled(ControlsData::disabled)
        .class("container");
    })
    .title("Popup")
    .run();
}
