use crate::prelude::*;

pub enum TabEvent {
    SetSelected(usize),
}

#[derive(Lens)]
pub struct TabView {
    selected_index: usize,

    #[lens(ignore)]
    on_select: Option<Box<dyn Fn(&mut EventContext, usize)>>,
}

impl TabView {
    pub fn new<L, T, F>(cx: &mut Context, lens: L, content: F) -> Handle<Self>
    where
        L: Lens,
        <L as Lens>::Target: std::ops::Deref<Target = [T]>,
        T: Clone + 'static,
        F: 'static + Clone + Fn(&mut Context, Index<L, T>) -> TabPair,
    {
        Self { selected_index: 0, on_select: None }.build(cx, move |cx| {
            let content2 = content.clone();
            // Tab headers
            VStack::new(cx, move |cx| {
                Binding::new(cx, lens.map(|list| list.len()), move |cx, list_length| {
                    let list_length = list_length.get_fallible(cx).map_or(0, |d| d);
                    for index in 0..list_length {
                        let l = lens.index(index);
                        let builder = (content2)(cx, l).header;
                        TabHeader::new(cx, index, builder).bind(
                            TabView::selected_index,
                            move |handle, selected_index| {
                                let selected_index = selected_index.get(handle.cx);
                                handle.checked(selected_index == index);
                            },
                        );
                    }
                })
            })
            .class("tabview-tabheader-wrapper");

            Element::new(cx).class("tabview-divider");

            // Tab content
            VStack::new(cx, |cx| {
                Binding::new(cx, TabView::selected_index, move |cx, selected| {
                    let selected = selected.get(cx);
                    let l = lens.index(selected);
                    ((content)(cx, l).content)(cx);
                });
            })
            .class("tabview-content-wrapper");
        })
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

impl<'a> Handle<'a, TabView> {
    pub fn on_select(self, callback: impl Fn(&mut EventContext, usize) + 'static) -> Self {
        self.modify(|tabview: &mut TabView| tabview.on_select = Some(Box::new(callback)))
    }

    pub fn with_selected<U: Into<usize>>(mut self, selected: impl Res<U>) -> Self {
        let entity = self.entity();
        selected.set_or_bind(self.context(), entity, |cx, selected| {
            let index = selected.into();
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
            WindowEvent::MouseDown(button) if *button == MouseButton::Left => {
                cx.emit(TabEvent::SetSelected(self.index));
            }

            _ => {}
        });
    }
}
