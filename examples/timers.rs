use log::debug;
use vizia::prelude::*;

struct TimerApp {
    count: Signal<u32>,
    font_80: Signal<f32>,
    stretch_one: Signal<Units>,
    align_center: Signal<Alignment>,
    gap_8: Signal<Units>,
    title: Signal<&'static str>,
    size: Signal<(u32, u32)>,
}

impl App for TimerApp {
    fn new(cx: &mut Context) -> Self {
        Self {
            count: cx.state(0u32),
            font_80: cx.state(80.0),
            stretch_one: cx.state(Stretch(1.0)),
            align_center: cx.state(Alignment::Center),
            gap_8: cx.state(Pixels(8.0)),
            title: cx.state("Timer"),
            size: cx.state((300, 300)),
        }
    }

    fn on_build(self, cx: &mut Context) -> Self {
        let count = self.count;
        let font_80 = self.font_80;
        let stretch_one = self.stretch_one;
        let align_center = self.align_center;
        let gap_8 = self.gap_8;

        let timer =
            cx.add_timer(Duration::from_millis(100), None, move |cx, action| match action {
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

        Binding::new(cx, count, move |cx| {
            if *count.get(cx) >= 100 {
                cx.stop_timer(timer);
            }
        });

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

            Button::new(cx, |cx| Label::new(cx, "Start")).on_press(move |cx| {
                cx.start_timer(timer);
            });
            Button::new(cx, |cx| Label::new(cx, "Stop")).on_press(move |cx| {
                cx.stop_timer(timer);
            });
            Button::new(cx, |cx| Label::new(cx, "Reset")).on_press(move |cx| {
                cx.start_timer(reset_timer);
            });
            let use_one_sec = cx.state(false);
            let interval_label = use_one_sec.drv(cx, |v, _| {
                if *v { "Interval: 1s".to_string() } else { "Interval: 100ms".to_string() }
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

        self
    }

    fn window_config(&self) -> WindowConfig {
        let title = self.title;
        let size = self.size;
        window(move |app| app.title(title).inner_size(size))
    }
}

fn main() -> Result<(), ApplicationError> {
    TimerApp::run()
}
