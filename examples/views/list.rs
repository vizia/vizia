mod helpers;
use helpers::*;
use vizia::prelude::*;

#[derive(Lens)]
pub struct AppData {
    list: Vec<u32>,
    orientation: Orientation,
}

pub enum AppEvent {
    ToggleHorizontal,
}

impl Model for AppData {
    fn event(&mut self, _cx: &mut EventContext, event: &mut Event) {
        event.map(|app_event, _| match app_event {
            AppEvent::ToggleHorizontal => {
                if self.orientation == Orientation::Horizontal {
                    self.orientation = Orientation::Vertical;
                } else {
                    self.orientation = Orientation::Horizontal;
                }
            }
        });
    }
}

fn main() -> Result<(), ApplicationError> {
    Application::new(|cx| {
        let list: Vec<u32> = (0..15u32).collect();
        AppData { list, orientation: Orientation::Vertical }.build(cx);

        ExamplePage::new(cx, |cx| {
            Switch::new(
                cx,
                AppData::orientation.map(|orientation| *orientation == Orientation::Horizontal),
            )
            .on_toggle(|cx| cx.emit(AppEvent::ToggleHorizontal));

            List::new(cx, AppData::list, |cx, _, item| {
                Label::new(cx, item).hoverable(false);
            })
            .orientation(AppData::orientation)
            .selectable(Selectable::Single);

            List::new(cx, AppData::list, |cx, _, item| {
                Label::new(cx, item).hoverable(false);
            })
            .orientation(AppData::orientation)
            .selectable(Selectable::Single)
            .selection_follows_focus(true);

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
