use vizia::fonts::unicode_names::CANCEL;
use vizia::prelude::*;

#[derive(Debug, Lens)]
pub struct AppData {
    pub option1: bool,
    pub option2: bool,
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
                self.option1 ^= true;
            }

            AppEvent::ToggleOption2 => {
                self.option2 ^= true;
            }
        });
    }
}

fn main() {
    Application::new(|cx| {
        AppData { option1: true, option2: false }.build(cx);

        VStack::new(cx, |cx| {
            Label::new(cx, "Basic Checkboxes");

            HStack::new(cx, |cx| {
                // CBasic Checkboxes
                Checkbox::new(cx, AppData::option1)
                    .on_toggle(|cx| cx.emit(AppEvent::ToggleOption1));
                Checkbox::new(cx, AppData::option2)
                    .on_toggle(|cx| cx.emit(AppEvent::ToggleOption2));
                Checkbox::new(cx, AppData::option2)
                    .on_toggle(|cx| cx.emit(AppEvent::ToggleOption2))
                    .disabled(true);
                Checkbox::new(cx, AppData::option1)
                    .on_toggle(|cx| cx.emit(AppEvent::ToggleOption1))
                    .disabled(true);
            })
            .col_between(Pixels(4.0));

            Label::new(cx, "Checkbox with Label").top(Pixels(20.0));

            // Checkboxes with label
            let checkbox_identifier = cx.new_entity_identifier();
            HStack::new(cx, |cx| {
                Checkbox::new(cx, AppData::option1)
                    .on_toggle(|cx| cx.emit(AppEvent::ToggleOption1))
                    .identify(checkbox_identifier);
                Label::new(cx, "Checkbox").describing(checkbox_identifier);
            })
            .size(Auto)
            .col_between(Pixels(5.0))
            .child_top(Stretch(1.0))
            .child_bottom(Stretch(1.0));

            let checkbox_identifier = cx.new_entity_identifier();
            HStack::new(cx, |cx| {
                Checkbox::new(cx, AppData::option2)
                    .on_toggle(|cx| cx.emit(AppEvent::ToggleOption2))
                    .identify(checkbox_identifier);
                Label::new(cx, "Disabled").describing(checkbox_identifier);
            })
            .disabled(true)
            .size(Auto)
            .col_between(Pixels(5.0))
            .child_top(Stretch(1.0))
            .child_bottom(Stretch(1.0));

            Label::new(cx, "Checkbox with Label").top(Pixels(20.0)).top(Pixels(20.0));

            let checkbox_identifier = cx.new_entity_identifier();
            HStack::new(cx, |cx| {
                Checkbox::new(cx, AppData::option1)
                    .on_toggle(|cx| cx.emit(AppEvent::ToggleOption1))
                    .text(AppData::option1.map(|flag| if *flag { CANCEL } else { "" }))
                    .identify(checkbox_identifier);
                Label::new(cx, "Custom").describing(checkbox_identifier);
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
    //.ignore_default_theme()
    .title("Checkbox")
    .run();
}
