mod helpers;
use helpers::*;

use vizia::icons::{ICON_BOLD, ICON_CHECK, ICON_ITALIC, ICON_UNDERLINE};
use vizia::prelude::*;

struct AppData {
    bold: Signal<bool>,
    italic: Signal<bool>,
    underline: Signal<bool>,
}

enum AppEvent {
    ToggleBold,
    ToggleItalic,
    ToggleUnderline,
}

impl Model for AppData {
    fn event(&mut self, _cx: &mut EventContext, event: &mut Event) {
        event.map(|app_event, _| match app_event {
            AppEvent::ToggleBold => self.bold.update(|v| *v ^= true),
            AppEvent::ToggleItalic => self.italic.update(|v| *v ^= true),
            AppEvent::ToggleUnderline => self.underline.update(|v| *v ^= true),
        });
    }
}

fn main() -> Result<(), ApplicationError> {
    Application::new(|cx| {
        let bold = Signal::new(false);
        let italic = Signal::new(false);
        let underline = Signal::new(false);

        AppData { bold, italic, underline }.build(cx);

        ExamplePage::vertical(cx, |cx| {
            // Basic horizontal button group
            ButtonGroup::new(cx, |cx| {
                Button::new(cx, |cx| Label::new(cx, "One"));
                Button::new(cx, |cx| Label::new(cx, "Two"));
                Button::new(cx, |cx| Label::new(cx, "Three"));
            });

            // Button group with variants
            ButtonGroup::new(cx, |cx| {
                Button::new(cx, |cx| Label::new(cx, "One"));
                Button::new(cx, |cx| Label::new(cx, "Two"));
                Button::new(cx, |cx| Label::new(cx, "Three"));
            })
            .variant(ButtonVariant::Secondary);

            ButtonGroup::new(cx, |cx| {
                Button::new(cx, |cx| Label::new(cx, "One"));
                Button::new(cx, |cx| Label::new(cx, "Two"));
                Button::new(cx, |cx| Label::new(cx, "Three"));
            })
            .variant(ButtonVariant::Outline);

            // Button group with icons
            ButtonGroup::new(cx, |cx| {
                Button::new(cx, |cx| {
                    HStack::new(cx, |cx| {
                        Svg::new(cx, ICON_CHECK).class("icon");
                        Label::new(cx, "Accept");
                    })
                });
                Button::new(cx, |cx| Label::new(cx, "Maybe"));
                Button::new(cx, |cx| Label::new(cx, "Decline"));
            });

            // Vertical button group
            ButtonGroup::new(cx, |cx| {
                Button::new(cx, |cx| Label::new(cx, "Top"));
                Button::new(cx, |cx| Label::new(cx, "Middle"));
                Button::new(cx, |cx| Label::new(cx, "Bottom"));
            })
            .vertical(true);

            // Toggle button group (text formatting toolbar)
            ButtonGroup::new(cx, |cx| {
                ToggleButton::new(cx, bold, |cx| Svg::new(cx, ICON_BOLD).class("icon"))
                    .on_toggle(|cx| cx.emit(AppEvent::ToggleBold));
                ToggleButton::new(cx, italic, |cx| Svg::new(cx, ICON_ITALIC).class("icon"))
                    .on_toggle(|cx| cx.emit(AppEvent::ToggleItalic));
                ToggleButton::new(cx, underline, |cx| Svg::new(cx, ICON_UNDERLINE).class("icon"))
                    .on_toggle(|cx| cx.emit(AppEvent::ToggleUnderline));
            });
        });
    })
    .title("Button Group")
    .inner_size((700, 500))
    .run()
}
