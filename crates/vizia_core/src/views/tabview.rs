use std::ops::Deref;
use std::sync::Arc;

use crate::icons::ICON_X;
use crate::prelude::*;

pub enum TabListEvent {
    SetSelected(usize),
    CloseFocused,
}

pub struct TabView {
    selected_index: Signal<usize>,
    is_vertical: Signal<bool>,
    on_select: Option<Box<dyn Fn(&mut EventContext, usize)>>,
}

impl TabView {
    pub fn new<S, V, T, F>(cx: &mut Context, list: S, content: F) -> Handle<Self>
    where
        S: Res<V> + 'static,
        V: Deref<Target = [T]> + Clone + 'static,
        T: PartialEq + Clone + 'static,
        F: 'static + Clone + Fn(&mut Context, usize, T) -> TabPair,
    {
        let selected_index = Signal::new(0usize);
        let is_vertical = Signal::new(false);
        let list = list.to_signal(cx);

        Self { selected_index, is_vertical, on_select: None }
            .build(cx, move |cx| {
                let tabview_entity = cx.current();
                let content_for_headers = content.clone();

                TabList::new(cx, list, move |cx, index, item, is_selected| {
                    let builder = (content_for_headers)(cx, index, item).header;

                    Tab::with_content(cx, index, builder)
                        .checked(is_selected)
                        .selected(is_selected)
                        .toggle_class("vertical", is_vertical)
                })
                .vertical(is_vertical)
                .on_select(move |cx, index| {
                    cx.emit_to(tabview_entity, TabListEvent::SetSelected(index))
                })
                .toggle_class("vertical", is_vertical);

                Divider::new(cx).toggle_class("vertical", is_vertical);

                VStack::new(cx, move |cx| {
                    Binding::new(cx, list, move |cx| {
                        let list_values = list.get();
                        let content = content.clone();
                        Binding::new(cx, selected_index, move |cx| {
                            let selected = selected_index.get();
                            if let Some(item) = list_values.get(selected).cloned() {
                                ((content)(cx, selected, item).content)(cx);
                            }
                        });
                    });
                })
                .overflow(Overflow::Hidden)
                .class("tabview-content-wrapper");
            })
            .toggle_class("vertical", is_vertical)
    }
}

impl View for TabView {
    fn element(&self) -> Option<&'static str> {
        Some("tabview")
    }

    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|tab_event, meta| match tab_event {
            TabListEvent::SetSelected(index) => {
                if self.selected_index.get() != *index {
                    self.selected_index.set(*index);
                    if let Some(callback) = &self.on_select {
                        (callback)(cx, *index);
                    }
                }
                meta.consume();
            }

            TabListEvent::CloseFocused => {}
        });
    }
}

impl Handle<'_, TabView> {
    pub fn vertical(self) -> Self {
        self.modify(|tabview: &mut TabView| {
            tabview.is_vertical.set(true);
        })
    }

    pub fn on_select(self, callback: impl Fn(&mut EventContext, usize) + 'static) -> Self {
        self.modify(|tabview: &mut TabView| tabview.on_select = Some(Box::new(callback)))
    }

    pub fn with_selected<U: Into<usize>>(mut self, selected: impl Res<U>) -> Self {
        let _entity = self.entity();
        selected.set_or_bind(self.context(), |cx, selected| {
            let index = selected.get_value(cx).into();
            cx.emit(TabListEvent::SetSelected(index));
        });

        self
    }
}

pub struct TabPair {
    pub header: Box<dyn Fn(&mut Context)>,
    pub content: Box<dyn Fn(&mut Context)>,
}

impl TabPair {
    pub fn new<H, C>(header: H, content: C) -> Self
    where
        H: 'static + Fn(&mut Context),
        C: 'static + Fn(&mut Context),
    {
        Self { header: Box::new(header), content: Box::new(content) }
    }
}

pub struct Tab {
    on_close: Option<Arc<dyn Fn(&mut EventContext) + Send + Sync>>,
    has_close: Signal<bool>,
}

impl Tab {
    pub fn with_content<F>(cx: &mut Context, index: usize, content: F) -> Handle<Self>
    where
        F: 'static + Fn(&mut Context),
    {
        let has_close = Signal::new(false);

        Self { on_close: None, has_close }
            .build(cx, move |cx| {
                (content)(cx);

                Binding::new(cx, has_close, move |cx| {
                    if has_close.get() {
                        let on_close = cx.data::<Tab>().on_close.clone().unwrap();
                        Button::new(cx, |cx| Svg::new(cx, ICON_X))
                            .class("close")
                            .variant(ButtonVariant::Text)
                            .navigable(false)
                            .focusable(false)
                            .on_press(move |cx| (on_close)(cx));
                    }
                });
            })
            .role(Role::Tab)
            .navigable(false)
            .focusable(false)
            .toggle_class("closeable", has_close)
            .layout_type(LayoutType::Row)
            .on_press(move |cx| {
                cx.emit(ListEvent::Select(index));
            })
    }

    pub fn new<T: ToStringLocalized + 'static>(
        cx: &mut Context,
        index: usize,
        label: impl Res<T> + Clone + 'static,
        selected: impl Res<bool> + 'static,
    ) -> Handle<Self> {
        Self::with_content(cx, index, move |cx| {
            Label::new(cx, label.clone()).hoverable(false);
        })
        .checked(selected)
    }
}

impl View for Tab {
    fn element(&self) -> Option<&'static str> {
        Some("tab")
    }
}

impl Handle<'_, Tab> {
    /// Set the callback triggered when the close button of the tab is pressed.
    /// The tab close button is not displayed by default. Setting this callback causes the close button to be displayed.
    pub fn on_close(self, callback: impl Fn(&mut EventContext) + 'static + Send + Sync) -> Self {
        self.modify(|tab: &mut Tab| {
            tab.on_close = Some(Arc::new(callback));
            tab.has_close.set(true);
        })
    }
}

pub struct TabList {
    is_vertical: Signal<bool>,
    selected_index: Signal<Option<usize>>,
    selected_indices: Signal<Vec<usize>>,
    on_select: Option<Box<dyn Fn(&mut EventContext, usize)>>,
    on_close: Option<Box<dyn Fn(&mut EventContext, usize)>>,
}

impl TabList {
    pub fn new<S, V, T, F, H>(cx: &mut Context, list: S, item_content: F) -> Handle<Self>
    where
        S: Res<V> + 'static,
        V: Deref<Target = [T]> + Clone + 'static,
        T: PartialEq + Clone + 'static,
        F: 'static + Clone + for<'a> Fn(&'a mut Context, usize, T, Memo<bool>) -> Handle<'a, H>,
        H: View,
    {
        let is_vertical = Signal::new(false);
        let selected_index = Signal::new(Some(0));
        let selected_indices = Signal::new(vec![0usize]);

        Self { is_vertical, selected_index, selected_indices, on_select: None, on_close: None }
            .build(cx, move |cx| {
                let item_content = item_content.clone();
                let selected_indices = selected_indices;
                let list_entity = List::new_custom_items_with_selection(
                    cx,
                    list,
                    move |cx, index, item, is_selected| {
                        let is_selected_for_scroll = is_selected;
                        (item_content)(cx, index, item.get(), is_selected).bind(
                            is_selected_for_scroll,
                            move |handle| {
                                if is_selected_for_scroll.get() {
                                    handle.cx.emit(ScrollEvent::ScrollToView(handle.entity()));
                                }
                            },
                        )
                    },
                )
                .horizontal(is_vertical.map(|vertical| !*vertical))
                .selectable(Selectable::Single)
                .selection(selected_indices)
                .selection_follows_focus(true)
                .on_select(|cx, index| cx.emit(TabListEvent::SetSelected(index)))
                .role(Role::TabList)
                .show_horizontal_scrollbar(is_vertical.map(|vertical| !*vertical))
                .show_vertical_scrollbar(is_vertical.map(|vertical| *vertical))
                .entity();

                cx.with_current(list_entity, |cx| {
                    Keymap::from(vec![(
                        KeyChord::new(Modifiers::empty(), Code::Delete),
                        KeymapEntry::new("Close Focused Tab", |cx| {
                            cx.emit(TabListEvent::CloseFocused)
                        }),
                    )])
                    .build(cx);
                });
            })
            .toggle_class("vertical", is_vertical)
    }
}

impl Handle<'_, TabList> {
    pub fn selection<U: Into<usize> + Clone + 'static>(
        self,
        selected: impl Res<U> + 'static,
    ) -> Self {
        let selected = selected.to_signal(self.cx);
        self.bind(selected, move |handle| {
            let index = selected.get().into();
            handle.modify(|tablist: &mut TabList| {
                tablist.selected_indices.set(vec![index]);
                tablist.selected_index.set(Some(index));
            });
        })
    }

    pub fn vertical(self, vertical: impl Res<bool> + 'static) -> Self {
        let vertical = vertical.to_signal(self.cx);
        self.bind(vertical, move |handle| {
            let vertical = vertical.get();
            handle.modify(|tablist: &mut TabList| tablist.is_vertical.set(vertical));
        })
    }

    pub fn on_select(self, callback: impl Fn(&mut EventContext, usize) + 'static) -> Self {
        self.modify(|tablist: &mut TabList| tablist.on_select = Some(Box::new(callback)))
    }

    pub fn on_close(self, callback: impl Fn(&mut EventContext, usize) + 'static) -> Self {
        self.modify(|tablist: &mut TabList| tablist.on_close = Some(Box::new(callback)))
    }
}

impl View for TabList {
    fn element(&self) -> Option<&'static str> {
        Some("tablist")
    }

    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|tab_list_event, meta| match tab_list_event {
            TabListEvent::SetSelected(index) => {
                self.selected_indices.set(vec![*index]);
                self.selected_index.set(Some(*index));
                if let Some(callback) = &self.on_select {
                    (callback)(cx, *index);
                }
                meta.consume();
            }

            TabListEvent::CloseFocused => {
                if let Some(index) = self.selected_index.get() {
                    if let Some(callback) = &self.on_close {
                        (callback)(cx, index);
                    }
                }
                meta.consume();
            }
        });
    }
}
