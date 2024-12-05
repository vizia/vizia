use vizia::prelude::*;

use crate::components::DemoRegion;

#[derive(Lens)]
pub struct CheckboxData {
    check_a: bool,
}

pub enum CheckboxEvent {
    ToggleA,
}

impl Model for CheckboxData {
    fn event(&mut self, _: &mut EventContext, event: &mut Event) {
        event.map(|checkbox_event, _| match checkbox_event {
            CheckboxEvent::ToggleA => {
                self.check_a ^= true;
            }
        });
    }
}

pub fn checkbox(cx: &mut Context) {
    CheckboxData { check_a: true }.build(cx);

    VStack::new(cx, |cx| {
        Markdown::new(cx, "# Checkbox
A checkbox can be used to display a boolean value, or to select one or more items from a set of options.        
        ");

        Divider::new(cx).top(Pixels(12.0)).bottom(Pixels(12.0));

        Markdown::new(cx, "### Basic checkboxes");

        DemoRegion::new(cx, |cx|{
            Checkbox::new(cx, CheckboxData::check_a)
                .on_toggle(|cx| cx.emit(CheckboxEvent::ToggleA));
        }, r#"Checkbox::new(cx, CheckboxData::check_a)
    .on_toggle(|cx| cx.emit(CheckboxEvent::ToggleA));"#);

        Markdown::new(cx, "### Labelled checkbox
A `HStack` can be used to add a label to a checkbox. The describing modifier can be used to link a label to a particular checkbox. Pressing on the label will then toggle the corresponding checkbox.        
        ");

        DemoRegion::new(cx, |cx|{
            HStack::new(cx, |cx| {
                Checkbox::new(cx, CheckboxData::check_a)
                    .on_toggle(|cx| cx.emit(CheckboxEvent::ToggleA))
                    .id("check");
                Label::new(cx, "Label").describing("check");
            })
            .size(Auto)
            .alignment(Alignment::Center)
            .horizontal_gap(Pixels(8.0));
        }, r#"HStack::new(cx, |cx| {
    Checkbox::new(cx, CheckboxData::check_a)
        .on_toggle(|cx| cx.emit(CheckboxEvent::ToggleA))
        .id("check");
    Label::new(cx, "Label").describing("check");
})
.size(Auto)
.horizontal_gap(Pixels(8.0));"#);
    })
    .class("panel");
}
