mod helpers;
use helpers::*;
use vizia::prelude::*;
fn main() -> Result<(), ApplicationError> {
    let (app, (title, size)) = Application::new_with_state(|cx| {
        let chips = cx.state(vec!["red".to_string(), "green".to_string(), "blue".to_string()]);
        let orientation = cx.state(Orientation::Horizontal);
        let gap_4 = cx.state(Pixels(4.0));

        ExamplePage::vertical(cx, |cx| {
            Chip::static_text(cx, "Chip");
            List::new(cx, chips, move |cx, index, item| {
                let chips = chips;
                Chip::new(cx, item).on_close(move |cx| {
                    chips.update(cx, |chips| {
                        if index < chips.len() {
                            chips.remove(index);
                        }
                    });
                });
            })
            .orientation(orientation)
            .horizontal_gap(gap_4);
        });
        (cx.state("Chip"), cx.state((400, 200)))
    });

    app.title(title).inner_size(size).run()
}
