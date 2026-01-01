mod helpers;
use helpers::*;
use vizia::prelude::*;

fn main() -> Result<(), ApplicationError> {
    let (app, title) = Application::new_with_state(|cx| {
        let options = cx.state(vec![
            "One",
            "Two",
            "Three",
            "Four",
            "Five",
            "Six really long",
            "Seven",
            "Eight",
            "Nine",
            "Ten",
            "Eleven",
            "Twelve",
        ]);
        let selected_option = cx.state(usize::MAX);
        let placeholder = cx.state("Select an option...");
        let width_150 = cx.state(Pixels(150.0));
        let width_100 = cx.state(Pixels(100.0));

        ExamplePage::vertical(cx, move |cx| {
            PickList::new(cx, options, selected_option, true)
                .placeholder(placeholder)
                .on_select(move |cx, index| selected_option.set(cx, index))
                .width(width_150);

            PickList::new(cx, options, selected_option, true)
                .placeholder(placeholder)
                .on_select(move |cx, index| selected_option.set(cx, index))
                .width(width_100);
        });
        cx.state("Picklist")
    });

    app.title(title).run()
}
