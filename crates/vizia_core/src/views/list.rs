use std::{collections::BTreeSet, ops::Deref, rc::Rc};
use vizia_reactive::{Scope, SignalGet, SignalWith, UpdaterEffect};

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
    selection: Signal<BTreeSet<usize>>,
    /// Whether the list items are selectable.
    selectable: Signal<Selectable>,
    /// The index of the currently focused item in the list.
    focused: Signal<Option<usize>>,
    /// Whether the selection should follow the focus.
    selection_follows_focus: Signal<bool>,
    /// Minimum number of selected items.
    min_selected: Signal<usize>,
    /// Maximum number of selected items.
    max_selected: Signal<usize>,
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
/// The user provides `Vec<T>` wrapped in an outer signal.
/// This handler creates internal signals for each item and maintains them.
/// Value changes to existing items update their internal signals (zero entity rebuilds).
/// Structural changes (add/remove/reorder) are handled by diffing values and rebuilding from the first changed position.
struct ListItemsBinding<T: 'static> {
    entity: Entity,
    list_entity: Entity,
    get_fn: Box<dyn Fn() -> Vec<T>>,
    item_content: Rc<dyn Fn(&mut Context, usize, Signal<T>)>,
    selection: Signal<BTreeSet<usize>>,
    focused: Signal<Option<usize>>,
    /// Internal signals for each list item.
    item_signals: Vec<Signal<T>>,
    /// Entity IDs of the ListItem views.
    item_entities: Vec<Entity>,
    /// Previous values, used for value-based diffing.
    prev_values: Vec<T>,
    scope: Scope,
}

impl<T: PartialEq + Clone + 'static> ListItemsBinding<T> {
    fn create<S, V>(
        cx: &mut Context,
        list_entity: Entity,
        list: S,
        selection: Signal<BTreeSet<usize>>,
        focused: Signal<Option<usize>>,
        item_content: Rc<dyn Fn(&mut Context, usize, Signal<T>)>,
    ) where
        S: SignalGet<V> + SignalWith<V> + Copy + 'static,
        V: Deref<Target = [T]> + Clone + 'static,
    {
        let entity = cx.entity_manager.create();
        cx.tree.add(entity, cx.current()).expect("Failed to add to tree");
        cx.tree.set_ignored(entity, true);

        let scope = Scope::new();
        let initial_values: Vec<T> = scope.enter(|| {
            UpdaterEffect::new(
                move || list.with(|list| list.deref().to_vec()),
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
            get_fn: Box::new(move || list.with_untracked(|list| list.deref().to_vec())),
            item_content,
            selection,
            focused,
            item_signals: Vec::new(),
            item_entities: Vec::new(),
            prev_values: Vec::new(),
            scope,
        };

        // Build initial items.
        for (index, value) in initial_values.iter().enumerate() {
            let signal = Signal::new(value.clone());
            let entity = binding.create_item_entity(cx, index, signal);
            binding.item_signals.push(signal);
            binding.item_entities.push(entity);
            binding.prev_values.push(value.clone());
        }
        binding.update_list_metadata(cx, initial_values.len());

        cx.bindings.insert(entity, Box::new(binding));

        let _: Handle<Self> =
            Handle { current: entity, entity, p: Default::default(), cx }.ignore();
    }

    fn update_list_metadata(&self, cx: &mut Context, len: usize) {
        if let Some(view) = cx.views.get_mut(&self.list_entity) {
            if let Some(list) = view.downcast_mut::<List>() {
                list.num_items = len;
                list.normalize_selection_state();
            }
        }
    }

    fn create_item_entity(&self, cx: &mut Context, index: usize, signal: Signal<T>) -> Entity {
        let mut created = Entity::null();
        let item_content = self.item_content.clone();
        let selection = self.selection;
        let focused = self.focused;

        cx.with_current(self.entity, |cx| {
            created = ListItem::new(cx, index, signal, selection, focused, {
                let item_content = item_content.clone();
                move |cx, index, item| (item_content)(cx, index, item)
            })
            .entity();
        });

        created
    }
}

impl<T: PartialEq + Clone + 'static> BindingHandler for ListItemsBinding<T> {
    fn update(&mut self, cx: &mut Context) {
        let new_values = (self.get_fn)();
        let new_len = new_values.len();

        // Find the first position where values differ.
        let first_diff = self
            .prev_values
            .iter()
            .zip(new_values.iter())
            .position(|(old, new)| old != new)
            .unwrap_or(self.prev_values.len().min(new_len));

        // Remove all entities from first_diff onward.
        for entity in self.item_entities.drain(first_diff..) {
            cx.remove(entity);
        }
        self.item_signals.truncate(first_diff);

        // Update existing signals or create new items from first_diff onward.
        for (i, value) in new_values[first_diff..].iter().enumerate() {
            let index = first_diff + i;
            if index < self.item_signals.len() {
                // Update existing signal
                self.item_signals[index].set(value.clone());
            } else {
                // Create new signal and item
                let signal = Signal::new(value.clone());
                let entity = self.create_item_entity(cx, index, signal);
                self.item_signals.push(signal);
                self.item_entities.push(entity);
            }
        }

        self.prev_values = new_values;
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
    fn selection_limits(&self) -> (usize, usize) {
        let mut min_selected = self.min_selected.get();
        let mut max_selected = self.max_selected.get();

        match self.selectable.get() {
            Selectable::None => {
                min_selected = 0;
                max_selected = 0;
            }

            Selectable::Single => {
                min_selected = min_selected.min(1);
                max_selected = 1;
            }

            Selectable::Multi => {}
        }

        max_selected = max_selected.min(self.num_items);
        min_selected = min_selected.min(max_selected);

        (min_selected, max_selected)
    }

    fn normalize_selection_state(&mut self) {
        let (min_selected, max_selected) = self.selection_limits();

        let mut selection = self.selection.get();
        selection.retain(|index| *index < self.num_items);

        while selection.len() > max_selected {
            if let Some(last) = selection.iter().next_back().copied() {
                selection.remove(&last);
            } else {
                break;
            }
        }

        if selection.len() < min_selected {
            for index in 0..self.num_items {
                selection.insert(index);
                if selection.len() >= min_selected {
                    break;
                }
            }
        }

        let mut focused = self.focused.get();
        if focused.is_some_and(|index| index >= self.num_items) {
            focused = self.num_items.checked_sub(1);
        }

        self.selection.set(selection);
        self.focused.set(focused);
    }

    /// Creates a new [List] view from a reactive or static list of values.
    ///
    /// `list` accepts any [`Res<V>`] source where `V` derefs to `[T]` — for example a
    /// `Signal<Vec<T>>` for a reactive list, or a plain `Vec<T>` for a static list.
    /// The list creates and manages internal signals for each item automatically.
    /// Value changes to existing items update their internal signals with zero entity rebuilds.
    /// Structural changes (add/remove/reorder) are handled by diffing values and rebuilding from the first changed position.
    pub fn new<S, V, T>(
        cx: &mut Context,
        list: S,
        item_content: impl 'static + Fn(&mut Context, usize, Signal<T>),
    ) -> Handle<Self>
    where
        S: Res<V> + 'static,
        V: Deref<Target = [T]> + Clone + 'static,
        T: PartialEq + Clone + 'static,
    {
        let content: Rc<dyn Fn(&mut Context, usize, Signal<T>)> = Rc::new(item_content);
        let selection = Signal::new(BTreeSet::default());
        let selectable = Signal::new(Selectable::None);
        let focused = Signal::new(None);
        let min_selected = Signal::new(0);
        let max_selected = Signal::new(usize::MAX);
        let orientation = Signal::new(Orientation::Vertical);
        let scroll_to_cursor = Signal::new(false);
        let scroll_x = Signal::new(0.0);
        let scroll_y = Signal::new(0.0);
        let show_horizontal_scrollbar = Signal::new(true);
        let show_vertical_scrollbar = Signal::new(true);

        Self {
            num_items: 0,
            selection,
            selectable,
            focused,
            selection_follows_focus: Signal::new(false),
            min_selected,
            max_selected,
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

            let list_signal = list.to_signal(cx);
            ScrollView::new(cx, move |cx| {
                ListItemsBinding::create(
                    cx,
                    list_entity,
                    list_signal,
                    selection,
                    focused,
                    content.clone(),
                );
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
        .orientation(orientation)
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
                let (min_selected, max_selected) = self.selection_limits();
                let mut selection = self.selection.get();
                let mut focused = self.focused.get();
                match selectable {
                    Selectable::Single => {
                        if selection.contains(&index) {
                            if min_selected == 0 {
                                selection.clear();
                                focused = None;
                            }
                        } else {
                            selection.clear();
                            selection.insert(index);
                            focused = Some(index);
                            if let Some(on_select) = &self.on_select {
                                on_select(cx, index);
                            }
                        }
                    }

                    Selectable::Multi => {
                        if selection.contains(&index) {
                            if selection.len() > min_selected {
                                selection.remove(&index);
                                if focused == Some(index) {
                                    focused = selection.iter().next_back().copied();
                                }
                            }
                        } else {
                            if selection.len() < max_selected {
                                selection.insert(index);
                                focused = Some(index);
                                if let Some(on_select) = &self.on_select {
                                    on_select(cx, index);
                                }
                            }
                        }
                    }

                    Selectable::None => {}
                }

                self.selection.set(selection);
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
                let (min_selected, _) = self.selection_limits();
                if min_selected == 0 {
                    self.selection.set(BTreeSet::default());
                }
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

/// Modifiers for changing the behavior and selection state of a [List].
pub trait ListModifiers: Sized {
    /// Sets the selected items of the list from signal of type indices.
    fn selection<R>(self, selection: impl Res<R> + 'static) -> Self
    where
        R: Deref<Target = [usize]> + Clone + 'static;

    /// Sets the callback triggered when a [ListItem] is selected.
    fn on_select<F>(self, callback: F) -> Self
    where
        F: 'static + Fn(&mut EventContext, usize);

    /// Set the selectable state of the [List].
    fn selectable<U: Into<Selectable> + Clone + 'static>(
        self,
        selectable: impl Res<U> + 'static,
    ) -> Self;

    /// Sets the minimum number of selected items.
    fn min_selected(self, min_selected: impl Res<usize> + 'static) -> Self;

    /// Sets the maximum number of selected items.
    fn max_selected(self, max_selected: impl Res<usize> + 'static) -> Self;

    /// Sets whether the selection should follow the focus.
    fn selection_follows_focus<U: Into<bool> + Clone + 'static>(
        self,
        flag: impl Res<U> + 'static,
    ) -> Self;

    /// Sets the orientation of the list.
    fn horizontal<U: Into<bool> + Clone + 'static>(self, horizontal: impl Res<U> + 'static)
    -> Self;

    /// Sets whether the scrollbar should move to the cursor when pressed.
    fn scroll_to_cursor(self, flag: bool) -> Self;

    /// Sets a callback which will be called when a scrollview is scrolled, either with the mouse wheel, touchpad, or using the scroll bars.
    fn on_scroll(
        self,
        callback: impl Fn(&mut EventContext, f32, f32) + 'static + Send + Sync,
    ) -> Self;

    /// Set the horizontal scroll position of the [ScrollView]. Accepts a value or signal of type an `f32` between 0 and 1.
    fn scroll_x(self, scrollx: impl Res<f32> + 'static) -> Self;

    /// Set the vertical scroll position of the [ScrollView]. Accepts a value or signal of type an `f32` between 0 and 1.
    fn scroll_y(self, scrollx: impl Res<f32> + 'static) -> Self;

    /// Sets whether the horizontal scrollbar should be visible.
    fn show_horizontal_scrollbar(self, flag: impl Res<bool> + 'static) -> Self;

    /// Sets whether the vertical scrollbar should be visible.
    fn show_vertical_scrollbar(self, flag: impl Res<bool> + 'static) -> Self;
}

impl ListModifiers for Handle<'_, List> {
    fn selection<R>(self, selection: impl Res<R> + 'static) -> Self
    where
        R: Deref<Target = [usize]> + Clone + 'static,
    {
        let selection = selection.to_signal(self.cx);
        self.bind(selection, move |handle| {
            selection.with(|selected_indices| {
                handle.modify(|list| {
                    let mut selection = BTreeSet::default();
                    let mut focused = None;
                    for idx in selected_indices.deref().iter().copied() {
                        selection.insert(idx);
                        focused = Some(idx);
                    }
                    list.selection.set(selection);
                    list.focused.set(focused);
                    list.normalize_selection_state();
                });
            });
        })
    }

    fn on_select<F>(self, callback: F) -> Self
    where
        F: 'static + Fn(&mut EventContext, usize),
    {
        self.modify(|list: &mut List| list.on_select = Some(Box::new(callback)))
    }

    fn selectable<U: Into<Selectable> + Clone + 'static>(
        self,
        selectable: impl Res<U> + 'static,
    ) -> Self {
        let selectable = selectable.to_signal(self.cx);
        self.bind(selectable, move |handle| {
            let selectable = selectable.get();
            let s = selectable.into();
            handle.modify(|list: &mut List| {
                list.selectable.set(s);
                list.normalize_selection_state();
            });
        })
    }

    fn min_selected(self, min_selected: impl Res<usize> + 'static) -> Self {
        let min_selected = min_selected.to_signal(self.cx);
        self.bind(min_selected, move |handle| {
            let min_selected = min_selected.get();
            handle.modify(|list: &mut List| {
                list.min_selected.set(min_selected);
                list.normalize_selection_state();
            });
        })
    }

    fn max_selected(self, max_selected: impl Res<usize> + 'static) -> Self {
        let max_selected = max_selected.to_signal(self.cx);
        self.bind(max_selected, move |handle| {
            let max_selected = max_selected.get();
            handle.modify(|list: &mut List| {
                list.max_selected.set(max_selected);
                list.normalize_selection_state();
            });
        })
    }

    fn selection_follows_focus<U: Into<bool> + Clone + 'static>(
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

    fn horizontal<U: Into<bool> + Clone + 'static>(
        self,
        horizontal: impl Res<U> + 'static,
    ) -> Self {
        let horizontal = horizontal.to_signal(self.cx);
        self.bind(horizontal, move |handle| {
            let horizontal = horizontal.get();
            let horizontal = horizontal.into();
            handle.modify(|list: &mut List| {
                list.orientation.set(if horizontal {
                    Orientation::Horizontal
                } else {
                    Orientation::Vertical
                });
            });
        })
    }

    fn scroll_to_cursor(self, flag: bool) -> Self {
        self.modify(|list| {
            list.scroll_to_cursor.set(flag);
        })
    }

    fn on_scroll(
        self,
        callback: impl Fn(&mut EventContext, f32, f32) + 'static + Send + Sync,
    ) -> Self {
        self.modify(|list: &mut List| list.on_scroll = Some(Box::new(callback)))
    }

    fn scroll_x(self, scrollx: impl Res<f32> + 'static) -> Self {
        let scrollx = scrollx.to_signal(self.cx);
        self.bind(scrollx, move |handle| {
            let scrollx = scrollx.get();
            let sx = scrollx;
            handle.modify(|list| {
                list.scroll_x.set(sx);
            });
        })
    }

    fn scroll_y(self, scrollx: impl Res<f32> + 'static) -> Self {
        let scrollx = scrollx.to_signal(self.cx);
        self.bind(scrollx, move |handle| {
            let scrolly = scrollx.get();
            let sy = scrolly;
            handle.modify(|list| {
                list.scroll_y.set(sy);
            });
        })
    }

    fn show_horizontal_scrollbar(self, flag: impl Res<bool> + 'static) -> Self {
        let flag = flag.to_signal(self.cx);
        self.bind(flag, move |handle| {
            let show_scrollbar = flag.get();
            let s = show_scrollbar;
            handle.modify(|list| {
                list.show_horizontal_scrollbar.set(s);
            });
        })
    }

    fn show_vertical_scrollbar(self, flag: impl Res<bool> + 'static) -> Self {
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
pub struct ListItem {
    selected: Memo<bool>,
}

impl ListItem {
    /// Create a new [ListItem] view.
    pub fn new<'a, T: Clone + 'static, M: SignalGet<T> + 'static>(
        cx: &'a mut Context,
        index: usize,
        item: M,
        selection: impl SignalMap<BTreeSet<usize>> + SignalGet<BTreeSet<usize>>,
        focused: impl SignalMap<Option<usize>>,
        item_content: impl 'static + Fn(&mut Context, usize, M),
    ) -> Handle<'a, Self> {
        let is_focused =
            focused.map(move |focused| focused.as_ref().is_some_and(|f| *f == index)).get();
        let focused_signal =
            focused.map(move |focused| focused.as_ref().is_some_and(|f| *f == index));
        let is_selected = selection.map(move |selection| selection.contains(&index));
        Self { selected: is_selected }
            .build(cx, move |cx| {
                item_content(cx, index, item);
            })
            .role(Role::ListItem)
            .toggle_class("focused", focused_signal)
            .checked(selection.map(move |selection| selection.contains(&index)))
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

    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|window_event, _| match window_event {
            WindowEvent::GeometryChanged(geo) => {
                if self.selected.get() && geo.contains(GeoChanged::HEIGHT_CHANGED) {
                    cx.emit(ScrollEvent::ScrollToView(cx.current()));
                }
            }

            _ => {}
        });
    }
}
