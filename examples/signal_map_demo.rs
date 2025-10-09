use vizia::prelude::*;

// Example demonstrating signal map functionality
struct MapDemo {
    number: Signal<i32>,
}

impl App for MapDemo {
    fn new(cx: &mut Context) -> Self {
        Self { number: cx.state(5) }
    }

    fn on_build(self, cx: &mut Context) -> Self {
        VStack::new(cx, |cx| {
            Label::new(cx, "Signal Map Demo").font_size(24.0).font_weight(FontWeightKeyword::Bold);

            // Original value
            Label::new(cx, self.number.map(|n| format!("Original: {}", n))).font_size(18.0);

            // Squared value using map
            Label::new(cx, self.number.map(|n| format!("Squared: {}", n * n))).font_size(18.0);

            // Even/odd using map
            Label::new(
                cx,
                self.number.map(|n| {
                    if n % 2 == 0 {
                        format!("{} is even", n)
                    } else {
                        format!("{} is odd", n)
                    }
                }),
            )
            .font_size(18.0);

            // Double using map
            Label::new(cx, self.number.map(|n| format!("Double: {}", n * 2))).font_size(18.0);

            // Controls
            HStack::new(cx, |cx| {
                Button::new(cx, |cx| Label::new(cx, "Decrement")).on_press(move |cx| {
                    self.number.update(cx, |n| *n -= 1);
                });

                Button::new(cx, |cx| Label::new(cx, "Increment")).on_press(move |cx| {
                    self.number.update(cx, |n| *n += 1);
                });

                Button::new(cx, |cx| Label::new(cx, "Reset to 5")).on_press(move |cx| {
                    self.number.set(cx, 5);
                });
            })
            .gap(Pixels(10.0));
        })
        .alignment(Alignment::Center)
        .gap(Pixels(15.0));

        self
    }
}

fn main() -> Result<(), ApplicationError> {
    MapDemo::build().title("Signal Map Demo").inner_size((400, 350)).run()
}
