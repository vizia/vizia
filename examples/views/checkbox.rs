mod helpers;
use helpers::*;
use vizia::icons::{ICON_EYE, ICON_EYE_OFF};
use vizia::prelude::*;

fn main() -> Result<(), ApplicationError> {
    let (app, (title, size)) = Application::new_with_state(|cx| {
        let option1 = cx.state(true);
        let option2 = cx.state(false);
        let auto = cx.state(Auto);
        let gap_5 = cx.state(Pixels(5.0));
        let gap_10 = cx.state(Pixels(10.0));
        let align_center = cx.state(Alignment::Center);
        let icon_off = cx.state(ICON_EYE_OFF);
        let icon_on = cx.state(ICON_EYE);

        ExamplePage::vertical(cx, |cx| {
            Label::static_text(cx, "Checkbox with label").class("h2");

            VStack::new(cx, |cx| {
                HStack::new(cx, |cx| {
                    Checkbox::new(cx, option1)
                        .on_toggle(move |cx| {
                            option1.set(cx, !option1.get(cx));
                            option2.set(cx, !option2.get(cx));
                        })
                        .id("checkbox_1");
                    Label::static_text(cx, "Checkbox 1").describing("checkbox_1");
                })
                .size(auto)
                .horizontal_gap(gap_5)
                .alignment(align_center);

                HStack::new(cx, |cx| {
                    Checkbox::new(cx, option2)
                        .on_toggle(move |cx| {
                            option1.set(cx, !option1.get(cx));
                            option2.set(cx, !option2.get(cx));
                        })
                        .id("checkbox_2");
                    Label::static_text(cx, "Checkbox 2").describing("checkbox_2");
                })
                .size(auto)
                .horizontal_gap(gap_5)
                .alignment(align_center);
            })
            .vertical_gap(gap_10)
            .size(auto);

            Label::static_text(cx, "Checkbox with custom icon and label").class("h2");

            HStack::new(cx, |cx| {
                Checkbox::with_icons(cx, option1, Some(icon_off), Some(icon_on))
                    .on_toggle(move |cx| {
                        option1.set(cx, !option1.get(cx));
                        option2.set(cx, !option2.get(cx));
                    })
                    .id("checkbox_3");
                Label::static_text(cx, "Checkbox 3").describing("checkbox_3");
            })
            .size(auto)
            .horizontal_gap(gap_5)
            .alignment(align_center);
        });
        (cx.state("Checkbox"), cx.state((300, 320)))
    });

    app.title(title).inner_size(size).run()
}
