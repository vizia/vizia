mod helpers;
use helpers::*;
use vizia::prelude::*;

fn main() -> Result<(), ApplicationError> {
    let (app, title) = Application::new_with_state(|cx| {
        let list = cx.state((1..100u32).collect::<Vec<_>>());
        let list_size = cx.state(Pixels(300.0));
        let selectable_single = cx.state(Selectable::Single);
        let follows_focus = cx.state(true);
        let not_hoverable = cx.state(false);

        ExamplePage::new(cx, move |cx| {
            VirtualList::new(cx, list, 40.0, move |cx, index, item| {
                let dark = cx.state(index % 2 == 0);
                Label::new(cx, item)
                    .toggle_class("dark", dark)
                    .hoverable(not_hoverable)
            })
            .size(list_size)
            .selectable(selectable_single)
            .selection_follows_focus(follows_focus);
        });
        cx.state("Virtual List")
    });

    app.title(title).run()
}
