mod helpers;
use helpers::*;

use log::debug;
use vizia::icons::ICON_CHECK;
use vizia::prelude::*;

fn main() -> Result<(), ApplicationError> {
    let (app, (title, size)) = Application::new_with_state(|cx| {
        let auto = cx.state(Auto);
        let gap_10 = cx.state(Pixels(10.0));
        let icon_check = cx.state(ICON_CHECK);
        ExamplePage::new(cx, |cx| {
            HStack::new(cx, |cx| {
                let accent = cx.state(ButtonVariant::Accent);
                let outline = cx.state(ButtonVariant::Outline);
                let text = cx.state(ButtonVariant::Text);

                Button::new(cx, |cx| Label::static_text(cx, "Button"))
                    .on_press(|_cx| debug!("Button Pressed!"));
                Button::new(cx, |cx| Label::static_text(cx, "Accent Button")).variant(accent);
                Button::new(cx, |cx| Label::static_text(cx, "Outline Button")).variant(outline);
                Button::new(cx, |cx| Label::static_text(cx, "Text Button")).variant(text);
                Button::new(cx, |cx| {
                    HStack::new(cx, |cx| {
                        Svg::new(cx, icon_check).class("icon");
                        Label::static_text(cx, "Button with Icon");
                    })
                });
                Button::new(cx, |cx| Svg::new(cx, icon_check).class("icon"));
            })
            .size(auto)
            .horizontal_gap(gap_10);
        });
        (cx.state("Button"), cx.state((700, 200)))
    });

    app.title(title).inner_size(size).run()
}
