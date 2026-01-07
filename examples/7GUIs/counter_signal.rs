use vizia::prelude::*;

pub enum CounterEvent {
    Increment,
    Decrement,
}

struct CounterApp {
    count: Signal<i32>,
}

impl App for CounterApp {
    fn app_name() -> &'static str {
        "Counter"
    }

    fn new(cx: &mut Context) -> Self {
        Self { count: cx.state(0) }
    }

    fn on_build(self, cx: &mut Context) -> Self {
        let count = self.count;
        let doubled = count.drv(cx, |v, _| *v * 2);

        HStack::new(cx, move |cx| {
            Button::new(cx, |cx| Label::new(cx, "Increment"))
                .on_press(|cx| cx.emit(CounterEvent::Increment));

            Button::new(cx, |cx| Label::new(cx, "Decrement"))
                .on_press(|cx| cx.emit(CounterEvent::Decrement));

            Label::new(cx, count);
            Label::new(cx, doubled);
        })
        .alignment(Alignment::Center)
        .gap(Pixels(50.0));

        self
    }

    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|app_event, _| match app_event {
            CounterEvent::Increment => {
                self.count.upd(cx, |count| *count += 1);
            }
            CounterEvent::Decrement => {
                self.count.upd(cx, |count| *count -= 1);
            }
        });
    }

    fn window_config(&self) -> WindowConfig {
        window(|app| app.inner_size((400, 100)))
    }
}

fn main() -> Result<(), ApplicationError> {
    CounterApp::run()
}
