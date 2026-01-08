mod helpers;
use helpers::*;
use vizia::prelude::*;

struct RatingApp {
    rating1: Signal<u32>,
    rating2: Signal<u32>,
}

impl App for RatingApp {
    fn new(cx: &mut Context) -> Self {
        Self {
            rating1: cx.state(3u32),
            rating2: cx.state(7u32),
        }
    }

    fn on_build(self, cx: &mut Context) -> Self {
        let rating1 = self.rating1;
        let rating2 = self.rating2;

        ExamplePage::vertical(cx, move |cx| {
            Rating::new(cx, 5, rating1).two_way();
            Rating::new(cx, 10, rating2).two_way();
        });
        self
    }

    fn window_config(&self) -> WindowConfig {
        window(|app| app.inner_size((400, 200)))
    }
}

fn main() -> Result<(), ApplicationError> {
    RatingApp::run()
}
