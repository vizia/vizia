use crate::{
    fonts::vizia_icons::{CHEVRON_DOWN, CHEVRON_RIGHT, CROSS},
    prelude::*,
};

#[derive(Clone, Debug, PartialEq, Data, Lens)]
pub struct Notification {
    container_open: bool,
}

#[derive(Clone, Debug, PartialEq, Data)]
pub enum NotificationEvent {
    ToggleContainer,
}

impl Notification {
    pub fn new(cx: &mut Context, header: String, text: Option<String>) -> Handle<Self> {
        Self { container_open: false }
            .build(cx, move |cx| {
                Element::new(cx).class("color-strip");
                VStack::new(cx, move |cx| {
                    let some_text = text.is_some();
                    HStack::new(cx, move |cx| {
                        Label::new(cx, &header);
                        HStack::new(cx, |cx| {
                            if some_text {
                                Button::new(
                                    cx,
                                    |ex| ex.emit(NotificationEvent::ToggleContainer),
                                    |cx| {
                                        Label::new(
                                            cx,
                                            Notification::container_open.map(|open| {
                                                if *open {
                                                    CHEVRON_DOWN
                                                } else {
                                                    CHEVRON_RIGHT
                                                }
                                            }),
                                        )
                                        .font("vizia_icons")
                                        .class("icon")
                                    },
                                )
                                .class("icon")
                                .checked(Notification::container_open);
                            }
                            Button::new(
                                cx,
                                |_| (),
                                |cx| Label::new(cx, CROSS).font("vizia_icons").class("icon"),
                            )
                            .class("icon");
                        })
                        .class("icon-container");
                    })
                    .class("notification-header")
                    .bind(Notification::container_open, |h, open| {
                        if open.get(h.cx) {
                            h.background_gradient(
                                LinearGradient::new(GradientDirection::LeftToRight)
                                    .add_stop(Percentage(0.0), Color::from("#51afef22"))
                                    .add_stop(Percentage(100.0), Color::from("#51afef22")),
                            );
                        } else {
                            h.background_gradient(
                                LinearGradient::new(GradientDirection::LeftToRight)
                                    .add_stop(Percentage(0.0), Color::from("#51afef22"))
                                    .add_stop(Percentage(25.0), Color::transparent()),
                            );
                        }
                    });

                    Binding::new(cx, Notification::container_open, move |cx, open| {
                        if open.get(cx) {
                            if let Some(text) = &text {
                                HStack::new(cx, move |cx| {
                                    Label::new(cx, text);
                                })
                                .class("notification-container");
                            }
                        }
                    });
                });
            })
            .layout_type(LayoutType::Row)
            .navigable(true)
    }
}

impl View for Notification {
    fn element(&self) -> Option<&'static str> {
        Some("notification")
    }

    fn event(&mut self, _: &mut EventContext, event: &mut Event) {
        event.map(|window_event, _| match window_event {
            NotificationEvent::ToggleContainer => self.container_open = !self.container_open,
        });
    }
}
