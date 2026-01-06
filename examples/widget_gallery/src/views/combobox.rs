use vizia::prelude::*;

use crate::components::DemoRegion;

pub fn combobox(cx: &mut Context) {
    VStack::new(cx, move |cx| {
        let options = cx.state(vec![
            "One", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "Nine", "Ten",
        ]);
        let selected_option = cx.state(0usize);
        let width_100 = cx.state(Pixels(100.0));

        Markdown::new(cx, "# Combobox");

        Divider::new(cx);

        Markdown::new(cx, "### Basic combobox");

        DemoRegion::new(
            cx,
            move |cx| {
                ComboBox::new(cx, options, selected_option).width(width_100);
            },
            r#"let options = cx.state(vec![
    "One", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "Nine", "Ten",
]);
let selected_option = cx.state(0usize);
let width_100 = cx.state(Pixels(100.0));
ComboBox::new(cx, options, selected_option)
    .width(width_100);"#,
        );
    })
    .class("panel");
}
