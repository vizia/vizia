use crate::icons::ICON_CHEVRON_DOWN;
use crate::prelude::*;

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
        T: 'static + Data + ToStringLocalized,
        L2: Lens<Target = usize>,
    {
        Self { on_select: None }.build(cx, |cx| {
            // Dropdown List
            Dropdown::new(
                cx,
                move |cx| {
                    // A Label and an Icon
                    HStack::new(cx, move |cx| {
                        Label::new(cx, "")
                            .bind(list_lens, move |handle, list| {
                                handle.bind(selected, move |handle, sel| {
                                    let selected_index = sel.get(handle.cx);

                                    handle.text(list.index(selected_index));
                                });
                            })
                            .hoverable(false);
                        if show_handle {
                            Label::new(cx, ICON_CHEVRON_DOWN).class("icon").hoverable(false);
                        }
                    })
                    .col_between(Stretch(1.0))
                },
                move |cx| {
                    let window_height = cx.cache.get_height(Entity::root());
                    let scale = cx.scale_factor();
                    ScrollView::new(cx, 0.0, 0.0, false, true, move |cx| {
                        List::new(cx, list_lens, move |cx, index, item| {
                            Label::new(cx, item)
                                .child_top(Stretch(1.0))
                                .child_bottom(Stretch(1.0))
                                .checked(selected.map(move |selected| *selected == index))
                                .navigable(true)
                                .on_press(move |cx| {
                                    cx.emit(PickListEvent::SetOption(index));
                                    cx.emit(PopupEvent::Close);
                                });
                        });
                    })
                    .height(Auto)
                    .max_height(Pixels(window_height / scale));
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
