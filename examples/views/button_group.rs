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
    Bold,
    Italic,
    Underline,
}

impl Model for AppData {
    fn event(&mut self, _cx: &mut EventContext, event: &mut Event) {
        event.map(|app_event, _| match app_event {
            AppEvent::Bold => self.bold.update(|v| *v ^= true),
            AppEvent::Italic => self.italic.update(|v| *v ^= true),
            AppEvent::Underline => self.underline.update(|v| *v ^= true),
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
                Button::new(cx, |cx| Label::new(cx, Localized::new("one")));
                Button::new(cx, |cx| Label::new(cx, Localized::new("two")));
                Button::new(cx, |cx| Label::new(cx, Localized::new("three")));
            });

            // Button group with variants
            ButtonGroup::new(cx, |cx| {
                Button::new(cx, |cx| Label::new(cx, Localized::new("one")));
                Button::new(cx, |cx| Label::new(cx, Localized::new("two")));
                Button::new(cx, |cx| Label::new(cx, Localized::new("three")));
            })
            .variant(ButtonVariant::Secondary);

            ButtonGroup::new(cx, |cx| {
                Button::new(cx, |cx| Label::new(cx, Localized::new("one")));
                Button::new(cx, |cx| Label::new(cx, Localized::new("two")));
                Button::new(cx, |cx| Label::new(cx, Localized::new("three")));
            })
            .variant(ButtonVariant::Outline);

            // Button group with icons
            ButtonGroup::new(cx, |cx| {
                Button::new(cx, |cx| {
                    HStack::new(cx, |cx| {
                        Svg::new(cx, ICON_CHECK).class("icon");
                        Label::new(cx, Localized::new("button-accept"));
                    })
                });
                Button::new(cx, |cx| Label::new(cx, Localized::new("button-maybe")));
                Button::new(cx, |cx| Label::new(cx, Localized::new("button-decline")));
            });

            // Vertical button group
            ButtonGroup::new(cx, |cx| {
                Button::new(cx, |cx| Label::new(cx, Localized::new("button-top")));
                Button::new(cx, |cx| Label::new(cx, Localized::new("button-middle")));
                Button::new(cx, |cx| Label::new(cx, Localized::new("button-bottom")));
            })
            .vertical(true);

            // Toggle button group (text formatting toolbar)
            ButtonGroup::new(cx, |cx| {
                ToggleButton::new(cx, bold, |cx| Svg::new(cx, ICON_BOLD).class("icon"))
                    .on_toggle(|cx| cx.emit(AppEvent::Bold));
                ToggleButton::new(cx, italic, |cx| Svg::new(cx, ICON_ITALIC).class("icon"))
                    .on_toggle(|cx| cx.emit(AppEvent::Italic));
                ToggleButton::new(cx, underline, |cx| Svg::new(cx, ICON_UNDERLINE).class("icon"))
                    .on_toggle(|cx| cx.emit(AppEvent::Underline));
            });
        });
    })
    .title(Localized::new("view-title-button-group"))
    .inner_size((700, 500))
    .run()
}
