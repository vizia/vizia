use vizia::prelude::*;

use crate::DemoRegion;

#[derive(Lens)]
pub struct ProgressData {
    progress: f32,
    timer: Timer,
}

#[derive(Debug)]
pub enum ProgressEvent {
    Tick,
}

impl Model for ProgressData {
    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|app_event, _| match app_event {
            ProgressEvent::Tick => {
                self.progress = cx
                    .query_timer(self.timer, |timer_state| timer_state.progress().unwrap())
                    .unwrap();
                if self.progress >= 1.0 {
                    cx.start_timer(self.timer);
                }
            }
        });
    }
}

pub fn progressbar(cx: &mut Context) {
    let timer =
        cx.add_timer(Duration::from_millis(100), Some(Duration::from_secs(5)), |cx, action| {
            if matches!(action, TimerAction::Tick(_)) {
                cx.emit(ProgressEvent::Tick)
            }
        });

    cx.start_timer(timer);

    ProgressData { progress: 0.0, timer }.build(cx);

    VStack::new(cx, |cx| {
        Label::new(cx, "ProgressBar").class("title");
        Label::new(cx, "").class("paragraph");

        Divider::new(cx).top(Pixels(12.0)).bottom(Pixels(12.0));

        Label::new(cx, "Basic progress bar").class("header");
        DemoRegion::new(
            cx,
            |cx| {
                ProgressBar::horizontal(cx, ProgressData::progress).width(Pixels(300.0));
            },
            r#"ProgressBar::horizontal(cx, ProgressData::progress).width(Pixels(300.0));"#,
        );
    })
    .class("panel");
}
