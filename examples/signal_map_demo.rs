use vizia::prelude::*;

// Example demonstrating Signal::drv() for concise derived signals
struct MapDemo {
    number: Signal<i32>,
}

impl App for MapDemo {
    fn app_name() -> &'static str {
        "Signal::drv() Demo"
    }

    fn new(cx: &mut Context) -> Self {
        Self { number: cx.state(5) }
    }

    fn on_build(self, cx: &mut Context) -> Self {
        VStack::new(cx, |cx| {
            Label::new(cx, "Signal::drv() Demo").font_size(24.0).font_weight(FontWeightKeyword::Bold);

            // Original value - using drv for simple formatting
            let original = self.number.drv(cx, |v, _| format!("Original: {v}"));
            Label::new(cx, original).font_size(18.0);

            // Squared value - drv with computation
            let squared = self.number.drv(cx, |v, _| format!("Squared: {}", v * v));
            Label::new(cx, squared).font_size(18.0);

            // Even/odd - drv with conditional
            let parity = self.number.drv(cx, |v, _| {
                if v % 2 == 0 {
                    format!("{v} is even")
                } else {
                    format!("{v} is odd")
                }
            });
            Label::new(cx, parity).font_size(18.0);

            // Double - drv one-liner
            let doubled = self.number.drv(cx, |v, _| format!("Double: {}", v * 2));
            Label::new(cx, doubled).font_size(18.0);

            // Controls
            HStack::new(cx, |cx| {
                Button::new(cx, |cx| Label::new(cx, "Decrement")).on_press(move |cx| {
                    self.number.upd(cx, |n| *n -= 1);
                });

                Button::new(cx, |cx| Label::new(cx, "Increment")).on_press(move |cx| {
                    self.number.upd(cx, |n| *n += 1);
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
        window(|app| app.inner_size((400, 350)))
    }
}

fn main() -> Result<(), ApplicationError> {
    MapDemo::run()
}
