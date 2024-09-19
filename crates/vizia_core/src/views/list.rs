use std::{collections::BTreeSet, ops::Deref, rc::Rc};

use crate::prelude::*;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum Selectable {
    #[default]
    None,
    Single,
    Multi,
}

impl_res_simple!(Selectable);

pub enum ListEvent {
    Select(usize),
    SelectFocused,
    SelectNext,
    SelectPrev,
    FocusNext,
    FocusPrev,
    ClearSelection,
}

/// A view for creating a list of items from a binding to a `Vec<T>`
#[derive(Lens)]
pub struct List {
    list_len: usize,
    selected: BTreeSet<usize>,
    selectable: Selectable,
    focused: Option<usize>,
    on_select: Option<Box<dyn Fn(&mut EventContext, usize)>>,
}

impl List {
    pub fn new<L: Lens, T: 'static>(
        cx: &mut Context,
        list: L,
        item_content: impl 'static + Fn(&mut Context, usize, MapRef<L, T>),
    ) -> Handle<Self>
    where
        L::Target: Deref<Target = [T]>,
    {
        Self::new_generic(cx, list, |list| list.len(), |list, index| &list[index], item_content)
    }

    /// Creates a new List view with a binding to the given lens and a template for constructing the list items
    pub fn new_generic<L: Lens, T: 'static>(
        cx: &mut Context,
        list: L,
        list_len: impl 'static + Fn(&L::Target) -> usize,
        list_index: impl 'static + Copy + Fn(&L::Target, usize) -> &T,
        item_content: impl 'static + Fn(&mut Context, usize, MapRef<L, T>),
    ) -> Handle<Self> {
        let content = Rc::new(item_content);
        let num_items = list.map(list_len);
        Self {
            list_len: num_items.get(cx),
            selected: BTreeSet::default(),
            selectable: Selectable::Single,
            focused: None,
            on_select: None,
        }
        .build(cx, move |cx| {
            Keymap::from(vec![
                (
                    KeyChord::new(Modifiers::empty(), Code::ArrowDown),
                    KeymapEntry::new("Select Next", |cx| cx.emit(ListEvent::FocusNext)),
                ),
                (
                    KeyChord::new(Modifiers::empty(), Code::ArrowUp),
                    KeymapEntry::new("Select Previous", |cx| cx.emit(ListEvent::FocusPrev)),
                ),
                (
                    KeyChord::new(Modifiers::empty(), Code::Escape),
                    KeymapEntry::new("Clear Selection", |cx| cx.emit(ListEvent::ClearSelection)),
                ),
                (
                    KeyChord::new(Modifiers::empty(), Code::Enter),
                    KeymapEntry::new("Select Focused", |cx| cx.emit(ListEvent::SelectFocused)),
                ),
            ])
            .build(cx);

            // ScrollView::new(cx, 0.0, 0.0, false, true, move |cx| {
            // Bind to the list data
            Binding::new(cx, num_items, move |cx, num_items| {
                // If the number of list items is different to the number of children of the ListView
                // then remove and rebuild all the children

                for index in 0..num_items.get(cx) {
                    let item = list.map_ref(move |list| list_index(list, index));
                    let content = content.clone();
                    ListItem::new(cx, index, item, move |cx, index, item| {
                        content(cx, index, item);
                    });
                    // item_content(cx, index, item);
                }
            });
            // });
        })
        .width(Stretch(1.0))
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
                        self.selected.clear();
                        self.selected.insert(index);
                        self.focused = Some(index);
                        if let Some(on_select) = &self.on_select {
                            on_select(cx, index);
                        }
                    }

                    Selectable::Multi => {
                        self.selected.insert(index);
                        self.focused = Some(index);
                        if let Some(on_select) = &self.on_select {
                            on_select(cx, index);
                        }
                    }

                    Selectable::None => {}
                }
            }
            ListEvent::SelectFocused => {
                if let Some(focused) = &self.focused {
                    println!("{}", focused);
                    cx.emit(ListEvent::Select(*focused))
                }
            }
            ListEvent::SelectNext => {
                if self.selected.is_empty() {
                    self.selected.insert(0);
                } else if let Some(last) = self.selected.last().copied() {
                    self.selected.clear();
                    self.selected.insert((last + 1).min(self.list_len - 1));
                }
            }
            ListEvent::SelectPrev => {
                if let Some(first) = self.selected.first().copied() {
                    self.selected.clear();
                    self.selected
                        .insert(first.saturating_sub(1).min(self.list_len.saturating_sub(1)));
                }
            }

            ListEvent::ClearSelection => {
                self.selected.clear();
            }
            ListEvent::FocusNext => {
                if let Some(focused) = &mut self.focused {
                    *focused = focused.saturating_add(1).min(self.list_len.saturating_sub(1));
                } else {
                    self.focused = Some(0);
                }
            }
            ListEvent::FocusPrev => {
                if let Some(focused) = &mut self.focused {
                    *focused = focused.saturating_sub(1);
                } else {
                    self.focused = Some(self.list_len.saturating_sub(1));
                }
            }
        })
    }
}

impl<'v> Handle<'v, List> {
    pub fn selected<S: Lens>(self, selected: S) -> Self
    where
        S::Target: Deref<Target = [usize]> + Data,
    {
        self.bind(selected, |handle, s| {
            let ss = s.get(&handle).deref().to_vec();
            handle.modify(|list| list.selected.extend(ss.iter()));
        })
    }

    pub fn on_select<F>(self, callback: F) -> Self
    where
        F: 'static + Fn(&mut EventContext, usize),
    {
        self.modify(|list: &mut List| list.on_select = Some(Box::new(callback)))
    }
}

pub struct ListItem {}

impl ListItem {
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
            .toggle_class("focused", List::focused.map(move |focused| *focused == Some(index)))
            .on_press(move |cx| cx.emit(ListEvent::Select(index)))
    }
}

impl View for ListItem {
    fn element(&self) -> Option<&'static str> {
        Some("list-item")
    }
}
