use crate::context::TreeProps;
use crate::icons::{ICON_CHECK, ICON_CHEVRON_DOWN};
use crate::prelude::*;
use std::rc::Rc;

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
    pub fn new<T>(
        cx: &mut Context,
        list: Signal<Vec<T>>,
        selected: Signal<usize>,
        show_handle: bool,
    ) -> Handle<Self>
    where
        T: 'static + ToStringLocalized,
    {
        let placeholder = cx.state(String::new());
        let is_open = cx.state(false);
        let display_text = cx.state(String::new());
        let list_len = cx.state(0usize);
        let focused = cx.state(None::<usize>);
        let false_signal = cx.state(false);
        let true_signal = cx.state(true);
        let stretch_one = cx.state(Stretch(1.0));
        let stretch_two = cx.state(Stretch(2.0));
        let gap_small = cx.state(Pixels(8.0));
        let icon_size = cx.state(Pixels(16.0));
        let text_overflow = cx.state(TextOverflow::Ellipsis);
        let chevron_icon = cx.state(ICON_CHEVRON_DOWN);
        let check_icon = cx.state(ICON_CHECK);

        let update_display = Rc::new({
            let list = list;
            let selected = selected;
            let placeholder = placeholder;
            let display_text = display_text;
            let list_len = list_len;
            let focused = focused;
            move |cx: &mut Context| {
                let (text, len) = {
                    let items = list.get(cx);
                    let selected_index = *selected.get(cx);
                    let text = if selected_index < items.len() {
                        items[selected_index].to_string_local(cx)
                    } else {
                        placeholder.get(cx).clone()
                    };
                    (text, items.len())
                };
                let mut event_cx = EventContext::new(cx);
                display_text.set(&mut event_cx, text);
                list_len.set(&mut event_cx, len);
                focused.update(&mut event_cx, |focused| {
                    if let Some(index) = *focused {
                        if index >= len {
                            *focused = if len > 0 { Some(len - 1) } else { None };
                        }
                    }
                });
            }
        });

        update_display(cx);

        let update_display_list = update_display.clone();
        Binding::new(cx, list, move |cx| {
            update_display_list(cx);
        });

        let update_display_selected = update_display.clone();
        Binding::new(cx, selected, move |cx| {
            update_display_selected(cx);
        });

        let update_display_placeholder = update_display.clone();
        Binding::new(cx, placeholder, move |cx| {
            update_display_placeholder(cx);
        });

        Self { on_select: None, placeholder, is_open }
            .build(cx, move |cx| {
                Button::new(cx, |cx| {
                    // A Label and an optional Icon
                    HStack::new(cx, move |cx| {
                        Label::new(cx, display_text)
                            .width(stretch_two)
                            .text_wrap(false_signal)
                            .text_overflow(text_overflow)
                            .hoverable(false_signal);
                        if show_handle {
                            Svg::new(cx, chevron_icon)
                                .class("icon")
                                .size(icon_size)
                                .hoverable(false_signal);
                        }
                    })
                    .width(stretch_one)
                    //.gap(Stretch(1.0))
                    .gap(gap_small)
                })
                .width(stretch_one)
                .on_press(|cx| cx.emit(PopupEvent::Open));

                let arrow_size = cx.state(Length::Value(LengthValue::Px(4.0)));
                Binding::new(cx, is_open, move |cx| {
                    if *is_open.get(cx) {
                        let list = list;
                        let selected = selected;
                        let focused = focused;
                        let list_len = list_len;
                        let len = list.get(cx).len();
                        let selected_index = *selected.get(cx);
                        let initial_focus = if len == 0 {
                            None
                        } else if selected_index < len {
                            Some(selected_index)
                        } else {
                            Some(0)
                        };
                        let mut event_cx = EventContext::new(cx);
                        list_len.set(&mut event_cx, len);
                        focused.set(&mut event_cx, initial_focus);

                        Popup::new(cx, move |cx| {
                            PickListList::new(cx, focused, list_len, move |cx| {
                                ScrollView::new(cx, move |cx| {
                                    Binding::new(cx, list, move |cx| {
                                        let item_count = list.get(cx).len();
                                        for index in 0..item_count {
                                            let label_text = {
                                                let items = list.get(cx);
                                                items[index].to_string_local(cx)
                                            };
                                            PickListItem::new(
                                                cx,
                                                index,
                                                selected,
                                                focused,
                                                move |cx| {
                                                    Element::new(cx).class("focus-indicator");
                                                    Svg::new(cx, check_icon)
                                                        .class("checkmark")
                                                        .size(icon_size);
                                                    let label = cx.state(label_text);
                                                    Label::new(cx, label).hoverable(false_signal);
                                                },
                                            );
                                        }
                                    });
                                });
                            })
                            .class("selectable")
                            .focused(true_signal);
                        })
                        .arrow_size(arrow_size)
                        .on_blur(|cx| cx.emit(PopupEvent::Close));
                    }
                });
            })
            .navigable(false_signal)
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
                self.is_open.set(cx, true);

                meta.consume();
            }

            PopupEvent::Close => {
                self.is_open.set(cx, false);
                let e = cx.first_child();
                cx.with_current(e, |cx| cx.focus());
                meta.consume();
            }

            PopupEvent::Switch => {
                let is_open = *self.is_open.get(cx);
                self.is_open.set(cx, !is_open);
                meta.consume();
            }
        });
    }
}

impl Handle<'_, PickList> {
    /// Sets the placeholder text that appears when the textbox has no value.
    pub fn placeholder<P: ToStringLocalized + Clone>(self, placeholder: Signal<P>) -> Self {
        self.bind(placeholder, |handle, val| {
            let txt = val.get(&handle).to_string_local(&handle);
            handle.modify2(|picklist, cx| picklist.placeholder.set(cx, txt));
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

struct PickListList {
    focused: Signal<Option<usize>>,
    list_len: Signal<usize>,
}

impl PickListList {
    fn new(
        cx: &mut Context,
        focused: Signal<Option<usize>>,
        list_len: Signal<usize>,
        content: impl FnOnce(&mut Context),
    ) -> Handle<Self> {
        Self { focused, list_len }.build(cx, content)
    }
}

impl View for PickListList {
    fn element(&self) -> Option<&'static str> {
        Some("list")
    }

    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|window_event, meta| match window_event {
            WindowEvent::KeyDown(code, _) => match code {
                Code::ArrowDown => {
                    let len = *self.list_len.get(cx);
                    if len == 0 {
                        return;
                    }
                    self.focused.update(cx, |focused| {
                        let next = match *focused {
                            Some(index) => (index + 1).min(len.saturating_sub(1)),
                            None => 0,
                        };
                        *focused = Some(next);
                    });
                    meta.consume();
                }

                Code::ArrowUp => {
                    let len = *self.list_len.get(cx);
                    if len == 0 {
                        return;
                    }
                    self.focused.update(cx, |focused| {
                        let next = match *focused {
                            Some(index) => index.saturating_sub(1),
                            None => len.saturating_sub(1),
                        };
                        *focused = Some(next);
                    });
                    meta.consume();
                }

                Code::Enter | Code::Space => {
                    if let Some(index) = *self.focused.get(cx) {
                        cx.emit(PickListEvent::SetOption(index));
                        cx.emit(PopupEvent::Close);
                    }
                    meta.consume();
                }

                Code::Escape => {
                    cx.emit(PopupEvent::Close);
                    meta.consume();
                }

                _ => {}
            },

            _ => {}
        });
    }
}

struct PickListItem {}

impl PickListItem {
    fn new(
        cx: &mut Context,
        index: usize,
        selected: Signal<usize>,
        focused: Signal<Option<usize>>,
        content: impl FnOnce(&mut Context),
    ) -> Handle<Self> {
        let is_selected = cx.derived({
            let selected = selected;
            move |store| *selected.get(store) == index
        });
        let is_focused = cx.derived({
            let focused = focused;
            move |store| focused.get(store).as_ref().is_some_and(|focused| *focused == index)
        });
        let navigable = cx.state(true);
        Self {}
            .build(cx, content)
            .role(Role::ListItem)
            .navigable(navigable)
            .checked(is_selected)
            .toggle_class("focused", is_focused)
            .on_press(move |cx| {
                cx.emit(PickListEvent::SetOption(index));
                cx.emit(PopupEvent::Close);
            })
    }
}

impl View for PickListItem {
    fn element(&self) -> Option<&'static str> {
        Some("list-item")
    }
}
