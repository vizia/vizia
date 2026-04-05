mod helpers;
use helpers::*;
use vizia::prelude::*;

pub struct AppData {
    list: Signal<Vec<Signal<u32>>>,
    orientation: Signal<Orientation>,
    selectable: Signal<Selectable>,
    selection_follows_focus: Signal<bool>,
    show_horizontal_scrollbar: Signal<bool>,
    show_vertical_scrollbar: Signal<bool>,
    scroll_x: Signal<f32>,
    scroll_y: Signal<f32>,
}

pub enum AppEvent {
    ToggleHorizontal,
}

impl Model for AppData {
    fn event(&mut self, _cx: &mut EventContext, event: &mut Event) {
        event.map(|app_event, _| match app_event {
            AppEvent::ToggleHorizontal => {
                self.orientation.update(|orientation| {
                    if *orientation == Orientation::Horizontal {
                        *orientation = Orientation::Vertical;
                    } else {
                        *orientation = Orientation::Horizontal;
                    }
                });
            }
        });

        let _ = self.list;
        let _ = self.selectable;
        let _ = self.selection_follows_focus;
        let _ = self.show_horizontal_scrollbar;
        let _ = self.show_vertical_scrollbar;
        let _ = self.scroll_x;
        let _ = self.scroll_y;
    }
}

fn main() -> Result<(), ApplicationError> {
    Application::new(|cx| {
        let list = Signal::new((0..15u32).map(Signal::new).collect::<Vec<_>>());
        let orientation = Signal::new(Orientation::Vertical);
        let selectable = Signal::new(Selectable::Single);
        let selection_follows_focus = Signal::new(true);
        let show_horizontal_scrollbar = Signal::new(true);
        let show_vertical_scrollbar = Signal::new(true);
        let scroll_x = Signal::new(0.0f32);
        let scroll_y = Signal::new(0.0f32);
        let is_horizontal = Memo::new(move |_| orientation.get() == Orientation::Horizontal);

        AppData {
            list,
            orientation,
            selectable,
            selection_follows_focus,
            show_horizontal_scrollbar,
            show_vertical_scrollbar,
            scroll_x,
            scroll_y,
        }
        .build(cx);

        ExamplePage::new(cx, |cx| {
            Switch::new(cx, is_horizontal).on_toggle(|cx| cx.emit(AppEvent::ToggleHorizontal));

            List::new(cx, list, |cx, _, item| {
                Label::new(cx, item).hoverable(false);
            })
            .orientation(orientation)
            .selectable(selectable)
            .show_horizontal_scrollbar(show_horizontal_scrollbar)
            .show_vertical_scrollbar(show_vertical_scrollbar)
            .scroll_x(scroll_x)
            .scroll_y(scroll_y);

            List::new(cx, list, |cx, _, item| {
                Label::new(cx, item).hoverable(false);
            })
            .orientation(orientation)
            .selectable(selectable)
            .selection_follows_focus(selection_follows_focus)
            .show_horizontal_scrollbar(show_horizontal_scrollbar)
            .show_vertical_scrollbar(show_vertical_scrollbar)
            .scroll_x(scroll_x)
            .scroll_y(scroll_y);

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
