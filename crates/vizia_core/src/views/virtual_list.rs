use std::ops::{Deref, Range};

use crate::prelude::*;

#[derive(Lens)]
pub struct VirtualList {
    num_items: usize,
    item_height: f32,
    visible_range: Range<usize>,
    scroll_y: f32,
    scroll_to_cursor: bool,
    on_change: Option<Box<dyn Fn(&mut EventContext, Range<usize>)>>,
}

pub enum VirtualListEvent {
    SetNumItems(usize),
    SetScrollY(f32),
}

impl VirtualList {
    pub fn new<V: View, L: Lens, T: 'static>(
        cx: &mut Context,
        list: L,
        item_height: f32,
        item_content: impl Fn(&mut Context, usize, Index<L, T>) -> Handle<V> + Copy + 'static,
    ) -> Handle<Self>
    where
        <L as Lens>::Target: Deref<Target = [T]>,
    {
        let num_items = list.map(|list| list.len());
        Self {
            num_items: num_items.get(cx),
            item_height,
            visible_range: 0..0,
            scroll_y: 0.0,
            scroll_to_cursor: true,
            on_change: None,
        }
        .build(cx, |cx| {
            ScrollView::new(cx, 0.0, 0.0, false, true, move |cx| {
                // The ScrollView contains a VStack which is sized to the total height
                // needed to fit all items. This ensures we have a correct scroll bar.
                VStack::new(cx, |cx| {
                    // Within the VStack we create a view for each visible item.
                    // This binding ensures the amount of views stay up to date.
                    let num_visible_items = Self::visible_range.map(Range::len);
                    Binding::new(cx, num_visible_items, move |cx, lens| {
                        for i in 0..lens.get(cx) {
                            // Each item of the range maps to an index into the backing list.
                            // As we scroll the index may change, representing an item going in/out of visibility.
                            // Wrap `item_content` in a binding to said index, so it rebuilds only when necessary.
                            let item_index = Self::visible_item_index(i);
                            HStack::new(cx, move |cx| {
                                Binding::new(cx, item_index, move |cx, lens| {
                                    let index = lens.get(cx);
                                    let item = list.index(index);
                                    item_content(cx, index, item).height(Percentage(100.0));
                                });
                            })
                            .height(Pixels(item_height))
                            .position_type(PositionType::SelfDirected)
                            .bind(item_index, move |handle, lens| {
                                let index = lens.get(&handle);
                                handle.top(Pixels(index as f32 * item_height));
                            });
                        }
                    });
                })
                .bind(num_items, move |handle, lens| {
                    let num_items = lens.get(&handle);
                    handle
                        .height(Pixels(num_items as f32 * item_height))
                        .context()
                        .emit(VirtualListEvent::SetNumItems(num_items));
                });
            })
            .scroll_to_cursor(true)
            .on_scroll(|cx, _, y| {
                cx.emit(VirtualListEvent::SetScrollY(y));
            });
        })
    }

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
        let scale_factor = cx.scale_factor();
        let visible_height = cx.cache.get_height(current) / scale_factor;

        let item_height = self.item_height;
        let total_height = item_height * (self.num_items as f32);

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

        if let Some(callback) = &self.on_change {
            (callback)(cx, self.visible_range.clone())
        }
    }
}

impl View for VirtualList {
    fn element(&self) -> Option<&'static str> {
        Some("virtual_list")
    }

    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|virtual_list_event, _| match virtual_list_event {
            VirtualListEvent::SetNumItems(num_items) => {
                if self.num_items != *num_items {
                    self.num_items = *num_items;
                    self.recalc(cx);
                    // Bit of a hack until scrollview is changed to take a lens for x/y
                    cx.emit_custom(
                        Event::new(ScrollEvent::SetY(0.0))
                            .propagate(Propagation::Subtree)
                            .origin(cx.current)
                            .target(cx.current),
                    );
                }
            }

            VirtualListEvent::SetScrollY(scroll_y) => {
                self.scroll_y = *scroll_y;
                self.recalc(cx);

                if let Some(callback) = &self.on_change {
                    (callback)(cx, self.visible_range.clone())
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

impl<'a> Handle<'a, VirtualList> {
    pub fn scroll_to_cursor(self, flag: bool) -> Self {
        self.modify(|virtual_list: &mut VirtualList| {
            virtual_list.scroll_to_cursor = flag;
        })
    }

    pub fn on_change(self, callback: impl Fn(&mut EventContext, Range<usize>) + 'static) -> Self {
        self.modify(|virtual_list| virtual_list.on_change = Some(Box::new(callback)))
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
