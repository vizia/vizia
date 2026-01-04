mod helpers;
use helpers::*;

use log::debug;
use vizia::icons::ICON_CHECK;
use vizia::prelude::*;

fn main() -> Result<(), ApplicationError> {
    ButtonApp::run()
}

struct ButtonApp;

impl App for ButtonApp {
    fn new(_cx: &mut Context) -> Self {
        Self
    }

    fn on_build(self, cx: &mut Context) -> Self {
        ExamplePage::new(cx, |cx| {
            HStack::new(cx, |cx| {
                Button::new(cx, |cx| Label::new(cx, "Button"))
                    .on_press(|_cx| debug!("Button Pressed!"));
                Button::new(cx, |cx| Label::new(cx, "Accent Button")).variant(ButtonVariant::Accent);
                Button::new(cx, |cx| Label::new(cx, "Outline Button")).variant(ButtonVariant::Outline);
                Button::new(cx, |cx| Label::new(cx, "Text Button")).variant(ButtonVariant::Text);
                Button::new(cx, |cx| {
                    HStack::new(cx, |cx| {
                        Svg::new(cx, ICON_CHECK).class("icon");
                        Label::new(cx, "Button with Icon");
                    })
                });
                Button::new(cx, |cx| Svg::new(cx, ICON_CHECK).class("icon"));
            })
            .size(Auto)
            .horizontal_gap(Pixels(10.0));
        });
        self
    }

    fn window_config(&self) -> WindowConfig {
        window(|app| app.title("Button").inner_size((700, 200)))
    }
}