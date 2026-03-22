use std::{
    collections::BTreeSet,
    ops::{Deref, Range},
};

use crate::prelude::*;

/// A view for creating a list of items from an iterable signal. Rather than creating a view for each item, items are recycled in the list.
pub struct VirtualList {
    scroll_to_cursor: Signal<bool>,
    /// Callback that is called when the list is scrolled.
    on_scroll: Option<Box<dyn Fn(&mut EventContext, f32, f32) + Send + Sync>>,
    /// The number of items in the list.
    num_items: usize,
    /// The height of each item in the list.
    item_height: f32,
    visible_range: Signal<Range<usize>>,
    scroll_x: Signal<f32>,
    scroll_y: Signal<f32>,
    show_horizontal_scrollbar: Signal<bool>,
    show_vertical_scrollbar: Signal<bool>,
    selected: Signal<BTreeSet<usize>>,
    selectable: Signal<Selectable>,
    focused: Signal<Option<usize>>,
    /// Whether the selection should follow the focus.
    selection_follows_focus: bool,
    /// Callback that is called when an item is selected.
    on_select: Option<Box<dyn Fn(&mut EventContext, usize)>>,
}

impl VirtualList {
    fn evaluate_index(index: usize, start: usize, end: usize) -> usize {
        match end - start {
            0 => 0,
            len => start + (len - (start % len) + index) % len,
        }
    }

    fn recalc(&mut self, cx: &mut EventContext) {
        if self.num_items == 0 {
            self.visible_range.set(0..0);
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
        let visible_start = empty_height * self.scroll_y.get();
        let visible_end = visible_start + visible_items_height;

        // The indices of the first and last item of the visible area.
        let mut start_index = (visible_start / item_height).trunc() as usize;
        let mut end_index = 1 + (visible_end / item_height).trunc() as usize;

        // Ensure we always have (num_visible_items + 1) items when possible
        let desired_range_size = (num_visible_items as usize) + 1;
        end_index = end_index.min(self.num_items);

        let current_range_size = end_index.saturating_sub(start_index);

        if current_range_size < desired_range_size {
            match end_index == self.num_items {
                // Try to extend backwards if we're at the end of the list
                true => {
                    start_index =
                        start_index.saturating_sub(desired_range_size - current_range_size);
                }
                // Try to extend forwards if we have room
                false if end_index < self.num_items => {
                    end_index = (start_index + desired_range_size).min(self.num_items);
                }
                _ => {}
            }
        }

        self.visible_range.set(start_index..end_index);
    }
}

impl VirtualList {
    /// Creates a new [VirtualList] view from a reactive list source.
    pub fn new<V: View, S, R, T>(
        cx: &mut Context,
        list: S,
        item_height: f32,
        item_content: impl 'static + Copy + Fn(&mut Context, usize, T) -> Handle<V>,
    ) -> Handle<Self>
    where
        S: SignalGet<R> + Copy + 'static,
        R: Deref<Target = [T]> + Clone + 'static,
        T: Clone + 'static,
    {
        let visible_range = Signal::new(0..0);
        let scroll_to_cursor = Signal::new(true);
        let scroll_x = Signal::new(0.0);
        let scroll_y = Signal::new(0.0);
        let show_horizontal_scrollbar = Signal::new(false);
        let show_vertical_scrollbar = Signal::new(true);
        let selected = Signal::new(BTreeSet::default());
        let selectable = Signal::new(Selectable::None);
        let focused = Signal::new(None);
        let selectable_class = Signal::new(false);

        Self {
            scroll_to_cursor,
            on_scroll: None,
            num_items: list.get().len(),
            item_height,
            visible_range,
            scroll_x,
            scroll_y,
            show_horizontal_scrollbar,
            show_vertical_scrollbar,
            selected,
            selectable,
            focused,
            selection_follows_focus: false,
            on_select: None,
        }
        .build(cx, |cx| {
            let list_entity = cx.current();

            Binding::new(cx, selectable, move |_, selectable| {
                selectable_class.set(selectable != Selectable::None);
            });

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

            ScrollView::new(cx, move |cx| {
                Binding::new(cx, list, move |cx, list_values| {
                    let list_values = list_values.clone();
                    let num_items = list_values.len();

                    if let Some(view) = cx.views.get_mut(&list_entity) {
                        if let Some(list_view) = view.downcast_mut::<VirtualList>() {
                            list_view.num_items = num_items;
                        }
                    }

                    cx.emit(ScrollEvent::SetY(0.0));

                    VStack::new(cx, move |cx| {
                        Binding::new(cx, visible_range, move |cx, visible_range| {
                            for i in 0..visible_range.len().min(num_items) {
                                let index = VirtualList::evaluate_index(
                                    i,
                                    visible_range.start,
                                    visible_range.end,
                                );
                                let values = list_values.clone();

                                if index >= values.len() {
                                    continue;
                                }

                                let item = values[index].clone();

                                let is_focused = Signal::new(false);
                                let is_selected = Signal::new(false);

                                Binding::new(cx, focused, move |_, focused| {
                                    is_focused.set(focused.as_ref().is_some_and(|f| *f == index));
                                });

                                Binding::new(cx, selected, move |_, selected| {
                                    is_selected.set(selected.contains(&index));
                                });

                                item_content(cx, index, item)
                                    .height(Percentage(100.0))
                                    .min_width(Auto)
                                    .height(Pixels(item_height))
                                    .position_type(PositionType::Absolute)
                                    .top(Pixels(index as f32 * item_height))
                                    .toggle_class("focused", is_focused)
                                    .checked(is_selected)
                                    .on_press(move |cx| cx.emit(ListEvent::Select(index)));
                            }
                        });
                    })
                    .height(Pixels(num_items as f32 * item_height));
                });
            })
            .show_horizontal_scrollbar(show_horizontal_scrollbar)
            .show_vertical_scrollbar(show_vertical_scrollbar)
            .scroll_to_cursor(scroll_to_cursor)
            .scroll_x(scroll_x)
            .scroll_y(scroll_y)
            .on_scroll(|cx, x, y| {
                if y.is_finite() && x.is_finite() {
                    cx.emit(ListEvent::Scroll(x, y));
                }
            });
        })
        .toggle_class("selectable", selectable_class)
        .navigable(true)
        .role(Role::List)
    }
}

impl View for VirtualList {
    fn element(&self) -> Option<&'static str> {
        Some("virtual-list")
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

                if let Some(current_focused) = focused.as_mut() {
                    if *current_focused < self.num_items.saturating_sub(1) {
                        *current_focused = current_focused.saturating_add(1);
                        if self.selection_follows_focus {
                            cx.emit(ListEvent::SelectFocused);
                        }
                    }
                } else {
                    focused = Some(0);
                    if self.selection_follows_focus {
                        cx.emit(ListEvent::SelectFocused);
                    }
                }

                self.focused.set(focused);

                meta.consume();
            }

            ListEvent::FocusPrev => {
                let mut focused = self.focused.get();

                if let Some(current_focused) = focused.as_mut() {
                    if *current_focused > 0 {
                        *current_focused = current_focused.saturating_sub(1);
                        if self.selection_follows_focus {
                            cx.emit(ListEvent::SelectFocused);
                        }
                    }
                } else {
                    focused = Some(self.num_items.saturating_sub(1));
                    if self.selection_follows_focus {
                        cx.emit(ListEvent::SelectFocused);
                    }
                }

                self.focused.set(focused);

                meta.consume();
            }

            ListEvent::Scroll(x, y) => {
                self.scroll_x.set(x);
                self.scroll_y.set(y);

                self.recalc(cx);

                if let Some(callback) = &self.on_scroll {
                    (callback)(cx, x, y);
                }

                meta.consume();
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
    /// Sets the selected items of the list from signal of type indices.
    pub fn selected<R>(self, selected: impl Res<R>) -> Self
    where
        R: Deref<Target = [usize]>,
    {
        self.bind(selected, |handle, selected| {
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

    /// Sets the callback triggered when a virtual list item is selected.
    pub fn on_select<F>(self, callback: F) -> Self
    where
        F: 'static + Fn(&mut EventContext, usize),
    {
        self.modify(|list| list.on_select = Some(Box::new(callback)))
    }

    /// Set the selectable state of the [VirtualList].
    pub fn selectable<U: Into<Selectable>>(self, selectable: impl Res<U>) -> Self {
        self.bind(selectable, |handle, selectable| {
            let s = selectable.into();
            handle.modify(|list| list.selectable.set(s));
        })
    }

    /// Sets whether the selection should follow the focus.
    pub fn selection_follows_focus<U: Into<bool>>(self, flag: impl Res<U>) -> Self {
        self.bind(flag, |handle, selection_follows_focus| {
            let s = selection_follows_focus.into();
            handle.modify(|list| list.selection_follows_focus = s);
        })
    }

    /// Sets whether the scrollbar should move to the cursor when pressed.
    pub fn scroll_to_cursor(self, flag: bool) -> Self {
        self.modify(|virtual_list: &mut VirtualList| virtual_list.scroll_to_cursor.set(flag))
    }

    /// Sets a callback which will be called when a scrollview is scrolled, either with the mouse wheel, touchpad, or using the scroll bars.
    pub fn on_scroll(
        self,
        callback: impl Fn(&mut EventContext, f32, f32) + 'static + Send + Sync,
    ) -> Self {
        self.modify(|list| list.on_scroll = Some(Box::new(callback)))
    }

    /// Set the horizontal scroll position of the [VirtualList]. Accepts a value or signal of type an `f32` between 0 and 1.
    pub fn scroll_x(self, scrollx: impl Res<f32>) -> Self {
        self.bind(scrollx, |handle, scrollx| {
            let sx = scrollx;
            handle.modify(|list| list.scroll_x.set(sx));
        })
    }

    /// Set the vertical scroll position of the [VirtualList]. Accepts a value or signal of type an `f32` between 0 and 1.
    pub fn scroll_y(self, scrollx: impl Res<f32>) -> Self {
        self.bind(scrollx, |handle, scrolly| {
            let sy = scrolly;
            handle.modify(|list| list.scroll_y.set(sy));
        })
    }

    /// Sets whether the horizontal scrollbar should be visible.
    pub fn show_horizontal_scrollbar(self, flag: impl Res<bool>) -> Self {
        self.bind(flag, |handle, show_scrollbar| {
            let s = show_scrollbar;
            handle.modify(|list| list.show_horizontal_scrollbar.set(s));
        })
    }

    /// Sets whether the vertical scrollbar should be visible.
    pub fn show_vertical_scrollbar(self, flag: impl Res<bool>) -> Self {
        self.bind(flag, |handle, show_scrollbar| {
            let s = show_scrollbar;
            handle.modify(|list| list.show_vertical_scrollbar.set(s));
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
