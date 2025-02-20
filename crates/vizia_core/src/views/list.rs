use std::{collections::BTreeSet, ops::Deref, rc::Rc};

use crate::prelude::*;

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

    Scroll(f32, f32),
}

/// A view for creating a list of items from a binding to an iteratable list.
#[derive(Lens)]
pub struct List {
    list_len: usize,
    selected: BTreeSet<usize>,
    selectable: Selectable,
    focused: Option<usize>,
    focus_visible: bool,
    selection_follows_focus: bool,
    horizontal: bool,
    on_select: Option<Box<dyn Fn(&mut EventContext, usize)>>,
    /// Callback called when the scrollview is scrolled.
    #[lens(ignore)]
    on_scroll: Option<Box<dyn Fn(&mut EventContext, f32, f32) + Send + Sync>>,
    scroll_x: f32,
    scroll_y: f32,
    /// Whether the horizontal scrollbar should be visible.
    pub show_horizontal_scrollbar: bool,
    /// Whether the vertical scrollbar should be visible.
    pub show_vertical_scrollbar: bool,
}

impl List {
    /// Creates a new [List] view.
    pub fn new<L: Lens, T: 'static>(
        cx: &mut Context,
        list: L,
        item_content: impl 'static + Fn(&mut Context, usize, MapRef<L, T>),
    ) -> Handle<Self>
    where
        L::Target: Deref<Target = [T]> + Data,
    {
        Self::new_generic(
            cx,
            list,
            |list| list.len(),
            |list, index| &list[index],
            |_| true,
            item_content,
        )
    }

    /// Creates a new [List] view with a provided filter closure.
    pub fn new_filtered<L: Lens, T: 'static>(
        cx: &mut Context,
        list: L,
        filter: impl 'static + Clone + FnMut(&&T) -> bool,
        item_content: impl 'static + Fn(&mut Context, usize, MapRef<L, T>),
    ) -> Handle<Self>
    where
        L::Target: Deref<Target = [T]> + Data,
    {
        let f = filter.clone();
        Self::new_generic(
            cx,
            list,
            move |list| list.iter().filter(filter.clone()).count(),
            move |list, index| &list[index],
            f,
            item_content,
        )
    }

    /// Creates a new [List] view with a binding to the given lens and a template for constructing the list items.
    pub fn new_generic<L: Lens, T: 'static>(
        cx: &mut Context,
        list: L,
        list_len: impl 'static + Fn(&L::Target) -> usize,
        list_index: impl 'static + Clone + Fn(&L::Target, usize) -> &T,
        filter: impl 'static + Clone + FnMut(&&T) -> bool,
        item_content: impl 'static + Fn(&mut Context, usize, MapRef<L, T>),
    ) -> Handle<Self>
    where
        L::Target: Deref<Target = [T]> + Data,
    {
        let content = Rc::new(item_content);
        let num_items = list.map(list_len);
        Self {
            list_len: num_items.get(cx),
            selected: BTreeSet::default(),
            selectable: Selectable::None,
            focused: None,
            focus_visible: false,
            selection_follows_focus: false,
            horizontal: false,
            on_select: None,
            on_scroll: None,
            scroll_x: 0.0,
            scroll_y: 0.0,
            show_horizontal_scrollbar: true,
            show_vertical_scrollbar: true,
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

            Binding::new(cx, List::horizontal, |cx, horizontal| {
                if horizontal.get(cx) {
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
                Binding::new(cx, num_items, move |cx, _| {
                    // If the number of list items is different to the number of children of the ListView
                    // then remove and rebuild all the children

                    let mut f = filter.clone();
                    let ll = list
                        .get(cx)
                        .iter()
                        .enumerate()
                        .filter(|(_, v)| f(v))
                        .map(|(idx, _)| idx)
                        .collect::<Vec<_>>();

                    for index in ll.into_iter() {
                        let ll = list_index.clone();
                        let item = list.map_ref(move |list| ll(list, index));
                        let content = content.clone();
                        ListItem::new(cx, index, item, move |cx, index, item| {
                            content(cx, index, item);
                        });
                    }
                });
            })
            .show_horizontal_scrollbar(Self::show_horizontal_scrollbar)
            .show_vertical_scrollbar(Self::show_vertical_scrollbar)
            .scroll_x(Self::scroll_x)
            .scroll_y(Self::scroll_y)
            .on_scroll(|cx, x, y| cx.emit(ListEvent::Scroll(x, y)));
        })
        .toggle_class("selectable", List::selectable.map(|s| *s != Selectable::None))
        .toggle_class("horizontal", List::horizontal)
        .navigable(true)
        .role(Role::List)
    }
}

impl View for List {
    fn element(&self) -> Option<&'static str> {
        Some("list")
    }

    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.take(|list_event, _| match list_event {
            ListEvent::Select(index) => {
                cx.focus();
                match self.selectable {
                    Selectable::Single => {
                        if self.selected.contains(&index) {
                            self.selected.clear();
                            self.focused = None;
                        } else {
                            self.selected.clear();
                            self.selected.insert(index);
                            self.focused = Some(index);
                            self.focus_visible = false;
                            if let Some(on_select) = &self.on_select {
                                on_select(cx, index);
                            }
                        }
                    }

                    Selectable::Multi => {
                        if self.selected.contains(&index) {
                            self.selected.remove(&index);
                            self.focused = None;
                        } else {
                            self.selected.insert(index);
                            self.focused = Some(index);
                            self.focus_visible = false;
                            if let Some(on_select) = &self.on_select {
                                on_select(cx, index);
                            }
                        }
                    }

                    Selectable::None => {}
                }
            }

            ListEvent::SelectFocused => {
                if let Some(focused) = &self.focused {
                    cx.emit(ListEvent::Select(*focused))
                }
            }

            ListEvent::ClearSelection => {
                self.selected.clear();
            }

            ListEvent::FocusNext => {
                if let Some(focused) = &mut self.focused {
                    *focused = focused.saturating_add(1);

                    if *focused >= self.list_len {
                        *focused = 0;
                    }
                } else {
                    self.focused = Some(0);
                }

                self.focus_visible = true;

                if self.selection_follows_focus {
                    cx.emit(ListEvent::SelectFocused);
                }
            }

            ListEvent::FocusPrev => {
                if let Some(focused) = &mut self.focused {
                    if *focused == 0 {
                        *focused = self.list_len;
                    }

                    *focused = focused.saturating_sub(1);
                } else {
                    self.focused = Some(self.list_len.saturating_sub(1));
                }

                self.focus_visible = true;

                if self.selection_follows_focus {
                    cx.emit(ListEvent::SelectFocused);
                }
            }

            ListEvent::Scroll(x, y) => {
                self.scroll_x = x;
                self.scroll_y = y;
                if let Some(callback) = &self.on_scroll {
                    (callback)(cx, x, y);
                }
            }
        })
    }
}

impl Handle<'_, List> {
    /// Sets the  selected items of the list. Takes a lens to a list of indices.
    pub fn selected<S: Lens>(self, selected: S) -> Self
    where
        S::Target: Deref<Target = [usize]> + Data,
    {
        self.bind(selected, |handle, s| {
            let ss = s.get(&handle).deref().to_vec();
            handle.modify(|list| {
                list.selected.clear();
                for idx in ss {
                    list.selected.insert(idx);
                    list.focused = Some(idx);
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
    pub fn selectable<U: Into<Selectable>>(self, selectable: impl Res<U>) -> Self {
        self.bind(selectable, |handle, selectable| {
            let s = selectable.get(&handle).into();
            handle.modify(|list: &mut List| list.selectable = s);
        })
    }

    /// Sets whether the selection should follow the focus.
    pub fn selection_follows_focus<U: Into<bool>>(self, flag: impl Res<U>) -> Self {
        self.bind(flag, |handle, selection_follows_focus| {
            let s = selection_follows_focus.get(&handle).into();
            handle.modify(|list: &mut List| list.selection_follows_focus = s);
        })
    }

    // todo: replace with orientation
    /// Sets the orientation of the list.
    pub fn horizontal<U: Into<bool>>(self, flag: impl Res<U>) -> Self {
        self.bind(flag, |handle, horizontal| {
            let s = horizontal.get(&handle).into();
            handle.modify(|list: &mut List| list.horizontal = s);
        })
    }

    /// Sets a callback which will be called when a scrollview is scrolled, either with the mouse wheel, touchpad, or using the scroll bars.
    pub fn on_scroll(
        self,
        callback: impl Fn(&mut EventContext, f32, f32) + 'static + Send + Sync,
    ) -> Self {
        self.modify(|list: &mut List| list.on_scroll = Some(Box::new(callback)))
    }

    /// Set the horizontal scroll position of the [ScrollView]. Accepts a value or lens to an 'f32' between 0 and 1.
    pub fn scroll_x(self, scrollx: impl Res<f32>) -> Self {
        self.bind(scrollx, |handle, scrollx| {
            let sx = scrollx.get(&handle);
            handle.modify(|list| list.scroll_x = sx);
        })
    }

    /// Set the vertical scroll position of the [ScrollView]. Accepts a value or lens to an 'f32' between 0 and 1.
    pub fn scroll_y(self, scrollx: impl Res<f32>) -> Self {
        self.bind(scrollx, |handle, scrolly| {
            let sy = scrolly.get(&handle);
            handle.modify(|list| list.scroll_y = sy);
        })
    }

    /// Sets whether the horizontal scrollbar should be visible.
    pub fn show_horizontal_scrollbar(self, flag: impl Res<bool>) -> Self {
        self.bind(flag, |handle, show_scrollbar| {
            let s = show_scrollbar.get(&handle);
            handle.modify(|list| list.show_horizontal_scrollbar = s);
        })
    }

    /// Sets whether the vertical scrollbar should be visible.
    pub fn show_vertical_scrollbar(self, flag: impl Res<bool>) -> Self {
        self.bind(flag, |handle, show_scrollbar| {
            let s = show_scrollbar.get(&handle);
            handle.modify(|list| list.show_vertical_scrollbar = s);
        })
    }
}

/// A view which represents a selectable item within a list.
pub struct ListItem {}

impl ListItem {
    /// Create a new [ListItem] view.
    pub fn new<L: Lens, T: 'static>(
        cx: &mut Context,
        index: usize,
        item: MapRef<L, T>,
        item_content: impl 'static + Fn(&mut Context, usize, MapRef<L, T>),
    ) -> Handle<Self> {
        Self {}
            .build(cx, move |cx| {
                item_content(cx, index, item);
            })
            .role(Role::ListItem)
            .checked(List::selected.map(move |selected| selected.contains(&index)))
            //.toggle_class("focused", List::focused.map(move |focused| *focused == Some(index)))
            // .focused_with_visibility(
            //     List::focused.map(move |f| *f == Some(index)),
            //     List::focus_visible,
            // )
            .on_press(move |cx| cx.emit(ListEvent::Select(index)))
    }
}

impl View for ListItem {
    fn element(&self) -> Option<&'static str> {
        Some("list-item")
    }
}
