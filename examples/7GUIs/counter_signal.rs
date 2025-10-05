use vizia::prelude::*;

fn main() -> Result<(), ApplicationError> {
    Application::new(|cx| {
        Counter::new(cx);
    })
    .title("Counter")
    .inner_size((400, 100))
    .run()
}

struct Counter {
    count: Signal<i32>,
}

impl Counter {
    fn new(cx: &mut Context) -> Handle<'_, Self> {
        Self { count: cx.state(0) }.build(cx, |cx| {})
    }
}

impl View for Counter {
    fn element(&self) -> Option<&'static str> {
        Some("counter")
    }

    fn on_build(self, cx: &mut Context) -> Self {
        HStack::new(cx, move |cx| {
            Button::new(cx, |cx| Label::new(cx, "Increment"))
                .on_press(move |cx| self.count.update(cx, |v| *v += 1));

            Button::new(cx, |cx| Label::new(cx, "Decrement"))
                .on_press(move |cx| self.count.update(cx, |v| *v -= 1));

            Label::new(cx, self.count);

            // Derived state - only recomputed when the count changes
            let doubled = cx.derived(move |s| *self.count.get(s) * 2);

            Label::new(cx, doubled);
        })
        .alignment(Alignment::Center)
        .gap(Pixels(50.0));

        self
    }
}
