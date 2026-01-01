use vizia::prelude::*;

fn celsius_to_fahrenheit(c: f32) -> f32 { c * 9.0 / 5.0 + 32.0 }
fn fahrenheit_to_celsius(f: f32) -> f32 { (f - 32.0) * 5.0 / 9.0 }

fn main() -> Result<(), ApplicationError> {
    let (app, (title, size)) = Application::new_with_state(|cx| {
        let celsius = cx.state(20.0f32);
        let fahrenheit = cx.state(celsius_to_fahrenheit(20.0));

        HStack::new(cx, |cx| {
            Textbox::new(cx, celsius)
                .on_submit(move |cx, val, _| {
                    fahrenheit.set(cx, celsius_to_fahrenheit(val));
                });
            Label::static_text(cx, "C");

            Textbox::new(cx, fahrenheit)
                .on_submit(move |cx, val, _| {
                    celsius.set(cx, fahrenheit_to_celsius(val));
                });
            Label::static_text(cx, "F");
        });

        (cx.state("Temperature Converter"), cx.state((400, 100)))
    });

    app.title(title).inner_size(size).run()
}
