mod helpers;
use helpers::*;
use vizia::prelude::*;

fn main() -> Result<(), ApplicationError> {
    ListApp::run()
}

struct ListApp {
    list: Signal<Vec<u32>>,
    orientation: Signal<Orientation>,
}

impl App for ListApp {
    fn new(cx: &mut Context) -> Self {
        Self {
            list: cx.state((0..15u32).collect::<Vec<_>>()),
            orientation: cx.state(Orientation::Vertical),
        }
    }

    fn on_build(self, cx: &mut Context) -> Self {
        let list = self.list;
        let orientation = self.orientation;

        let is_horizontal = cx.derived(move |s| *orientation.get(s) == Orientation::Horizontal);

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
            .selectable(Selectable::Single);

            List::new(cx, list, move |cx, _index, item| {
                Label::new(cx, item).hoverable(false);
            })
            .orientation(orientation)
            .selectable(Selectable::Single)
            .selection_follows_focus(true);
        });
        self
    }

    fn window_config(&self) -> WindowConfig {
        window(|app| app.title("List"))
    }
}
