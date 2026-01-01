use vizia::prelude::*;

const STYLE: &str = r#"
    :root {
        padding: 10px;
    }

    hstack {
        height: 35px;
        horizontal-gap: 10px;
    }

    button {
        width: 1s;
    }

    label, slider {
        top: 1s;
        bottom: 1s;
    }
"#;

fn main() -> Result<(), ApplicationError> {
    let (app, (title, size)) = Application::new_with_state(|cx: &mut Context| {
        cx.add_stylesheet(STYLE).expect("Failed to add stylesheet");

        // State signals
        let total_time = cx.state(0.0f32);
        let elapsed_time = cx.state(0.0f32);
        let progress = cx.state(0.0f32);

        let timer = cx.add_timer(Duration::from_millis(100), None, move |cx, _| {
            let current = *elapsed_time.get(cx);
            let total = *total_time.get(cx);
            let new_elapsed = current + 0.1;
            elapsed_time.set(cx, new_elapsed);

            let p = if total > 0.0 { (new_elapsed / total).clamp(0.0, 1.0) } else { 0.0 };
            progress.set(cx, p);
        });

        VStack::new(cx, move |cx| {
            HStack::new(cx, |cx| {
                Label::static_text(cx, "Elapsed Time:");
                ProgressBar::horizontal(cx, progress);
            });

            let elapsed_label = cx.derived({
                let elapsed_time = elapsed_time;
                move |store| format!("{:.1}s", elapsed_time.get(store))
            });
            Label::new(cx, elapsed_label);

            HStack::new(cx, move |cx| {
                Label::static_text(cx, "Duration:");
                Slider::new(cx, total_time).range(0.0..30.0).on_change(move |cx, v| {
                    total_time.set(cx, v);
                    // Restart timer if needed
                    let elapsed = *elapsed_time.get(cx);
                    if elapsed < v && !cx.timer_is_running(timer) {
                        cx.start_timer(timer);
                    } else if elapsed >= v {
                        cx.stop_timer(timer);
                    }
                });
            });

            Button::new(cx, |cx| Label::static_text(cx, "Reset")).on_press(move |cx| {
                elapsed_time.set(cx, 0.0);
                progress.set(cx, 0.0);
                let total = *total_time.get(cx);
                if total > 0.0 && !cx.timer_is_running(timer) {
                    cx.start_timer(timer);
                }
            });
        });
        (cx.state("Timer"), cx.state((300, 150)))
    });

    app.title(title).inner_size(size).run()
}
