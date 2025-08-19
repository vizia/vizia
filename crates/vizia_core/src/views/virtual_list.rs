use std::{
    collections::BTreeSet,
    ops::{Deref, Range},
};

use crate::prelude::*;

/// A view for creating a list of items from a binding to an iteratable list. Rather than creating a view for each item, items are recycled in the list.
#[derive(Lens)]
pub struct VirtualList {
    scroll_to_cursor: bool,
    on_change: Option<Box<dyn Fn(&mut EventContext, Range<usize>)>>,
    on_scroll: Option<Box<dyn Fn(&mut EventContext, f32, f32) + Send + Sync>>,
    num_items: usize,
    item_height: f32,
    visible_range: Range<usize>,
    scroll_x: f32,
    scroll_y: f32,
    selected: BTreeSet<usize>,
    selectable: Selectable,
    focused: Option<usize>,
    focus_visible: bool,
    selection_follows_focus: bool,
    on_select: Option<Box<dyn Fn(&mut EventContext, usize)>>,
}

pub(crate) enum VirtualListEvent {
    Scroll(f32, f32),
}

// #[derive(Lens)]
// struct VirtualListData {
//     num_items: usize,
//     item_height: f32,
//     visible_range: Range<usize>,
//     scroll_y: f32,
// }

impl VirtualList {
    fn evaluate_index(index: usize, start: usize, end: usize) -> usize {
        match end - start {
            0 => 0,
            len => start + (len - (start % len) + index) % len,
        }
    }

    fn visible_item_index(index: usize) -> impl Lens<Target = usize> {
        Self::visible_range.map(move |range| Self::evaluate_index(index, range.start, range.end))
    }

    fn recalc(&mut self, cx: &mut EventContext) {
        if self.num_items == 0 {
            self.visible_range = 0..0;
            return;
        }

        let current = cx.current();
        let current_height = cx.cache.get_height(current);
        if current_height == f32::MAX {
            return;
        }

        let item_height = self.item_height;
        let total_height = item_height * (self.num_items as f32);
        let visible_height = current_height / cx.scale_factor();

        let mut num_visible_items = (visible_height / item_height).ceil();
        num_visible_items += 1.0; // To account for partially-visible items.

        let visible_items_height = item_height * num_visible_items;
        let empty_height = (total_height - visible_items_height).max(0.0);

        // The pixel offsets within the container to the visible area.
        let visible_start = empty_height * self.scroll_y;
        let visible_end = visible_start + visible_items_height;

        // The indices of the first and last item of the visible area.
        let start_index = (visible_start / item_height).trunc() as usize;
        let end_index = 1 + (visible_end / item_height).trunc() as usize;

        self.visible_range = start_index..end_index.min(self.num_items);
    }
}

// impl Model for VirtualList {
//     fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
//         event.map(|virtual_list_event, _| match virtual_list_event {
//             VirtualListEvent::Scroll(x, y) => {
//                 self.scroll_y = *y;
//                 self.recalc(cx);
//             }
//         });

//         event.map(|window_event, _| match window_event {
//             WindowEvent::GeometryChanged(geo) => {
//                 if geo.intersects(GeoChanged::WIDTH_CHANGED | GeoChanged::HEIGHT_CHANGED) {
//                     self.recalc(cx);
//                 }
//             }

//             _ => {}
//         });
//     }
// }

impl VirtualList {
    /// Creates a new [VirtualList] view.
    pub fn new<V: View, L: Lens, T: 'static>(
        cx: &mut Context,
        list: L,
        item_height: f32,
        item_content: impl 'static + Copy + Fn(&mut Context, usize, MapRef<L, T>) -> Handle<V>,
    ) -> Handle<Self>
    where
        L::Target: Deref<Target = [T]>,
    {
        Self::new_generic(
            cx,
            list,
            |list| list.len(),
            |list, index| &list[index],
            item_height,
            item_content,
        )
    }

    /// Creates a new [VirtualList] view with a binding to the given lens and a template for constructing the list items.
    pub fn new_generic<V: View, L: Lens, T: 'static>(
        cx: &mut Context,
        list: L,
        list_len: impl 'static + Fn(&L::Target) -> usize,
        list_index: impl 'static + Copy + Fn(&L::Target, usize) -> &T,
        item_height: f32,
        item_content: impl 'static + Copy + Fn(&mut Context, usize, MapRef<L, T>) -> Handle<V>,
    ) -> Handle<Self> {
        let num_items = list.map(list_len);
        Self {
            scroll_to_cursor: true,
            on_change: None,
            on_scroll: None,
            num_items: num_items.get(cx),
            item_height,
            visible_range: 0..0,
            scroll_x: 0.0,
            scroll_y: 0.0,
            selected: BTreeSet::default(),
            selectable: Selectable::None,
            focused: None,
            focus_visible: false,
            selection_follows_focus: false,
            on_select: None,
        }
        .build(cx, |cx| {
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

            // Binding::new(cx, num_items, move |cx, lens| {
            //     let num_items = lens.get(cx);

            //     let mut data =
            //         VirtualListData { num_items, item_height, visible_range: 0..0, scroll_y: 0.0 };
            //     data.recalc(&mut EventContext::new_with_current(cx, vl));
            //     data.build(cx);
            // });

            ScrollView::new(cx, move |cx| {
                Binding::new(cx, num_items, move |cx, lens| {
                    let num_items = lens.get(cx);
                    cx.emit(ScrollEvent::SetY(0.0));
                    // The ScrollView contains a VStack which is sized to the total height
                    // needed to fit all items. This ensures we have a correct scroll bar.
                    VStack::new(cx, |cx| {
                        // Within the VStack we create a view for each visible item.
                        // This binding ensures the amount of views stay up to date.
                        let num_visible_items = VirtualList::visible_range.map(Range::len);
                        Binding::new(cx, num_visible_items, move |cx, lens| {
                            for i in 0..lens.get(cx).min(num_items) {
                                // Each item of the range maps to an index into the backing list.
                                // As we scroll the index may change, representing an item going in/out of visibility.
                                // Wrap `item_content` in a binding to said index, so it rebuilds only when necessary.
                                let item_index = VirtualList::visible_item_index(i);
                                Binding::new(cx, item_index, move |cx, lens| {
                                    let index = lens.get(cx);

                                    let item = list.map_ref(move |list| list_index(list, index));

                                    ListItem::new(
                                        cx,
                                        index,
                                        item,
                                        VirtualList::selected,
                                        move |cx, index, item| {
                                            item_content(cx, index, item).height(Percentage(100.0));
                                        },
                                    )
                                    .min_width(Auto)
                                    .height(Pixels(item_height))
                                    .position_type(PositionType::Absolute)
                                    .bind(
                                        item_index,
                                        move |handle, lens| {
                                            let index = lens.get(&handle);
                                            handle.top(Pixels(index as f32 * item_height));
                                        },
                                    );

                                    // HStack::new(cx, move |cx| {
                                    //     item_content(cx, index, item).height(Percentage(100.0));
                                    // })
                                    // .min_width(Auto)
                                    // .height(Pixels(item_height))
                                    // .position_type(PositionType::Absolute)
                                    // .bind(
                                    //     item_index,
                                    //     move |handle, lens| {
                                    //         let index = lens.get(&handle);
                                    //         handle.top(Pixels(index as f32 * item_height));
                                    //     },
                                    // );
                                });
                            }
                        })
                    })
                    .class("list-item")
                    .absolute_auto(true)
                    .min_width(Auto)
                    .height(Pixels(num_items as f32 * item_height));
                })
            })
            .scroll_x(Self::scroll_x)
            .scroll_y(Self::scroll_y)
            //.show_horizontal_scrollbar(true)
            .scroll_to_cursor(true)
            .on_scroll(|cx, x, y| {
                if y.is_finite() && x.is_finite() {
                    cx.emit(VirtualListEvent::Scroll(x, y));
                }
            });
        })
        .toggle_class("selectable", VirtualList::selectable.map(|s| *s != Selectable::None))
        .bind(num_items, |handle, num_items| {
            let ni = num_items.get(&handle);
            handle.modify(|list| list.num_items = ni);
        })
    }
}

impl View for VirtualList {
    fn element(&self) -> Option<&'static str> {
        Some("virtual-list")
    }

    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|virtual_list_event, _| match virtual_list_event {
            VirtualListEvent::Scroll(x, y) => {
                self.scroll_x = *x;
                self.scroll_y = *y;
                self.recalc(cx);
                if let Some(callback) = &self.on_scroll {
                    (callback)(cx, *x, *y);
                }
            }
        });

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

                    if *focused >= self.num_items {
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
                        *focused = self.num_items;
                    }

                    *focused = focused.saturating_sub(1);
                } else {
                    self.focused = Some(self.num_items.saturating_sub(1));
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
        });

        event.map(|window_event, _| match window_event {
            WindowEvent::GeometryChanged(geo) => {
                if geo.intersects(GeoChanged::WIDTH_CHANGED | GeoChanged::HEIGHT_CHANGED) {
                    self.recalc(cx);
                }
            }

            _ => {}
        });
    }
}

impl Handle<'_, VirtualList> {
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
        self.modify(|list| list.on_select = Some(Box::new(callback)))
    }

    /// Set the selectable state of the [List].
    pub fn selectable<U: Into<Selectable>>(self, selectable: impl Res<U>) -> Self {
        self.bind(selectable, |handle, selectable| {
            let s = selectable.get(&handle).into();
            handle.modify(|list| list.selectable = s);
        })
    }

    /// Sets whether the selection should follow the focus.
    pub fn selection_follows_focus<U: Into<bool>>(self, flag: impl Res<U>) -> Self {
        self.bind(flag, |handle, selection_follows_focus| {
            let s = selection_follows_focus.get(&handle).into();
            handle.modify(|list| list.selection_follows_focus = s);
        })
    }

    /// Sets whether the scrollbar should move to the cursor when pressed.
    pub fn scroll_to_cursor(self, flag: bool) -> Self {
        self.modify(|virtual_list: &mut VirtualList| {
            virtual_list.scroll_to_cursor = flag;
        })
    }

    /// Sets a callback which will be called when a scrollview is scrolled, either with the mouse wheel, touchpad, or using the scroll bars.
    pub fn on_scroll(
        self,
        callback: impl Fn(&mut EventContext, f32, f32) + 'static + Send + Sync,
    ) -> Self {
        self.modify(|list| list.on_scroll = Some(Box::new(callback)))
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
}

#[cfg(test)]
mod tests {
    use super::*;

    fn evaluate_indices(range: Range<usize>) -> Vec<usize> {
        (0..range.len())
            .map(|index| VirtualList::evaluate_index(index, range.start, range.end))
            .collect()
    }

    #[test]
    fn test_evaluate_index() {
        // Move forward by 0
        assert_eq!(evaluate_indices(0..4), [0, 1, 2, 3]);
        // Move forward by 1
        assert_eq!(evaluate_indices(1..5), [4, 1, 2, 3]);
        // Move forward by 2
        assert_eq!(evaluate_indices(2..6), [4, 5, 2, 3]);
        // Move forward by 3
        assert_eq!(evaluate_indices(3..7), [4, 5, 6, 3]);
        // Move forward by 4
        assert_eq!(evaluate_indices(4..8), [4, 5, 6, 7]);
        // Move forward by 5
        assert_eq!(evaluate_indices(5..9), [8, 5, 6, 7]);
        // Move forward by 6
        assert_eq!(evaluate_indices(6..10), [8, 9, 6, 7]);
        // Move forward by 7
        assert_eq!(evaluate_indices(7..11), [8, 9, 10, 7]);
        // Move forward by 8
        assert_eq!(evaluate_indices(8..12), [8, 9, 10, 11]);
        // Move forward by 9
        assert_eq!(evaluate_indices(9..13), [12, 9, 10, 11]);
    }
}
