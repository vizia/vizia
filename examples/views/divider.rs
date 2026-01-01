mod helpers;
use helpers::*;
use vizia::prelude::*;

fn main() -> Result<(), ApplicationError> {
    let (app, (title, size)) = Application::new_with_state(|cx| {
        ExamplePage::new(cx, |cx| {
            HStack::new(cx, |cx| {
                Divider::new(cx);
                Divider::horizontal(cx);
                Divider::vertical(cx);
            });
            VStack::new(cx, |cx| {
                Divider::new(cx);
                Divider::horizontal(cx);
                Divider::vertical(cx);
            });
        });
        (cx.state("Divider"), cx.state((350, 300)))
    });

    app.title(title).inner_size(size).run()
}
