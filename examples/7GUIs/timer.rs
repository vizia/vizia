use std::time::Duration;

use vizia::prelude::*;

const STYLE: &str = r#"
    :root {
        child-space: 10px;
    }

    hstack {
        height: 35px;
        col-between: 10px;
    }

    button {
        width: 1s;
    }

    label, slider {
        top: 1s;
        bottom: 1s;
    }
"#;

#[derive(Lens)]
struct TimerData {
    #[lens(ignore)]
    timer: Timer,
    total_time: Duration,
    elapsed_time: Duration,
    progress: f32,
}

enum TimerEvent {
    Tick,
    SetDuration(f32),
    Reset,
}

impl TimerData {
    fn new(timer: Timer) -> Self {
        Self { timer, total_time: Duration::ZERO, elapsed_time: Duration::ZERO, progress: 0.0 }
    }

    fn should_start(&self) -> bool {
        self.total_time > self.elapsed_time
    }
}

impl Model for TimerData {
    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|event, _| match event {
            TimerEvent::Tick => {
                self.elapsed_time = self.elapsed_time.saturating_add(Duration::from_millis(100));
                self.progress = (self.elapsed_time.as_secs_f32() / self.total_time.as_secs_f32())
                    .clamp(0.0, 1.0);

                if self.progress == 1.0 {
                    cx.stop_timer(self.timer);
                }
            }

            TimerEvent::SetDuration(v) => {
                self.total_time = Duration::from_secs_f32(*v);

                if !self.should_start() {
                    cx.stop_timer(self.timer);
                } else if !cx.timer_is_running(self.timer) {
                    cx.start_timer(self.timer);
                }
            }

            TimerEvent::Reset => {
                self.elapsed_time = Duration::default();
                if self.should_start() && !cx.timer_is_running(self.timer) {
                    cx.start_timer(self.timer);
                }
            }
        })
    }
}

fn main() {
    Application::new(|cx: &mut Context| {
        cx.add_stylesheet(STYLE).expect("Failed to add stylesheet");

        let timer =
            cx.add_timer(Duration::from_millis(100), None, |cx, _| cx.emit(TimerEvent::Tick));

        TimerData::new(timer).build(cx);

        VStack::new(cx, |cx| {
            HStack::new(cx, |cx| {
                Label::new(cx, "Elapsed Time:");
                ProgressBar::new(cx, TimerData::progress, Orientation::Horizontal);
            });

            Label::new(cx, TimerData::elapsed_time.map(|v| format!("{:.1}s", v.as_secs_f32())));

            HStack::new(cx, |cx| {
                Label::new(cx, "Duration:");
                Slider::new(cx, TimerData::total_time.map(|v| v.as_secs_f32()))
                    .range(0.0..30.0)
                    .on_changing(|cx, v| cx.emit(TimerEvent::SetDuration(v)));
            });

            Button::new(cx, |cx| cx.emit(TimerEvent::Reset), |cx| Label::new(cx, "Reset"));
        });
    })
    .title("Timer")
    .inner_size((300, 150))
    .run()
}
