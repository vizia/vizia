use std::ops::Deref;
use std::sync::Arc;

use crate::icons::ICON_X;
use crate::prelude::*;

pub enum TabListEvent {
    SetSelected(usize),
    CloseFocused,
    RequestClose(usize),
    SetTabListName(String),
    SetTabListLabeledBy(String),
}

pub struct TabView {
    selected_index: Signal<usize>,
    is_vertical: Signal<bool>,
    tablist_name: Signal<String>,
    tablist_labeled_by: Signal<Option<String>>,
    on_select: Option<Box<dyn Fn(&mut EventContext, usize)>>,
    on_close: Option<Box<dyn Fn(&mut EventContext, usize)>>,
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
        let tablist_name = Signal::new(String::from("Tabs"));
        let tablist_labeled_by = Signal::new(None::<String>);
        let list = list.to_signal(cx);

        Self {
            selected_index,
            is_vertical,
            tablist_name,
            tablist_labeled_by,
            on_select: None,
            on_close: None,
        }
        .build(cx, move |cx| {
            let tabview_entity = cx.current();
            let content_for_headers = content.clone();

            let tablist_entity = TabList::new(cx, list, move |cx, index, item, is_selected| {
                let tab_id = format!("{}-tab-{}", tabview_entity, index);
                let panel_id = format!("{}-panel-{}", tabview_entity, index);
                let tab_pair = (content_for_headers)(cx, index, item);
                let builder = tab_pair.header;
                let menu = tab_pair.menu;
                let closeable = tab_pair.closeable;

                let mut tab = Tab::with_content(cx, index, builder)
                    .id(tab_id)
                    .controls(panel_id)
                    .checked(is_selected)
                    .focused(is_selected)
                    .selected(is_selected)
                    .toggle_class("vertical", is_vertical);

                if let Some(menu_builder) = menu {
                    tab = tab.menu(menu_builder);
                }

                if closeable {
                    tab = tab.on_close(move |cx| {
                        cx.emit_to(tabview_entity, TabListEvent::RequestClose(index));
                    });
                }

                tab
            })
            .vertical(is_vertical)
            .orientation(is_vertical.map(|vertical| {
                if *vertical { Orientation::Vertical } else { Orientation::Horizontal }
            }))
            .name(tablist_name)
            .selection(selected_index)
            .on_select(move |cx, index| {
                cx.emit_to(tabview_entity, TabListEvent::SetSelected(index))
            })
            .on_close(move |cx, index| {
                cx.emit_to(tabview_entity, TabListEvent::RequestClose(index))
            })
            .toggle_class("vertical", is_vertical)
            .entity();

            Binding::new(cx, tablist_labeled_by, move |cx| {
                if let Some(label_id) = tablist_labeled_by.get() {
                    cx.style.labelled_by.insert(tablist_entity, label_id);
                    cx.style.needs_access_update(tablist_entity);
                }
            });

            Divider::new(cx).toggle_class("vertical", is_vertical);

            VStack::new(cx, move |cx| {
                Binding::new(cx, list, move |cx| {
                    let list_values = list.get();
                    let content_for_panels = content.clone();

                    for (index, item) in list_values.iter().cloned().enumerate() {
                        let content_for_panel = content_for_panels.clone();
                        let tab_id = format!("{}-tab-{}", tabview_entity, index);
                        let panel_id = format!("{}-panel-{}", tabview_entity, index);

                        TabPanel::new(cx, index, selected_index, move |cx| {
                            ((content_for_panel)(cx, index, item.clone()).content)(cx);
                        })
                        .id(panel_id)
                        .role(Role::TabPanel)
                        .labeled_by(tab_id)
                        .hidden(selected_index.map(move |selected| *selected != index));
                    }
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

            TabListEvent::RequestClose(index) => {
                if let Some(callback) = &self.on_close {
                    (callback)(cx, *index);
                }
                meta.consume();
            }

            TabListEvent::SetTabListName(name) => {
                self.tablist_name.set_if_changed(name.clone());
                meta.consume();
            }

            TabListEvent::SetTabListLabeledBy(id) => {
                self.tablist_labeled_by.set_if_changed(Some(id.clone()));
                meta.consume();
            }
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

    pub fn on_close(self, callback: impl Fn(&mut EventContext, usize) + 'static) -> Self {
        self.modify(|tabview: &mut TabView| tabview.on_close = Some(Box::new(callback)))
    }

    pub fn tablist_name<U>(mut self, name: impl Res<U>) -> Self
    where
        U: ToStringLocalized + 'static,
    {
        name.set_or_bind(self.context(), |cx, name| {
            let name = name.get_value(cx).to_string_local(cx);
            cx.emit(TabListEvent::SetTabListName(name));
        });

        self
    }

    pub fn tablist_labeled_by<U>(mut self, id: impl Res<U>) -> Self
    where
        U: Into<String> + Clone + 'static,
    {
        id.set_or_bind(self.context(), |cx, id| {
            cx.emit(TabListEvent::SetTabListLabeledBy(id.get_value(cx).into()));
        });

        self
    }

    pub fn with_selected<U: Into<usize> + Clone + 'static>(
        mut self,
        selected: impl Res<U> + 'static,
    ) -> Self {
        let _entity = self.entity();
        let selected = selected.to_signal(self.context());
        self.bind(selected, move |handle| {
            let index = selected.get().into();
            handle.modify(|tabview: &mut TabView| {
                tabview.selected_index.set(index);
            });
        })
    }
}

/// A panel associated with a tab index.
///
/// The panel content is shown when its index matches the selected index and hidden otherwise.
pub struct TabPanel {}

impl TabPanel {
    pub fn new<U, F>(
        cx: &mut Context,
        index: usize,
        selected_index: impl Res<U> + 'static,
        content: F,
    ) -> Handle<Self>
    where
        U: Into<usize> + Clone + 'static,
        F: 'static + Fn(&mut Context),
    {
        let selected_index = selected_index.to_signal(cx);

        Self {}
            .build(cx, move |cx| {
                (content)(cx);
            })
            .display(selected_index.map(move |selected| {
                if selected.clone().into() == index { Display::Flex } else { Display::None }
            }))
    }
}

impl View for TabPanel {
    fn element(&self) -> Option<&'static str> {
        Some("tabpanel")
    }
}

pub struct TabPair {
    pub header: Box<dyn Fn(&mut Context)>,
    pub content: Box<dyn Fn(&mut Context)>,
    pub menu: Option<Box<dyn for<'a> Fn(&'a mut Context) -> Handle<'a, Popover>>>,
    pub closeable: bool,
}

impl TabPair {
    pub fn new<H, C>(header: H, content: C) -> Self
    where
        H: 'static + Fn(&mut Context),
        C: 'static + Fn(&mut Context),
    {
        Self { header: Box::new(header), content: Box::new(content), menu: None, closeable: false }
    }

    pub fn menu<M>(mut self, menu: M) -> Self
    where
        M: 'static + for<'a> Fn(&'a mut Context) -> Handle<'a, Popover>,
    {
        self.menu = Some(Box::new(menu));
        self
    }

    pub fn closeable(mut self, closeable: bool) -> Self {
        self.closeable = closeable;
        self
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
                            .focusable(true)
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
                        (item_content)(cx, index, item.get(), is_selected)
                            .bind(is_selected_for_scroll, move |handle| {
                                if is_selected_for_scroll.get() {
                                    handle.cx.emit(ScrollEvent::ScrollToView(handle.entity()));
                                }
                            })
                            .on_geo_changed(move |cx, geo| {
                                if is_selected_for_scroll.get()
                                    && geo.intersects(GeoChanged::WIDTH_CHANGED)
                                {
                                    cx.emit(ScrollEvent::ScrollToView(cx.current()));
                                }
                            })
                    },
                )
                .horizontal(is_vertical.map(|vertical| !*vertical))
                .selectable(Selectable::Single)
                .min_selected(1)
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

            TabListEvent::RequestClose(_) => {}

            TabListEvent::SetTabListName(_) => {}

            TabListEvent::SetTabListLabeledBy(_) => {}
        });
    }
}
