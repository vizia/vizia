mod helpers;
use helpers::*;

use log::debug;
use vizia::icons::ICON_CHECK;
use vizia::prelude::*;

fn main() -> Result<(), ApplicationError> {
    Application::new(|cx| {
        ExamplePage::vertical(cx, |cx| {
            HStack::new(cx, |cx| {
                // Basic Button
                Button::new(cx, |cx| Label::new(cx, "Button"))
                    .on_press(|_cx| debug!("Button Pressed!"));
                // Accent Button
                Button::new(cx, |cx| Label::new(cx, "Accent Button"))
                    .variant(ButtonVariant::Accent);
                // Outline Button
                Button::new(cx, |cx| Label::new(cx, "Outline Button"))
                    .variant(ButtonVariant::Outline);
                // Ghost Button
                Button::new(cx, |cx| Label::new(cx, "Text Button")).variant(ButtonVariant::Text);
                // Button with Icon
                Button::new(cx, |cx| {
                    HStack::new(cx, |cx| {
                        Svg::new(cx, ICON_CHECK).class("icon");
                        Label::new(cx, "Button with Icon");
                    })
                });
            })
            .size(Auto)
            .horizontal_gap(Pixels(10.0));

            HStack::new(cx, |cx| {
                Button::new(cx, |cx| {
                    HStack::new(cx, |cx| {
                        Svg::new(cx, ICON_CHECK).class("icon");
                        Label::new(cx, "Button with Icon");
                    })
                });
                Button::new(cx, |cx| {
                    HStack::new(cx, |cx| {
                        Svg::new(cx, ICON_CHECK).class("icon");
                        Label::new(cx, "Button with Icon");
                    })
                })
                .variant(ButtonVariant::Accent);
                Button::new(cx, |cx| {
                    HStack::new(cx, |cx| {
                        Svg::new(cx, ICON_CHECK).class("icon");
                        Label::new(cx, "Button with Icon");
                    })
                })
                .variant(ButtonVariant::Outline);
                Button::new(cx, |cx| {
                    HStack::new(cx, |cx| {
                        Svg::new(cx, ICON_CHECK).class("icon");
                        Label::new(cx, "Button with Icon");
                    })
                })
                .variant(ButtonVariant::Text);
            })
            .size(Auto)
            .horizontal_gap(Pixels(10.0));

            HStack::new(cx, |cx| {
                Button::new(cx, |cx| Svg::new(cx, ICON_CHECK).class("icon"));
                Button::new(cx, |cx| Svg::new(cx, ICON_CHECK).class("icon"))
                    .variant(ButtonVariant::Accent);
                Button::new(cx, |cx| Svg::new(cx, ICON_CHECK).class("icon"))
                    .variant(ButtonVariant::Outline);
                Button::new(cx, |cx| Svg::new(cx, ICON_CHECK).class("icon"))
                    .variant(ButtonVariant::Text);
            })
            .size(Auto)
            .horizontal_gap(Pixels(10.0));
        });
    })
    .title("Button")
    .inner_size((700, 200))
    .run()
}
