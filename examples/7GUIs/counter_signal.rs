use vizia::prelude::*;

fn main() -> Result<(), ApplicationError> {
    let (app, (title, size)) = Application::new_with_state(|cx| {
        Counter::new(cx);
        (cx.state("Counter"), cx.state((400, 100)))
    });

    app.title(title).inner_size(size).run()
}

struct Counter {
    count: Signal<i32>,
}

pub enum CounterEvent {
    Increment,
    Decrement,
}

impl Counter {
    fn new(cx: &mut Context) -> Handle<'_, Self> {
        Self { count: cx.state(0) }.build(cx, |_cx| {})
    }
}

impl View for Counter {
    fn element(&self) -> Option<&'static str> {
        Some("counter")
    }

    fn view(self, cx: &mut Context) -> Self {
        let align_center = cx.state(Alignment::Center);
        let gap_50 = cx.state(Pixels(50.0));
        HStack::new(cx, move |cx| {
            Button::new(cx, |cx| Label::static_text(cx, "Increment"))
                .on_press(|cx| cx.emit(CounterEvent::Increment));

            Button::new(cx, |cx| Label::static_text(cx, "Decrement"))
                .on_press(|cx| cx.emit(CounterEvent::Decrement));

            Label::new(cx, self.count);

            // Derived state - only recomputed when the count changes
            let doubled = cx.derived(move |s| *self.count.get(s) * 2);

            Label::new(cx, doubled);
        })
        .alignment(align_center)
        .gap(gap_50);

        self
    }

    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|app_event, _| match app_event {
            CounterEvent::Increment => {
                self.count.update(cx, |count| *count += 1);
            }
            CounterEvent::Decrement => {
                self.count.update(cx, |count| *count -= 1);
            }
        });
    }
}
