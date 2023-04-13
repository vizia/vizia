mod helpers;
use helpers::*;
use vizia::prelude::*;

fn main() {
    Application::new(|cx| {
        ExamplePage::new(cx, |cx| {
            Notification::new(
                cx,
                "Notification Title".to_string(),
                Some("This is some information about the notification you just got!".to_string()),
            );
        });
    })
    .title("Notification")
    .run();
}
