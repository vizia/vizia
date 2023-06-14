mod helpers;
use helpers::*;
use vizia::prelude::*;

#[derive(Clone, Lens)]
struct AppData {
    chip: String,
    chips: Vec<String>,
}

impl Model for AppData {
    fn event(&mut self, _cx: &mut EventContext, event: &mut Event) {
        event.map(|app_event, _| match app_event {
            AppEvent::CloseChip(index) => {
                self.chips.remove(*index);
            }
        })
    }
}

enum AppEvent {
    CloseChip(usize),
}

fn main() {
    Application::new(|cx| {
        AppData {
            chip: "Chip".to_string(),
            chips: vec!["red".to_string(), "green".to_string(), "blue".to_string()],
        }
        .build(cx);

        ExamplePage::new(cx, |cx| {
            Chip::new(cx, AppData::chip).background_color(Color::from("#ff004444"));
            List::new(cx, AppData::chips, |cx, index, item| {
                Chip::new(cx, item)
                    .on_close(move |cx| cx.emit(AppEvent::CloseChip(index)))
                    .background_color(Color::from("#ff000044"));
            })
            .layout_type(LayoutType::Row);
        });
    })
    .title("Chip")
    .inner_size((400, 200))
    .run();
}
