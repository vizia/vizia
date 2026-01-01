mod helpers;
use helpers::*;
use vizia::prelude::*;

fn main() -> Result<(), ApplicationError> {
    let (app, (title, size)) = Application::new_with_state(|cx| {
        let rating1 = cx.state(3u32);
        let rating2 = cx.state(7u32);

        ExamplePage::vertical(cx, move |cx| {
            Rating::new(cx, 5, rating1).on_change(move |cx, rating| {
                rating1.set(cx, rating);
            });
            Rating::new(cx, 10, rating2).on_change(move |cx, rating| {
                rating2.set(cx, rating);
            });
        });
        (cx.state("Rating"), cx.state((400, 200)))
    });

    app.title(title).inner_size(size).run()
}
