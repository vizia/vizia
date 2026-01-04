mod helpers;
use helpers::*;

use vizia::icons::{ICON_BOLD, ICON_ITALIC, ICON_UNDERLINE};
use vizia::prelude::*;

fn main() -> Result<(), ApplicationError> {
    ToggleButtonApp::run()
}

struct ToggleButtonApp {
    bold: Signal<bool>,
    italic: Signal<bool>,
    underline: Signal<bool>,
}

impl App for ToggleButtonApp {
    fn new(cx: &mut Context) -> Self {
        Self {
            bold: cx.state(false),
            italic: cx.state(false),
            underline: cx.state(false),
        }
    }

    fn on_build(self, cx: &mut Context) -> Self {
        let bold = self.bold;
        let italic = self.italic;
        let underline = self.underline;

        ExamplePage::vertical(cx, |cx| {
            ToggleButton::new(cx, bold, |cx| Label::new(cx, "Bold")).two_way();

            ButtonGroup::new(cx, |cx| {
                ToggleButton::new(cx, bold, |cx| Svg::new(cx, ICON_BOLD)).two_way();

                ToggleButton::new(cx, italic, |cx| Svg::new(cx, ICON_ITALIC)).two_way();

                ToggleButton::new(cx, underline, |cx| Svg::new(cx, ICON_UNDERLINE)).two_way();
            });
        });
        self
    }

    fn window_config(&self) -> WindowConfig {
        window(|app| app.title("ToggleButton").inner_size((700, 200)))
    }
}
