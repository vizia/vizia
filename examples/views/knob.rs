use vizia::prelude::*;

#[derive(Lens)]
pub struct AppData {
    value: f32,
    tick: u8,
}

pub const KNOB_TICKS: u8 = 5;

#[derive(Debug)]
pub enum AppEvent {
    SetValue(f32),
    SetTick(u8),
}

impl Model for AppData {
    fn event(&mut self, _: &mut EventContext, event: &mut Event) {
        event.map(|app_event, _| match app_event {
            AppEvent::SetValue(value) => self.value = *value,

            AppEvent::SetTick(tick) => self.tick = *tick,
        });
    }
}

const CENTER_LAYOUT: &str = "crates/vizia_core/resources/themes/center_layout.css";
#[allow(dead_code)]
const DARK_THEME: &str = "crates/vizia_core/resources/themes/dark_theme.css";
#[allow(dead_code)]
const LIGHT_THEME: &str = "crates/vizia_core/resources/themes/light_theme.css";

fn main() {
    Application::new(|cx| {
        cx.add_stylesheet(CENTER_LAYOUT).expect("Failed to find stylesheet");
        cx.add_stylesheet(DARK_THEME).expect("Failed to find stylesheet");

        AppData { value: 0.2, tick: 3 }.build(cx);

        HStack::new(cx, |cx| {
            Knob::new(cx, AppData::value, 0.5, 300.0, 0.0, false).on_changing(|cx, val| {
                cx.emit(AppEvent::SetValue(val));
            });
            Knob::new_discrete(cx, AppData::tick, 3, 120.0, 0.0, KNOB_TICKS, false).on_changing(
                |cx, val| {
                    cx.emit(AppEvent::SetTick(val));
                },
            );
            Knob::new_discrete(cx, AppData::tick, 3, 300.0, 0.0, KNOB_TICKS, false)
                .on_changing(|cx, val| {
                    cx.emit(AppEvent::SetTick(val));
                })
                .knob_type(KnobType::Arc);
            Knob::new(cx, AppData::value, 0.5, 300.0, 0.0, true)
                .on_changing(|cx, val| {
                    cx.emit(AppEvent::SetValue(val));
                })
                .class("small");

            Knob::new_discrete(cx, AppData::tick, 3, 120.0, 45.0, KNOB_TICKS, false)
                .on_changing(|cx, val| {
                    cx.emit(AppEvent::SetTick(val));
                })
                .knob_type(KnobType::Tick)
                .class("small");
        })
        .class("container");
    })
    .ignore_default_theme()
    .title("Knob")
    .run();
}
