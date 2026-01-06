mod helpers;
use helpers::*;
use vizia::prelude::*;

struct ListApp {
    list: Signal<Vec<u32>>,
    orientation: Signal<Orientation>,
    selectable_single: Signal<Selectable>,
    selection_follows_focus: Signal<bool>,
}

impl App for ListApp {
    fn new(cx: &mut Context) -> Self {
        Self {
            list: cx.state((0..15u32).collect::<Vec<_>>()),
            orientation: cx.state(Orientation::Vertical),
            selectable_single: cx.state(Selectable::Single),
            selection_follows_focus: cx.state(true),
        }
    }

    fn on_build(self, cx: &mut Context) -> Self {
        let list = self.list;
        let orientation = self.orientation;
        let selectable_single = self.selectable_single;
        let selection_follows_focus = self.selection_follows_focus;

        let is_horizontal = orientation.drv(cx, |v, _| *v == Orientation::Horizontal);

        ExamplePage::new(cx, move |cx| {
            Switch::new(cx, is_horizontal).on_toggle(move |cx| {
                orientation.update(cx, |o| {
                    *o = if *o == Orientation::Horizontal {
                        Orientation::Vertical
                    } else {
                        Orientation::Horizontal
                    };
                });
            });

            List::new(cx, list, move |cx, _index, item| {
                Label::new(cx, item).hoverable(false);
            })
            .orientation(orientation)
            .selectable(selectable_single);

            List::new(cx, list, move |cx, _index, item| {
                Label::new(cx, item).hoverable(false);
            })
            .orientation(orientation)
            .selectable(selectable_single)
            .selection_follows_focus(selection_follows_focus);
        });
        self
    }

    fn window_config(&self) -> WindowConfig {
        window(|app| app.title("List"))
    }
}

fn main() -> Result<(), ApplicationError> {
    ListApp::run()
}
