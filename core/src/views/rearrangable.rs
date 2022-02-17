use crate::{View, Context, Handle, ForEach, Event, Lens, Model, Binding, HStack, MouseButtonState, Actions};

pub struct Rearrangable {
}

#[derive(Lens)]
pub struct RearrangeState {
    held: Option<usize>,
}

impl Model for RearrangeState {
    fn event(&mut self, _cx: &mut Context, event: &mut Event) {
        match event.message.downcast() {
            Some(RearrangeStateEvent::Set(idx)) => self.held = *idx,
            _ => {}
        }
    }
}

#[derive(Debug)]
enum RearrangeStateEvent {
    Set(Option<usize>)
}

impl Rearrangable {
    pub fn new<F, F2>(cx: &mut Context, count: usize, builder: F, swapper: F2) -> Handle<'_, Self>
    where
        F: 'static + Fn(&mut Context, usize) + Clone,
        F2: 'static + Fn(&mut Context, usize, usize) + Clone,
    {
        Self { }
            .build2(cx, move |cx| {
                if cx.data::<RearrangeState>().is_none() {
                    RearrangeState { held: None }.build(cx);
                }
                Binding::new(cx, RearrangeState::held, move |cx, held| {
                    let builder = builder.clone();
                    let swapper = swapper.clone();
                    ForEach::new(cx, 0..count, move |cx, idx| {
                        let builder = builder.clone();
                        let swapper = swapper.clone();
                        let handle = HStack::new(cx, move |cx| {
                            (builder)(cx, idx);
                        });
                        handle.cx.style.classes.remove(handle.entity);
                        if Some(idx) == *held.get(handle.cx) {
                            handle.class("dragging")
                        } else {
                            handle
                        }
                            .on_press(move |cx| {
                                cx.emit(RearrangeStateEvent::Set(Some(idx)));
                            })
                            .on_release(move |cx| {
                                cx.emit(RearrangeStateEvent::Set(None));
                            })
                            .on_over(move |cx| {
                                if cx.mouse.left.state == MouseButtonState::Pressed {
                                    if let Some(held_idx) = *held.get(cx) {
                                        if held_idx != idx {
                                            cx.emit(RearrangeStateEvent::Set(Some(idx)));
                                            (swapper.clone())(cx, held_idx, idx);
                                        }
                                    }
                                }
                            });
                    });
                });
            })
    }
}

impl View for Rearrangable {
    fn element(&self) -> Option<String> {
        Some("rearrangable".to_owned())
    }
}
