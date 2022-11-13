use crate::prelude::*;

const ICON_CHEVRON_DOWN: &str = "\u{1F783}";
const ICON_CHEVRON_RIGHT: &str = "\u{1F782}";
const ICON_CLOSE: &str = "\u{1F7AA}";

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
                                    Label::new(cx, "v").bind(
                                        Notification::container_open,
                                        |h, open| {
                                            if open.get(h.cx) {
                                                h.text(ICON_CHEVRON_DOWN);
                                            } else {
                                                h.text(ICON_CHEVRON_RIGHT);
                                            }
                                        },
                                    );
                                })
                                .on_press_down(|ex| ex.emit(NotificationEvent::ToggleContainer))
                                .class("icon")
                                .checked(Notification::container_open);
                            }
                            HStack::new(cx, |cx| {
                                Label::new(cx, ICON_CLOSE);
                            })
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
