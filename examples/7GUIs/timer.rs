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

struct TimerApp {
    total_time: Signal<f32>,
    elapsed_time: Signal<f32>,
    progress: Signal<f32>,
}

impl App for TimerApp {
    fn app_name() -> &'static str {
        "Timer"
    }

    fn new(cx: &mut Context) -> Self {
        Self {
            total_time: cx.state(0.0f32),
            elapsed_time: cx.state(0.0f32),
            progress: cx.state(0.0f32),
        }
    }

    fn on_build(self, cx: &mut Context) -> Self {
        cx.add_stylesheet(STYLE).expect("Failed to add stylesheet");

        let total_time = self.total_time;
        let elapsed_time = self.elapsed_time;
        let progress = self.progress;

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
                Label::new(cx, "Elapsed Time:");
                ProgressBar::horizontal(cx, progress);
            });

            let elapsed_label = elapsed_time.drv(cx, |v, _| format!("{:.1}s", v));
            Label::new(cx, elapsed_label);

            HStack::new(cx, move |cx| {
                Label::new(cx, "Duration:");
                Slider::new(cx, total_time).range(0.0..30.0).on_change(move |cx, v| {
                    total_time.set(cx, v);
                    let elapsed = *elapsed_time.get(cx);
                    if elapsed < v && !cx.timer_is_running(timer) {
                        cx.start_timer(timer);
                    } else if elapsed >= v {
                        cx.stop_timer(timer);
                    }
                });
            });

            Button::new(cx, |cx| Label::new(cx, "Reset")).on_press(move |cx| {
                elapsed_time.set(cx, 0.0);
                progress.set(cx, 0.0);
                let total = *total_time.get(cx);
                if total > 0.0 && !cx.timer_is_running(timer) {
                    cx.start_timer(timer);
                }
            });
        });

        self
    }

    fn window_config(&self) -> WindowConfig {
        window(|app| app.inner_size((300, 150)))
    }
}

fn main() -> Result<(), ApplicationError> {
    TimerApp::run()
}
