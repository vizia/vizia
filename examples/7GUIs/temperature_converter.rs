use vizia::prelude::*;

fn celsius_to_fahrenheit(temp: f32) -> f32 {
    temp * (9. / 5.) + 32.
}

fn fahrenheit_to_celsius(temp: f32) -> f32 {
    (temp - 32.) * (5. / 9.)
}

fn main() -> Result<(), ApplicationError> {
    let (app, (title, size)) = Application::new_with_state(|cx| {
        // Two writable signals that stay in sync
        let celsius = cx.state(5.0f32);
        let fahrenheit = cx.state(celsius_to_fahrenheit(5.0));
        let stretch_one = cx.state(Stretch(1.0));
        let align_center = cx.state(Alignment::Center);
        let gap_10 = cx.state(Pixels(10.0));

        HStack::new(cx, |cx| {
            // Celsius input - updates fahrenheit when edited
            Textbox::new(cx, celsius)
                .on_submit(move |cx, val, _| {
                    fahrenheit.set(cx, celsius_to_fahrenheit(val));
                })
                .width(stretch_one);
            Label::static_text(cx, "Celsius");

            // Fahrenheit input - updates celsius when edited
            Textbox::new(cx, fahrenheit)
                .on_submit(move |cx, val, _| {
                    celsius.set(cx, fahrenheit_to_celsius(val));
                })
                .width(stretch_one);
            Label::static_text(cx, "Fahrenheit");
        })
        .alignment(align_center)
        .horizontal_gap(gap_10);
        (cx.state("Temperature Converter"), cx.state((450, 100)))
    });

    app.title(title).inner_size(size).run()
}
