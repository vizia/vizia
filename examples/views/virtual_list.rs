mod helpers;
use helpers::*;
use vizia::prelude::*;

pub struct AppData {
    selected: Signal<Vec<usize>>,
}

pub enum AppEvent {
    SetSelected(usize),
}

impl Model for AppData {
    fn event(&mut self, _cx: &mut EventContext, event: &mut Event) {
        event.map(|app_event, _| match app_event {
            AppEvent::SetSelected(index) => self.selected.set(vec![*index]),
        });
    }
}

fn main() -> Result<(), ApplicationError> {
    Application::new(|cx| {
        let list = Signal::new((1..100u32).collect::<Vec<_>>());
        let selected = Signal::new(vec![0usize]);
        let selection_follows_focus = Signal::new(true);
        AppData { selected }.build(cx);

        ExamplePage::new(cx, |cx| {
            VirtualList::new(cx, list, 40.0, |cx, index, item| {
                Label::new(cx, item).toggle_class("dark", index % 2 == 0).hoverable(false)
            })
            .size(Pixels(300.0))
            .selection(selected)
            .on_select(|cx, index| cx.emit(AppEvent::SetSelected(index)))
            .selectable(Selectable::Single)
            .selection_follows_focus(selection_follows_focus);
        });
    })
    .title(Localized::new("view-title-virtual-list"))
    .run()
}
