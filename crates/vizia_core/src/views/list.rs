use std::ops::Deref;

use crate::binding::MapRef;
use crate::prelude::*;
use hashbrown::HashSet;
use vizia_input::Code;

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
    SetFocus(Option<usize>),
    FocusNext(bool),
    FocusPrev(bool),
    ClearSelection,
}

/// A view for creating a list of items from a binding to a `Vec<T>`
#[derive(Lens)]
pub struct List {
    last_selection: Option<usize>,
    selected: HashSet<usize>,
    selectable: Selectable,
    focused: Option<usize>,
    list_len: usize,
}

impl List {
    pub fn new<L: Lens, T: 'static>(
        cx: &mut Context,
        list: L,
        item_content: impl 'static + Copy + Fn(&mut Context, usize, MapRef<L, T>),
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
        item_content: impl 'static + Copy + Fn(&mut Context, usize, MapRef<L, T>),
    ) -> Handle<Self> {
        let num_items = list.map(list_len);
        Self {
            last_selection: None,
            selected: HashSet::new(),
            selectable: Selectable::None,
            focused: None,
            list_len: num_items.get(cx),
        }
        .build(cx, move |cx| {
            Keymap::from(vec![
                (
                    KeyChord::new(Modifiers::empty(), Code::ArrowDown),
                    KeymapEntry::new((), |cx| cx.emit(ListEvent::FocusNext(false))),
                ),
                (
                    KeyChord::new(Modifiers::empty(), Code::ArrowUp),
                    KeymapEntry::new((), |cx| cx.emit(ListEvent::FocusPrev(false))),
                ),
                // (
                //     KeyChord::new(Modifiers::empty(), Code::Space),
                //     KeymapEntry::new((), |cx| cx.emit(ListEvent::SelectFocused)),
                // ),
                (
                    KeyChord::new(Modifiers::SHIFT, Code::ArrowDown),
                    KeymapEntry::new((), |cx| {
                        cx.emit(ListEvent::FocusNext(true));
                        // cx.emit(ListEvent::SelectFocused);
                    }),
                ),
                (
                    KeyChord::new(Modifiers::SHIFT, Code::ArrowUp),
                    KeymapEntry::new((), |cx| {
                        cx.emit(ListEvent::FocusPrev(true));
                        // cx.emit(ListEvent::SelectFocused);
                    }),
                ),
                (
                    KeyChord::new(Modifiers::empty(), Code::Escape),
                    KeymapEntry::new((), |cx| cx.emit(ListEvent::ClearSelection)),
                ),
            ])
            //.label("List")
            .build(cx);

            ScrollView::new(cx, 0.0, 0.0, false, true, move |cx| {
                // Bind to the list data
                Binding::new(cx, num_items, move |cx, num_items| {
                    // If the number of list items is different to the number of children of the ListView
                    // then remove and rebuild all the children

                    for index in 0..num_items.get(cx) {
                        let item = list.map_ref(move |list| list_index(list, index));
                        ListItem::new(cx, index, |cx| {
                            item_content(cx, index, item);
                        })
                        // Set the checked state based on whether this item is selected
                        .checked(List::selected.map(move |selected| selected.contains(&index)))
                        .focused(Self::focused.map(move |nav| *nav == Some(index)));
                    }
                });
            });
        })
        .role(Role::List)
    }
}

impl View for List {
    fn element(&self) -> Option<&'static str> {
        Some("list")
    }

    fn event(&mut self, _cx: &mut EventContext, event: &mut Event) {
        event.map(|list_event, _| match list_event {
            ListEvent::Select(index) => match self.selectable {
                Selectable::Single => {
                    self.selected.clear();
                    self.selected.insert(*index);
                }

                Selectable::Multi => {
                    if !self.selected.remove(index) {
                        self.selected.insert(*index);
                    }

                    self.last_selection = Some(*index);
                }

                _ => {}
            },

            ListEvent::SelectFocused => {
                if let Some(focused) = self.focused {
                    match self.selectable {
                        Selectable::Single => {
                            self.selected.clear();
                            self.selected.insert(focused);
                        }

                        Selectable::Multi => {
                            if !self.selected.remove(&focused) {
                                self.selected.insert(focused);
                            }
                        }

                        _ => {}
                    }
                }
            }

            ListEvent::SetFocus(index) => {
                self.focused = *index;
            }

            ListEvent::FocusNext(select) => {
                if let Some(focused) = &mut self.focused {
                    if *focused == self.list_len - 1 {
                        *focused = self.list_len - 1;
                    } else {
                        let next_focus = focused.saturating_add(1);
                        // if *select {
                        match self.selectable {
                            Selectable::Single => {
                                *focused = next_focus;
                                self.selected.clear();
                                self.selected.insert(*focused);
                            }

                            Selectable::Multi => {
                                if let Some(current_selected) = self.last_selection {
                                    if *focused < current_selected {
                                        self.selected.remove(&*focused);
                                    }
                                    *focused = next_focus;
                                    if *focused > current_selected {
                                        if !self.selected.remove(&*focused) {
                                            self.selected.insert(*focused);
                                        }
                                    }
                                }
                            }

                            _ => {}
                        }
                        // }
                    }
                }
            }

            ListEvent::FocusPrev(select) => {
                if let Some(focused) = &mut self.focused {
                    if *focused != 0 {
                        // if *select {
                        let prev_focus = focused.saturating_sub(1);
                        match self.selectable {
                            Selectable::Single => {
                                *focused = prev_focus;
                                self.selected.clear();
                                self.selected.insert(*focused);
                            }

                            Selectable::Multi => {
                                if let Some(current_selected) = self.last_selection {
                                    if *focused > current_selected {
                                        self.selected.remove(&*focused);
                                    }
                                    *focused = prev_focus;
                                    if *focused < current_selected {
                                        if !self.selected.remove(&*focused) {
                                            self.selected.insert(*focused);
                                        }
                                    }
                                }
                            }

                            _ => {}
                        }
                    }
                    // }
                }
            }

            ListEvent::ClearSelection => {
                self.selected.clear();
            }
        });
    }
}

impl Handle<'_, List> {
    pub fn selectable(self, flag: impl Res<Selectable>) -> Self {
        self.bind(flag, |handle, flag| {
            let selectable = flag.get(&handle);
            handle.modify(|list: &mut List| list.selectable = selectable);
        })
    }
}

pub struct ListItem {
    index: usize,
}

impl ListItem {
    pub fn new<F>(cx: &mut Context, index: usize, content: F) -> Handle<Self>
    where
        F: FnOnce(&mut Context),
    {
        Self { index }
            .build(cx, |cx| {
                (content)(cx);
            })
            .navigable(true)
            // Set the selected item to this one if pressed
            .on_press(move |cx| {
                cx.emit(ListEvent::Select(index));
                cx.focus();
            })
            .role(Role::ListItem)
    }
}

impl View for ListItem {
    fn element(&self) -> Option<&'static str> {
        Some("list-item")
    }

    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|window_event, _| match window_event {
            WindowEvent::FocusIn => {
                cx.emit(ListEvent::SetFocus(Some(self.index)));
            }

            WindowEvent::FocusOut => {
                cx.emit(ListEvent::SetFocus(None));
            }

            _ => {}
        })
    }
}
