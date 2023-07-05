use crate::prelude::*;

#[derive(Lens)]
pub struct VirtualList {
    offset: usize,
    num_items: usize,
    item_height: f32,
    visible_items: Vec<usize>,
    scrolly: f32,
    scroll_to_cursor: bool,
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
        item: impl Fn(&mut Context, usize, Then<L, Index<Vec<T>, T>>) -> Handle<V> + 'static,
    ) -> Handle<Self>
    where
        L: Lens<Target = Vec<T>>,
        T: Data + 'static,
    {
        let num_items = list.get(cx).len();

        Self {
            offset: 0,
            num_items,
            item_height: height,
            visible_items: (0..10).collect::<Vec<_>>(),
            scrolly: 0.0,
            scroll_to_cursor: true,
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
                .height(Pixels(num_items as f32 * height));
            })
            .scroll_to_cursor(true)
            .on_scroll(|cx, _, y| {
                cx.emit(VirtualListEvent::SetScrollY(y));
            });
        })
    }
}

impl View for VirtualList {
    fn element(&self) -> Option<&'static str> {
        Some("virtual_list")
    }

    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|virtual_list_event, _| match virtual_list_event {
            VirtualListEvent::SetNumItems(num_items) => {
                self.visible_items.clear();
                for i in 0..*num_items {
                    self.visible_items.push(i);
                }
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

                self.visible_items.clear();
                for i in self.offset..(self.offset + num_items) {
                    self.visible_items.push(i);
                }
            }
        });

        event.map(|window_event, _| match window_event {
            WindowEvent::GeometryChanged(geo) => {
                if geo.contains(GeoChanged::WIDTH_CHANGED)
                    || geo.contains(GeoChanged::HEIGHT_CHANGED)
                {
                    let current = cx.current();
                    let dpi = cx.scale_factor();
                    let container_height = cx.cache.get_height(current) / dpi;
                    let num_items =
                        ((container_height + self.item_height) / self.item_height).ceil() as usize;

                    let total_height = self.num_items as f32 * self.item_height;
                    let offsety = ((total_height - container_height) * self.scrolly).round() * dpi;
                    self.offset = (offsety / self.item_height / dpi).ceil() as usize;
                    self.offset = self.offset.saturating_sub(1);

                    self.visible_items.clear();
                    for i in self.offset..(self.offset + num_items) {
                        self.visible_items.push(i);
                    }
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
}
