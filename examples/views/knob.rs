mod helpers;
use helpers::*;
use vizia::prelude::*;

fn main() -> Result<(), ApplicationError> {
    Application::new(|cx| {
        let value = cx.state(0.2f32);

        ExamplePage::new(cx, move |cx| {
            Knob::new(cx, 0.5, value, false).on_change(move |cx, val| {
                value.set(cx, val);
            });
        });
    })
    .title("Knob")
    .inner_size((300, 300))
    .run()
}
