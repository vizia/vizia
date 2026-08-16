mod helpers;
use helpers::*;
use vizia::prelude::*;

pub struct AppData {
    selected: Signal<Vec<usize>>,
    horizontal: Signal<bool>,
}

pub enum AppEvent {
    SetSelected(usize),
    ToggleHorizontal,
}

impl Model for AppData {
    fn event(&mut self, _cx: &mut EventContext, event: &mut Event) {
        event.map(|app_event, _| match app_event {
            AppEvent::SetSelected(index) => self.selected.set(vec![*index]),
            AppEvent::ToggleHorizontal => self.horizontal.update(|horizontal| *horizontal ^= true),
        });
    }
}

fn main() -> Result<(), ApplicationError> {
    Application::new(|cx| {
        let list = Signal::new((1..100u32).collect::<Vec<_>>());
        let selected = Signal::new(vec![0usize]);
        let horizontal = Signal::new(false);
        let selection_follows_focus = Signal::new(true);
        AppData { selected, horizontal }.build(cx);

        ExamplePage::new(cx, |cx| {
            Switch::new(cx, horizontal).on_toggle(|cx| cx.emit(AppEvent::ToggleHorizontal));

            VirtualList::new(cx, list, 40.0, |cx, index, item| {
                Label::new(cx, item).toggle_class("dark", index % 2 == 0).hoverable(false)
            })
            .size(Pixels(300.0))
            .horizontal(horizontal)
            .show_horizontal_scrollbar(horizontal)
            .show_vertical_scrollbar(horizontal.map(|h| !*h))
            .selection(selected)
            .on_select(|cx, index| cx.emit(AppEvent::SetSelected(index)))
            .selectable(Selectable::Single)
            .selection_follows_focus(selection_follows_focus)
            .type_ahead_text(move |_cx, index| list.get().get(index).map(|item| item.to_string()));
        });
    })
    .title(Localized::new("view-title-virtual-list"))
    .run()
}
