use std::ops::Deref;

use crate::prelude::*;

pub enum AccordionEvent {
    SetOpen(Option<usize>),
    ClearHeaders,
    RegisterHeader(usize, Entity),
    FocusNextHeader,
    FocusPrevHeader,
    FocusFirstHeader,
    FocusLastHeader,
}

/// A view which organizes content into expandable sections.
///
/// The accordion is implemented using internal [Collapsible] views and maintains
/// a single-open-section behavior.
pub struct Accordion {
    open_index: Signal<Option<usize>>,
    header_entities: Vec<Entity>,
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

        Self { open_index, header_entities: Vec::new() }.build(cx, move |cx| {
            Keymap::from(vec![
                (
                    KeyChord::new(Modifiers::empty(), Code::ArrowDown),
                    KeymapEntry::new("Accordion Focus Next", |cx| {
                        cx.emit(AccordionEvent::FocusNextHeader)
                    }),
                ),
                (
                    KeyChord::new(Modifiers::empty(), Code::ArrowUp),
                    KeymapEntry::new("Accordion Focus Previous", |cx| {
                        cx.emit(AccordionEvent::FocusPrevHeader)
                    }),
                ),
                (
                    KeyChord::new(Modifiers::empty(), Code::Home),
                    KeymapEntry::new("Accordion Focus First", |cx| {
                        cx.emit(AccordionEvent::FocusFirstHeader)
                    }),
                ),
                (
                    KeyChord::new(Modifiers::empty(), Code::End),
                    KeymapEntry::new("Accordion Focus Last", |cx| {
                        cx.emit(AccordionEvent::FocusLastHeader)
                    }),
                ),
            ])
            .build(cx);

            Binding::new(cx, list, move |cx| {
                let list_values = list.get();
                let content = content.clone();
                let list_length = list.with(|list| list.len());

                cx.emit(AccordionEvent::ClearHeaders);

                for (index, item) in list_values.iter().cloned().enumerate() {
                    let pair = (content)(cx, index, item);

                    Collapsible::new(cx, pair.header, pair.content)
                        .on_build(move |cx| {
                            if let Some(header) = cx.nth_child(0) {
                                cx.emit(AccordionEvent::RegisterHeader(index, header));
                            }
                        })
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

    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|accordion_event, _| match accordion_event {
            AccordionEvent::SetOpen(index) => {
                self.open_index.set(*index);
            }

            AccordionEvent::ClearHeaders => {
                self.header_entities.clear();
            }

            AccordionEvent::RegisterHeader(index, entity) => {
                if self.header_entities.len() <= *index {
                    self.header_entities.resize(*index + 1, Entity::null());
                }

                self.header_entities[*index] = *entity;
            }

            AccordionEvent::FocusNextHeader => {
                if let Some(index) = self.focused_header_index(cx) {
                    let next_index = (index + 1) % self.header_entities.len();
                    let next_header = self.header_entities[next_index];
                    cx.with_current(next_header, |cx| cx.focus());
                }
            }

            AccordionEvent::FocusPrevHeader => {
                if let Some(index) = self.focused_header_index(cx) {
                    let prev_index =
                        if index == 0 { self.header_entities.len() - 1 } else { index - 1 };
                    let prev_header = self.header_entities[prev_index];
                    cx.with_current(prev_header, |cx| cx.focus());
                }
            }

            AccordionEvent::FocusFirstHeader => {
                if let Some(first_header) = self.header_entities.first().copied() {
                    cx.with_current(first_header, |cx| cx.focus());
                }
            }

            AccordionEvent::FocusLastHeader => {
                if let Some(last_header) = self.header_entities.last().copied() {
                    cx.with_current(last_header, |cx| cx.focus());
                }
            }
        });

        event.map(|window_event, meta| match window_event {
            WindowEvent::KeyDown(code, _) => match code {
                Code::ArrowDown | Code::ArrowUp | Code::Home | Code::End => {
                    if self.focused_header_index(cx).is_some() {
                        meta.consume();
                    }
                }
                _ => {}
            },
            _ => {}
        });
    }
}

impl Accordion {
    fn focused_header_index(&self, cx: &EventContext) -> Option<usize> {
        let focused = cx.focused();
        self.header_entities.iter().position(|header| {
            !header.is_null() && (focused == *header || focused.is_descendant_of(cx.tree, *header))
        })
    }
}

impl Handle<'_, Accordion> {
    /// Sets the open section by index.
    pub fn with_open<U: Into<Option<usize>>>(mut self, index: impl Res<U>) -> Self {
        let entity = self.entity();
        index.set_or_bind(self.context(), move |cx, index| {
            cx.emit_to(entity, AccordionEvent::SetOpen(index.get_value(cx).into()));
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
