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

        // Derived signals recompute automatically
        let squared = cx.derived(move |s| number.get(s) * number.get(s));
        let is_even = cx.derived(move |s| number.get(s) % 2 == 0);
        let parity = cx.derived(move |s| if *is_even.get(s) { "even" } else { "odd" });

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
