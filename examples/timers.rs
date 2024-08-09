use log::debug;
use vizia::prelude::*;

#[derive(Lens)]
pub struct AppState {
    pub count: u32,
    pub timer: Timer,
}

#[derive(Debug)]
enum AppEvent {
    Increment,
    Reset,
}

impl Model for AppState {
    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|app_event, _| match app_event {
            AppEvent::Increment => {
                self.count += 1;
                if self.count >= 100 {
                    cx.stop_timer(self.timer);
                }
            }

            AppEvent::Reset => {
                self.count = 0;
            }
        });
    }
}

fn main() -> Result<(), ApplicationError> {
    Application::new(|cx| {
        // Emit event every second
        let timer = cx.add_timer(Duration::from_millis(10), None, |cx, action| match action {
            TimerAction::Start => {
                debug!("Start timer");
            }

            TimerAction::Stop => {
                debug!("Stop timer");
            }

            TimerAction::Tick(_delta) => {
                cx.emit(AppEvent::Increment);
            }
        });

        AppState { count: 0, timer }.build(cx);

        VStack::new(cx, |cx| {
            Label::new(cx, AppState::count).font_size(100.0);

            Button::new(cx, |cx| Label::new(cx, "Start")).on_press(move |cx| {
                cx.start_timer(timer);
            });
            Button::new(cx, |cx| Label::new(cx, "Stop")).on_press(move |cx| {
                cx.stop_timer(timer);
            });
            Button::new(cx, |cx| Label::new(cx, "Reset")).on_press(move |cx| {
                cx.schedule_emit(AppEvent::Reset, Instant::now() + Duration::from_secs(2));
            });
            Button::new(cx, |cx| Label::new(cx, "1s Interval")).on_press(move |cx| {
                cx.modify_timer(timer, |timer_state| {
                    timer_state.set_interval(Duration::from_secs(1));
                });
            });
        })
        .size(Auto)
        .space(Units::Stretch(1.0))
        .child_space(Stretch(1.0))
        .row_between(Units::Pixels(10.0));
    })
    .title("Timer")
    .inner_size((300, 300))
    .run()
}
