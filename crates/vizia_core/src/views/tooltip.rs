use std::sync::Arc;

use crate::prelude::*;

pub enum TooltipEvent {
    Ok,
    SetOnOk(Option<Arc<dyn Fn(&mut EventContext) + Send + Sync>>),
}

pub struct Tooltip {
    on_ok: Option<Arc<dyn Fn(&mut EventContext) + Send + Sync>>,
}

impl Tooltip {
    pub fn new<F, T>(cx: &mut Context, title: impl Res<T>, content: F) -> Handle<Self>
    where
        F: FnOnce(&mut Context),
        T: ToString,
    {
        Self { on_ok: None }.build(cx, |cx| {
            HStack::new(cx, |cx| {
                Element::new(cx).class("tooltip-pointer");
                //.translate((2.0, 7.0)).rotate(45.0f32);
            })
            .class("tooltip-pointer-wrapper");

            VStack::new(cx, |cx| {
                Label::new(cx, title).class("title");
                (content)(cx);
                HStack::new(cx, |cx| {
                    Button::new(cx, |ex| ex.emit(TooltipEvent::Ok), |cx| Label::new(cx, "Ok"));
                })
                .class("tooltip-button-wrapper");
            })
            .class("tooltip-wrapper");
        })
    }
}

impl<'a> Handle<'a, Tooltip> {
    pub fn on_ok<F>(self, callback: F) -> Self
    where
        F: 'static + Fn(&mut EventContext) + Send + Sync,
    {
        self.cx.emit_to(self.entity(), TooltipEvent::SetOnOk(Some(Arc::new(callback))));

        self
    }
}

impl View for Tooltip {
    fn element(&self) -> Option<&'static str> {
        Some("tooltip")
    }

    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|e, _| match e {
            TooltipEvent::SetOnOk(callback) => {
                self.on_ok = callback.clone();
            }

            TooltipEvent::Ok => {
                if let Some(callback) = &self.on_ok {
                    (callback)(cx);
                }
            }
        });
    }
}

// Sequential Tooltip

pub enum TooltipSeqEvent {
    Next,
    Prev,

    SetOnNext(Option<Arc<dyn Fn(&mut EventContext) + Send + Sync>>),
    SetOnPrev(Option<Arc<dyn Fn(&mut EventContext) + Send + Sync>>),
}

pub struct TooltipSeq {
    on_next: Option<Arc<dyn Fn(&mut EventContext) + Send + Sync>>,
    on_prev: Option<Arc<dyn Fn(&mut EventContext) + Send + Sync>>,
}

impl TooltipSeq {
    pub fn new<F, T>(cx: &mut Context, title: impl Res<T>, content: F) -> Handle<Self>
    where
        F: FnOnce(&mut Context),
        T: ToString,
    {
        Self { on_next: None, on_prev: None }.build(cx, |cx| {
            HStack::new(cx, |cx| {
                Element::new(cx).class("tooltip-pointer");
                //.translate((2.0, 7.0)).rotate(45.0f32);
            })
            .class("tooltip-pointer-wrapper");

            VStack::new(cx, |cx| {
                Label::new(cx, title).class("title");
                (content)(cx);
                HStack::new(cx, |cx| {
                    Button::new(
                        cx,
                        |ex| ex.emit(TooltipSeqEvent::Prev),
                        |cx| Label::new(cx, "Prev"),
                    );
                    Button::new(
                        cx,
                        |ex| ex.emit(TooltipSeqEvent::Next),
                        |cx| Label::new(cx, "Next"),
                    );
                })
                .class("tooltip-button-wrapper");
            })
            .class("tooltip-wrapper");
        })
    }
}

impl<'a> Handle<'a, TooltipSeq> {
    pub fn on_next<F>(self, callback: F) -> Self
    where
        F: 'static + Fn(&mut EventContext) + Send + Sync,
    {
        self.cx.emit_to(self.entity(), TooltipSeqEvent::SetOnNext(Some(Arc::new(callback))));

        self
    }

    pub fn on_prev<F>(self, callback: F) -> Self
    where
        F: 'static + Fn(&mut EventContext) + Send + Sync,
    {
        self.cx.emit_to(self.entity(), TooltipSeqEvent::SetOnPrev(Some(Arc::new(callback))));

        self
    }
}

impl View for TooltipSeq {
    fn element(&self) -> Option<&'static str> {
        Some("tooltip")
    }

    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|e, _| match e {
            TooltipSeqEvent::Next => {
                if let Some(callback) = &self.on_next {
                    (callback)(cx);
                }
            }

            TooltipSeqEvent::Prev => {
                if let Some(callback) = &self.on_prev {
                    (callback)(cx);
                }
            }

            TooltipSeqEvent::SetOnNext(callback) => {
                self.on_next = callback.clone();
            }

            TooltipSeqEvent::SetOnPrev(callback) => {
                self.on_prev = callback.clone();
            }
        });
    }
}
