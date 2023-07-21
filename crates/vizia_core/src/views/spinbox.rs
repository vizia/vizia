use crate::icons::{
    ICON_CHEVRON_DOWN, ICON_CHEVRON_LEFT, ICON_CHEVRON_RIGHT, ICON_CHEVRON_UP, ICON_MINUS,
    ICON_PLUS,
};
use crate::prelude::*;

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

#[derive(Clone, Copy, Debug, PartialEq, Eq, Data)]
pub enum SpinboxKind {
    Horizontal,
    Vertical,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SpinboxIcons {
    PlusMinus,
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
        <L as Lens>::Target: Data + ToStringLocalized,
    {
        Self::custom(cx, move |cx| Label::new(cx, lens), kind, icons)
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
                    .class("icon")
                    .bind(Spinbox::kind, move |handle, spinbox_kind| {
                        match spinbox_kind.get(handle.cx) {
                            SpinboxKind::Horizontal => {
                                handle
                                    .text(match icons {
                                        SpinboxIcons::PlusMinus => ICON_MINUS,
                                        SpinboxIcons::Chevrons => ICON_CHEVRON_LEFT,
                                    })
                                    .on_press(|ex| ex.emit(SpinboxEvent::Decrement));
                            }

                            SpinboxKind::Vertical => {
                                handle
                                    .text(match icons {
                                        SpinboxIcons::PlusMinus => ICON_PLUS,
                                        SpinboxIcons::Chevrons => ICON_CHEVRON_UP,
                                    })
                                    .on_press(|ex| ex.emit(SpinboxEvent::Increment));
                            }
                        }
                    })
                    .class("spinbox-button");
                (content)(cx).class("spinbox-value").width(Stretch(1.0));
                Label::new(cx, "")
                    .class("icon")
                    .bind(Spinbox::kind, move |handle, spinbox_kind| {
                        match spinbox_kind.get(handle.cx) {
                            SpinboxKind::Horizontal => {
                                handle
                                    .text(match icons {
                                        SpinboxIcons::PlusMinus => ICON_PLUS,
                                        SpinboxIcons::Chevrons => ICON_CHEVRON_RIGHT,
                                    })
                                    .on_press(|ex| ex.emit(SpinboxEvent::Increment));
                            }

                            SpinboxKind::Vertical => {
                                handle
                                    .text(match icons {
                                        SpinboxIcons::PlusMinus => ICON_MINUS,
                                        SpinboxIcons::Chevrons => ICON_CHEVRON_DOWN,
                                    })
                                    .on_press(|ex| ex.emit(SpinboxEvent::Decrement));
                            }
                        }
                    })
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
