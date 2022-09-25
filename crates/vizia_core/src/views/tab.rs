use crate::{
    prelude::*,
    state::{Index, Then},
};

pub enum TabEvent {
    SetSelected(usize),
}

#[derive(Lens)]
pub struct TabView {
    selected_index: usize,
}

impl TabView {
    pub fn new<L, T, F>(cx: &mut Context, lens: L, content: F) -> Handle<Self>
    where
        L: Lens<Target = Vec<T>>,
        T: 'static,
        F: 'static + Copy + Fn(&mut Context, Then<L, Index<Vec<T>, T>>) -> TabPair,
    {
        Self { selected_index: 0 }.build(cx, move |cx| {
            VStack::new(cx, move |cx| {
                let lens2 = lens.clone();
                // Tab headers
                HStack::new(cx, move |cx| {
                    Binding::new(cx, lens.clone().map(|list| list.len()), move |cx, list_length| {
                        let list_length = list_length.get_fallible(cx).map_or(0, |d| d);
                        for index in 0..list_length {
                            let l = lens.clone().index(index);
                            let builder = (content)(cx, l).header;
                            TabHeader::new(cx, index, builder);
                        }
                    })
                })
                .height(Auto);

                // Tab content
                Binding::new(cx, TabView::selected_index, move |cx, selected| {
                    let selected = selected.get(cx);
                    let l = lens2.clone().index(selected);
                    ((content)(cx, l).content)(cx);
                });
            });
        })
    }
}

impl View for TabView {
    fn event(&mut self, _cx: &mut EventContext, event: &mut Event) {
        event.map(|tab_event, meta| match tab_event {
            TabEvent::SetSelected(index) => {
                self.selected_index = *index;
                meta.consume();
            }
        });
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
        Self { index }.build(cx, |cx| (content)(cx)).size(Auto)
    }
}

impl View for TabHeader {
    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|window_event, _meta| match window_event {
            WindowEvent::MouseDown(button) if *button == MouseButton::Left => {
                //if meta.target == cx.current() {
                cx.emit(TabEvent::SetSelected(self.index));
                //}
            }

            _ => {}
        });
    }
}
