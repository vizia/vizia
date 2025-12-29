mod helpers;
use helpers::*;
use vizia::prelude::*;

fn main() -> Result<(), ApplicationError> {
    Application::new(|cx| {
        let option1 = cx.state(true);
        let option2 = cx.state(false);

        ExamplePage::vertical(cx, |cx| {
            Label::new(cx, "Basic Switches");

            HStack::new(cx, |cx| {
                Switch::new(cx, option1)
                    .on_toggle(move |cx| option1.update(cx, |v| *v = !*v))
                    .id("Switch_1");
                Label::new(cx, "Switch 1").describing("Switch_1");
            })
            .size(Auto)
            .horizontal_gap(Pixels(5.0))
            .alignment(Alignment::Center);

            HStack::new(cx, |cx| {
                Switch::new(cx, option2)
                    .on_toggle(move |cx| option2.update(cx, |v| *v = !*v))
                    .id("Switch_2");
                Label::new(cx, "Switch 2").describing("Switch_2");
            })
            .size(Auto)
            .horizontal_gap(Pixels(5.0))
            .alignment(Alignment::Center);
        });
    })
    .title("Switch")
    .run()
}
