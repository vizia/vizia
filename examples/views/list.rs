mod helpers;
use helpers::*;
use vizia::prelude::*;

fn main() -> Result<(), ApplicationError> {
    let (app, title) = Application::new_with_state(|cx| {
        let list = cx.state((0..15u32).collect::<Vec<_>>());
        let orientation = cx.state(Orientation::Vertical);
        let is_horizontal = cx.derived({
            let orientation = orientation;
            move |s| *orientation.get(s) == Orientation::Horizontal
        });
        let not_hoverable = cx.state(false);
        let selectable_single = cx.state(Selectable::Single);
        let follows_focus = cx.state(true);

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
                Label::new(cx, item).hoverable(not_hoverable);
            })
            .orientation(orientation)
            .selectable(selectable_single);

            List::new(cx, list, move |cx, _index, item| {
                Label::new(cx, item).hoverable(not_hoverable);
            })
            .orientation(orientation)
            .selectable(selectable_single)
            .selection_follows_focus(follows_focus);
        });
        cx.state("List")
    });

    app.title(title).run()
}
