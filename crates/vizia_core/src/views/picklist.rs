use std::ops::Deref;

use crate::context::TreeProps;
use crate::icons::{ICON_CHECK, ICON_CHEVRON_DOWN};
use crate::prelude::*;

/// A view which allows the user to select an item from a dropdown list.
pub struct PickList {
    on_select: Option<Box<dyn Fn(&mut EventContext, usize)>>,
    placeholder: Signal<String>,
    is_open: Signal<bool>,
}

pub(crate) enum PickListEvent {
    SetOption(usize),
}

impl PickList {
    /// Creates a new [PickList] view.
    pub fn new<L, S, V, T>(
        cx: &mut Context,
        list: L,
        selected: S,
        show_handle: bool,
    ) -> Handle<Self>
    where
        L: SignalGet<V> + SignalMapExt<V> + Res<V> + 'static,
        V: Deref<Target = [Signal<T>]> + Clone + 'static,
        T: 'static + ToStringLocalized + Clone,
        S: Res<usize> + 'static,
    {
        let is_open = Signal::new(false);
        let placeholder = Signal::new(String::new());
        let selected = selected.to_signal(cx);
        Self { on_select: None, placeholder, is_open }
            .build(cx, |cx| {
                Button::new(cx, |cx| {
                    // A Label and an optional Icon
                    HStack::new(cx, move |cx| {
                        Label::new(cx, placeholder)
                            .bind(list, move |handle| {
                                let list = list.get();
                                handle.bind(selected, move |handle| {
                                    let selected_index = selected.get();
                                    let list_len = list.len();
                                    if selected_index < list_len {
                                        let selected_text =
                                            if let Some(item) = list.get(selected_index) {
                                                item.get().to_string_local(&handle)
                                            } else {
                                                placeholder.get()
                                            };
                                        handle.text(selected_text);
                                    } else {
                                        handle.text(placeholder.get());
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

                Binding::new(cx, is_open, move |cx| {
                    let is_open = is_open.get();
                    if is_open {
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
                self.is_open.set(true);

                meta.consume();
            }

            PopupEvent::Close => {
                self.is_open.set(false);
                let e = cx.first_child();
                cx.with_current(e, |cx| cx.focus());
                meta.consume();
            }

            PopupEvent::Switch => {
                self.is_open.set(!self.is_open.get());
                meta.consume();
            }
        });
    }
}

impl Handle<'_, PickList> {
    /// Sets the placeholder text that appears when the textbox has no value.
    pub fn placeholder<P: ToStringLocalized + Clone + 'static>(
        self,
        placeholder: impl Res<P> + 'static,
    ) -> Self {
        let placeholder = placeholder.to_signal(self.cx);
        self.bind(placeholder, move |handle| {
            let val = placeholder.get();
            let txt = val.to_string_local(&handle);
            handle.modify(|picklist| picklist.placeholder.set(txt));
        })
    }

    /// Sets the callback triggered when an option is selected.
    pub fn on_select<F>(self, callback: F) -> Self
    where
        F: 'static + Fn(&mut EventContext, usize),
    {
        self.modify(|picklist: &mut PickList| picklist.on_select = Some(Box::new(callback)))
    }
}
