mod helpers;
pub use helpers::*;
use vizia::{icons::ICON_X, prelude::*};

#[derive(Lens)]
pub struct AppData {
    tabs: Vec<&'static str>,
}

impl Model for AppData {}

fn main() -> Result<(), ApplicationError> {
    Application::new(|cx| {
        AppData { tabs: vec!["Tab1", "Tab2", "Tab3", "Tab4", "Tab5", "Tab6"] }.build(cx);

        ExamplePage::new(cx, |cx| {
            VStack::new(cx, |cx| {
                TabBar::new(cx, AppData::tabs, |cx, index, item| {
                    HStack::new(cx, |cx| {
                        Label::new(cx, item);
                        Button::new(cx, |cx| Svg::new(cx, ICON_X).size(Stretch(1.0)))
                            .variant(ButtonVariant::Text)
                            .pointer_events(PointerEvents::Auto)
                            .class("close-icon")
                            .height(Pixels(16.0))
                            .width(Pixels(16.0))
                            .padding(Pixels(0.0))
                            .alignment(Alignment::Center);
                    })
                    .pointer_events(PointerEvents::None)
                    .height(Pixels(32.0))
                    .width(Auto)
                    .min_width(Pixels(100.0))
                    .alignment(Alignment::Left)
                    .padding(Pixels(4.0))
                    .gap(Stretch(1.0));
                });
            });

            // TabView::new(cx, AppData::tabs, |cx, item| match item.get(cx) {
            //     "Tab1" => TabPair::new(
            //         move |cx| {
            //             Label::new(cx, item).hoverable(false);
            //             Element::new(cx).class("indicator");
            //         },
            //         |cx| {
            //             Element::new(cx).size(Pixels(200.0)).background_color(Color::red());
            //         },
            //     ),

            //     "Tab2" => TabPair::new(
            //         move |cx| {
            //             Label::new(cx, item).hoverable(false);
            //             Element::new(cx).class("indicator");
            //         },
            //         |cx| {
            //             Element::new(cx).size(Pixels(200.0)).background_color(Color::blue());
            //         },
            //     ),

            //     _ => unreachable!(),
            // })
            // .width(Pixels(500.0))
            // .height(Pixels(300.0));
        });
    })
    .title("Tabview")
    .run()
}
