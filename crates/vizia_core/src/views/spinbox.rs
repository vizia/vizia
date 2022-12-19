use crate::{fonts::vizia_icons::*, prelude::*};

pub enum SpinboxEvent {
    Increment,
    Decrement,
}

#[derive(Lens)]
pub struct Spinbox {
    kind: SpinboxKind,

    on_decrement: Option<Box<dyn Fn(&mut EventContext) + Send + Sync>>,
    on_increment: Option<Box<dyn Fn(&mut EventContext) + Send + Sync>>,
}

#[derive(Clone, Copy, Debug, Data, PartialEq)]
pub enum SpinboxKind {
    Horizontal,
    Vertical,
}

#[derive(Clone, Copy, Debug, Data, PartialEq)]
pub enum SpinboxIcons {
    Math, //Don't know how to call this tbh
    Chevrons,
}

impl Spinbox {
    pub fn new<L: Lens>(
        cx: &mut Context,
        lens: L,
        kind: SpinboxKind,
        icons: SpinboxIcons,
    ) -> Handle<Spinbox>
    where
        <L as Lens>::Target: Data + ToString,
    {
        Self::custom(cx, move |cx| Label::new(cx, lens.clone()), kind, icons)
    }

    pub fn custom<F, V>(
        cx: &mut Context,
        content: F,
        kind: SpinboxKind,
        icons: SpinboxIcons,
    ) -> Handle<Spinbox>
    where
        F: Fn(&mut Context) -> Handle<V>,
        V: 'static + View,
    {
        Self { kind, on_decrement: None, on_increment: None }
            .build(cx, move |cx| {
                Label::new(cx, "")
                    .font("icons")
                    .bind(Spinbox::kind, move |handle, spinbox_kind| {
                        match spinbox_kind.get(handle.cx) {
                            SpinboxKind::Horizontal => {
                                handle
                                    .text(match icons {
                                        SpinboxIcons::Math => MINUS,
                                        SpinboxIcons::Chevrons => CHEVRON_LEFT,
                                    })
                                    .on_press(|ex| ex.emit(SpinboxEvent::Decrement));
                            }

                            SpinboxKind::Vertical => {
                                handle
                                    .text(match icons {
                                        SpinboxIcons::Math => PLUS,
                                        SpinboxIcons::Chevrons => CHEVRON_UP,
                                    })
                                    .on_press(|ex| ex.emit(SpinboxEvent::Increment));
                            }
                        }
                    })
                    .class("icon")
                    .font("vizia_icons")
                    .class("spinbox-button");
                (content)(cx).class("spinbox-value");
                Label::new(cx, "")
                    .font("icons")
                    .bind(Spinbox::kind, move |handle, spinbox_kind| {
                        match spinbox_kind.get(handle.cx) {
                            SpinboxKind::Horizontal => {
                                handle
                                    .text(match icons {
                                        SpinboxIcons::Math => PLUS,
                                        SpinboxIcons::Chevrons => CHEVRON_RIGHT,
                                    })
                                    .on_press(|ex| ex.emit(SpinboxEvent::Increment));
                            }

                            SpinboxKind::Vertical => {
                                handle
                                    .text(match icons {
                                        SpinboxIcons::Math => MINUS,
                                        SpinboxIcons::Chevrons => CHEVRON_DOWN,
                                    })
                                    .on_press(|ex| ex.emit(SpinboxEvent::Decrement));
                            }
                        }
                    })
                    .class("icon")
                    .font("vizia_icons")
                    .class("spinbox-button");
            })
            .toggle_class("horizontal", Spinbox::kind.map(|kind| kind == &SpinboxKind::Horizontal))
            .toggle_class("vertical", Spinbox::kind.map(|kind| kind == &SpinboxKind::Vertical))
            .navigable(true)
    }
}

impl<'a> Handle<'a, Spinbox> {
    pub fn on_increment<F>(self, callback: F) -> Self
    where
        F: 'static + Fn(&mut EventContext) + Send + Sync,
    {
        self.modify(|spinbox: &mut Spinbox| spinbox.on_increment = Some(Box::new(callback)))
    }

    pub fn on_decrement<F>(self, callback: F) -> Self
    where
        F: 'static + Fn(&mut EventContext) + Send + Sync,
    {
        self.modify(|spinbox: &mut Spinbox| spinbox.on_decrement = Some(Box::new(callback)))
    }
}

impl View for Spinbox {
    fn element(&self) -> Option<&'static str> {
        Some("spinbox")
    }

    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|spinbox_event, _| match spinbox_event {
            SpinboxEvent::Increment => {
                if let Some(callback) = &self.on_increment {
                    (callback)(cx)
                }
            }

            SpinboxEvent::Decrement => {
                if let Some(callback) = &self.on_decrement {
                    (callback)(cx)
                }
            }
        });
    }
}
