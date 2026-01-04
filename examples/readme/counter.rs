use vizia::prelude::*;

fn main() -> Result<(), ApplicationError> {
    CounterApp::run()
}

struct CounterApp {
    count: Signal<i32>,
    title: Signal<&'static str>,
    size: Signal<(u32, u32)>,
}

impl App for CounterApp {
    fn new(cx: &mut Context) -> Self {
        Self {
            count: cx.state(0i32),
            title: cx.state("Counter"),
            size: cx.state((200, 100)),
        }
    }

    fn on_build(self, cx: &mut Context) -> Self {
        let count = self.count;
        HStack::new(cx, move |cx| {
            Button::new(cx, |cx| Label::new(cx, "-"))
                .on_press(move |cx| count.update(cx, |n| *n -= 1));

            Label::new(cx, count);

            Button::new(cx, |cx| Label::new(cx, "+"))
                .on_press(move |cx| count.update(cx, |n| *n += 1));
        });

        self
    }

    fn window_config(&self) -> WindowConfig {
        let title = self.title;
        let size = self.size;
        window(move |app| app.title(title).inner_size(size))
    }
}
