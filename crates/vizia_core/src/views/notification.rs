use crate::{
    icons::{ICON_CHEVRON_DOWN, ICON_CHEVRON_RIGHT, ICON_X},
    prelude::*,
};

#[derive(Clone, Debug, PartialEq, Lens)]
pub struct Notification {
    container_open: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub enum NotificationEvent {
    ToggleContainer,
}

impl Notification {
    pub fn new(cx: &mut Context, header: String, content: Option<String>) -> Handle<Self> {
        Self { container_open: false }
            .build(cx, move |cx| {
                let some_text = content.is_some();
                HStack::new(cx, move |cx| {
                    Label::new(cx, &header);
                    if some_text {
                        Button::new(
                            cx,
                            |ex| ex.emit(NotificationEvent::ToggleContainer),
                            |cx| {
                                Label::new(
                                    cx,
                                    Notification::container_open.map(|open| {
                                        if *open {
                                            ICON_CHEVRON_DOWN
                                        } else {
                                            ICON_CHEVRON_RIGHT
                                        }
                                    }),
                                )
                                .class("icon")
                            },
                        )
                        .class("icon")
                        .checked(Notification::container_open);
                    }
                    Button::new(cx, |_| (), |cx| Label::new(cx, ICON_X).class("icon"))
                        .class("icon");
                })
                .class("notification-header");

                Binding::new(cx, Notification::container_open, move |cx, open| {
                    if open.get(cx) {
                        if let Some(text) = &content {
                            HStack::new(cx, move |cx| {
                                Label::new(cx, text).text_wrap(true);
                            })
                            .class("notification-container");
                        }
                    }
                });
            })
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
