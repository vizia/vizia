mod helpers;
use helpers::*;
use vizia::prelude::*;

fn main() -> Result<(), ApplicationError> {
    VirtualListApp::run()
}

struct VirtualListApp {
    list: Signal<Vec<u32>>,
}

impl App for VirtualListApp {
    fn new(cx: &mut Context) -> Self {
        Self {
            list: cx.state((1..100u32).collect::<Vec<_>>()),
        }
    }

    fn on_build(self, cx: &mut Context) -> Self {
        let list = self.list;

        ExamplePage::new(cx, move |cx| {
            VirtualList::new(cx, list, 40.0, move |cx, index, item| {
                let dark = cx.state(index % 2 == 0);
                Label::new(cx, item).toggle_class("dark", dark).hoverable(false)
            })
            .size(Pixels(300.0))
            .selectable(Selectable::Single)
            .selection_follows_focus(true);
        });
        self
    }

    fn window_config(&self) -> WindowConfig {
        window(|app| app.title("Virtual List"))
    }
}
