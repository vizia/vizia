use std::ops::{Deref, Range};

use crate::prelude::*;

#[derive(Lens)]
pub struct VirtualList {
    num_items: usize,
    item_offset: usize,
    item_height: f32,
    visible_items: Vec<usize>,
    scrolly: f32,
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
        Self {
            num_items: 0,
            item_offset: 0,
            item_height,
            visible_items: Vec::new(),
            scrolly: 0.0,
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
                    let num_visible_items = Self::visible_items.map(Vec::len);
                    Binding::new(cx, num_visible_items, move |cx, lens| {
                        for i in 0..lens.get(cx) {
                            // Each visible item is an index into the backing list.
                            // As we scroll the index my change, representing an item going in/out of visibility.
                            // Wrap `item_content`` in a binding to the index so it only rebuilds when necessary.
                            let item_index = Self::visible_items.index(i);
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
                                let item_index = lens.get(&handle);
                                handle.top(Pixels(item_index as f32 * item_height));
                            });
                        }
                    });
                })
                .bind(list.map(|list| list.len()), move |handle, lens| {
                    let num_items = lens.get(&handle);
                    handle
                        .height(Pixels(num_items as f32 * item_height))
                        .context()
                        .emit(VirtualListEvent::SetNumItems(num_items));
                });
            })
            .on_scroll(|cx, _, y| {
                cx.emit(VirtualListEvent::SetScrollY(y));
            });
        })
    }

    fn visible_range(&self) -> Range<usize> {
        let mut min = self.num_items;
        let mut max = 0;
        for item in &self.visible_items {
            min = min.min(*item);
            max = max.max(*item);
        }
        min..max
    }

    fn recalc(&mut self, cx: &mut EventContext) {
        let current = cx.current();
        let dpi = cx.scale_factor();
        let visible_height = cx.cache.get_height(current) / dpi;

        let item_height = self.item_height;
        let total_height = item_height * self.num_items as f32;

        let mut num_visible_items = (visible_height / item_height).ceil() as usize;
        num_visible_items += 1; // Plus one to support partially visible end-items.

        let offsety = ((total_height - visible_height) * self.scrolly).round() * dpi;
        self.item_offset = (offsety / item_height / dpi).ceil() as usize;
        self.item_offset = self.item_offset.saturating_sub(1);

        let mut min = self.item_offset;
        let mut max = (self.item_offset + num_visible_items).clamp(0, self.num_items);

        if self.visible_items.len() != num_visible_items {
            self.visible_items.clear();
            self.visible_items.extend(min..max);
            return;
        }

        for item in &mut self.visible_items {
            match *item {
                // If the front item has fallen off, swap in a new item from the back.
                i if (i < min) && (i < max) => {
                    max -= 1;
                    *item = max;
                }
                // If the back item has fallen off, swap in a new item from the front.
                i if (i >= min) && (i >= max) => {
                    *item = min;
                    min += 1;
                }
                _ => {}
            }
        }

        if let Some(callback) = &self.on_change {
            (callback)(cx, self.visible_range())
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

            VirtualListEvent::SetScrollY(scrolly) => {
                self.scrolly = *scrolly;
                self.recalc(cx);

                if let Some(callback) = &self.on_change {
                    (callback)(cx, self.visible_range())
                }
            }
        });

        event.map(|window_event, _| match window_event {
            WindowEvent::GeometryChanged(geo) => {
                if geo.contains(GeoChanged::WIDTH_CHANGED)
                    || geo.contains(GeoChanged::HEIGHT_CHANGED)
                {
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
