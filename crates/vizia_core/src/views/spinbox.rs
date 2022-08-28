use std::sync::Arc;

use crate::prelude::*;

pub struct SpinboxData {
    on_decrement: Option<Arc<dyn Fn(&mut EventContext) + Send + Sync>>,
    on_increment: Option<Arc<dyn Fn(&mut EventContext) + Send + Sync>>,
}

impl SpinboxData {
    pub fn new() -> Self {
        Self { on_decrement: None, on_increment: None }
    }
}

pub enum SpinboxEvent {
    Increment,
    Decrement,
    SetOnIncrement(Option<Arc<dyn Fn(&mut EventContext) + Send + Sync>>),
    SetOnDecrement(Option<Arc<dyn Fn(&mut EventContext) + Send + Sync>>),
}

impl Model for SpinboxData {
    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|e, _| match e {
            SpinboxEvent::Increment => {
                if let Some(callback) = &self.on_increment {
                    println!("pressed increment");
                    (callback)(cx)
                }
            }

            SpinboxEvent::Decrement => {
                if let Some(callback) = &self.on_decrement {
                    println!("pressed decrement");
                    (callback)(cx)
                }
            }

            SpinboxEvent::SetOnIncrement(callback) => {
                self.on_increment = callback.clone();
            }

            SpinboxEvent::SetOnDecrement(callback) => {
                self.on_decrement = callback.clone();
            }
        })
    }
}

pub struct Spinbox {}

// #[derive(Clone, Debug)]
// pub enum SpinboxKind {
//     Horizontal,
//     Vertical,
// }

impl Spinbox {
    pub fn new<L: Lens>(cx: &mut Context, lens: L) -> Handle<Spinbox>
    where
        <L as Lens>::Target: Data + ToString,
    {
        Self {}
            .build(cx, |cx| {
                let sd = SpinboxData { on_decrement: None, on_increment: None };
                let parent = cx.current().parent(&cx.tree).unwrap();
                cx.with_current(parent, |cx| sd.build(cx));
                cx.emit_to(cx.current(), ());

                Binding::new(cx, lens, |cx, lens| {
                    let lens = lens.get(cx);
                    HStack::new(cx, |cx| {
                        Element::new(cx)
                            .text("-")
                            .on_press(|ex| ex.emit(SpinboxEvent::Decrement))
                            .class("spinbox-button")
                            .hoverable(true);
                        Element::new(cx).text(&lens.to_string()).class("spinbox-value");
                        Element::new(cx)
                            .text("+")
                            .on_press(|ex| ex.emit(SpinboxEvent::Increment))
                            .class("spinbox-button")
                            .hoverable(true);
                    });
                })
            })
            .keyboard_navigatable(true)
    }
}

impl<'a> Handle<'a, Spinbox> {
    pub fn on_increment<F>(self, callback: F) -> Self
    where
        F: 'static + Fn(&mut EventContext) + Send + Sync,
    {
        self.cx.emit(SpinboxEvent::SetOnIncrement(Some(Arc::new(callback))));

        self
    }

    pub fn on_decrement<F>(self, callback: F) -> Self
    where
        F: 'static + Fn(&mut EventContext) + Send + Sync,
    {
        self.cx.emit(SpinboxEvent::SetOnDecrement(Some(Arc::new(callback))));

        self
    }
}

impl View for Spinbox {
    fn element(&self) -> Option<&'static str> {
        Some("spinbox")
    }
}
