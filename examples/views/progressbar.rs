mod helpers;
use helpers::*;
use std::cell::Cell;
use vizia::prelude::*;

fn main() -> Result<(), ApplicationError> {
    ProgressbarApp::run()
}

struct ProgressbarApp {
    progress: Signal<f32>,
}

impl App for ProgressbarApp {
    fn new(cx: &mut Context) -> Self {
        Self {
            progress: cx.state(0.0f32),
        }
    }

    fn on_build(self, cx: &mut Context) -> Self {
        let progress = self.progress;
        let interval = Duration::from_millis(100);
        let duration = Duration::from_secs(5);

        // Use Cell to allow the callback to access the timer handle
        let timer_handle: std::rc::Rc<Cell<Option<Timer>>> = std::rc::Rc::new(Cell::new(None));
        let timer_handle_clone = timer_handle.clone();

        let timer = cx.add_timer(interval, Some(duration), move |cx, action| match action {
            TimerAction::Tick(delta) => {
                let step = (interval + delta).as_secs_f32() / duration.as_secs_f32();
                progress.update(cx, |value| {
                    *value = (*value + step).clamp(0.0, 1.0);
                });
            }
            TimerAction::Stop => {
                progress.set(cx, 0.0);
                if let Some(t) = timer_handle_clone.get() {
                    cx.start_timer(t);
                }
            }
            _ => {}
        });

        timer_handle.set(Some(timer));
        cx.start_timer(timer);

        ExamplePage::vertical(cx, move |cx| {
            ProgressBar::horizontal(cx, progress).width(Pixels(300.0));
        });
        self
    }

    fn window_config(&self) -> WindowConfig {
        window(|app| app.title("ProgressBar").inner_size((750, 550)))
    }
}
