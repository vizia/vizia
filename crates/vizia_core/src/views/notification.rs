use crate::prelude::*;

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
                                HStack::new(cx, |cx| {
                                    Label::new(cx, "v");
                                })
                                .on_press_down(|ex| ex.emit(NotificationEvent::ToggleContainer))
                                .class("icon")
                                .checked(Notification::container_open);
                            }
                            HStack::new(cx, |cx| {
                                Label::new(cx, "X");
                            })
                            .class("icon");
                        })
                        .class("icon-container");
                    })
                    .class("notification-header")
                    .background_color(Notification::container_open.map(|v| {
                        if *v {
                            Color::from("#51afef22")
                        } else {
                            Color::from("#242424")
                        }
                    }));
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
