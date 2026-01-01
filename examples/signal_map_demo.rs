use vizia::prelude::*;

// Example demonstrating signal map functionality
struct MapDemo {
    number: Signal<i32>,
}

impl App for MapDemo {
    fn new(cx: &mut Context) -> Self {
        Self { number: cx.state(5) }
    }

    fn view(self, cx: &mut Context) -> Self {
        let font_24 = cx.state(24.0);
        let font_18 = cx.state(18.0);
        let weight_bold = cx.state(FontWeightKeyword::Bold);
        let gap_10 = cx.state(Pixels(10.0));
        let gap_15 = cx.state(Pixels(15.0));
        let align_center = cx.state(Alignment::Center);

        VStack::new(cx, |cx| {
            Label::static_text(cx, "Signal Map Demo")
                .font_size(font_24)
                .font_weight(weight_bold);

            // Original value
            let original = cx.derived({
                let number = self.number;
                move |store| format!("Original: {}", number.get(store))
            });
            Label::new(cx, original).font_size(font_18);

            // Squared value using map
            let squared = cx.derived({
                let number = self.number;
                move |store| {
                    let n = number.get(store);
                    format!("Squared: {}", n * n)
                }
            });
            Label::new(cx, squared).font_size(font_18);

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
            Label::new(cx, parity).font_size(font_18);

            // Double using map
            let doubled = cx.derived({
                let number = self.number;
                move |store| format!("Double: {}", number.get(store) * 2)
            });
            Label::new(cx, doubled).font_size(font_18);

            // Controls
            HStack::new(cx, |cx| {
                Button::new(cx, |cx| Label::static_text(cx, "Decrement")).on_press(move |cx| {
                    self.number.update(cx, |n| *n -= 1);
                });

                Button::new(cx, |cx| Label::static_text(cx, "Increment")).on_press(move |cx| {
                    self.number.update(cx, |n| *n += 1);
                });

                Button::new(cx, |cx| Label::static_text(cx, "Reset to 5")).on_press(move |cx| {
                    self.number.set(cx, 5);
                });
            })
            .gap(gap_10);
        })
        .alignment(align_center)
        .gap(gap_15);

        self
    }
}

fn main() -> Result<(), ApplicationError> {
    let (app, (title, size)) = Application::new_with_state(|cx| {
        let title = cx.state("Signal Map Demo".to_string());
        let size = cx.state((400, 350));

        MapDemo::new(cx).view(cx);

        (title, size)
    });

    app.title(title).inner_size(size).run()
}
