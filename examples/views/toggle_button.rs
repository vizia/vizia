mod helpers;
use helpers::*;

use vizia::icons::{ICON_BOLD, ICON_ITALIC, ICON_UNDERLINE};
use vizia::prelude::*;

pub struct AppData {
    bold: Signal<bool>,
    italic: Signal<bool>,
    underline: Signal<bool>,
}

pub enum AppEvent {
    ToggleBold,
    ToggleItalic,
    ToggleUnderline,
}

impl Model for AppData {
    fn event(&mut self, _cx: &mut EventContext, event: &mut Event) {
        event.map(|app_event, _| match app_event {
            AppEvent::ToggleBold => {
                self.bold.update(|bold| *bold ^= true);
            }

            AppEvent::ToggleItalic => {
                self.italic.update(|italic| *italic ^= true);
            }

            AppEvent::ToggleUnderline => {
                self.underline.update(|underline| *underline ^= true);
            }
        })
    }
}

fn main() -> Result<(), ApplicationError> {
    Application::new(|cx| {
        let bold = Signal::new(false);
        let italic = Signal::new(false);
        let underline = Signal::new(false);

        AppData { bold, italic, underline }.build(cx);

        ExamplePage::vertical(cx, |cx| {
            ToggleButton::new(cx, bold, |cx| Label::new(cx, "Bold"))
                .on_toggle(|cx| cx.emit(AppEvent::ToggleBold));

            ButtonGroup::new(cx, |cx| {
                ToggleButton::new(cx, bold, |cx| Svg::new(cx, ICON_BOLD))
                    .on_toggle(|cx| cx.emit(AppEvent::ToggleBold));

                ToggleButton::new(cx, italic, |cx| Svg::new(cx, ICON_ITALIC))
                    .on_toggle(|cx| cx.emit(AppEvent::ToggleItalic));

                ToggleButton::new(cx, underline, |cx| Svg::new(cx, ICON_UNDERLINE))
                    .on_toggle(|cx| cx.emit(AppEvent::ToggleUnderline));
            });
        });
    })
    .title("ToggleButton")
    .inner_size((700, 200))
    .run()
}
