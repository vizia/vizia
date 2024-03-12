use vizia::prelude::*;

use crate::DemoRegion;

#[derive(Lens)]
pub struct ProgressData {
    progress: f32,
}

impl Model for ProgressData {}

pub fn progressbar(cx: &mut Context) {
    ProgressData { progress: 0.5 }.build(cx);

    VStack::new(cx, |cx| {
        Label::new(cx, "ProgressBar").class("title");
        Label::new(cx, "").class("paragraph");

        Divider::new(cx).top(Pixels(12.0)).bottom(Pixels(12.0));

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
