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

struct TimerData {
    timer: Timer,
    total_time: Signal<Duration>,
    elapsed_time: Signal<Duration>,
    progress: Signal<f32>,
}

enum TimerEvent {
    Tick,
    SetDuration(f32),
    Reset,
}

impl TimerData {
    fn new(timer: Timer) -> Self {
        Self {
            timer,
            total_time: Signal::new(Duration::ZERO),
            elapsed_time: Signal::new(Duration::ZERO),
            progress: Signal::new(0.0),
        }
    }

    fn should_start(&self) -> bool {
        self.total_time.get() > self.elapsed_time.get()
    }
}

impl Model for TimerData {
    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|event, _| match event {
            TimerEvent::Tick => {
                self.elapsed_time.update(|elapsed| {
                    *elapsed = elapsed.saturating_add(Duration::from_millis(100))
                });
                let elapsed = self.elapsed_time.get();
                let total = self.total_time.get();
                let progress = (elapsed.as_secs_f32() / total.as_secs_f32()).clamp(0.0, 1.0);
                self.progress.set(progress);

                if progress == 1.0 {
                    cx.stop_timer(self.timer);
                }
            }

            TimerEvent::SetDuration(v) => {
                self.total_time.set(Duration::from_secs_f32(*v));

                if !self.should_start() {
                    cx.stop_timer(self.timer);
                } else if !cx.timer_is_running(self.timer) {
                    cx.start_timer(self.timer);
                }
            }

            TimerEvent::Reset => {
                self.elapsed_time.set(Duration::default());
                self.progress.set(0.0);
                if self.should_start() && !cx.timer_is_running(self.timer) {
                    cx.start_timer(self.timer);
                }
            }
        })
    }
}

fn main() -> Result<(), ApplicationError> {
    Application::new(|cx: &mut Context| {
        cx.add_stylesheet(STYLE).expect("Failed to add stylesheet");

        let timer =
            cx.add_timer(Duration::from_millis(100), None, |cx, _| cx.emit(TimerEvent::Tick));

        let timer_data = TimerData::new(timer);
        let progress = timer_data.progress;
        let elapsed_time = timer_data.elapsed_time;
        let total_time = timer_data.total_time;

        timer_data.build(cx);

        VStack::new(cx, |cx| {
            HStack::new(cx, |cx| {
                Label::new(cx, "Elapsed Time:");
                ProgressBar::new(cx, progress, Orientation::Horizontal);
            });

            Label::new(cx, Memo::new(move |_| format!("{:.1}s", elapsed_time.get().as_secs_f32())));

            HStack::new(cx, |cx| {
                Label::new(cx, "Duration:");
                Slider::new(cx, Memo::new(move |_| total_time.get().as_secs_f32()))
                    .range(0.0..30.0)
                    .on_change(|cx, v| cx.emit(TimerEvent::SetDuration(v)));
            });

            Button::new(cx, |cx| Label::new(cx, "Reset")).on_press(|cx| cx.emit(TimerEvent::Reset));
        });
    })
    .title("Timer")
    .inner_size((300, 150))
    .run()
}
