use vizia::fonts::unicode_names::CHECK;
use vizia::prelude::*;

const CENTER_LAYOUT: &str = "crates/vizia_core/resources/themes/center_layout.css";
#[allow(dead_code)]
const DARK_THEME: &str = "crates/vizia_core/resources/themes/dark_theme.css";
#[allow(dead_code)]
const LIGHT_THEME: &str = "crates/vizia_core/resources/themes/light_theme.css";

#[derive(Lens)]
pub struct AppData {
    disabled: bool,
}

pub enum AppEvent {
    ToggleDisabled,
}

impl Model for AppData {
    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|app_event, _| match app_event {
            AppEvent::ToggleDisabled => {
                self.disabled ^= true;
            }
        });
    }
}

fn main() {
    Application::new(|cx| {
        cx.add_stylesheet(CENTER_LAYOUT).expect("Failed to find stylesheet");
        cx.add_stylesheet(DARK_THEME).expect("Failed to find stylesheet");

        AppData { disabled: false }.build(cx);

        HStack::new(cx, |cx| {
            Switch::new(cx, AppData::disabled).on_toggle(|cx| cx.emit(AppEvent::ToggleDisabled));
            Label::new(cx, "Toggle Disabled");
        })
        .position_type(PositionType::SelfDirected)
        .left(Stretch(1.0))
        .right(Pixels(20.0))
        .top(Pixels(20.0))
        .child_top(Stretch(1.0))
        .child_bottom(Stretch(1.0))
        .col_between(Pixels(5.0))
        .size(Auto);

        VStack::new(cx, |cx| {
            // Basic Button
            Button::new(cx, |_| {}, |cx| Label::new(cx, "Button")).disabled(AppData::disabled);
            // Accent Button
            Button::new(cx, |_| {}, |cx| Label::new(cx, "Accent Button"))
                .class("accent")
                .disabled(AppData::disabled);
            // Outline Button
            Button::new(cx, |_| {}, |cx| Label::new(cx, "Outline Button"))
                .class("outline")
                .disabled(AppData::disabled);
            // Ghost Button
            Button::new(cx, |_| {}, |cx| Label::new(cx, "Ghost Button"))
                .class("ghost")
                .disabled(AppData::disabled);
            // Button with Icon
            Button::new(
                cx,
                |_| {},
                |cx| {
                    HStack::new(cx, |cx| {
                        Label::new(cx, CHECK).class("icon");
                        Label::new(cx, "Button with Icon");
                    })
                    .size(Auto)
                    .child_space(Stretch(1.0))
                    .col_between(Pixels(4.0))
                },
            )
            .disabled(AppData::disabled);
        })
        .class("container");
    })
    .ignore_default_theme()
    .title("Button")
    .run();
}
