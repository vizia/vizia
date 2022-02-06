use std::time::{Duration, Instant};
use vizia::*;

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
    fn event(&mut self, cx: &mut Context, event: &mut Event) {
        match event.message.downcast() {
            Some(AppEvent::Tick(instant)) => {
                self.count += 1;
                let next = *instant + Duration::from_secs(1);
                self.current_timer = Some(cx.schedule_emit(AppEvent::Tick(next), next));
            }

            Some(AppEvent::Stop) => {
                if let Some(timer) = self.current_timer.take() {
                    cx.cancel_scheduled(timer).unwrap();
                }
            }

            Some(AppEvent::Start) => {
                if self.current_timer.is_none() {
                    let next = Instant::now() + Duration::from_secs(1);
                    self.current_timer = Some(cx.schedule_emit(AppEvent::Tick(next), next));
                }
            }

            None => {}
        }
    }
}

fn main() {
    Application::new(WindowDescription::new().with_title("Timer"), |cx| {
        AppState { count: 0, current_timer: None }.build(cx);
        VStack::new(cx, |cx| {
            Binding::new(cx, AppState::count, move |cx, count| {
                let count = *count.get(cx);
                Label::new(cx, &count.to_string()).font_size(100.0);
            });
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
        .space(Units::Stretch(1.0))
        .row_between(Units::Pixels(10.0));
    })
    .run();
}
