mod helpers;
use helpers::*;
use vizia::prelude::*;

#[derive(Lens)]
pub struct AppData {
    list: Vec<u32>,
    horizontal: bool,
}

pub enum AppEvent {
    ToggleHorizontal,
}

impl Model for AppData {
    fn event(&mut self, _cx: &mut EventContext, event: &mut Event) {
        event.map(|app_event, _| match app_event {
            AppEvent::ToggleHorizontal => self.horizontal = !self.horizontal,
        });
    }
}

fn main() -> Result<(), ApplicationError> {
    Application::new(|cx| {
        let list: Vec<u32> = (0..15u32).collect();
        AppData { list, horizontal: false }.build(cx);

        ExamplePage::vertical(cx, |cx| {
            Switch::new(cx, AppData::horizontal)
                .on_toggle(|cx| cx.emit(AppEvent::ToggleHorizontal));
            List::new(cx, AppData::list, |cx, _, item| {
                Label::new(cx, item).hoverable(false);
            })
            .horizontal(AppData::horizontal)
            .selectable(Selectable::Single)
            .selection_follows_focus(true);

            List::new_filtered(
                cx,
                AppData::list,
                |item| *item % 2 == 0,
                |cx, _, item| {
                    Label::new(cx, item).hoverable(false);
                },
            )
            .horizontal(AppData::horizontal)
            .selectable(Selectable::Single)
            .selection_follows_focus(true);
        });
    })
    .title("List")
    .run()
}
