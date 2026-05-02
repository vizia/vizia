use vizia::prelude::*;

use crate::DemoRegion;

pub fn hstack(cx: &mut Context) {
    VStack::new(cx, |cx| {
        Label::new(cx, Localized::new("hstack")).class("panel-title");

        Divider::new(cx);

        DemoRegion::new(cx, "HStack", |cx| {
            HStack::new(cx, |cx| {
                Element::new(cx).size(Pixels(100.0)).background_color(Color::red());
                Element::new(cx).size(Pixels(100.0)).background_color(Color::green());
                Element::new(cx).size(Pixels(100.0)).background_color(Color::blue());
            })
            .height(Auto)
            .alignment(Alignment::Center);
        });
    })
    .class("panel");
}

pub fn vstack(cx: &mut Context) {
    VStack::new(cx, |cx| {
        Label::new(cx, Localized::new("vstack")).class("panel-title");

        Divider::new(cx);

        DemoRegion::new(cx, "VStack", |cx| {
            VStack::new(cx, |cx| {
                Element::new(cx).size(Pixels(100.0)).background_color(Color::red());
                Element::new(cx).size(Pixels(100.0)).background_color(Color::green());
                Element::new(cx).size(Pixels(100.0)).background_color(Color::blue());
            })
            .height(Auto)
            .alignment(Alignment::Center);
        });
    })
    .class("panel");
}

pub fn zstack(cx: &mut Context) {
    VStack::new(cx, |cx| {
        Label::new(cx, Localized::new("zstack")).class("panel-title");

        Divider::new(cx);

        DemoRegion::new(cx, "ZStack", |cx| {
            ZStack::new(cx, |cx| {
                Element::new(cx).size(Pixels(100.0)).background_color(Color::red());
                Element::new(cx).size(Pixels(70.0)).background_color(Color::green());
                Element::new(cx).size(Pixels(40.0)).background_color(Color::blue());
            })
            .size(Auto)
            .padding(Pixels(20.0))
            .alignment(Alignment::Center);
        });
    })
    .class("panel");
}
