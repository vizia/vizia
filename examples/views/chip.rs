mod helpers;
use helpers::*;
use vizia::prelude::*;
fn main() -> Result<(), ApplicationError> {
    ChipApp::run()
}

struct ChipApp {
    chips: Signal<Vec<String>>,
}

impl App for ChipApp {
    fn new(cx: &mut Context) -> Self {
        Self {
            chips: cx.state(vec!["red".to_string(), "green".to_string(), "blue".to_string()]),
        }
    }

    fn on_build(self, cx: &mut Context) -> Self {
        let chips = self.chips;
        ExamplePage::vertical(cx, |cx| {
            Chip::new(cx, "Chip");
            List::new(cx, chips, move |cx, index, item| {
                let chips = chips;
                Chip::new(cx, item).on_close(move |cx| {
                    chips.update(cx, |chips| {
                        if index < chips.len() {
                            chips.remove(index);
                        }
                    });
                });
            })
            .orientation(Orientation::Horizontal)
            .horizontal_gap(Pixels(4.0));
        });
        self
    }

    fn window_config(&self) -> WindowConfig {
        window(|app| app.title("Chip").inner_size((400, 200)))
    }
}
