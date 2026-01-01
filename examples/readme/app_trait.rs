use vizia::prelude::*;

struct Counter {
    count: Signal<i32>,
}

impl App for Counter {
    fn new(cx: &mut Context) -> Self {
        Self { count: cx.state(0) }
    }

    fn view(self, cx: &mut Context) -> Self {
        let doubled = cx.derived(move |s| self.count.get(s) * 2);

        VStack::new(cx, move |cx| {
            Label::new(cx, self.count);
            Label::new(cx, doubled);

            HStack::new(cx, move |cx| {
                Button::new(cx, |cx| Label::static_text(cx, "Decrement"))
                    .on_press(move |cx| self.count.update(cx, |n| *n -= 1));
                Button::new(cx, |cx| Label::static_text(cx, "Increment"))
                    .on_press(move |cx| self.count.update(cx, |n| *n += 1));
            });
        });

        self
    }
}

fn main() -> Result<(), ApplicationError> {
    let (app, (title, size)) = Application::new_with_state(|cx| {
        Counter::new(cx).view(cx);
        (cx.state("App Trait Example"), cx.state((300, 150)))
    });

    app.title(title).inner_size(size).run()
}
