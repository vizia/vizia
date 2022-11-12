use crate::fonts::icons_names::DOWN;
use crate::prelude::*;

pub struct PickList {
    on_select: Option<Box<dyn Fn(&mut EventContext, usize)>>,
}

pub enum PickListEvent {
    SetOption(usize),
}

impl PickList {
    pub fn new<L1, L2, T>(cx: &mut Context, list_lens: L1, selected: L2) -> Handle<Self>
    where
        L1: Lens<Target = Vec<T>>,
        T: 'static + Data + ToString,
        L2: Lens<Target = usize>,
    {
        Self { on_select: None }.build(cx, |cx| {
            // Dropdown List
            Dropdown::new(
                cx,
                move |cx| {
                    // A Label and an Icon
                    HStack::new(cx, move |cx| {
                        Label::new(cx, "").bind(list_lens, move |handle, list| {
                            handle.bind(selected, move |handle, selected| {
                                let selected_index = selected.get(handle.cx);

                                handle.text(list.clone().index(selected_index));
                            });
                        });
                        Label::new(cx, DOWN).font("icons");
                    })
                    .child_left(Pixels(5.0))
                    .child_right(Pixels(5.0))
                    .col_between(Stretch(1.0))
                },
                move |cx| {
                    List::new(cx, list_lens, move |cx, index, item| {
                        Label::new(cx, item)
                            .child_top(Stretch(1.0))
                            .child_bottom(Stretch(1.0))
                            .checked(selected.map(move |selected| *selected == index))
                            .on_press(move |cx| {
                                cx.emit(PickListEvent::SetOption(index));
                                cx.emit(PopupEvent::Close);
                            });
                    });
                },
            );
        })
    }
}

impl View for PickList {
    fn element(&self) -> Option<&'static str> {
        Some("picklist")
    }

    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|picklist_event, _| match picklist_event {
            PickListEvent::SetOption(index) => {
                if let Some(callback) = &self.on_select {
                    (callback)(cx, *index);
                }
            }
        });
    }
}

impl<'v> Handle<'v, PickList> {
    pub fn on_select<F>(self, callback: F) -> Self
    where
        F: 'static + Fn(&mut EventContext, usize),
    {
        self.modify(|picklist: &mut PickList| picklist.on_select = Some(Box::new(callback)))
    }
}
