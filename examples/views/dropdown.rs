mod helpers;
use helpers::*;
use vizia::prelude::*;

struct DropdownApp {
    list: Signal<Vec<String>>,
    selected: Signal<usize>,
    selectable_single: Signal<Selectable>,
}

impl App for DropdownApp {
    fn new(cx: &mut Context) -> Self {
        Self {
            list: cx.state(vec!["Red".to_string(), "Green".to_string(), "Blue".to_string()]),
            selected: cx.state(0usize),
            selectable_single: cx.state(Selectable::Single),
        }
    }

    fn on_build(self, cx: &mut Context) -> Self {
        let list = self.list;
        let selected = self.selected;
        let selectable_single = self.selectable_single;

        // Derived signal for the selected item text
        let selected_text = cx.derived(move |s| {
            let idx = *selected.get(s);
            let items = list.get(s);
            items.as_slice().get(idx).cloned().unwrap_or_default()
        });
        let selected_indices = cx.derived(move |s| vec![*selected.get(s)]);

        ExamplePage::new(cx, |cx| {
            Dropdown::new(
                cx,
                move |cx| {
                    Button::new(cx, |cx| Label::new(cx, selected_text))
                        .on_press(|cx| cx.emit(PopupEvent::Switch));
                },
                move |cx| {
                    List::new(cx, list, move |cx, _, item| {
                        Label::new(cx, item).hoverable(false);
                    })
                    .selectable(selectable_single)
                    .selected(selected_indices)
                    .on_select(move |cx, sel| {
                        selected.set(cx, sel);
                        cx.emit(PopupEvent::Close);
                    });
                },
            )
            .width(Pixels(100.0));
        });
        self
    }

    fn window_config(&self) -> WindowConfig {
        window(|app| app.title("Dropdown").inner_size((350, 300)))
    }
}

fn main() -> Result<(), ApplicationError> {
    DropdownApp::run()
}
