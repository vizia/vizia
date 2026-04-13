use std::ops::Deref;

use crate::context::TreeProps;
use crate::icons::{ICON_CHECK, ICON_CHEVRON_DOWN};
use crate::prelude::*;

/// A view which allows the user to select an item from a dropdown list.
pub struct Select {
    on_select: Option<Box<dyn Fn(&mut EventContext, usize)>>,
    placeholder: Signal<String>,
    is_open: Signal<bool>,
    min_selected: Signal<usize>,
    max_selected: Signal<usize>,
}

pub(crate) enum SelectEvent {
    SetOption(usize),
}

impl Select {
    /// Creates a new [Select] view.
    pub fn new<L, S, V, T>(
        cx: &mut Context,
        list: L,
        selected: S,
        show_handle: bool,
    ) -> Handle<Self>
    where
        L: SignalGet<V> + SignalMap<V> + Res<V> + 'static,
        V: Deref<Target = [T]> + Clone + 'static,
        T: 'static + ToStringLocalized + Clone + PartialEq,
        S: Res<Option<usize>> + 'static,
    {
        let is_open = Signal::new(false);
        let placeholder = Signal::new(String::new());
        let min_selected = Signal::new(0);
        let max_selected = Signal::new(usize::MAX);
        let selected = selected.to_signal(cx);
        Self { on_select: None, placeholder, is_open, min_selected, max_selected }
            .build(cx, |cx| {
                Button::new(cx, |cx| {
                    // A Label and an optional Icon
                    HStack::new(cx, move |cx| {
                        Label::new(cx, placeholder)
                            .bind(list, move |handle| {
                                //let list = list.get();
                                handle.bind(selected, move |handle| {
                                    let selected_index = selected.get();
                                    let list_len = list.with(|list| list.len());
                                    if let Some(index) = selected_index {
                                        if index < list_len {
                                            let item = Memo::new(move |_| {
                                                list.with(move |list| list.get(index).cloned())
                                            });

                                            if let Some(_) = item.get() {
                                                handle
                                                    .text(item.map(move |it| it.clone().unwrap()));
                                            } else {
                                                handle.text(placeholder.get());
                                            };
                                        } else {
                                            handle.text(placeholder.get());
                                        }
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
                .variant(ButtonVariant::Outline)
                .width(Stretch(1.0))
                .on_press(|cx| cx.emit(PopupEvent::Open));

                Binding::new(cx, is_open, move |cx| {
                    let is_open = is_open.get();
                    if is_open {
                        Popover::new(cx, |cx| {
                            List::new(cx, list, move |cx, _, item| {
                                Svg::new(cx, ICON_CHECK).class("checkmark").size(Pixels(16.0));
                                Label::new(cx, item.map(|v| v.clone())).hoverable(false);
                            })
                            .selectable(Selectable::Single)
                            .min_selected(min_selected)
                            .max_selected(max_selected)
                            .selected(
                                selected.map(|s| {
                                    if let Some(index) = s { vec![*index] } else { vec![] }
                                }),
                            )
                            .on_select(|cx, index| {
                                cx.emit(SelectEvent::SetOption(index));
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

impl View for Select {
    fn element(&self) -> Option<&'static str> {
        Some("select")
    }

    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|select_event, _| match select_event {
            SelectEvent::SetOption(index) => {
                if let Some(callback) = &self.on_select {
                    (callback)(cx, *index);
                }
            }
        });

        event.map(|popup_event, meta| match popup_event {
            PopupEvent::Open => {
                self.is_open.set_if_changed(true);

                meta.consume();
            }

            PopupEvent::Close => {
                self.is_open.set_if_changed(false);
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

impl Handle<'_, Select> {
    /// Sets the placeholder text that appears when the textbox has no value.
    pub fn placeholder<P: ToStringLocalized + Clone + 'static>(
        self,
        placeholder: impl Res<P> + 'static,
    ) -> Self {
        let placeholder = placeholder.to_signal(self.cx);
        self.bind(placeholder, move |handle| {
            let val = placeholder.get();
            let txt = val.to_string_local(&handle);
            handle.modify(|select| select.placeholder.set(txt));
        })
    }

    /// Sets the callback triggered when an option is selected.
    pub fn on_select<F>(self, callback: F) -> Self
    where
        F: 'static + Fn(&mut EventContext, usize),
    {
        self.modify(|select: &mut Select| select.on_select = Some(Box::new(callback)))
    }

    /// Sets the minimum number of selected items.
    pub fn min_selected(self, min_selected: impl Res<usize> + 'static) -> Self {
        let min_selected = min_selected.to_signal(self.cx);
        self.bind(min_selected, move |handle| {
            let min_selected = min_selected.get();
            handle.modify(|select: &mut Select| select.min_selected.set(min_selected));
        })
    }

    /// Sets the maximum number of selected items.
    pub fn max_selected(self, max_selected: impl Res<usize> + 'static) -> Self {
        let max_selected = max_selected.to_signal(self.cx);
        self.bind(max_selected, move |handle| {
            let max_selected = max_selected.get();
            handle.modify(|select: &mut Select| select.max_selected.set(max_selected));
        })
    }
}
