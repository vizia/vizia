use std::{collections::BTreeSet, ops::Deref, rc::Rc};
use vizia_reactive::{Scope, SignalGet, UpdaterEffect};

use crate::prelude::*;
use crate::{binding::BindingHandler, context::SIGNAL_REBUILDS};

/// Represents how items can be selected in a list.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum Selectable {
    #[default]
    /// Items in the list cannot be selected.
    None,
    /// A single item in the list can be selected.
    Single,
    /// Multiple items in the list can be selected simultaneously.
    Multi,
}

impl_res_simple!(Selectable);

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

/// A view for creating a list of items from an iterable signal.
pub struct List {
    /// The number of items in the list.
    num_items: usize,
    /// The set of selected items in the list.
    selected: Signal<BTreeSet<usize>>,
    /// Whether the list items are selectable.
    selectable: Signal<Selectable>,
    /// The index of the currently focused item in the list.
    focused: Signal<Option<usize>>,
    /// Whether the selection should follow the focus.
    selection_follows_focus: Signal<bool>,
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

/// A binding handler that manages list item entities for a [List].
///
/// The user provides `Vec<Signal<T>>` — each item has its own signal.
/// Value changes propagate through the reactive system automatically (zero cost).
/// This handler only reacts to structural changes (add/remove/reorder) by
/// diffing signal identities and rebuilding from the first changed position.
struct ListItemsBinding<T: 'static> {
    entity: Entity,
    list_entity: Entity,
    get_fn: Box<dyn Fn() -> Vec<Signal<T>>>,
    item_content: Rc<dyn Fn(&mut Context, usize, Signal<T>)>,
    selected: Signal<BTreeSet<usize>>,
    focused: Signal<Option<usize>>,
    /// Signal IDs from the previous update, used for identity diffing.
    item_signals: Vec<Signal<T>>,
    /// Entity IDs of the ListItem views.
    item_entities: Vec<Entity>,
    scope: Scope,
}

impl<T: 'static> ListItemsBinding<T> {
    fn create<S, V>(
        cx: &mut Context,
        list_entity: Entity,
        list: S,
        selected: Signal<BTreeSet<usize>>,
        focused: Signal<Option<usize>>,
        item_content: Rc<dyn Fn(&mut Context, usize, Signal<T>)>,
    ) where
        S: SignalGet<V> + Copy + 'static,
        V: Deref<Target = [Signal<T>]> + Clone + 'static,
    {
        let entity = cx.entity_manager.create();
        cx.tree.add(entity, cx.current()).expect("Failed to add to tree");
        cx.tree.set_ignored(entity, true);

        let scope = Scope::new();
        let initial_signals: Vec<Signal<T>> = scope.enter(|| {
            UpdaterEffect::new(
                move || list.get().deref().to_vec(),
                move |_new_value| {
                    SIGNAL_REBUILDS.with_borrow_mut(|set| {
                        set.insert(entity);
                    });
                },
            )
        });

        let mut binding = Self {
            entity,
            list_entity,
            get_fn: Box::new(move || list.get_untracked().deref().to_vec()),
            item_content,
            selected,
            focused,
            item_signals: Vec::new(),
            item_entities: Vec::new(),
            scope,
        };

        // Build initial items.
        for (index, signal) in initial_signals.iter().copied().enumerate() {
            let entity = binding.create_item_entity(cx, index, signal);
            binding.item_signals.push(signal);
            binding.item_entities.push(entity);
        }
        binding.update_list_metadata(cx, initial_signals.len());

        cx.bindings.insert(entity, Box::new(binding));

        let _: Handle<Self> =
            Handle { current: entity, entity, p: Default::default(), cx }.ignore();
    }

    fn update_list_metadata(&self, cx: &mut Context, len: usize) {
        if let Some(view) = cx.views.get_mut(&self.list_entity) {
            if let Some(list) = view.downcast_mut::<List>() {
                list.num_items = len;

                list.selected.update(|selected| {
                    selected.retain(|index| *index < len);
                });

                list.focused.update(|focused| {
                    if focused.is_some_and(|index| index >= len) {
                        *focused = len.checked_sub(1);
                    }
                });
            }
        }
    }

    fn create_item_entity(&self, cx: &mut Context, index: usize, signal: Signal<T>) -> Entity {
        let mut created = Entity::null();
        let item_content = self.item_content.clone();
        let selected = self.selected;
        let focused = self.focused;

        cx.with_current(self.entity, |cx| {
            created = ListItem::new(cx, index, signal, selected, focused, {
                let item_content = item_content.clone();
                move |cx, index, item| (item_content)(cx, index, item)
            })
            .entity();
        });

        created
    }
}

impl<T: 'static> BindingHandler for ListItemsBinding<T> {
    fn update(&mut self, cx: &mut Context) {
        let new_signals = (self.get_fn)();
        let new_len = new_signals.len();

        // Find the first position where the signal identity differs.
        let first_diff = self
            .item_signals
            .iter()
            .zip(new_signals.iter())
            .position(|(old, new)| old != new)
            .unwrap_or(self.item_signals.len().min(new_len));

        // Remove all entities from first_diff onward.
        for entity in self.item_entities.drain(first_diff..) {
            cx.remove(entity);
        }
        self.item_signals.truncate(first_diff);

        // Rebuild from first_diff onward with the new signals.
        for (i, signal) in new_signals[first_diff..].iter().copied().enumerate() {
            let index = first_diff + i;
            let entity = self.create_item_entity(cx, index, signal);
            self.item_signals.push(signal);
            self.item_entities.push(entity);
        }

        self.update_list_metadata(cx, new_len);
    }

    fn remove(&self, _cx: &mut Context) {
        self.scope.dispose();
    }

    fn debug(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str("ListItemsBinding")
    }
}

impl List {
    /// Creates a new [List] view from a reactive list of signals.
    ///
    /// The user maintains a `Vec<Signal<T>>` wrapped in an outer signal.
    /// Item value changes propagate through individual signals with zero entity rebuilds.
    /// Structural changes (add/remove/reorder) are handled automatically via signal-identity diffing.
    pub fn new<S, V, T>(
        cx: &mut Context,
        list: S,
        item_content: impl 'static + Fn(&mut Context, usize, Signal<T>),
    ) -> Handle<Self>
    where
        S: SignalGet<V> + Copy + 'static,
        V: Deref<Target = [Signal<T>]> + Clone + 'static,
        T: 'static,
    {
        let content: Rc<dyn Fn(&mut Context, usize, Signal<T>)> = Rc::new(item_content);
        let selected = Signal::new(BTreeSet::default());
        let selectable = Signal::new(Selectable::None);
        let focused = Signal::new(None);
        let orientation = Signal::new(Orientation::Vertical);
        let scroll_to_cursor = Signal::new(false);
        let scroll_x = Signal::new(0.0);
        let scroll_y = Signal::new(0.0);
        let show_horizontal_scrollbar = Signal::new(true);
        let show_vertical_scrollbar = Signal::new(true);

        Self {
            num_items: list.get().len(),
            selected,
            selectable,
            focused,
            selection_follows_focus: Signal::new(false),
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
            let list_entity = cx.current();

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

            Binding::new(cx, orientation, move |cx| {
                let orientation = orientation.get();
                if orientation == Orientation::Horizontal {
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
                ListItemsBinding::create(cx, list_entity, list, selected, focused, content.clone());
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
        .toggle_class("selectable", selectable.map(|s| *s != Selectable::None))
        .toggle_class("horizontal", orientation.map(|o| *o == Orientation::Horizontal))
        .navigable(true)
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
                let selectable = self.selectable.get();
                let mut selected = self.selected.get();
                let mut focused = self.focused.get();
                match selectable {
                    Selectable::Single => {
                        if selected.contains(&index) {
                            selected.clear();
                            focused = None;
                        } else {
                            selected.clear();
                            selected.insert(index);
                            focused = Some(index);
                            if let Some(on_select) = &self.on_select {
                                on_select(cx, index);
                            }
                        }
                    }

                    Selectable::Multi => {
                        if selected.contains(&index) {
                            selected.remove(&index);
                            focused = None;
                        } else {
                            selected.insert(index);
                            focused = Some(index);
                            if let Some(on_select) = &self.on_select {
                                on_select(cx, index);
                            }
                        }
                    }

                    Selectable::None => {}
                }

                self.selected.set(selected);
                self.focused.set(focused);

                meta.consume();
            }

            ListEvent::SelectFocused => {
                if let Some(focused) = self.focused.get() {
                    cx.emit(ListEvent::Select(focused))
                }
                meta.consume();
            }

            ListEvent::ClearSelection => {
                self.selected.set(BTreeSet::default());
                meta.consume();
            }

            ListEvent::FocusNext => {
                let mut focused = self.focused.get();
                if let Some(f) = &mut focused {
                    if *f < self.num_items.saturating_sub(1) {
                        *f = f.saturating_add(1);
                        if self.selection_follows_focus.get() {
                            cx.emit(ListEvent::SelectFocused);
                        }
                    }
                } else {
                    focused = Some(0);
                    if self.selection_follows_focus.get() {
                        cx.emit(ListEvent::SelectFocused);
                    }
                }

                self.focused.set(focused);

                meta.consume();
            }

            ListEvent::FocusPrev => {
                let mut focused = self.focused.get();
                if let Some(f) = &mut focused {
                    if *f > 0 {
                        *f = f.saturating_sub(1);
                        if self.selection_follows_focus.get() {
                            cx.emit(ListEvent::SelectFocused);
                        }
                    }
                } else {
                    focused = Some(self.num_items.saturating_sub(1));
                    if self.selection_follows_focus.get() {
                        cx.emit(ListEvent::SelectFocused);
                    }
                }

                self.focused.set(focused);

                meta.consume();
            }

            ListEvent::Scroll(x, y) => {
                self.scroll_x.set(x);
                self.scroll_y.set(y);
                if let Some(callback) = &self.on_scroll {
                    (callback)(cx, x, y);
                }

                meta.consume();
            }
        })
    }
}

impl Handle<'_, List> {
    /// Sets the selected items of the list from signal of type indices.
    pub fn selected<R>(self, selected: impl Res<R> + 'static) -> Self
    where
        R: Deref<Target = [usize]> + Clone + 'static,
    {
        let selected = selected.to_signal(self.cx);
        self.bind(selected, move |handle| {
            let selected = selected.get();
            let ss = selected.deref().to_vec();
            handle.modify(|list| {
                let mut selected = BTreeSet::default();
                let mut focused = None;
                for idx in ss {
                    selected.insert(idx);
                    focused = Some(idx);
                }
                list.selected.set(selected);
                list.focused.set(focused);
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
    pub fn selectable<U: Into<Selectable> + Clone + 'static>(
        self,
        selectable: impl Res<U> + 'static,
    ) -> Self {
        let selectable = selectable.to_signal(self.cx);
        self.bind(selectable, move |handle| {
            let selectable = selectable.get();
            let s = selectable.into();
            handle.modify(|list: &mut List| {
                list.selectable.set(s);
            });
        })
    }

    /// Sets whether the selection should follow the focus.
    pub fn selection_follows_focus<U: Into<bool> + Clone + 'static>(
        self,
        flag: impl Res<U> + 'static,
    ) -> Self {
        let flag = flag.to_signal(self.cx);
        self.bind(flag, move |handle| {
            let selection_follows_focus = flag.get();
            let s = selection_follows_focus.into();
            handle.modify(|list: &mut List| list.selection_follows_focus.set(s));
        })
    }

    /// Sets the orientation of the list.
    pub fn orientation<U: Into<Orientation> + Clone + 'static>(
        self,
        orientation: impl Res<U> + 'static,
    ) -> Self {
        let orientation = orientation.to_signal(self.cx);
        self.bind(orientation, move |handle| {
            let orientation = orientation.get();
            let orientation = orientation.into();
            handle.modify(|list: &mut List| {
                list.orientation.set(orientation);
            });
        })
    }

    /// Sets whether the scrollbar should move to the cursor when pressed.
    pub fn scroll_to_cursor(self, flag: bool) -> Self {
        self.modify(|list| {
            list.scroll_to_cursor.set(flag);
        })
    }

    /// Sets a callback which will be called when a scrollview is scrolled, either with the mouse wheel, touchpad, or using the scroll bars.
    pub fn on_scroll(
        self,
        callback: impl Fn(&mut EventContext, f32, f32) + 'static + Send + Sync,
    ) -> Self {
        self.modify(|list: &mut List| list.on_scroll = Some(Box::new(callback)))
    }

    /// Set the horizontal scroll position of the [ScrollView]. Accepts a value or signal of type an `f32` between 0 and 1.
    pub fn scroll_x(self, scrollx: impl Res<f32> + 'static) -> Self {
        let scrollx = scrollx.to_signal(self.cx);
        self.bind(scrollx, move |handle| {
            let scrollx = scrollx.get();
            let sx = scrollx;
            handle.modify(|list| {
                list.scroll_x.set(sx);
            });
        })
    }

    /// Set the vertical scroll position of the [ScrollView]. Accepts a value or signal of type an `f32` between 0 and 1.
    pub fn scroll_y(self, scrollx: impl Res<f32> + 'static) -> Self {
        let scrollx = scrollx.to_signal(self.cx);
        self.bind(scrollx, move |handle| {
            let scrolly = scrollx.get();
            let sy = scrolly;
            handle.modify(|list| {
                list.scroll_y.set(sy);
            });
        })
    }

    /// Sets whether the horizontal scrollbar should be visible.
    pub fn show_horizontal_scrollbar(self, flag: impl Res<bool> + 'static) -> Self {
        let flag = flag.to_signal(self.cx);
        self.bind(flag, move |handle| {
            let show_scrollbar = flag.get();
            let s = show_scrollbar;
            handle.modify(|list| {
                list.show_horizontal_scrollbar.set(s);
            });
        })
    }

    /// Sets whether the vertical scrollbar should be visible.
    pub fn show_vertical_scrollbar(self, flag: impl Res<bool> + 'static) -> Self {
        let flag = flag.to_signal(self.cx);
        self.bind(flag, move |handle| {
            let show_scrollbar = flag.get();
            let s = show_scrollbar;
            handle.modify(|list| {
                list.show_vertical_scrollbar.set(s);
            });
        })
    }
}

/// A view which represents a selectable item within a list.
pub struct ListItem {}

impl ListItem {
    /// Create a new [ListItem] view.
    pub fn new<'a, T: 'static>(
        cx: &'a mut Context,
        index: usize,
        item: Signal<T>,
        selected: impl SignalMapExt<BTreeSet<usize>>,
        focused: impl SignalMapExt<Option<usize>>,
        item_content: impl 'static + Fn(&mut Context, usize, Signal<T>),
    ) -> Handle<'a, Self> {
        let is_focused =
            focused.map(move |focused| focused.as_ref().is_some_and(|f| *f == index)).get();
        let focused_signal =
            focused.map(move |focused| focused.as_ref().is_some_and(|f| *f == index));
        Self {}
            .build(cx, move |cx| {
                item_content(cx, index, item);
            })
            .role(Role::ListItem)
            .toggle_class("focused", focused_signal)
            .checked(selected.map(move |selected| selected.contains(&index)))
            .bind(focused_signal, move |handle| {
                let focused = focused_signal.get();
                if focused != is_focused {
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
