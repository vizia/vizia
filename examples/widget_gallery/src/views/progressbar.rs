use vizia::prelude::*;

use crate::DemoRegion;

#[derive(Lens)]
pub struct ProgressData {
    progress: f32,
}

pub enum ProgressEvent {
    SetProgress(f32),
}

impl Model for ProgressData {
    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|progress_event, _| match progress_event {
            ProgressEvent::SetProgress(progress) => self.progress = *progress,
        })
    }
}

pub fn progressbar(cx: &mut Context) {
    ProgressData { progress: 0.5 }.build(cx);

    VStack::new(cx, |cx| {
        Label::new(cx, "ProgressBar").class("title");
        Label::new(cx, "A label can be used to display a string of text.").class("paragraph");

        Divider::new(cx).top(Pixels(12.0)).bottom(Pixels(12.0));

        Label::new(cx, "Label").class("header");
        DemoRegion::new(
            cx,
            |cx| {
                ProgressBar::horizontal(cx, ProgressData::progress);
            },
            r#"ProgressBar::horizontal(cx, ProgressData::progress);"#,
        );
    })
    .class("panel");
}
