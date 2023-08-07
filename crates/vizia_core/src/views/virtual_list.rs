use std::ops::Range;

use crate::prelude::*;

#[derive(Lens)]
pub struct VirtualList {
    offset: usize,
    num_items: usize,
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
    pub fn new<V: View, L, T>(
        cx: &mut Context,
        list: L,
        height: f32,
        item: impl Fn(&mut Context, usize, Index<L, T>) -> Handle<V> + 'static,
    ) -> Handle<Self>
    where
        L: Lens,
        <L as Lens>::Target: std::ops::Deref<Target = [T]>,
        T: Data + 'static,
    {
        let num_items = list.map(|l| l.len()).get(cx);

        Self {
            offset: 0,
            num_items,
            item_height: height,
            visible_items: (0..10).collect::<Vec<_>>(),
            scrolly: 0.0,
            scroll_to_cursor: true,
            on_change: None,
        }
        .build(cx, |cx| {
            ScrollView::new(cx, 0.0, 0.0, false, true, move |cx| {
                VStack::new(cx, |cx| {
                    Binding::new(cx, VirtualList::visible_items, move |cx, visible_list| {
                        for i in visible_list.get(cx) {
                            let ptr = list.index(i);
                            (item)(cx, i, ptr)
                                .top(Pixels(i as f32 * height))
                                .height(Pixels(height))
                                .position_type(PositionType::SelfDirected);
                        }
                    });
                })
                .height(list.map(move |l| Pixels(l.len() as f32 * height)));
            })
            .scroll_to_cursor(true)
            .on_scroll(|cx, _, y| {
                cx.emit(VirtualListEvent::SetScrollY(y));
            });
        })
        .bind(list.map(|list| list.len()), |mut handle, len| {
            let len = len.get(&handle);
            handle.context().emit(VirtualListEvent::SetNumItems(len));
        })
    }

    fn recalc(&mut self, cx: &mut EventContext) {
        let current = cx.current();
        let dpi = cx.scale_factor();
        let container_height = cx.cache.get_height(current) / dpi;
        let num_items = ((container_height + self.item_height) / self.item_height).ceil() as usize;

        let total_height = self.num_items as f32 * self.item_height;
        let offsety = ((total_height - container_height) * self.scrolly).round() * dpi;
        self.offset = (offsety / self.item_height / dpi).ceil() as usize;
        self.offset = self.offset.saturating_sub(1);

        let start = self.offset;
        let end = (self.offset + num_items).clamp(0, self.num_items);

        self.visible_items.clear();
        for i in start..end {
            self.visible_items.push(i);
        }

        if let Some(callback) = &self.on_change {
            (callback)(cx, start..end)
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
                // self.visible_items.clear();
                // for i in 0..*num_items {
                //     self.visible_items.push(i);
                // }
            }

            VirtualListEvent::SetScrollY(scrolly) => {
                self.scrolly = *scrolly;
                let current = cx.current();
                let dpi = cx.scale_factor();
                let container_height = cx.cache.get_height(current) / dpi;
                let total_height = self.num_items as f32 * self.item_height;
                let offsety = ((total_height - container_height) * *scrolly).round() * dpi;
                self.offset = (offsety / self.item_height / dpi).ceil() as usize;
                self.offset = self.offset.saturating_sub(1);

                let num_items =
                    ((container_height + self.item_height) / self.item_height).ceil() as usize;

                let start = self.offset;
                let end = (self.offset + num_items).clamp(0, self.num_items);
                self.visible_items.clear();
                for i in start..end {
                    self.visible_items.push(i);
                }

                if let Some(callback) = &self.on_change {
                    (callback)(cx, start..end)
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
