use std::ops::Deref;

use crate::prelude::*;

pub enum AccordionEvent {
    SetOpen(Option<usize>),
}

/// A view which organizes content into expandable sections.
///
/// The accordion is implemented using internal [Collapsible] views and maintains
/// a single-open-section behavior.
pub struct Accordion {
    open_index: Signal<Option<usize>>,
}

impl Accordion {
    /// Creates a new [Accordion] view.
    pub fn new<S, V, T, F>(cx: &mut Context, list: S, content: F) -> Handle<Self>
    where
        S: Res<V> + 'static,
        V: Deref<Target = [T]> + Clone + 'static,
        T: Clone + 'static,
        F: 'static + Clone + Fn(&mut Context, usize, T) -> AccordionPair,
    {
        let open_index = Signal::new(None);
        let list = list.to_signal(cx);

        Self { open_index }.build(cx, move |cx| {
            Binding::new(cx, list, move |cx| {
                let list_values = list.get();
                let content = content.clone();
                let list_length = list.with(|list| list.len());

                for (index, item) in list_values.iter().cloned().enumerate() {
                    let pair = (content)(cx, index, item);

                    Collapsible::new(cx, pair.header, pair.content)
                        .open(open_index.map(move |open| *open == Some(index)))
                        .on_toggle(move |cx, is_open| {
                            cx.emit(AccordionEvent::SetOpen(if is_open {
                                Some(index)
                            } else {
                                None
                            }));
                        });
                    if index < list_length - 1 {
                        Divider::horizontal(cx);
                    }
                }
            });
        })
    }
}

impl View for Accordion {
    fn element(&self) -> Option<&'static str> {
        Some("accordion")
    }

    fn event(&mut self, _cx: &mut EventContext, event: &mut Event) {
        event.map(|accordion_event, _| match accordion_event {
            AccordionEvent::SetOpen(index) => {
                self.open_index.set(*index);
            }
        });
    }
}

impl Handle<'_, Accordion> {
    /// Sets the open section by index.
    pub fn with_open<U: Into<Option<usize>>>(mut self, index: impl Res<U>) -> Self {
        index.set_or_bind(self.context(), |cx, index| {
            cx.emit(AccordionEvent::SetOpen(index.get_value(cx).into()));
        });

        self
    }
}

pub struct AccordionPair {
    pub header: Box<dyn Fn(&mut Context)>,
    pub content: Box<dyn Fn(&mut Context)>,
}

impl AccordionPair {
    pub fn new<H, C>(header: H, content: C) -> Self
    where
        H: 'static + Fn(&mut Context),
        C: 'static + Fn(&mut Context),
    {
        Self { header: Box::new(header), content: Box::new(content) }
    }
}
