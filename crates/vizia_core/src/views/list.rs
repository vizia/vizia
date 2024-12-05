use std::ops::Deref;

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
    SetFocus(Option<usize>),
    FocusNext(bool),
    FocusPrev(bool),
    ClearSelection,
}

/// A view for creating a list of items from a binding to a `Vec<T>`
#[derive(Lens)]
pub struct List {
    list_len: usize,
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
        let num_items = list.map(list_len);
        Self { list_len: num_items.get(cx) }
            .build(cx, move |cx| {
                Keymap::from(vec![
                    (
                        KeyChord::new(Modifiers::empty(), Code::ArrowDown),
                        KeymapEntry::new("Focus Next", |cx| cx.emit(ListEvent::FocusNext(false))),
                    ),
                    (
                        KeyChord::new(Modifiers::empty(), Code::ArrowUp),
                        KeymapEntry::new("Focus Previous", |cx| {
                            cx.emit(ListEvent::FocusPrev(false))
                        }),
                    ),
                    // (
                    //     KeyChord::new(Modifiers::empty(), Code::Space),
                    //     KeymapEntry::new((), |cx| cx.emit(ListEvent::SelectFocused)),
                    // ),
                    (
                        KeyChord::new(Modifiers::SHIFT, Code::ArrowDown),
                        KeymapEntry::new("Select Next", |cx| {
                            cx.emit(ListEvent::FocusNext(true));
                            // cx.emit(ListEvent::SelectFocused);
                        }),
                    ),
                    (
                        KeyChord::new(Modifiers::SHIFT, Code::ArrowUp),
                        KeymapEntry::new("Select Previous", |cx| {
                            cx.emit(ListEvent::FocusPrev(true));
                            // cx.emit(ListEvent::SelectFocused);
                        }),
                    ),
                    (
                        KeyChord::new(Modifiers::empty(), Code::Escape),
                        KeymapEntry::new("Clear Selection", |cx| {
                            cx.emit(ListEvent::ClearSelection)
                        }),
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
                        item_content(cx, index, item);
                    }
                });
                // });
            })
            .role(Role::List)
    }
}

impl View for List {
    fn element(&self) -> Option<&'static str> {
        Some("list")
    }
}
