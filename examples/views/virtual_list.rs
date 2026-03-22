mod helpers;
use helpers::*;
use vizia::prelude::*;

pub struct AppData {
    list: Signal<Vec<u32>>,
    selected: Signal<Vec<usize>>,
    selection_follows_focus: Signal<bool>,
}

pub enum AppEvent {
    SetSelected(usize),
}

impl Model for AppData {
    fn event(&mut self, _cx: &mut EventContext, event: &mut Event) {
        event.map(|app_event, _| match app_event {
            AppEvent::SetSelected(index) => self.selected.set(vec![*index]),
        });

        let _ = self.list;
        let _ = self.selection_follows_focus;
    }
}

fn main() -> Result<(), ApplicationError> {
    Application::new(|cx| {
        let list = Signal::new((1..100u32).collect::<Vec<_>>());
        let selected = Signal::new(vec![0usize]);
        let selection_follows_focus = Signal::new(true);
        AppData { list, selected, selection_follows_focus }.build(cx);

        ExamplePage::new(cx, |cx| {
            VirtualList::new(cx, list, 40.0, |cx, index, item| {
                Label::new(cx, item).toggle_class("dark", index % 2 == 0).hoverable(false)
            })
            .size(Pixels(300.0))
            .selected(selected)
            .on_select(|cx, index| cx.emit(AppEvent::SetSelected(index)))
            .selectable(Selectable::Single)
            .selection_follows_focus(selection_follows_focus);
        });
    })
    .title("Virtual List")
    .run()
}
