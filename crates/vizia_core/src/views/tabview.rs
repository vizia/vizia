use crate::{icons::ICON_PLUS, prelude::*};

pub enum TabEvent {
    SetSelected(usize),
}

pub struct TabView {
    selected_index: Signal<usize>,
    is_vertical: Signal<bool>,
    on_select: Option<Box<dyn Fn(&mut EventContext, usize)>>,
}

impl TabView {
    pub fn new<T, F>(cx: &mut Context, list: Signal<Vec<T>>, content: F) -> Handle<Self>
    where
        T: Clone + 'static,
        F: 'static + Clone + Fn(&mut Context, Signal<T>) -> TabPair,
    {
        let selected_index = cx.state(0usize);
        let is_vertical = cx.state(false);
        let overflow_hidden = cx.state(Overflow::Hidden);
        let header_z = cx.state(1);
        let selected_item = cx.derived({
            let list = list;
            let selected_index = selected_index;
            move |s| {
                let items = list.get(s);
                items.get(*selected_index.get(s)).cloned()
            }
        });

        Self { selected_index, is_vertical, on_select: None }
            .build(cx, move |cx| {
                let content2 = content.clone();
                // Tab headers
                ScrollView::new(cx, move |cx| {
                    Binding::new(cx, list, move |cx| {
                        let list_len = list.get(cx).len();
                        if list_len > 0 {
                            let mut event_cx = EventContext::new(cx);
                            selected_index.update(&mut event_cx, |idx| {
                                if *idx >= list_len {
                                    *idx = list_len.saturating_sub(1);
                                }
                            });
                        }

                        let items = list.get(cx).clone();
                        for (index, item) in items.into_iter().enumerate() {
                            let item_signal = cx.state(item);
                            let builder = (content2)(cx, item_signal).header;
                            let is_selected = cx.derived({
                                let selected_index = selected_index;
                                move |store| *selected_index.get(store) == index
                            });
                            TabHeader::new(cx, index, builder)
                                .checked(is_selected)
                                .toggle_class("vertical", is_vertical);
                        }
                    })
                })
                .class("tabview-header")
                .z_index(header_z)
                .toggle_class("vertical", is_vertical);

                Divider::new(cx).toggle_class("vertical", is_vertical);

                // Tab content
                VStack::new(cx, move |cx| {
                    Binding::new(cx, selected_item, move |cx| {
                        if let Some(item) = selected_item.get(cx).clone() {
                            let item_signal = cx.state(item);
                            ((content)(cx, item_signal).content)(cx);
                        }
                    });
                })
                .overflow(overflow_hidden)
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
                self.selected_index.set(cx, *index);
                if let Some(callback) = &self.on_select {
                    (callback)(cx, *index);
                }
                meta.consume();
            }
        });
    }
}

impl Handle<'_, TabView> {
    pub fn vertical(self) -> Self {
        self.modify2(|tabview: &mut TabView, cx| tabview.is_vertical.set(cx, true))
    }

    pub fn on_select(self, callback: impl Fn(&mut EventContext, usize) + 'static) -> Self {
        self.modify(|tabview: &mut TabView| tabview.on_select = Some(Box::new(callback)))
    }

    pub fn with_selected(self, selected: Signal<usize>) -> Self {
        self.bind(selected, |handle, selected| {
            let index = *selected.get(&handle);
            handle.cx.emit(TabEvent::SetSelected(index));
        })
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

pub struct TabHeader {
    index: usize,
}

impl TabHeader {
    pub fn new<F>(cx: &mut Context, index: usize, content: F) -> Handle<Self>
    where
        F: 'static + Fn(&mut Context),
    {
        Self { index }.build(cx, |cx| (content)(cx))
    }
}

impl View for TabHeader {
    fn element(&self) -> Option<&'static str> {
        Some("tabheader")
    }

    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|window_event, _meta| match window_event {
            WindowEvent::PressDown { mouse: _ } => {
                cx.emit(TabEvent::SetSelected(self.index));
            }

            _ => {}
        });
    }
}

pub struct TabBar {}

impl TabBar {
    pub fn new<T: Clone + 'static>(
        cx: &mut Context,
        list: Signal<Vec<T>>,
        item_content: impl 'static + Fn(&mut Context, usize, Signal<T>),
    ) -> Handle<Self> {
        let align_left = cx.state(Alignment::Left);
        let layout_row = cx.state(LayoutType::Row);
        let selectable = cx.state(Selectable::Single);
        Self {}
            .build(cx, move |cx| {
                List::new(cx, list, item_content)
                    .selectable(selectable)
                    .layout_type(layout_row);
                let text_variant = cx.state(ButtonVariant::Text);
                let plus_icon = cx.state(ICON_PLUS);
                let icon_stretch = cx.state(Stretch(1.0));
                let zero_padding = cx.state(Pixels(0.0));
                let button_size = cx.state(Pixels(16.0));
                Button::new(cx, move |cx| Svg::new(cx, plus_icon).size(icon_stretch))
                    .variant(text_variant)
                    .padding(zero_padding)
                    .size(button_size);
            })
            .alignment(align_left)
            .layout_type(layout_row)
    }
}

impl View for TabBar {
    fn element(&self) -> Option<&'static str> {
        Some("tabbar")
    }
}
