use std::time::{Duration, Instant};
use vizia::prelude::*;

#[derive(Lens)]
pub struct AppState {
    pub count: u32,
    pub current_timer: Option<TimedEventHandle>,
}

#[derive(Debug)]
enum AppEvent {
    Tick(Instant),
    Stop,
    Start,
}

impl Model for AppState {
    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|app_event, _| match app_event {
            AppEvent::Tick(instant) => {
                self.count += 1;
                let next = *instant + Duration::from_secs(1);
                self.current_timer = Some(cx.schedule_emit(AppEvent::Tick(next), next));
            }

            AppEvent::Stop => {
                if let Some(timer) = self.current_timer.take() {
                    cx.cancel_scheduled(timer);
                }
            }

            AppEvent::Start => {
                if self.current_timer.is_none() {
                    let next = Instant::now() + Duration::from_secs(1);
                    self.current_timer = Some(cx.schedule_emit(AppEvent::Tick(next), next));
                }
            }
        });
    }
}

fn main() {
    Application::new(|cx| {
        AppState { count: 0, current_timer: None }.build(cx);
        VStack::new(cx, |cx| {
            Label::new(cx, AppState::count).font_size(100.0);

            Button::new(
                cx,
                |cx| {
                    cx.emit(AppEvent::Start);
                },
                |cx| Label::new(cx, "Start"),
            );
            Button::new(
                cx,
                |cx| {
                    cx.emit(AppEvent::Stop);
                },
                |cx| Label::new(cx, "Stop"),
            );
        })
        .size(Auto)
        .space(Units::Stretch(1.0))
        .row_between(Units::Pixels(10.0));
    })
    .title("Timer")
    .run();
}
