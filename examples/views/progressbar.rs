mod helpers;
use helpers::*;
use vizia::prelude::*;

#[derive(Lens)]
pub struct AppData {
    progress: f32,
    timer: Timer,
}

#[derive(Debug)]
pub enum AppEvent {
    Tick,
}

impl Model for AppData {
    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|app_event, _| match app_event {
            AppEvent::Tick => {
                self.progress = cx
                    .query_timer(self.timer, |timer_state| timer_state.progress().unwrap())
                    .unwrap();
                if self.progress >= 1.0 {
                    cx.start_timer(self.timer);
                }
            }
        });
    }
}

fn main() -> Result<(), ApplicationError> {
    Application::new(|cx: &mut Context| {
        let timer =
            cx.add_timer(Duration::from_millis(100), Some(Duration::from_secs(5)), |cx, action| {
                if matches!(action, TimerAction::Tick(_)) {
                    cx.emit(AppEvent::Tick)
                }
            });

        cx.start_timer(timer);

        AppData { progress: 0.0, timer }.build(cx);

        ExamplePage::vertical(cx, |cx| {
            ProgressBar::horizontal(cx, AppData::progress).width(Pixels(300.0));
        });
    })
    .title("ProgressBar")
    .inner_size((750, 550))
    .run()
}
