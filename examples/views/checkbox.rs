mod helpers;
use helpers::*;
use vizia::icons::{ICON_EYE, ICON_EYE_OFF};
use vizia::prelude::*;

#[derive(Debug, Lens)]
pub struct AppData {
    pub option1: Signal<bool>,
    pub option2: Signal<bool>,
}

#[derive(Debug)]
pub enum AppEvent {
    ToggleOptions,
}

impl Model for AppData {
    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|app_event, _| match app_event {
            AppEvent::ToggleOptions => {
                self.option1.set(cx, !self.option1.get(cx));
                self.option2.set(cx, !self.option2.get(cx));
            }
        });
    }
}

fn main() -> Result<(), ApplicationError> {
    Application::new(|cx| {
        AppData { option1: cx.state(true), option2: cx.state(false) }.build(cx);

        // ExamplePage::vertical(cx, |cx| {
        Label::new(cx, "Checkbox with label").class("h2");

        VStack::new(cx, |cx| {
            // Checkboxes with label
            HStack::new(cx, |cx| {
                Checkbox::new(cx, AppData::option1.get(cx))
                    .on_toggle(|cx| cx.emit(AppEvent::ToggleOptions))
                    .id("checkbox_1");
                Label::new(cx, "Checkbox 1").describing("checkbox_1");
            })
            .size(Auto)
            .horizontal_gap(Pixels(5.0))
            .alignment(Alignment::Center);

            HStack::new(cx, |cx| {
                Checkbox::new(cx, AppData::option2.get(cx))
                    .on_toggle(|cx| cx.emit(AppEvent::ToggleOptions))
                    .id("checkbox_2");
                Label::new(cx, "Checkbox 2").describing("checkbox_2");
            })
            .size(Auto)
            .horizontal_gap(Pixels(5.0))
            .alignment(Alignment::Center);
        })
        .vertical_gap(Pixels(10.0))
        .size(Auto);

        Label::new(cx, "Checkbox with custom icon and label").class("h2");

        HStack::new(cx, |cx| {
            Checkbox::with_icons(cx, AppData::option1.get(cx), Some(ICON_EYE_OFF), Some(ICON_EYE))
                .on_toggle(|cx| cx.emit(AppEvent::ToggleOptions))
                .id("checkbox_3");
            Label::new(cx, "Checkbox 3").describing("checkbox_3");
        })
        .size(Auto)
        .horizontal_gap(Pixels(5.0))
        .alignment(Alignment::Center);
        //});
    })
    .title("Checkbox")
    .inner_size((300, 320))
    .run()
}
