use instant::Duration;
use vizia::prelude::*;

#[derive(Lens)]
pub struct AppState {
    pub count: u32,
}

#[derive(Debug)]
enum AppEvent {
    Tick,
}

impl Model for AppState {
    fn event(&mut self, _cx: &mut EventContext, event: &mut Event) {
        event.map(|app_event, _| match app_event {
            AppEvent::Tick => {
                self.count += 1;
            }
        });
    }
}

fn main() {
    Application::new(|cx| {
        AppState { count: 0 }.build(cx);

        // Emit event every second
        let timer = cx.add_timer(
            TimerBuilder::new(Duration::from_secs(1), |cx| cx.emit(AppEvent::Tick))
                .with_duration(Duration::from_secs(5)),
        );

        VStack::new(cx, |cx| {
            Label::new(cx, AppState::count).font_size(100.0);

            Button::new(
                cx,
                move |cx| {
                    cx.start_timer(timer);
                },
                |cx| Label::new(cx, "Start"),
            );
            Button::new(
                cx,
                move |cx| {
                    cx.stop_timer(timer);
                },
                |cx| Label::new(cx, "Stop"),
            );
        })
        .size(Auto)
        .space(Units::Stretch(1.0))
        .row_between(Units::Pixels(10.0));
    })
    .title("Timer")
    .inner_size((300, 300))
    .run();
}
