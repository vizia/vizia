use std::ops::Deref;

use crate::context::TreeProps;
use crate::icons::{ICON_CHECK, ICON_CHEVRON_DOWN};
use crate::prelude::*;

#[derive(Lens)]
pub struct PickList {
    on_select: Option<Box<dyn Fn(&mut EventContext, usize)>>,
    placeholder: String,
    is_open: bool,
}

pub enum PickListEvent {
    SetOption(usize),
}

impl PickList {
    pub fn new<L1, L2, T>(
        cx: &mut Context,
        list: L1,
        selected: L2,
        show_handle: bool,
    ) -> Handle<Self>
    where
        L1: Lens,
        L1::Target: Deref<Target = [T]> + Data,
        T: 'static + Data + ToStringLocalized,
        L2: Lens<Target = usize>,
    {
        Self { on_select: None, placeholder: String::new(), is_open: false }
            .build(cx, |cx| {
                Button::new(cx, |cx| {
                    // A Label and an optional Icon
                    HStack::new(cx, move |cx| {
                        Label::new(cx, PickList::placeholder)
                            .bind(list, move |handle, list| {
                                handle.bind(selected, move |handle, sel| {
                                    let selected_index = sel.get(&handle);
                                    let list_len = list.map(|list| list.len()).get(&handle);
                                    if selected_index < list_len {
                                        handle.text(list.idx(selected_index));
                                    } else {
                                        handle.text(PickList::placeholder);
                                    }
                                });
                            })
                            .width(Stretch(2.0))
                            .text_wrap(false)
                            .text_overflow(TextOverflow::Ellipsis)
                            .hoverable(false);
                        if show_handle {
                            Svg::new(cx, ICON_CHEVRON_DOWN)
                                .class("icon")
                                .size(Pixels(16.0))
                                .hoverable(false);
                        }
                    })
                    .width(Stretch(1.0))
                    //.gap(Stretch(1.0))
                    .gap(Pixels(8.0))
                })
                .width(Stretch(1.0))
                .on_press(|cx| cx.emit(PopupEvent::Open));

                Binding::new(cx, PickList::is_open, move |cx, is_open| {
                    if is_open.get(cx) {
                        Popup::new(cx, |cx| {
                            List::new(cx, list, move |cx, _, item| {
                                Element::new(cx).class("focus-indicator");
                                Svg::new(cx, ICON_CHECK).class("checkmark").size(Pixels(16.0));
                                Label::new(cx, item).hoverable(false);
                            })
                            .selectable(Selectable::Single)
                            .selected(selected.map(|s| vec![*s]))
                            .on_select(|cx, index| {
                                cx.emit(PickListEvent::SetOption(index));
                                cx.emit(PopupEvent::Close);
                            })
                            .focused(true);
                        })
                        .arrow_size(Pixels(4.0))
                        .on_blur(|cx| cx.emit(PopupEvent::Close));
                    }
                });
            })
            .navigable(false)
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

        event.map(|popup_event, meta| match popup_event {
            PopupEvent::Open => {
                self.is_open = true;

                meta.consume();
            }

            PopupEvent::Close => {
                self.is_open = false;
                let e = cx.first_child();
                cx.with_current(e, |cx| cx.focus());
                meta.consume();
            }

            PopupEvent::Switch => {
                self.is_open ^= true;
                meta.consume();
            }
        });
    }
}

impl Handle<'_, PickList> {
    /// Sets the placeholder text that appears when the textbox has no value.
    pub fn placeholder<P: ToStringLocalized>(self, placeholder: impl Res<P>) -> Self {
        self.bind(placeholder, |handle, val| {
            let txt = val.get(&handle).to_string_local(&handle);
            handle.modify(|picklist| picklist.placeholder = txt);
        })
    }

    // Sets the callback triggered when an option is selected.
    pub fn on_select<F>(self, callback: F) -> Self
    where
        F: 'static + Fn(&mut EventContext, usize),
    {
        self.modify(|picklist: &mut PickList| picklist.on_select = Some(Box::new(callback)))
    }
}
