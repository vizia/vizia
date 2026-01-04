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
            let original = cx.derived({
                let number = self.number;
                move |store| format!("Original: {}", number.get(store))
            });
            Label::new(cx, original).font_size(18.0);

            // Squared value using map
            let squared = cx.derived({
                let number = self.number;
                move |store| {
                    let n = number.get(store);
                    format!("Squared: {}", n * n)
                }
            });
            Label::new(cx, squared).font_size(18.0);

            // Even/odd using map
            let parity = cx.derived({
                let number = self.number;
                move |store| {
                    let n = number.get(store);
                    if n % 2 == 0 {
                        format!("{n} is even")
                    } else {
                        format!("{n} is odd")
                    }
                }
            });
            Label::new(cx, parity).font_size(18.0);

            // Double using map
            let doubled = cx.derived({
                let number = self.number;
                move |store| format!("Double: {}", number.get(store) * 2)
            });
            Label::new(cx, doubled).font_size(18.0);

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

    fn window_config(&self) -> WindowConfig {
        window(|app| app.title("Signal Map Demo").inner_size((400, 350)))
    }
}

fn main() -> Result<(), ApplicationError> {
    MapDemo::run()
}
