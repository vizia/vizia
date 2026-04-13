use vizia::prelude::*;

use crate::DemoRegion;

pub fn grid(cx: &mut Context) {
    VStack::new(cx, |cx| {
        Markdown::new(
            cx,
            "# Grid
A grid arranges children into rows and columns defined by explicit track sizes.",
        );

        Divider::new(cx);

        Markdown::new(cx, "### 2×2 Grid");

        DemoRegion::new(cx, "2×2 Grid", |cx| {
            Grid::new(
                cx,
                vec![Stretch(1.0), Stretch(1.0)],
                vec![Pixels(80.0), Pixels(80.0)],
                |cx| {
                    Label::new(cx, "Col 0 · Row 0")
                        .column_start(0)
                        .row_start(0)
                        .background_color(Color::red())
                        .color(Color::white())
                        .height(Stretch(1.0))
                        .width(Stretch(1.0))
                        .alignment(Alignment::Center);

                    Label::new(cx, "Col 1 · Row 0")
                        .column_start(1)
                        .row_start(0)
                        .background_color(Color::blue())
                        .color(Color::white())
                        .height(Stretch(1.0))
                        .width(Stretch(1.0))
                        .alignment(Alignment::Center);

                    Label::new(cx, "Col 0 · Row 1")
                        .column_start(0)
                        .row_start(1)
                        .background_color(Color::green())
                        .color(Color::white())
                        .height(Stretch(1.0))
                        .width(Stretch(1.0))
                        .alignment(Alignment::Center);

                    Label::new(cx, "Col 1 · Row 1")
                        .column_start(1)
                        .row_start(1)
                        .background_color(Color::yellow())
                        .height(Stretch(1.0))
                        .width(Stretch(1.0))
                        .alignment(Alignment::Center);
                },
            )
            .width(Stretch(1.0))
            .height(Pixels(160.0));
        });

        Markdown::new(cx, "### CSS-driven Grid");

        DemoRegion::new(cx, "CSS Grid", |cx| {
            VStack::new(cx, |cx| {
                Label::new(cx, "Col 0 · Row 0")
                    .column_start(0)
                    .row_start(0)
                    .background_color(Color::red())
                    .color(Color::white())
                    .height(Stretch(1.0))
                    .width(Stretch(1.0))
                    .alignment(Alignment::Center);

                Label::new(cx, "Col 1 · Row 0")
                    .column_start(1)
                    .row_start(0)
                    .background_color(Color::blue())
                    .color(Color::white())
                    .height(Stretch(1.0))
                    .width(Stretch(1.0))
                    .alignment(Alignment::Center);

                Label::new(cx, "Col 0 · Row 1")
                    .column_start(0)
                    .row_start(1)
                    .background_color(Color::green())
                    .color(Color::white())
                    .height(Stretch(1.0))
                    .width(Stretch(1.0))
                    .alignment(Alignment::Center);

                Label::new(cx, "Col 1 · Row 1")
                    .column_start(1)
                    .row_start(1)
                    .background_color(Color::yellow())
                    .height(Stretch(1.0))
                    .width(Stretch(1.0))
                    .alignment(Alignment::Center);
            })
            .class("gallery-css-grid")
            .width(Stretch(1.0))
            .height(Pixels(160.0));
        });
    })
    .class("panel");
}
