use std::ops::Deref;
use std::sync::Arc;

use crate::icons::ICON_X;
use crate::prelude::*;

pub enum TabEvent {
    SetSelected(usize),
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
                let content_for_headers = content.clone();

                TabList::new(cx, list, move |cx, index, item| {
                    let builder = (content_for_headers)(cx, index, item).header;
                    let is_selected =
                        selected_index.map(move |selected_index| *selected_index == index);

                    Tab::with_content(cx, Some(index), builder)
                        .checked(is_selected)
                        .toggle_class("vertical", is_vertical);
                })
                .vertical(is_vertical)
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
            TabEvent::SetSelected(index) => {
                if self.selected_index.get() != *index {
                    self.selected_index.set(*index);
                    if let Some(callback) = &self.on_select {
                        (callback)(cx, *index);
                    }
                }
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

    pub fn with_selected<U: Into<usize>>(mut self, selected: impl Res<U>) -> Self {
        let _entity = self.entity();
        selected.set_or_bind(self.context(), |cx, selected| {
            let index = selected.get_value(cx).into();
            cx.emit(TabEvent::SetSelected(index));
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
    index: Option<usize>,
    on_close: Option<Arc<dyn Fn(&mut EventContext) + Send + Sync>>,
    has_close: Signal<bool>,
}

impl Tab {
    pub fn with_content<F>(cx: &mut Context, index: Option<usize>, content: F) -> Handle<Self>
    where
        F: 'static + Fn(&mut Context),
    {
        let has_close = Signal::new(false);

        Self { index, on_close: None, has_close }
            .build(cx, move |cx| {
                (content)(cx);

                Binding::new(cx, has_close, move |cx| {
                    if has_close.get() {
                        let on_close = cx.data::<Tab>().on_close.clone().unwrap();
                        Button::new(cx, |cx| Svg::new(cx, ICON_X))
                            .class("close-icon")
                            .height(Pixels(16.0))
                            .width(Pixels(16.0))
                            .alignment(Alignment::Center)
                            .variant(ButtonVariant::Text)
                            .on_press(move |cx| (on_close)(cx));
                    }
                });
            })
            .toggle_class("close", has_close)
            .layout_type(LayoutType::Row)
    }

    pub fn new<T: ToStringLocalized + 'static>(
        cx: &mut Context,
        label: impl Res<T> + Clone + 'static,
    ) -> Handle<Self> {
        Self::with_content(cx, None, move |cx| {
            Label::new(cx, label.clone()).hoverable(false);
        })
    }
}

impl View for Tab {
    fn element(&self) -> Option<&'static str> {
        Some("tab")
    }

    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|window_event, _meta| match window_event {
            WindowEvent::PressDown { mouse: _ } => {
                if let Some(index) = self.index {
                    cx.emit(TabEvent::SetSelected(index));
                }
            }

            _ => {}
        });
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
}

impl TabList {
    pub fn new<S, V, T, F>(cx: &mut Context, list: S, item_content: F) -> Handle<Self>
    where
        S: Res<V> + 'static,
        V: Deref<Target = [T]> + Clone + 'static,
        T: PartialEq + Clone + 'static,
        F: 'static + Clone + Fn(&mut Context, usize, T),
    {
        let is_vertical = Signal::new(false);

        Self { is_vertical }
            .build(cx, move |cx| {
                let item_content = item_content.clone();
                List::new(cx, list, move |cx, index, item| {
                    (item_content)(cx, index, item.get());
                })
                .horizontal(is_vertical.map(|vertical| !*vertical))
                .show_horizontal_scrollbar(is_vertical.map(|vertical| !*vertical))
                .show_vertical_scrollbar(is_vertical.map(|vertical| *vertical));
            })
            .toggle_class("vertical", is_vertical)
    }
}

impl Handle<'_, TabList> {
    pub fn vertical(self, vertical: impl Res<bool> + 'static) -> Self {
        let vertical = vertical.to_signal(self.cx);
        self.bind(vertical, move |handle| {
            let vertical = vertical.get();
            handle.modify(|tablist: &mut TabList| tablist.is_vertical.set(vertical));
        })
    }
}

impl View for TabList {
    fn element(&self) -> Option<&'static str> {
        Some("tablist")
    }
}
