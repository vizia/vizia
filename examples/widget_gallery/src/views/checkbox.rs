use vizia::prelude::*;

use crate::components::DemoRegion;

pub fn checkbox(cx: &mut Context) {
    let check_a = cx.state(true);
    let auto = cx.state(Auto);
    let align_center = cx.state(Alignment::Center);
    let gap_8 = cx.state(Pixels(8.0));

    VStack::new(cx, |cx| {
        Markdown::new(cx, "# Checkbox
A checkbox can be used to display a boolean value, or to select one or more items from a set of options.        
        ");

        Divider::new(cx);

        Markdown::new(cx, "### Basic checkboxes");

        DemoRegion::new(cx, |cx|{
            Checkbox::new(cx, check_a).two_way();
        }, r#"let check_a = cx.state(true);
Checkbox::new(cx, check_a).two_way();"#);

        Markdown::new(cx, "### Labelled checkbox
A `HStack` can be used to add a label to a checkbox. The describing modifier can be used to link a label to a particular checkbox. Pressing on the label will then toggle the corresponding checkbox.        
        ");

        DemoRegion::new(cx, |cx|{
            HStack::new(cx, |cx| {
                Checkbox::new(cx, check_a).two_way().id("check");
                Label::static_text(cx, "Label").describing("check");
            })
            .size(auto)
            .alignment(align_center)
            .horizontal_gap(gap_8);
        }, r#"let check_a = cx.state(true);
let auto = cx.state(Auto);
let align_center = cx.state(Alignment::Center);
let gap_8 = cx.state(Pixels(8.0));
HStack::new(cx, |cx| {
    Checkbox::new(cx, check_a).two_way().id("check");
    Label::static_text(cx, "Label").describing("check");
})
.size(auto)
.horizontal_gap(gap_8);"#);
    })
    .class("panel");
}
