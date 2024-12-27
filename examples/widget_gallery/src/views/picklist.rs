use vizia::prelude::*;

use crate::DemoRegion;

#[derive(Lens)]
struct PicklistData {
    options: Vec<&'static str>,
    selected_option: usize,
}

pub enum PicklistEvent {
    SetOption(usize),
}

impl Model for PicklistData {
    fn event(&mut self, _: &mut EventContext, event: &mut Event) {
        event.map(|picklist_event, _| match picklist_event {
            PicklistEvent::SetOption(index) => {
                self.selected_option = *index;
            }
        });
    }
}

pub fn picklist(cx: &mut Context) {
    PicklistData {
        options: vec![
            "One", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "Nine", "Ten",
            "Eleven", "Twelve",
        ],
        selected_option: 0,
    }
    .build(cx);

    VStack::new(cx, |cx| {
        Markdown::new(cx, "# Picklist");

        Divider::new(cx);

        Markdown::new(cx, "### Basic picklist");

        DemoRegion::new(
            cx,
            |cx| {
                PickList::new(cx, PicklistData::options, PicklistData::selected_option, true)
                    .on_select(|cx, index| cx.emit(PicklistEvent::SetOption(index)))
                    .width(Pixels(140.0));
            },
            r#"PickList::new(cx, PicklistData::options, PicklistData::selected_option, true)
    .on_select(|cx, index| cx.emit(PicklistEvent::SetOption(index)))
    .width(Pixels(140.0));"#,
        );
    })
    .class("panel");
}
