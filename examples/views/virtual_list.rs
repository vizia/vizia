mod helpers;
use helpers::*;
use vizia::prelude::*;

struct VirtualListApp {
    list: Signal<Vec<u32>>,
    selectable_single: Signal<Selectable>,
    selection_follows_focus: Signal<bool>,
}

impl App for VirtualListApp {
    fn new(cx: &mut Context) -> Self {
        Self {
            list: cx.state((1..100u32).collect::<Vec<_>>()),
            selectable_single: cx.state(Selectable::Single),
            selection_follows_focus: cx.state(true),
        }
    }

    fn on_build(self, cx: &mut Context) -> Self {
        let list = self.list;
        let selectable_single = self.selectable_single;
        let selection_follows_focus = self.selection_follows_focus;

        ExamplePage::new(cx, move |cx| {
            VirtualList::new(cx, list, 40.0, move |cx, index, item| {
                let dark = cx.state(index % 2 == 0);
                Label::new(cx, item).toggle_class("dark", dark).hoverable(false)
            })
            .size(Pixels(300.0))
            .selectable(selectable_single)
            .selection_follows_focus(selection_follows_focus);
        });
        self
    }
}

fn main() -> Result<(), ApplicationError> {
    VirtualListApp::run()
}
