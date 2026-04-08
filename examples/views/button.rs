mod helpers;
use std::alloc::Layout;

use helpers::*;

use log::debug;
use vizia::icons::ICON_CHECK;
use vizia::prelude::*;

fn main() -> Result<(), ApplicationError> {
    Application::new(|cx| {
        ExamplePage::vertical(cx, |cx| {
            HStack::new(cx, |cx| {
                // Basic Button
                Button::new(cx, |cx| Label::new(cx, Localized::new("button")))
                    .on_press(|_cx| debug!("Button Pressed!"));
                // Secondary Button
                Button::new(cx, |cx| Label::new(cx, Localized::new("secondary-button")))
                    .variant(ButtonVariant::Secondary);
                // Outline Button
                Button::new(cx, |cx| Label::new(cx, Localized::new("outline-button")))
                    .variant(ButtonVariant::Outline);
                // Text Button
                Button::new(cx, |cx| Label::new(cx, Localized::new("text-button")))
                    .variant(ButtonVariant::Text);
            })
            .wrap(LayoutWrap::Wrap)
            .width(Stretch(1.0))
            .height(Auto)
            .alignment(Alignment::Center)
            .gap(Pixels(8.0));

            HStack::new(cx, |cx| {
                Button::new(cx, |cx| {
                    HStack::new(cx, |cx| {
                        Svg::new(cx, ICON_CHECK).class("icon");
                        Label::new(cx, Localized::new("button-with-icon"));
                    })
                });
                Button::new(cx, |cx| {
                    HStack::new(cx, |cx| {
                        Svg::new(cx, ICON_CHECK).class("icon");
                        Label::new(cx, Localized::new("button-with-icon"));
                    })
                })
                .variant(ButtonVariant::Secondary);
                Button::new(cx, |cx| {
                    HStack::new(cx, |cx| {
                        Svg::new(cx, ICON_CHECK).class("icon");
                        Label::new(cx, Localized::new("button-with-icon"));
                    })
                })
                .variant(ButtonVariant::Outline);
                Button::new(cx, |cx| {
                    HStack::new(cx, |cx| {
                        Svg::new(cx, ICON_CHECK).class("icon");
                        Label::new(cx, Localized::new("button-with-icon"));
                    })
                })
                .variant(ButtonVariant::Text);
            })
            .height(Auto)
            .width(Stretch(1.0))
            .alignment(Alignment::Center)
            .wrap(LayoutWrap::Wrap)
            .gap(Pixels(8.0));

            HStack::new(cx, |cx| {
                Button::new(cx, |cx| Svg::new(cx, ICON_CHECK).class("icon"));
                Button::new(cx, |cx| Svg::new(cx, ICON_CHECK).class("icon"))
                    .variant(ButtonVariant::Secondary);
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
    .inner_size((700, 300))
    .run()
}
