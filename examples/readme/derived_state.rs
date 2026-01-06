use vizia::prelude::*;

struct DerivedStateApp {
    number: Signal<i32>,
    title: Signal<&'static str>,
    size: Signal<(u32, u32)>,
}

impl App for DerivedStateApp {
    fn new(cx: &mut Context) -> Self {
        Self {
            number: cx.state(5i32),
            title: cx.state("Derived State"),
            size: cx.state((300, 200)),
        }
    }

    fn on_build(self, cx: &mut Context) -> Self {
        let number = self.number;

        // Derived signals recompute automatically - using .drv() for concise syntax
        let squared = number.drv(cx, |v, _| v * v);
        let is_even = number.drv(cx, |v, _| v % 2 == 0);
        let parity = is_even.drv(cx, |v, _| if *v { "even" } else { "odd" });

        VStack::new(cx, move |cx| {
            Label::new(cx, number);
            Label::new(cx, squared);
            Label::new(cx, parity);

            HStack::new(cx, move |cx| {
                Button::new(cx, |cx| Label::new(cx, "-"))
                    .on_press(move |cx| number.update(cx, |n| *n -= 1));
                Button::new(cx, |cx| Label::new(cx, "+"))
                    .on_press(move |cx| number.update(cx, |n| *n += 1));
            });
        });

        self
    }

    fn window_config(&self) -> WindowConfig {
        let title = self.title;
        let size = self.size;
        window(move |app| app.title(title).inner_size(size))
    }
}

fn main() -> Result<(), ApplicationError> {
    DerivedStateApp::run()
}
