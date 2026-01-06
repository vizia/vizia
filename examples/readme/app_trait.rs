use vizia::prelude::*;

struct Counter {
    count: Signal<i32>,
}

impl App for Counter {
    fn new(cx: &mut Context) -> Self {
        Self { count: cx.state(0) }
    }

    fn on_build(self, cx: &mut Context) -> Self {
        let doubled = self.count.drv(cx, |v, _| v * 2);

        VStack::new(cx, move |cx| {
            Label::new(cx, self.count);
            Label::new(cx, doubled);

            HStack::new(cx, move |cx| {
                Button::new(cx, |cx| Label::new(cx, "Decrement"))
                    .on_press(move |cx| self.count.update(cx, |n| *n -= 1));
                Button::new(cx, |cx| Label::new(cx, "Increment"))
                    .on_press(move |cx| self.count.update(cx, |n| *n += 1));
            });
        });

        self
    }

    fn window_config(&self) -> WindowConfig {
        window(|app| app.title("App Trait Example").inner_size((300, 150)))
    }
}

fn main() -> Result<(), ApplicationError> {
    Counter::run()
}
