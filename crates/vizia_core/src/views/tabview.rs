use crate::{icons::ICON_PLUS, prelude::*, views::list::Keyed};
use std::{
    cell::RefCell,
    collections::{HashMap, VecDeque},
    rc::Rc,
};

/// Internal trait for selecting the appropriate TabView build strategy.
#[doc(hidden)]
pub trait TabSource<T>: Sized {
    fn build_tabview<F>(self, cx: &mut Context, content: F) -> Handle<TabView>
    where
        T: Clone + 'static,
        F: 'static + Clone + Fn(&mut Context, Signal<T>) -> TabPair;
}

impl<T, R> TabSource<T> for R
where
    T: Clone + 'static,
    R: Res<Vec<T>> + 'static,
{
    fn build_tabview<F>(self, cx: &mut Context, content: F) -> Handle<TabView>
    where
        T: Clone + 'static,
        F: 'static + Clone + Fn(&mut Context, Signal<T>) -> TabPair,
    {
        TabView::new_generic(cx, self, content)
    }
}

impl<T, K, R, F> TabSource<T> for Keyed<T, K, R, F>
where
    T: Clone + 'static,
    K: Eq + std::hash::Hash + Clone + 'static,
    R: Res<Vec<T>> + 'static,
    F: 'static + Clone + Fn(&T) -> K,
{
    fn build_tabview<C>(self, cx: &mut Context, content: C) -> Handle<TabView>
    where
        T: Clone + 'static,
        C: 'static + Clone + Fn(&mut Context, Signal<T>) -> TabPair,
    {
        TabView::new_generic_keyed(cx, self.list, self.key, content)
    }
}

struct KeyedTabItem<T: 'static> {
    entity: Entity,
    item: Signal<T>,
    index: Signal<usize>,
}

pub enum TabEvent {
    SetSelected(usize),
}

pub struct TabView {
    selected_index: Signal<usize>,
    is_vertical: Signal<bool>,
    on_select: Option<Box<dyn Fn(&mut EventContext, usize)>>,
}

impl TabView {
    /// Creates a new [TabView] view.
    ///
    /// Accepts either a plain list value or a `Signal<Vec<T>>` for reactive state.
    /// Use `.keyed(|t| t.id)` for stable-key reuse when tab order changes.
    pub fn new<T, F>(cx: &mut Context, list: impl TabSource<T>, content: F) -> Handle<Self>
    where
        T: Clone + 'static,
        F: 'static + Clone + Fn(&mut Context, Signal<T>) -> TabPair,
    {
        list.build_tabview(cx, content)
    }

    fn new_generic<T, F>(
        cx: &mut Context,
        list: impl Res<Vec<T>> + 'static,
        content: F,
    ) -> Handle<Self>
    where
        T: Clone + 'static,
        F: 'static + Clone + Fn(&mut Context, Signal<T>) -> TabPair,
    {
        let list = list.into_signal(cx);
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
                            selected_index.upd(&mut event_cx, |idx| {
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

    fn new_generic_keyed<T, K, F>(
        cx: &mut Context,
        list: impl Res<Vec<T>> + 'static,
        key: impl 'static + Clone + Fn(&T) -> K,
        content: F,
    ) -> Handle<Self>
    where
        T: Clone + 'static,
        K: Eq + std::hash::Hash + Clone + 'static,
        F: 'static + Clone + Fn(&mut Context, Signal<T>) -> TabPair,
    {
        let list = list.into_signal(cx);
        let content: Rc<dyn Fn(&mut Context, Signal<T>) -> TabPair> = Rc::new(content);
        let key_fn = Rc::new(key);
        let keyed_items: Rc<RefCell<HashMap<K, VecDeque<KeyedTabItem<T>>>>> =
            Rc::new(RefCell::new(HashMap::new()));
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
                let key_fn = key_fn.clone();
                let keyed_items = keyed_items.clone();
                // Tab headers
                ScrollView::new(cx, move |cx| {
                    let content2 = content2.clone();
                    let key_fn = key_fn.clone();
                    let keyed_items = keyed_items.clone();
                    Binding::new(cx, list, move |cx| {
                        let binding_entity =
                            cx.tree.get_parent(cx.current()).unwrap_or(Entity::root());
                        let list_len = list.get(cx).len();
                        if list_len > 0 {
                            let mut event_cx = EventContext::new(cx);
                            selected_index.upd(&mut event_cx, |idx| {
                                if *idx >= list_len {
                                    *idx = list_len.saturating_sub(1);
                                }
                            });
                        }

                        let mut old_map = {
                            let mut map_ref = keyed_items.borrow_mut();
                            std::mem::take(&mut *map_ref)
                        };
                        let mut new_map: HashMap<K, VecDeque<KeyedTabItem<T>>> = HashMap::new();
                        let mut order: Vec<Entity> = Vec::new();

                        let items = list.get(cx).clone();
                        for (index, item) in items.into_iter().enumerate() {
                            let key = (key_fn)(&item);
                            let mut existing =
                                old_map.get_mut(&key).and_then(|queue| queue.pop_front());

                            if let Some(ref mut keyed_item) = existing {
                                let mut event_cx = EventContext::new(cx);
                                keyed_item.item.set(&mut event_cx, item);
                                keyed_item.index.set(&mut event_cx, index);
                            } else {
                                cx.with_current(binding_entity, |cx| {
                                    let item_signal = cx.state(item);
                                    let index_signal = cx.state(index);
                                    let builder = (content2)(cx, item_signal).header;
                                    let is_selected = cx.derived({
                                        let selected_index = selected_index;
                                        let index_signal = index_signal;
                                        move |store| {
                                            *selected_index.get(store) == *index_signal.get(store)
                                        }
                                    });
                                    let handle =
                                        TabHeader::new_with_signal(cx, index_signal, builder)
                                            .checked(is_selected)
                                            .toggle_class("vertical", is_vertical);
                                    existing = Some(KeyedTabItem {
                                        entity: handle.entity(),
                                        item: item_signal,
                                        index: index_signal,
                                    });
                                });
                            }

                            let keyed_item = existing.expect("Keyed tab item missing");
                            order.push(keyed_item.entity);
                            new_map.entry(key).or_default().push_back(keyed_item);
                        }

                        for (_, mut queue) in old_map {
                            for item in queue.drain(..) {
                                cx.remove(item.entity);
                            }
                        }

                        *keyed_items.borrow_mut() = new_map;

                        for entity in order {
                            cx.tree.set_parent(entity, binding_entity);
                        }
                        cx.needs_relayout();
                    })
                })
                .class("tabview-header")
                .z_index(header_z)
                .toggle_class("vertical", is_vertical);

                Divider::new(cx).toggle_class("vertical", is_vertical);

                // Tab content (not keyed - only shows selected item)
                let content = content.clone();
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
    index: Signal<usize>,
}

impl TabHeader {
    pub fn new<F>(cx: &mut Context, index: usize, content: F) -> Handle<Self>
    where
        F: 'static + Fn(&mut Context),
    {
        let index = cx.state(index);
        Self { index }.build(cx, |cx| (content)(cx))
    }

    fn new_with_signal<F>(cx: &mut Context, index: Signal<usize>, content: F) -> Handle<Self>
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
                cx.emit(TabEvent::SetSelected(*self.index.get(cx)));
            }

            _ => {}
        });
    }
}

pub struct TabBar {}

impl TabBar {
    /// Creates a new [TabBar] view.
    ///
    /// Accepts either a plain list or a keyed list for item reuse.
    /// Use `.keyed(|t| t.id)` for stable-key reuse when list order changes.
    pub fn new<T: Clone + 'static>(
        cx: &mut Context,
        list: impl ListSource<T>,
        item_content: impl 'static + Fn(&mut Context, usize, Signal<T>),
    ) -> Handle<Self> {
        let align_left = cx.state(Alignment::Left);
        let layout_row = cx.state(LayoutType::Row);
        let selectable = cx.state(Selectable::Single);
        Self {}
            .build(cx, move |cx| {
                List::new(cx, list, item_content).selectable(selectable).layout_type(layout_row);
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
