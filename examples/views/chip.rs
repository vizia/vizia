mod helpers;
use helpers::*;
use vizia::prelude::*;
#[derive(Clone)]
struct AppData {
    chip: Signal<String>,
    chips: Signal<Vec<String>>,
}
impl Model for AppData {
    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|app_event, _| match app_event {
            AppEvent::CloseChip(index) => {
                self.chips.update(cx, |chips| {
                    chips.remove(*index);
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
        let chip = cx.state("Chip".to_string());
        let chips = cx.state(vec!["red".to_string(), "green".to_string(), "blue".to_string()]);

        AppData { chip, chips }.build(cx);

        ExamplePage::vertical(cx, |cx| {
            Chip::new(cx, chip);
            // List::new(cx, chips, |cx, index, item| {
            //     Chip::new(cx, item).on_close(move |cx| cx.emit(AppEvent::CloseChip(index)));
            // })
            // .orientation(Orientation::Horizontal)
            // .horizontal_gap(Pixels(4.0));
        });
    })
    .title("Chip")
    .inner_size((400, 200))
    .run()
}
