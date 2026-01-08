use std::{
    cell::RefCell,
    collections::{BTreeSet, HashMap, VecDeque},
    marker::PhantomData,
    rc::Rc,
};

use crate::prelude::*;

/// Represents how items can be selected in a list.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Data)]
pub enum Selectable {
    #[default]
    /// Items in the list cannot be selected.
    None,
    /// A single item in the list can be selected.
    Single,
    /// Multiple items in the list can be selected simultaneously.
    Multi,
}

/// Events used by the [List] view
pub enum ListEvent {
    /// Selects a list item with the given index.
    Select(usize),
    /// Selects the focused list item.
    SelectFocused,
    ///  Moves the focus to the next item in the list.
    FocusNext,
    ///  Moves the focus to the previous item in the list.
    FocusPrev,
    /// Deselects all items from the list
    ClearSelection,
    /// Scrolls the list to the given x and y position.
    Scroll(f32, f32),
}

/// A view for creating a list of items from a binding to an iteratable list.
pub struct List {
    /// The number of items in the list.
    num_items: Signal<usize>,
    /// The set of selected items in the list.
    selected: Signal<BTreeSet<usize>>,
    /// Whether the list items are selectable.
    selectable: Signal<Selectable>,
    /// The index of the currently focused item in the list.
    focused: Signal<Option<usize>>,
    /// Whether the selection should follow the focus.
    selection_follows_focus: bool,
    /// The orientation of the list, either vertical or horizontal.
    orientation: Signal<Orientation>,
    /// Whether the scrollview should scroll to the cursor when the scrollbar is pressed.
    scroll_to_cursor: Signal<bool>,
    /// Callback called when a list item is selected.
    on_select: Option<Box<dyn Fn(&mut EventContext, usize)>>,
    /// Callback called when the scrollview is scrolled.
    on_scroll: Option<Box<dyn Fn(&mut EventContext, f32, f32) + Send + Sync>>,
    /// The horizontal scroll position of the list.
    scroll_x: Signal<f32>,
    /// The vertical scroll position of the list.
    scroll_y: Signal<f32>,
    /// Whether the horizontal scrollbar should be visible.
    show_horizontal_scrollbar: Signal<bool>,
    /// Whether the vertical scrollbar should be visible.
    show_vertical_scrollbar: Signal<bool>,
}

/// A keyed list wrapper used to reuse item entities when keys are stable.
/// Works with [`List::new`], [`TabView::new`], [`TabBar::new`], and [`PickList::new`].
pub struct Keyed<T, K, R, F> {
    pub(crate) list: R,
    pub(crate) key: F,
    pub(crate) _marker: PhantomData<fn() -> (T, K)>,
}

/// Wrap a list with a key function for reuse.
pub fn keyed<T, K, R, F>(list: R, key: F) -> Keyed<T, K, R, F> {
    Keyed { list, key, _marker: PhantomData }
}

/// Extension trait to opt into keyed reuse for list-like views.
pub trait KeyedExt<T>: Sized {
    /// Wrap this list with a key function for keyed reuse.
    /// Works with [`List::new`], [`TabView::new`], [`TabBar::new`], and [`PickList::new`].
    fn keyed<K, F>(self, key: F) -> Keyed<T, K, Self, F>;
}

impl<T, R> KeyedExt<T> for R
where
    T: Clone + 'static,
    R: Res<Vec<T>> + 'static,
{
    fn keyed<K, F>(self, key: F) -> Keyed<T, K, Self, F> {
        keyed(self, key)
    }
}

/// Internal trait for selecting the appropriate list build strategy.
#[doc(hidden)]
pub trait ListSource<T>: Sized {
    fn build<Filt, Item>(self, cx: &mut Context, filter: Filt, item_content: Item) -> Handle<List>
    where
        T: Clone + 'static,
        Filt: 'static + Clone + FnMut(&T) -> bool,
        Item: 'static + Fn(&mut Context, usize, Signal<T>);
}

impl<T, R> ListSource<T> for R
where
    T: Clone + 'static,
    R: Res<Vec<T>> + 'static,
{
    fn build<Filt, Item>(self, cx: &mut Context, filter: Filt, item_content: Item) -> Handle<List>
    where
        T: Clone + 'static,
        Filt: 'static + Clone + FnMut(&T) -> bool,
        Item: 'static + Fn(&mut Context, usize, Signal<T>),
    {
        List::new_generic(cx, self, filter, item_content)
    }
}

impl<T, K, R, F> ListSource<T> for Keyed<T, K, R, F>
where
    T: Clone + 'static,
    K: Eq + std::hash::Hash + Clone + 'static,
    R: Res<Vec<T>> + 'static,
    F: 'static + Clone + Fn(&T) -> K,
{
    fn build<Filt, Item>(self, cx: &mut Context, filter: Filt, item_content: Item) -> Handle<List>
    where
        T: Clone + 'static,
        Filt: 'static + Clone + FnMut(&T) -> bool,
        Item: 'static + Fn(&mut Context, usize, Signal<T>),
    {
        List::new_generic_keyed(cx, self.list, self.key, filter, item_content)
    }
}

struct KeyedItem<T: 'static> {
    entity: Entity,
    item: Signal<T>,
    index: Signal<usize>,
}

fn build_keyed_item<T: Clone + 'static>(
    cx: &mut Context,
    index: usize,
    item: T,
    selected: Signal<BTreeSet<usize>>,
    focused: Signal<Option<usize>>,
    item_content: &Rc<dyn Fn(&mut Context, usize, Signal<T>)>,
) -> KeyedItem<T> {
    let is_focused = focused.get(cx).as_ref().is_some_and(|f| *f == index);
    let mut handle = ListItem {}.build(cx, |_| {});
    let entity = handle.entity();
    let (item_signal, index_signal) = {
        let cx = handle.context();
        cx.with_current(entity, |cx| {
            let item_signal = cx.state(item);
            let index_signal = cx.state(index);
            (item_signal, index_signal)
        })
    };

    let content = item_content.clone();
    {
        let cx = handle.context();
        cx.with_current(entity, move |cx| {
            (content)(cx, index, item_signal);
        });
    }

    let (is_focused_signal, is_selected_signal) = {
        let cx = handle.context();
        let is_focused_signal = cx.derived({
            let focused = focused;
            let index_signal = index_signal;
            move |store| focused.get(store).as_ref().is_some_and(|f| *f == *index_signal.get(store))
        });
        let is_selected_signal = cx.derived({
            let selected = selected;
            let index_signal = index_signal;
            move |store| selected.get(store).contains(index_signal.get(store))
        });
        (is_focused_signal, is_selected_signal)
    };

    handle
        .role(Role::ListItem)
        .toggle_class("focused", is_focused_signal)
        .checked(is_selected_signal)
        .bind(focused, move |handle, foc| {
            let is_now_focused =
                foc.get(&handle).as_ref().is_some_and(|f| *f == *index_signal.get(&handle));
            if is_now_focused != is_focused {
                handle.cx.emit(ScrollEvent::ScrollToView(handle.entity()));
            }
        })
        .on_press(move |cx| {
            let index = *index_signal.get(cx);
            cx.emit(ListEvent::Select(index));
        });

    KeyedItem { entity, item: item_signal, index: index_signal }
}

impl List {
    /// Creates a new [List] view.
    ///
    /// Accepts either a plain list value or a `Signal<Vec<T>>` for reactive state.
    /// Use [`keyed`] or [`KeyedListExt::keyed`] to enable stable-key reuse.
    pub fn new<T: Clone + 'static>(
        cx: &mut Context,
        list: impl ListSource<T>,
        item_content: impl 'static + Fn(&mut Context, usize, Signal<T>),
    ) -> Handle<Self> {
        list.build(cx, |_| true, item_content)
    }

    /// Creates a new [List] view with a provided filter closure.
    pub fn new_filtered<T: Clone + 'static>(
        cx: &mut Context,
        list: impl ListSource<T>,
        filter: impl 'static + Clone + FnMut(&T) -> bool,
        item_content: impl 'static + Fn(&mut Context, usize, Signal<T>),
    ) -> Handle<Self> {
        list.build(cx, filter, item_content)
    }

    /// Creates a new [List] view with a binding to the given signal and a template for constructing the list items.
    fn new_generic<T: Clone + 'static>(
        cx: &mut Context,
        list: impl Res<Vec<T>> + 'static,
        filter: impl 'static + Clone + FnMut(&T) -> bool,
        item_content: impl 'static + Fn(&mut Context, usize, Signal<T>),
    ) -> Handle<Self> {
        let list = list.into_signal(cx);
        let content = Rc::new(item_content);
        let num_items = cx.state(list.get(cx).len());
        let selected = cx.state(BTreeSet::<usize>::default());
        let selectable = cx.state(Selectable::None);
        let focused = cx.state(None::<usize>);
        let orientation = cx.state(Orientation::Vertical);
        let scroll_to_cursor = cx.state(false);
        let scroll_x = cx.state(0.0);
        let scroll_y = cx.state(0.0);
        let show_horizontal_scrollbar = cx.state(true);
        let show_vertical_scrollbar = cx.state(true);
        let is_selectable = cx.derived({
            let selectable = selectable;
            move |store| *selectable.get(store) != Selectable::None
        });
        let is_horizontal = cx.derived({
            let orientation = orientation;
            move |store| *orientation.get(store) == Orientation::Horizontal
        });
        let navigable = cx.state(true);
        Self {
            num_items,
            selected,
            selectable,
            focused,
            selection_follows_focus: false,
            orientation,
            scroll_to_cursor,
            on_select: None,
            on_scroll: None,
            scroll_x,
            scroll_y,
            show_horizontal_scrollbar,
            show_vertical_scrollbar,
        }
        .build(cx, move |cx| {
            Keymap::from(vec![
                (
                    KeyChord::new(Modifiers::empty(), Code::ArrowDown),
                    KeymapEntry::new("Focus Next", |cx| cx.emit(ListEvent::FocusNext)),
                ),
                (
                    KeyChord::new(Modifiers::empty(), Code::ArrowUp),
                    KeymapEntry::new("Focus Previous", |cx| cx.emit(ListEvent::FocusPrev)),
                ),
                (
                    KeyChord::new(Modifiers::empty(), Code::Space),
                    KeymapEntry::new("Select Focused", |cx| cx.emit(ListEvent::SelectFocused)),
                ),
                (
                    KeyChord::new(Modifiers::empty(), Code::Enter),
                    KeymapEntry::new("Select Focused", |cx| cx.emit(ListEvent::SelectFocused)),
                ),
            ])
            .build(cx);

            // Update keymap based on orientation without affecting layout.
            Binding::new(cx, orientation, move |cx| {
                if *orientation.get(cx) == Orientation::Horizontal {
                    cx.emit(KeymapEvent::RemoveAction(
                        KeyChord::new(Modifiers::empty(), Code::ArrowDown),
                        "Focus Next",
                    ));

                    cx.emit(KeymapEvent::RemoveAction(
                        KeyChord::new(Modifiers::empty(), Code::ArrowUp),
                        "Focus Previous",
                    ));

                    cx.emit(KeymapEvent::InsertAction(
                        KeyChord::new(Modifiers::empty(), Code::ArrowRight),
                        KeymapEntry::new("Focus Next", |cx| cx.emit(ListEvent::FocusNext)),
                    ));

                    cx.emit(KeymapEvent::InsertAction(
                        KeyChord::new(Modifiers::empty(), Code::ArrowLeft),
                        KeymapEntry::new("Focus Previous", |cx| cx.emit(ListEvent::FocusPrev)),
                    ));
                }
            });

            ScrollView::new(cx, move |cx| {
                // Bind to the list data
                Binding::new(cx, list, move |cx| {
                    let items = list.get(cx).clone();
                    let mut event_cx = EventContext::new(cx);
                    num_items.set(&mut event_cx, items.len());

                    let mut f = filter.clone();
                    for (index, item) in items.iter().enumerate() {
                        if !f(item) {
                            continue;
                        }

                        let item_signal = cx.state(item.clone());
                        let content = content.clone();
                        ListItem::new(
                            cx,
                            index,
                            item_signal,
                            selected,
                            focused,
                            move |cx, index, item| {
                                content(cx, index, item);
                            },
                        );
                    }
                });
            })
            .show_horizontal_scrollbar(show_horizontal_scrollbar)
            .show_vertical_scrollbar(show_vertical_scrollbar)
            .scroll_to_cursor(scroll_to_cursor)
            .scroll_x(scroll_x)
            .scroll_y(scroll_y)
            .on_scroll(|cx, x, y| {
                if y.is_finite() {
                    cx.emit(ListEvent::Scroll(x, y));
                }
            });
        })
        .toggle_class("selectable", is_selectable)
        .toggle_class("horizontal", is_horizontal)
        .navigable(navigable)
        .role(Role::List)
    }

    /// Creates a new [List] view with stable keys for item reuse.
    fn new_generic_keyed<T: Clone + 'static, K: Eq + std::hash::Hash + Clone + 'static>(
        cx: &mut Context,
        list: impl Res<Vec<T>> + 'static,
        key: impl 'static + Clone + Fn(&T) -> K,
        filter: impl 'static + Clone + FnMut(&T) -> bool,
        item_content: impl 'static + Fn(&mut Context, usize, Signal<T>),
    ) -> Handle<Self> {
        let list = list.into_signal(cx);
        let content: Rc<dyn Fn(&mut Context, usize, Signal<T>)> = Rc::new(item_content);
        let key_fn = Rc::new(key);
        let keyed_items: Rc<RefCell<HashMap<K, VecDeque<KeyedItem<T>>>>> =
            Rc::new(RefCell::new(HashMap::new()));
        let num_items = cx.state(list.get(cx).len());
        let selected = cx.state(BTreeSet::<usize>::default());
        let selectable = cx.state(Selectable::None);
        let focused = cx.state(None::<usize>);
        let orientation = cx.state(Orientation::Vertical);
        let scroll_to_cursor = cx.state(false);
        let scroll_x = cx.state(0.0);
        let scroll_y = cx.state(0.0);
        let show_horizontal_scrollbar = cx.state(true);
        let show_vertical_scrollbar = cx.state(true);
        let is_selectable = cx.derived({
            let selectable = selectable;
            move |store| *selectable.get(store) != Selectable::None
        });
        let is_horizontal = cx.derived({
            let orientation = orientation;
            move |store| *orientation.get(store) == Orientation::Horizontal
        });
        let navigable = cx.state(true);
        Self {
            num_items,
            selected,
            selectable,
            focused,
            selection_follows_focus: false,
            orientation,
            scroll_to_cursor,
            on_select: None,
            on_scroll: None,
            scroll_x,
            scroll_y,
            show_horizontal_scrollbar,
            show_vertical_scrollbar,
        }
        .build(cx, move |cx| {
            Keymap::from(vec![
                (
                    KeyChord::new(Modifiers::empty(), Code::ArrowDown),
                    KeymapEntry::new("Focus Next", |cx| cx.emit(ListEvent::FocusNext)),
                ),
                (
                    KeyChord::new(Modifiers::empty(), Code::ArrowUp),
                    KeymapEntry::new("Focus Previous", |cx| cx.emit(ListEvent::FocusPrev)),
                ),
                (
                    KeyChord::new(Modifiers::empty(), Code::Space),
                    KeymapEntry::new("Select Focused", |cx| cx.emit(ListEvent::SelectFocused)),
                ),
                (
                    KeyChord::new(Modifiers::empty(), Code::Enter),
                    KeymapEntry::new("Select Focused", |cx| cx.emit(ListEvent::SelectFocused)),
                ),
            ])
            .build(cx);

            // Update keymap based on orientation without affecting layout.
            Binding::new(cx, orientation, move |cx| {
                if *orientation.get(cx) == Orientation::Horizontal {
                    cx.emit(KeymapEvent::RemoveAction(
                        KeyChord::new(Modifiers::empty(), Code::ArrowDown),
                        "Focus Next",
                    ));

                    cx.emit(KeymapEvent::RemoveAction(
                        KeyChord::new(Modifiers::empty(), Code::ArrowUp),
                        "Focus Previous",
                    ));

                    cx.emit(KeymapEvent::InsertAction(
                        KeyChord::new(Modifiers::empty(), Code::ArrowRight),
                        KeymapEntry::new("Focus Next", |cx| cx.emit(ListEvent::FocusNext)),
                    ));

                    cx.emit(KeymapEvent::InsertAction(
                        KeyChord::new(Modifiers::empty(), Code::ArrowLeft),
                        KeymapEntry::new("Focus Previous", |cx| cx.emit(ListEvent::FocusPrev)),
                    ));
                }
            });

            ScrollView::new(cx, move |cx| {
                let content = content.clone();
                let key_fn = key_fn.clone();
                let keyed_items = keyed_items.clone();
                Binding::new(cx, list, move |cx| {
                    let binding_entity = cx.tree.get_parent(cx.current()).unwrap_or(Entity::root());
                    let items = list.get(cx).clone();
                    {
                        let mut event_cx = EventContext::new(cx);
                        num_items.set(&mut event_cx, items.len());
                    }

                    let mut old_map = {
                        let mut map_ref = keyed_items.borrow_mut();
                        std::mem::take(&mut *map_ref)
                    };
                    let mut new_map: HashMap<K, VecDeque<KeyedItem<T>>> = HashMap::new();
                    let mut order: Vec<Entity> = Vec::new();

                    let mut f = filter.clone();
                    for (index, item) in items.iter().enumerate() {
                        if !f(item) {
                            continue;
                        }
                        let key = (key_fn)(item);
                        let mut existing =
                            old_map.get_mut(&key).and_then(|queue| queue.pop_front());
                        let item_value = item.clone();

                        if let Some(ref mut keyed_item) = existing {
                            let mut event_cx = EventContext::new(cx);
                            keyed_item.item.set(&mut event_cx, item_value);
                            keyed_item.index.set(&mut event_cx, index);
                        } else {
                            cx.with_current(binding_entity, |cx| {
                                existing = Some(build_keyed_item(
                                    cx, index, item_value, selected, focused, &content,
                                ));
                            });
                        }

                        let keyed_item = existing.expect("Keyed list item missing");
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
                });
            })
            .show_horizontal_scrollbar(show_horizontal_scrollbar)
            .show_vertical_scrollbar(show_vertical_scrollbar)
            .scroll_to_cursor(scroll_to_cursor)
            .scroll_x(scroll_x)
            .scroll_y(scroll_y)
            .on_scroll(|cx, x, y| {
                if y.is_finite() {
                    cx.emit(ListEvent::Scroll(x, y));
                }
            });
        })
        .toggle_class("selectable", is_selectable)
        .toggle_class("horizontal", is_horizontal)
        .navigable(navigable)
        .role(Role::List)
    }
}

impl View for List {
    fn element(&self) -> Option<&'static str> {
        Some("list")
    }

    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.take(|list_event, meta| match list_event {
            ListEvent::Select(index) => {
                cx.focus();
                let current_selectable = *self.selectable.get(cx);
                match current_selectable {
                    Selectable::Single => {
                        let contains = self.selected.get(cx).contains(&index);
                        if contains {
                            self.selected.upd(cx, |s| s.clear());
                            self.focused.set(cx, None);
                        } else {
                            self.selected.upd(cx, |s| {
                                s.clear();
                                s.insert(index);
                            });
                            self.focused.set(cx, Some(index));
                            if let Some(on_select) = &self.on_select {
                                on_select(cx, index);
                            }
                        }
                    }

                    Selectable::Multi => {
                        let contains = self.selected.get(cx).contains(&index);
                        if contains {
                            self.selected.upd(cx, |s| {
                                s.remove(&index);
                            });
                            self.focused.set(cx, None);
                        } else {
                            self.selected.upd(cx, |s| {
                                s.insert(index);
                            });
                            self.focused.set(cx, Some(index));
                            if let Some(on_select) = &self.on_select {
                                on_select(cx, index);
                            }
                        }
                    }

                    Selectable::None => {}
                }

                meta.consume();
            }

            ListEvent::SelectFocused => {
                if let Some(focused) = *self.focused.get(cx) {
                    cx.emit(ListEvent::Select(focused))
                }
                meta.consume();
            }

            ListEvent::ClearSelection => {
                self.selected.upd(cx, |s| s.clear());
                meta.consume();
            }

            ListEvent::FocusNext => {
                let current_focused = *self.focused.get(cx);
                let current_num_items = *self.num_items.get(cx);
                if let Some(focused) = current_focused {
                    if focused < current_num_items.saturating_sub(1) {
                        self.focused.set(cx, Some(focused.saturating_add(1)));
                        if self.selection_follows_focus {
                            cx.emit(ListEvent::SelectFocused);
                        }
                    }
                } else {
                    self.focused.set(cx, Some(0));
                    if self.selection_follows_focus {
                        cx.emit(ListEvent::SelectFocused);
                    }
                }

                meta.consume();
            }

            ListEvent::FocusPrev => {
                let current_focused = *self.focused.get(cx);
                let current_num_items = *self.num_items.get(cx);
                if let Some(focused) = current_focused {
                    if focused > 0 {
                        self.focused.set(cx, Some(focused.saturating_sub(1)));
                        if self.selection_follows_focus {
                            cx.emit(ListEvent::SelectFocused);
                        }
                    }
                } else {
                    self.focused.set(cx, Some(current_num_items.saturating_sub(1)));
                    if self.selection_follows_focus {
                        cx.emit(ListEvent::SelectFocused);
                    }
                }

                meta.consume();
            }

            ListEvent::Scroll(x, y) => {
                self.scroll_x.set(cx, x);
                self.scroll_y.set(cx, y);
                if let Some(callback) = &self.on_scroll {
                    (callback)(cx, x, y);
                }

                meta.consume();
            }
        })
    }
}

impl Handle<'_, List> {
    /// Sets the selected items of the list. Takes a signal of a list of indices.
    pub fn selected(self, selected: Signal<Vec<usize>>) -> Self {
        self.bind(selected, |handle, s| {
            let ss = s.get(&handle).clone();
            handle.modify2(|list, cx| {
                list.selected.upd(cx, |sel| {
                    sel.clear();
                    for idx in ss {
                        sel.insert(idx);
                    }
                });
                if let Some(&last) = list.selected.get(cx).iter().last() {
                    list.focused.set(cx, Some(last));
                }
            });
        })
    }

    /// Sets the callback triggered when a [ListItem] is selected.
    pub fn on_select<F>(self, callback: F) -> Self
    where
        F: 'static + Fn(&mut EventContext, usize),
    {
        self.modify(|list: &mut List| list.on_select = Some(Box::new(callback)))
    }

    /// Set the selectable state of the [List].
    pub fn selectable(self, selectable: Signal<Selectable>) -> Self {
        self.bind(selectable, |handle, selectable| {
            let s = *selectable.get(&handle);
            handle.modify2(|list: &mut List, cx| list.selectable.set(cx, s));
        })
    }

    /// Sets whether the selection should follow the focus.
    pub fn selection_follows_focus(self, flag: Signal<bool>) -> Self {
        self.bind(flag, |handle, selection_follows_focus| {
            let s = *selection_follows_focus.get(&handle);
            handle.modify(|list: &mut List| list.selection_follows_focus = s);
        })
    }

    /// Sets the orientation of the list.
    pub fn orientation(self, orientation: Signal<Orientation>) -> Self {
        self.bind(orientation, |handle, orientation| {
            let orientation = *orientation.get(&handle);
            handle.modify2(|list: &mut List, cx| {
                list.orientation.set(cx, orientation);
            });
        })
    }

    /// Sets whether the scrollbar should move to the cursor when pressed.
    pub fn scroll_to_cursor(self, flag: Signal<bool>) -> Self {
        self.bind(flag, |handle, flag| {
            let flag = *flag.get(&handle);
            handle.modify2(|list, cx| list.scroll_to_cursor.set(cx, flag));
        })
    }

    /// Sets a callback which will be called when a scrollview is scrolled, either with the mouse wheel, touchpad, or using the scroll bars.
    pub fn on_scroll(
        self,
        callback: impl Fn(&mut EventContext, f32, f32) + 'static + Send + Sync,
    ) -> Self {
        self.modify(|list: &mut List| list.on_scroll = Some(Box::new(callback)))
    }

    /// Set the horizontal scroll position of the [ScrollView]. Accepts a signal to an 'f32' between 0 and 1.
    pub fn scroll_x(self, scrollx: Signal<f32>) -> Self {
        self.bind(scrollx, |handle, scrollx| {
            let sx = *scrollx.get(&handle);
            handle.modify2(|list, cx| list.scroll_x.set(cx, sx));
        })
    }

    /// Set the vertical scroll position of the [ScrollView]. Accepts a signal to an 'f32' between 0 and 1.
    pub fn scroll_y(self, scrolly: Signal<f32>) -> Self {
        self.bind(scrolly, |handle, scrolly| {
            let sy = *scrolly.get(&handle);
            handle.modify2(|list, cx| list.scroll_y.set(cx, sy));
        })
    }

    /// Sets whether the horizontal scrollbar should be visible.
    pub fn show_horizontal_scrollbar(self, flag: Signal<bool>) -> Self {
        self.bind(flag, |handle, show_scrollbar| {
            let show_scrollbar = *show_scrollbar.get(&handle);
            handle.modify2(|list, cx| list.show_horizontal_scrollbar.set(cx, show_scrollbar));
        })
    }

    /// Sets whether the vertical scrollbar should be visible.
    pub fn show_vertical_scrollbar(self, flag: Signal<bool>) -> Self {
        self.bind(flag, |handle, show_scrollbar| {
            let show_scrollbar = *show_scrollbar.get(&handle);
            handle.modify2(|list, cx| list.show_vertical_scrollbar.set(cx, show_scrollbar));
        })
    }
}

/// A view which represents a selectable item within a list.
pub struct ListItem {}

impl ListItem {
    /// Create a new [ListItem] view.
    pub fn new<T: Clone + 'static>(
        cx: &mut Context,
        index: usize,
        item: impl Res<T> + 'static,
        selected: impl Res<BTreeSet<usize>> + 'static,
        focused: impl Res<Option<usize>> + 'static,
        item_content: impl 'static + Fn(&mut Context, usize, Signal<T>),
    ) -> Handle<Self> {
        let item = item.into_signal(cx);
        let selected = selected.into_signal(cx);
        let focused = focused.into_signal(cx);
        let is_focused = focused.get(cx).as_ref().is_some_and(|f| *f == index);
        let is_focused_signal = cx.derived({
            let focused = focused;
            move |store| focused.get(store).as_ref().is_some_and(|i| *i == index)
        });
        let is_selected_signal = cx.derived({
            let selected = selected;
            move |store| selected.get(store).contains(&index)
        });
        Self {}
            .build(cx, move |cx| {
                item_content(cx, index, item);
            })
            .role(Role::ListItem)
            .toggle_class("focused", is_focused_signal)
            .checked(is_selected_signal)
            .bind(focused, move |handle, foc| {
                let is_now_focused = foc.get(&handle).as_ref().is_some_and(|f| *f == index);
                if is_now_focused != is_focused {
                    handle.cx.emit(ScrollEvent::ScrollToView(handle.entity()));
                }
            })
            .on_press(move |cx| cx.emit(ListEvent::Select(index)))
    }
}

impl View for ListItem {
    fn element(&self) -> Option<&'static str> {
        Some("list-item")
    }
}
