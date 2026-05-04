use vizia::prelude::*;

use crate::DemoRegion;

pub struct ListData {
    list: Signal<Vec<Signal<u32>>>,
    selectable: Signal<Selectable>,
    show_vertical_scrollbar: Signal<bool>,
}

impl Model for ListData {
    fn event(&mut self, _cx: &mut EventContext, _event: &mut Event) {
        let _ = self.list;
        let _ = self.selectable;
        let _ = self.show_vertical_scrollbar;
    }
}

pub fn list(cx: &mut Context) {
    let list = Signal::new((1..14u32).map(Signal::new).collect::<Vec<_>>());
    let selectable = Signal::new(Selectable::Single);
    let show_vertical_scrollbar = Signal::new(true);
    ListData { list, selectable, show_vertical_scrollbar }.build(cx);

    VStack::new(cx, |cx| {
        Label::new(cx, Localized::new("list")).class("panel-title");

        Divider::new(cx);

        DemoRegion::new(cx, "Basic List", move |cx| {
            List::new(cx, list, |cx, index, item| {
                Label::new(cx, item)
                    .toggle_class("dark", index % 2 == 0)
                    .width(Stretch(1.0))
                    .height(Pixels(30.0))
                    .hoverable(false);
            })
            .selectable(selectable)
            .show_vertical_scrollbar(show_vertical_scrollbar)
            .size(Pixels(300.0));
        });
    })
    .class("panel");
}
