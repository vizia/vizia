use vizia::prelude::*;

use crate::DemoRegion;

pub fn accordion(cx: &mut Context) {
    let items = Signal::new(vec![
        ("accordion-item-what-is-vizia-title", "accordion-item-what-is-vizia-body"),
        ("accordion-item-style-views-title", "accordion-item-style-views-body"),
        ("accordion-item-reactive-title", "accordion-item-reactive-body"),
    ]);
    let open_indices = Signal::new(vec![0usize]);

    VStack::new(cx, |cx| {
        Label::new(cx, Localized::new("accordion")).class("panel-title");
        Label::new(cx, Localized::new("accordion").attribute("description"))
            .class("panel-description");
        Divider::new(cx);

        DemoRegion::new(cx, Localized::new("accordion"), move |cx| {
            Accordion::new(cx, items, |_cx, _index, item| {
                let header = item.0;
                let content = item.1;
                AccordionPair::new(
                    move |cx| {
                        Label::new(cx, Localized::new(header)).hoverable(false);
                    },
                    move |cx| {
                        Label::new(cx, Localized::new(content))
                            .hoverable(false)
                            .width(Stretch(1.0));
                    },
                )
            })
            .open(open_indices)
            .on_toggle(move |_cx, index, is_open| {
                open_indices.update(|indices| {
                    if is_open {
                        indices.clear();
                        indices.push(index);
                    } else {
                        indices.retain(|&open_index| open_index != index);
                    }
                });
            })
            .width(Stretch(1.0));
        });
    })
    .class("panel");
}
