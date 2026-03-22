mod helpers;
use helpers::*;
use vizia::prelude::*;

struct AppData {
    chips: Signal<Vec<String>>,
}

impl Model for AppData {
    fn event(&mut self, _cx: &mut EventContext, event: &mut Event) {
        event.map(|app_event, _| match app_event {
            AppEvent::CloseChip(index) => {
                self.chips.update(|chips| {
                    if *index < chips.len() {
                        chips.remove(*index);
                    }
                });
            }
        })
    }
}

enum AppEvent {
    CloseChip(usize),
}

fn main() -> Result<(), ApplicationError> {
    Application::new(|cx| {
        let chip = Signal::new("Chip".to_string());
        let chips = Signal::new(vec!["red".to_string(), "green".to_string(), "blue".to_string()]);

        AppData { chips }.build(cx);

        ExamplePage::vertical(cx, |cx| {
            Chip::new(cx, chip);

            Binding::new(cx, chips, move |cx, chips| {
                let chips = chips.clone();

                HStack::new(cx, move |cx| {
                    for (index, item) in chips.iter().enumerate() {
                        let item = item.clone();
                        Chip::new(cx, item).on_close(move |cx| cx.emit(AppEvent::CloseChip(index)));
                    }
                })
                .horizontal_gap(Pixels(4.0));
            });
        });
    })
    .title("Chip")
    .inner_size((400, 200))
    .run()
}
