mod helpers;
use helpers::*;
use vizia::prelude::*;

pub struct AppData {
    pub option1: Signal<bool>,
    pub option2: Signal<bool>,
}

#[derive(Debug)]
pub enum AppEvent {
    ToggleOption1,
    ToggleOption2,
}

impl Model for AppData {
    fn event(&mut self, _cx: &mut EventContext, event: &mut Event) {
        event.map(|app_event, _| match app_event {
            AppEvent::ToggleOption1 => {
                self.option1.update(|option1| *option1 ^= true);
            }

            AppEvent::ToggleOption2 => {
                self.option2.update(|option2| *option2 ^= true);
            }
        });
    }
}

fn main() -> Result<(), ApplicationError> {
    Application::new(|cx| {
        let option1 = Signal::new(true);
        let option2 = Signal::new(false);

        AppData { option1, option2 }.build(cx);

        ExamplePage::vertical(cx, |cx| {
            Label::new(cx, Localized::new("switch-basic"));

            HStack::new(cx, |cx| {
                Switch::new(cx, option1)
                    .on_toggle(|cx| cx.emit(AppEvent::ToggleOption1))
                    .id("Switch_1");
                Label::new(cx, Localized::new("switch-1")).describing("Switch_1");
            })
            .size(Auto)
            .horizontal_gap(Pixels(5.0))
            .alignment(Alignment::Center);

            HStack::new(cx, |cx| {
                Switch::new(cx, option2)
                    .on_toggle(|cx| cx.emit(AppEvent::ToggleOption2))
                    .id("Switch_2");
                Label::new(cx, Localized::new("switch-2")).describing("Switch_2");
            })
            .size(Auto)
            .horizontal_gap(Pixels(5.0))
            .alignment(Alignment::Center);
        });
    })
    .title(Localized::new("view-title-switch"))
    .run()
}
