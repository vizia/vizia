use log::debug;
use vizia::prelude::*;

fn main() -> Result<(), ApplicationError> {
    let (app, (title, size)) = Application::new_with_state(|cx| {
        let count = cx.state(0u32);
        let font_80 = cx.state(80.0);
        let stretch_one = cx.state(Stretch(1.0));
        let align_center = cx.state(Alignment::Center);
        let gap_8 = cx.state(Pixels(8.0));

        // Main timer - ticks every 100ms
        let timer = cx.add_timer(Duration::from_millis(100), None, move |cx, action| match action {
            TimerAction::Start => {
                debug!("Start timer");
            }

            TimerAction::Stop => {
                debug!("Stop timer");
            }

            TimerAction::Tick(_delta) => {
                count.update(cx, |count| *count += 1);
            }
        });

        // Stop main timer when count reaches 100
        Binding::new(cx, count, move |cx| {
            if *count.get(cx) >= 100 {
                cx.stop_timer(timer);
            }
        });

        // One-shot reset timer - fires once after 2 seconds, then stops
        let reset_timer = cx.add_timer(
            Duration::from_secs(2),
            Some(Duration::from_secs(2)),
            move |cx, action| {
                if matches!(action, TimerAction::Tick(_)) {
                    count.set(cx, 0);
                }
            },
        );

        VStack::new(cx, |cx| {
            Label::new(cx, count).font_size(font_80);

            Button::new(cx, |cx| Label::static_text(cx, "Start")).on_press(move |cx| {
                cx.start_timer(timer);
            });
            Button::new(cx, |cx| Label::static_text(cx, "Stop")).on_press(move |cx| {
                cx.stop_timer(timer);
            });
            Button::new(cx, |cx| Label::static_text(cx, "Reset")).on_press(move |cx| {
                cx.start_timer(reset_timer);
            });
            let use_one_sec = cx.state(false);
            let interval_label = cx.derived({
                let use_one_sec = use_one_sec;
                move |store| {
                    if *use_one_sec.get(store) {
                        "Interval: 1s".to_string()
                    } else {
                        "Interval: 100ms".to_string()
                    }
                }
            });
            Button::new(cx, |cx| Label::new(cx, interval_label)).on_press(move |cx| {
                let new_flag = !*use_one_sec.get(cx);
                use_one_sec.set(cx, new_flag);
                let interval =
                    if new_flag { Duration::from_secs(1) } else { Duration::from_millis(100) };
                cx.modify_timer(timer, |timer_state| {
                    timer_state.set_interval(interval);
                });
            });
        })
        .size(stretch_one)
        .alignment(align_center)
        .gap(gap_8);
        (cx.state("Timer"), cx.state((300, 300)))
    });

    app.title(title).inner_size(size).run()
}
