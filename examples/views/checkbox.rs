use vizia::prelude::*;

#[derive(Debug, Lens)]
pub struct AppData {
    pub option: bool,
}

#[derive(Debug)]
pub enum AppEvent {
    ToggleOption,
}

impl Model for AppData {
    fn event(&mut self, _cx: &mut Context, event: &mut Event) {
        event.map(|app_event, _| match app_event {
            AppEvent::ToggleOption => {
                self.option ^= true;
            }
        });
    }
}

fn main() {
    Application::new(|cx| {
        AppData { option: false }.build(cx);

        VStack::new(cx, |cx| {
            // Checkbox
            Checkbox::new(cx, AppData::option).on_toggle(|cx| cx.emit(AppEvent::ToggleOption));

            // Checkboxe with label
            HStack::new(cx, |cx| {
                Checkbox::new(cx, AppData::option).on_toggle(|cx| cx.emit(AppEvent::ToggleOption));
                Label::new(cx, "Checkbox");
            })
            .size(Auto)
            .col_between(Pixels(5.0))
            .child_top(Stretch(1.0))
            .child_bottom(Stretch(1.0));
        })
        .size(Auto)
        .row_between(Pixels(10.0))
        .space(Stretch(1.0));
    })
    .title("Checkbox")
    .run();
}
