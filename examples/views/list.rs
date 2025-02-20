mod helpers;
use helpers::*;
use vizia::prelude::*;

#[derive(Lens)]
pub struct AppData {
    list: Vec<u32>,
    horizontal: bool,
    selected: usize,
}

pub enum AppEvent {
    ToggleHorizontal,
    SetSelected(usize),
}

impl Model for AppData {
    fn event(&mut self, _cx: &mut EventContext, event: &mut Event) {
        event.map(|app_event, _| match app_event {
            AppEvent::ToggleHorizontal => self.horizontal = !self.horizontal,
            AppEvent::SetSelected(index) => self.selected = *index,
        });
    }
}

fn main() -> Result<(), ApplicationError> {
    Application::new(|cx| {
        let list: Vec<u32> = (0..15u32).collect();
        AppData { list, horizontal: false, selected: 0 }.build(cx);

        ExamplePage::vertical(cx, |cx| {
            Switch::new(cx, AppData::horizontal)
                .on_toggle(|cx| cx.emit(AppEvent::ToggleHorizontal));
            List::new(cx, AppData::list, |cx, _, item| {
                Label::new(cx, item).hoverable(false);
            })
            .horizontal(AppData::horizontal)
            .selectable(Selectable::Single)
            .selected(AppData::selected.map(|s| vec![*s]))
            .selection_follows_focus(true)
            .on_select(|cx, index| cx.emit(AppEvent::SetSelected(index)));

            // List::new_filtered(
            //     cx,
            //     AppData::list,
            //     |item| *item % 2 == 0,
            //     |cx, _, item| {
            //         Label::new(cx, item).hoverable(false);
            //     },
            // )
            // .horizontal(AppData::horizontal)
            // .selectable(Selectable::Single)
            // .selection_follows_focus(true);
        });
    })
    .title("List")
    .run()
}
