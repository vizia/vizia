use vizia::prelude::*;

struct CounterApp {
    count: Signal<i32>,
}

impl App for CounterApp {
    fn new(cx: &mut Context) -> Self {
        Self {
            count: cx.state(0i32),
        }
    }

    fn on_build(self, cx: &mut Context) -> Self {
        let count = self.count;
        HStack::new(cx, move |cx| {
            Button::new(cx, |cx| Label::new(cx, "Increment"))
                .on_press(move |cx| count.upd(cx, |value| *value += 1));

            Button::new(cx, |cx| Label::new(cx, "Decrement"))
                .on_press(move |cx| count.upd(cx, |value| *value -= 1));

            Label::new(cx, count);
        })
        .alignment(Alignment::Center)
        .gap(Pixels(50.0));

        self
    }

    fn window_config(&self) -> WindowConfig {
        window(|app| app.inner_size((400, 100)))
    }
}

fn main() -> Result<(), ApplicationError> {
    CounterApp::run()
}
