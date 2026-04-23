mod helpers;
use helpers::*;
use vizia::prelude::*;

pub struct AppData {
    progress: Signal<f32>,
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
                let progress = cx
                    .query_timer(self.timer, |timer_state| timer_state.progress().unwrap())
                    .unwrap();
                self.progress.set(progress);
                if progress >= 1.0 {
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

        let progress = Signal::new(0.0);
        AppData { progress, timer }.build(cx);

        ExamplePage::vertical(cx, |cx| {
            ProgressBar::horizontal(cx, progress).width(Pixels(300.0));
        });
    })
    .title(Localized::new("view-title-progressbar"))
    .inner_size((750, 550))
    .run()
}
