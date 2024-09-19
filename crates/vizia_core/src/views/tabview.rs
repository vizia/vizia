use crate::prelude::*;

pub enum TabEvent {
    SetSelected(usize),
}

#[derive(Lens)]
pub struct TabView {
    selected_index: usize,
    is_vertical: bool,

    #[lens(ignore)]
    on_select: Option<Box<dyn Fn(&mut EventContext, usize)>>,
}

impl TabView {
    pub fn new<L, T, F>(cx: &mut Context, lens: L, content: F) -> Handle<Self>
    where
        L: Lens<Target: std::ops::Deref<Target = [T]>>,
        T: Clone + 'static,
        F: 'static + Clone + Fn(&mut Context, Index<L, T>) -> TabPair,
    {
        Self { selected_index: 0, is_vertical: false, on_select: None }
            .build(cx, move |cx| {
                let content2 = content.clone();
                // Tab headers
                ScrollView::new(cx, 0.0, 0.0, true, true, move |cx| {
                    //VStack::new(cx, move |cx| {
                    Binding::new(cx, lens.map(|list| list.len()), move |cx, list_length| {
                        let list_length = list_length.get(cx);
                        for index in 0..list_length {
                            let l = lens.idx(index);
                            let builder = (content2)(cx, l).header;
                            TabHeader::new(cx, index, builder)
                                .bind(TabView::selected_index, move |handle, selected_index| {
                                    let selected_index = selected_index.get(handle.cx);
                                    handle.checked(selected_index == index);
                                })
                                .toggle_class("vertical", TabView::is_vertical);
                        }
                    })
                    //})
                    //.toggle_class("vertical", TabView::is_vertical)
                    //.class("tabview-tabheader-wrapper");
                })
                .class("tabview-header")
                .z_index(1)
                .toggle_class("vertical", TabView::is_vertical);

                Divider::new(cx).toggle_class("vertical", TabView::is_vertical);

                // Tab content
                VStack::new(cx, |cx| {
                    Binding::new(cx, TabView::selected_index, move |cx, selected| {
                        let selected = selected.get(cx);
                        let l = lens.idx(selected);
                        ((content)(cx, l).content)(cx);
                    });
                })
                .overflow(Overflow::Hidden)
                .class("tabview-content-wrapper");
            })
            .toggle_class("vertical", TabView::is_vertical)
    }
}

impl View for TabView {
    fn element(&self) -> Option<&'static str> {
        Some("tabview")
    }

    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|tab_event, meta| match tab_event {
            TabEvent::SetSelected(index) => {
                self.selected_index = *index;
                if let Some(callback) = &self.on_select {
                    (callback)(cx, self.selected_index);
                }
                meta.consume();
            }
        });
    }
}

impl Handle<'_, TabView> {
    pub fn vertical(self) -> Self {
        self.modify(|tabview: &mut TabView| tabview.is_vertical = true)
    }

    pub fn on_select(self, callback: impl Fn(&mut EventContext, usize) + 'static) -> Self {
        self.modify(|tabview: &mut TabView| tabview.on_select = Some(Box::new(callback)))
    }

    pub fn with_selected<U: Into<usize>>(mut self, selected: impl Res<U>) -> Self {
        let entity = self.entity();
        selected.set_or_bind(self.context(), entity, |cx, selected| {
            let index = selected.get(cx).into();
            cx.emit(TabEvent::SetSelected(index));
        });

        self
    }
}

pub struct TabPair {
    pub header: Box<dyn Fn(&mut Context)>,
    pub content: Box<dyn Fn(&mut Context)>,
}

impl TabPair {
    pub fn new<H, C>(header: H, content: C) -> Self
    where
        H: 'static + Fn(&mut Context),
        C: 'static + Fn(&mut Context),
    {
        Self { header: Box::new(header), content: Box::new(content) }
    }
}

pub struct TabHeader {
    index: usize,
}

impl TabHeader {
    pub fn new<F>(cx: &mut Context, index: usize, content: F) -> Handle<Self>
    where
        F: 'static + Fn(&mut Context),
    {
        Self { index }.build(cx, |cx| (content)(cx))
    }
}

impl View for TabHeader {
    fn element(&self) -> Option<&'static str> {
        Some("tabheader")
    }

    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|window_event, _meta| match window_event {
            WindowEvent::PressDown { mouse: _ } => {
                cx.emit(TabEvent::SetSelected(self.index));
            }

            _ => {}
        });
    }
}
