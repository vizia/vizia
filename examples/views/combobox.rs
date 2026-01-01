mod helpers;
use helpers::*;
use vizia::prelude::*;

fn main() -> Result<(), ApplicationError> {
    let (app, (title, size)) = Application::new_with_state(|cx| {
        let options = cx.state(vec![
            "One",
            "Two",
            "Three",
            "Four",
            "Five",
            "Six something long",
            "Seven",
            "Eight",
            "Nine",
            "Ten",
        ]);
        let selected_option = cx.state(0usize);
        let combo_width = cx.state(Pixels(140.0));
        let combo_top = cx.state(Pixels(100.0));

        ExamplePage::new(cx, |cx| {
            ComboBox::new(cx, options, selected_option)
                .width(combo_width)
                .top(combo_top);
        });
        (cx.state("Combobox"), cx.state((400, 400)))
    });

    app.title(title).inner_size(size).run()
}
