use vizia::prelude::*;

use crate::components::DemoRegion;

pub struct SwitchData {
    flag: Signal<bool>,
    notifications: Signal<bool>,
    autosave: Signal<bool>,
    analytics: Signal<bool>,
}

pub enum SwitchEvent {
    ToggleFlag,
    ToggleNotifications,
    ToggleAutosave,
    ToggleAnalytics,
}

impl Model for SwitchData {
    fn event(&mut self, _: &mut EventContext, event: &mut Event) {
        event.map(|checkbox_event, _| match checkbox_event {
            SwitchEvent::ToggleFlag => {
                self.flag.update(|flag| *flag ^= true);
            }
            SwitchEvent::ToggleNotifications => {
                self.notifications.update(|notifications| *notifications ^= true);
            }
            SwitchEvent::ToggleAutosave => {
                self.autosave.update(|autosave| *autosave ^= true);
            }
            SwitchEvent::ToggleAnalytics => {
                self.analytics.update(|analytics| *analytics ^= true);
            }
        });
    }
}

pub fn switch(cx: &mut Context) {
    let flag = Signal::new(true);
    let notifications = Signal::new(true);
    let autosave = Signal::new(false);
    let analytics = Signal::new(true);
    SwitchData { flag, notifications, autosave, analytics }.build(cx);

    VStack::new(cx, |cx| {
        Label::new(cx, Localized::new("switch")).class("panel-title");

        Divider::new(cx);

        DemoRegion::new(cx, "Basic Switch", move |cx| {
            Switch::new(cx, flag).on_toggle(|cx| cx.emit(SwitchEvent::ToggleFlag));
        });

        DemoRegion::new(cx, "Switch With Label", move |cx| {
            HStack::new(cx, |cx| {
                Switch::new(cx, flag)
                    .on_toggle(|cx| cx.emit(SwitchEvent::ToggleFlag))
                    .id("switch-airplane-mode");
                Label::new(cx, "Airplane mode").describing("switch-airplane-mode");
            })
            .size(Auto)
            .alignment(Alignment::Center)
            .horizontal_gap(Pixels(8.0));
        });

        DemoRegion::new(cx, "Disabled States", move |cx| {
            HStack::new(cx, |cx| {
                Switch::new(cx, true).disabled(true);
                Switch::new(cx, false).disabled(true);
            })
            .size(Auto)
            .alignment(Alignment::Center)
            .horizontal_gap(Pixels(12.0));
        });

        DemoRegion::new(cx, "RTL Layout", move |cx| {
            HStack::new(cx, |cx| {
                Switch::new(cx, flag)
                    .on_toggle(|cx| cx.emit(SwitchEvent::ToggleFlag))
                    .id("switch-rtl");
                Label::new(cx, "Right-to-left switch").describing("switch-rtl");
            })
            .size(Auto)
            .alignment(Alignment::Center)
            .horizontal_gap(Pixels(8.0))
            .direction(Direction::RightToLeft);
        });

        DemoRegion::new(cx, "Settings Group", move |cx| {
            VStack::new(cx, |cx| {
                HStack::new(cx, |cx| {
                    Switch::new(cx, notifications)
                        .on_toggle(|cx| cx.emit(SwitchEvent::ToggleNotifications))
                        .id("switch-notifications");
                    Label::new(
                        cx,
                        notifications
                            .map(|on| if *on { "Notifications: On" } else { "Notifications: Off" }),
                    )
                    .describing("switch-notifications");
                })
                .size(Auto)
                .alignment(Alignment::Center)
                .horizontal_gap(Pixels(8.0));

                HStack::new(cx, |cx| {
                    Switch::new(cx, autosave)
                        .on_toggle(|cx| cx.emit(SwitchEvent::ToggleAutosave))
                        .id("switch-autosave");
                    Label::new(
                        cx,
                        autosave.map(|on| if *on { "Autosave: On" } else { "Autosave: Off" }),
                    )
                    .describing("switch-autosave");
                })
                .size(Auto)
                .alignment(Alignment::Center)
                .horizontal_gap(Pixels(8.0));

                HStack::new(cx, |cx| {
                    Switch::new(cx, analytics)
                        .on_toggle(|cx| cx.emit(SwitchEvent::ToggleAnalytics))
                        .id("switch-analytics");
                    Label::new(
                        cx,
                        analytics.map(|on| if *on { "Analytics: On" } else { "Analytics: Off" }),
                    )
                    .describing("switch-analytics");
                })
                .size(Auto)
                .alignment(Alignment::Center)
                .horizontal_gap(Pixels(8.0));
            })
            .height(Auto)
            .gap(Pixels(10.0));
        });
    })
    .class("panel");
}
