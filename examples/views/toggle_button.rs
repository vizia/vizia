mod helpers;
use helpers::*;

use vizia::icons::{ICON_BOLD, ICON_ITALIC, ICON_UNDERLINE};
use vizia::prelude::*;

#[derive(Lens)]
pub struct AppData {
    bold: bool,
    italic: bool,
    underline: bool,
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
                self.bold ^= true;
            }

            AppEvent::ToggleItalic => {
                self.italic ^= true;
            }

            AppEvent::ToggleUnderline => {
                self.underline ^= true;
            }
        })
    }
}

fn main() -> Result<(), ApplicationError> {
    Application::new(|cx| {
        AppData { bold: false, italic: false, underline: false }.build(cx);

        ExamplePage::vertical(cx, |cx| {
            ToggleButton::new(cx, AppData::bold, |cx| Label::new(cx, "Bold"))
                .on_toggle(|cx| cx.emit(AppEvent::ToggleBold));

            ButtonGroup::new(cx, |cx| {
                ToggleButton::new(cx, AppData::bold, |cx| Svg::new(cx, ICON_BOLD))
                    .on_toggle(|cx| cx.emit(AppEvent::ToggleBold));

                ToggleButton::new(cx, AppData::italic, |cx| Svg::new(cx, ICON_ITALIC))
                    .on_toggle(|cx| cx.emit(AppEvent::ToggleItalic));

                ToggleButton::new(cx, AppData::underline, |cx| Svg::new(cx, ICON_UNDERLINE))
                    .on_toggle(|cx| cx.emit(AppEvent::ToggleUnderline));
            });
        });
    })
    .title("ToggleButton")
    .inner_size((700, 200))
    .run()
}
