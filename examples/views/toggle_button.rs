mod helpers;
use helpers::*;

use vizia::prelude::*;

#[derive(Lens)]
pub struct AppData {
    flag: bool,
}

impl Model for AppData {}

fn main() {
    Application::new(|cx| {
        AppData { flag: false }.build(cx);

        ExamplePage::vertical(cx, |cx| {
            ToggleButton::new(cx, AppData::flag, |cx| Label::new(cx, "Toggle"));
        });
    })
    .title("ToggleButton")
    .inner_size((700, 200))
    .run();
}
