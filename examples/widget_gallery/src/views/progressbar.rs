use vizia::prelude::*;

use crate::DemoRegion;

pub fn progressbar(cx: &mut Context) {
    let progress = cx.state(0.0f32);
    let width_300 = cx.state(Pixels(300.0));
    let duration_secs = 5.0f32;
    let timer = cx.add_timer(Duration::from_millis(100), None, move |cx, action| {
        if let TimerAction::Tick(delta) = action {
            let current = *progress.get(cx);
            let increment = delta.as_secs_f32() / duration_secs;
            let next = current + increment;
            progress.set(cx, if next >= 1.0 { 0.0 } else { next });
        }
    });

    cx.start_timer(timer);

    VStack::new(cx, move |cx| {
        Markdown::new(cx, "# ProgressBar");

        Divider::new(cx);

        Markdown::new(cx, "### Basic progress bar");

        DemoRegion::new(
            cx,
            move |cx| {
                ProgressBar::horizontal(cx, progress).width(width_300);
            },
            r#"let progress = cx.state(0.0f32);
let width_300 = cx.state(Pixels(300.0));
ProgressBar::horizontal(cx, progress).width(width_300);"#,
        );
    })
    .class("panel");
}
