use vizia::prelude::*;

use crate::DemoRegion;

pub struct VirtualListData {
    list: Signal<Vec<u32>>,
    selected: Signal<Vec<usize>>,
    selection_follows_focus: Signal<bool>,
}

pub enum VirtualListEvent {
    SetSelected(usize),
}

impl Model for VirtualListData {
    fn event(&mut self, _cx: &mut EventContext, event: &mut Event) {
        event.map(|virtual_list_event, _| match virtual_list_event {
            VirtualListEvent::SetSelected(index) => self.selected.set(vec![*index]),
        });

        let _ = self.list;
        let _ = self.selection_follows_focus;
    }
}

pub fn virtual_list(cx: &mut Context) {
    let list = Signal::new((1..100u32).collect::<Vec<_>>());
    let selected = Signal::new(vec![0usize]);
    let selection_follows_focus = Signal::new(true);
    VirtualListData { list, selected, selection_follows_focus }.build(cx);

    VStack::new(cx, |cx| {
        Markdown::new(cx, "# Virtual List");

        Divider::new(cx);

        Markdown::new(cx, "### Basic virtual list");

        DemoRegion::new(
            cx,
            move |cx| {
                VirtualList::new(cx, list, 40.0, |cx, index, item| {
                    Label::new(cx, item).toggle_class("dark", index % 2 == 0)
                })
                .selected(selected)
                .on_select(|cx, index| cx.emit(VirtualListEvent::SetSelected(index)))
                .selection_follows_focus(selection_follows_focus)
                .size(Pixels(300.0));
            },
            r#"VirtualList::new(cx, list, 40.0, |cx, index, item| {
        Label::new(cx, item).toggle_class("dark", index % 2 == 0)
    })
    .selected(selected)
    .on_select(|cx, index| cx.emit(VirtualListEvent::SetSelected(index)))
    .selection_follows_focus(selection_follows_focus)
    .size(Pixels(300.0));"#,
        );
    })
    .class("panel");
}
