mod helpers;
use helpers::*;
use vizia::prelude::*;

fn main() -> Result<(), ApplicationError> {
    Application::new(|cx| {
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
    })
    .title("Divider")
    .inner_size((350, 300))
    .run()
}
