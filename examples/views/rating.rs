mod helpers;
use helpers::*;
use vizia::prelude::*;

fn main() -> Result<(), ApplicationError> {
    RatingApp::run()
}

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
            Rating::new(cx, 5, rating1).on_change(move |cx, rating| {
                rating1.set(cx, rating);
            });
            Rating::new(cx, 10, rating2).on_change(move |cx, rating| {
                rating2.set(cx, rating);
            });
        });
        self
    }

    fn window_config(&self) -> WindowConfig {
        window(|app| app.title("Rating").inner_size((400, 200)))
    }
}
