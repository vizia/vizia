mod helpers;
use helpers::*;
use vizia::prelude::*;

fn main() -> Result<(), ApplicationError> {
    let (app, (title, size)) = Application::new_with_state(|cx| {
        let value = cx.state(0.2f32);

        ExamplePage::new(cx, move |cx| {
            Knob::new(cx, 0.5, value, false).on_change(move |cx, val| {
                value.set(cx, val);
            });
        });
        (cx.state("Knob"), cx.state((300, 300)))
    });

    app.title(title).inner_size(size).run()
}
