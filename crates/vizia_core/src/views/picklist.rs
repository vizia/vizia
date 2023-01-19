use crate::prelude::*;

const CHEVRON_DOWN: &str = "\u{e75c}";

pub struct PickList {
    on_select: Option<Box<dyn Fn(&mut EventContext, usize)>>,
}

pub enum PickListEvent {
    SetOption(usize),
}

impl PickList {
    pub fn new<L1, L2, T>(
        cx: &mut Context,
        list_lens: L1,
        selected: L2,
        show_handle: bool,
    ) -> Handle<Self>
    where
        L1: Lens<Target = Vec<T>>,
        T: 'static + Data + ToString,
        L2: Lens<Target = usize>,
    {
        Self { on_select: None }.build(cx, |cx| {
            let s = selected.clone();
            let l = list_lens.clone();
            // Dropdown List
            Dropdown::new(
                cx,
                move |cx| {
                    let l = l.clone();
                    let s = s.clone();
                    // A Label and an Icon
                    HStack::new(cx, move |cx| {
                        Label::new(cx, "")
                            .bind(l.clone(), move |handle, list| {
                                handle.bind(s.clone(), move |handle, sel| {
                                    let selected_index = sel.get(handle.cx);

                                    handle.text(list.clone().index(selected_index));
                                });
                            })
                            .hoverable(false);
                        if show_handle {
                            Label::new(cx, CHEVRON_DOWN).class("icon").hoverable(false);
                        }
                    })
                    .col_between(Stretch(1.0))
                },
                move |cx| {
                    let s = selected.clone();
                    let l = list_lens.clone();
                    List::new(cx, l.clone(), move |cx, index, item| {
                        Label::new(cx, item)
                            .child_top(Stretch(1.0))
                            .child_bottom(Stretch(1.0))
                            .checked(s.clone().map(move |selected| *selected == index))
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
