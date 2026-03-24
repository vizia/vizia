use std::ops::Deref;

use crate::prelude::*;

pub enum TabEvent {
    SetSelected(usize),
}

pub struct TabView {
    selected_index: Signal<usize>,
    is_vertical: Signal<bool>,
    on_select: Option<Box<dyn Fn(&mut EventContext, usize)>>,
}

impl TabView {
    pub fn new<S, V, T, F>(cx: &mut Context, list: S, content: F) -> Handle<Self>
    where
        S: SignalGet<V> + Copy + 'static,
        V: Deref<Target = [T]> + Clone + 'static,
        T: Clone + 'static,
        F: 'static + Clone + Fn(&mut Context, usize, T) -> TabPair,
    {
        let selected_index = Signal::new(0usize);
        let is_vertical = Signal::new(false);

        Self { selected_index, is_vertical, on_select: None }
            .build(cx, move |cx| {
                let content_for_headers = content.clone();

                ScrollView::new(cx, move |cx| {
                    Binding::new(cx, list, move |cx| {
                        let list_values = list.get();

                        for (index, item) in list_values.iter().cloned().enumerate() {
                            let builder = (content_for_headers)(cx, index, item).header;
                            let is_selected = Signal::new(false);

                            Binding::new(cx, selected_index, move |_cx| {
                                let selected_index = selected_index.get();
                                is_selected.set(selected_index == index);
                            });

                            TabHeader::new(cx, index, builder)
                                .checked(is_selected)
                                .toggle_class("vertical", is_vertical);
                        }
                    });
                })
                .class("tabview-header")
                .z_index(1)
                .toggle_class("vertical", is_vertical);

                Divider::new(cx).toggle_class("vertical", is_vertical);

                VStack::new(cx, move |cx| {
                    Binding::new(cx, list, move |cx| {
                        let list_values = list.get();
                        let content = content.clone();
                        Binding::new(cx, selected_index, move |cx| {
                            let selected = selected_index.get();
                            if let Some(item) = list_values.get(selected).cloned() {
                                ((content)(cx, selected, item).content)(cx);
                            }
                        });
                    });
                })
                .overflow(Overflow::Hidden)
                .class("tabview-content-wrapper");
            })
            .toggle_class("vertical", is_vertical)
    }
}

impl View for TabView {
    fn element(&self) -> Option<&'static str> {
        Some("tabview")
    }

    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|tab_event, meta| match tab_event {
            TabEvent::SetSelected(index) => {
                self.selected_index.set(*index);
                if let Some(callback) = &self.on_select {
                    (callback)(cx, *index);
                }
                meta.consume();
            }
        });
    }
}

impl Handle<'_, TabView> {
    pub fn vertical(self) -> Self {
        self.modify(|tabview: &mut TabView| {
            tabview.is_vertical.set(true);
        })
    }

    pub fn on_select(self, callback: impl Fn(&mut EventContext, usize) + 'static) -> Self {
        self.modify(|tabview: &mut TabView| tabview.on_select = Some(Box::new(callback)))
    }

    pub fn with_selected<U: Into<usize>>(mut self, selected: impl Res<U>) -> Self {
        let _entity = self.entity();
        selected.set_or_bind(self.context(), |cx, selected| {
            let index = selected.get_value(cx).into();
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
