mod helpers;
use helpers::*;
use vizia::prelude::*;

fn main() -> Result<(), ApplicationError> {
    let (app, title) = Application::new_with_state(|cx| {
        let option1 = cx.state(true);
        let option2 = cx.state(false);
        let auto = cx.state(Auto);
        let gap_5 = cx.state(Pixels(5.0));
        let align_center = cx.state(Alignment::Center);

        ExamplePage::vertical(cx, |cx| {
            Label::static_text(cx, "Basic Switches");

            HStack::new(cx, |cx| {
                Switch::new(cx, option1).two_way().id("Switch_1");
                Label::static_text(cx, "Switch 1").describing("Switch_1");
            })
            .size(auto)
            .horizontal_gap(gap_5)
            .alignment(align_center);

            HStack::new(cx, |cx| {
                Switch::new(cx, option2).two_way().id("Switch_2");
                Label::static_text(cx, "Switch 2").describing("Switch_2");
            })
            .size(auto)
            .horizontal_gap(gap_5)
            .alignment(align_center);
        });
        cx.state("Switch")
    });

    app.title(title).run()
}
