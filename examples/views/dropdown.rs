mod helpers;
use helpers::*;
use vizia::prelude::*;

#[derive(Lens)]
pub struct AppData {
    list: Vec<String>,
    selected: usize,
    choice: String,
}

pub enum AppEvent {
    SetSelected(usize),
}

impl Model for AppData {
    fn event(&mut self, _cx: &mut EventContext, event: &mut Event) {
        event.map(|app_event, _| match app_event {
            AppEvent::SetSelected(selected) => {
                self.selected = *selected;
                self.choice = self.list[*selected].clone();
            }
        })
    }
}

fn main() -> Result<(), ApplicationError> {
    Application::new(|cx| {
        AppData {
            list: vec!["Red".to_string(), "Green".to_string(), "Blue".to_string()],
            selected: 0,
            choice: "Red".to_string(),
        }
        .build(cx);

        ExamplePage::new(cx, |cx| {
            Dropdown::new(
                cx,
                move |cx| {
                    Button::new(cx, |cx| Label::new(cx, AppData::choice))
                        .on_press(|cx| cx.emit(PopupEvent::Switch));
                },
                move |cx| {
                    List::new(cx, AppData::list, |cx, _, item| {
                        Label::new(cx, item).hoverable(false);
                    })
                    .selectable(Selectable::Single)
                    .selected(AppData::selected.map(|s| vec![*s]))
                    .on_select(|cx, selected| {
                        cx.emit(AppEvent::SetSelected(selected));
                        cx.emit(PopupEvent::Close);
                    });
                },
            )
            .width(Pixels(100.0));
        });
    })
    .title("Dropdown")
    .inner_size((350, 300))
    .run()
}
