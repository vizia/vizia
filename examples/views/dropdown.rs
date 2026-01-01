mod helpers;
use helpers::*;
use vizia::prelude::*;

fn main() -> Result<(), ApplicationError> {
    let (app, (title, size)) = Application::new_with_state(|cx| {
        let list = cx.state(vec!["Red".to_string(), "Green".to_string(), "Blue".to_string()]);
        let selected = cx.state(0usize);
        let dropdown_width = cx.state(Pixels(100.0));
        let selectable_single = cx.state(Selectable::Single);
        let not_hoverable = cx.state(false);

        // Derived signal for the selected item text
        let selected_text = cx.derived({
            let list = list;
            let selected = selected;
            move |s| {
                let idx = *selected.get(s);
                let items = list.get(s);
                items.as_slice().get(idx).cloned().unwrap_or_default()
            }
        });
        let selected_indices = cx.derived({
            let selected = selected;
            move |s| vec![*selected.get(s)]
        });

        ExamplePage::new(cx, |cx| {
            Dropdown::new(
                cx,
                move |cx| {
                    Button::new(cx, |cx| Label::new(cx, selected_text))
                        .on_press(|cx| cx.emit(PopupEvent::Switch));
                },
                move |cx| {
                    List::new(cx, list, move |cx, _, item| {
                        Label::new(cx, item).hoverable(not_hoverable);
                    })
                    .selectable(selectable_single)
                    .selected(selected_indices)
                    .on_select(move |cx, sel| {
                        selected.set(cx, sel);
                        cx.emit(PopupEvent::Close);
                    });
                },
            )
            .width(dropdown_width);
        });
        (cx.state("Dropdown"), cx.state((350, 300)))
    });

    app.title(title).inner_size(size).run()
}
