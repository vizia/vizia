use vizia::prelude::*;

// Application events
#[derive(Debug, Clone)]
pub enum CounterEvent {
    Increment,
    Decrement,
    Reset,
}

// Counter application using the App trait with application-level events
struct CounterApp {
    count: Signal<i32>,
}

impl App for CounterApp {
    fn new(cx: &mut Context) -> Self {
        Self { count: cx.state(0) }
    }

    fn on_build(self, cx: &mut Context) -> Self {
        VStack::new(cx, |cx| {
            Label::new(cx, "Counter App").font_size(24.0).font_weight(FontWeightKeyword::Bold);

            // Display the current count
            let count_text = cx.derived({
                let count = self.count;
                move |store| format!("Count: {}", count.get(store))
            });
            Label::new(cx, count_text).font_size(32.0);

            // Counter controls - emit events instead of directly updating
            HStack::new(cx, |cx| {
                Button::new(cx, |cx| Label::new(cx, "Decrement"))
                    .on_press(|cx| cx.emit(CounterEvent::Decrement));

                Button::new(cx, |cx| Label::new(cx, "Reset"))
                    .on_press(|cx| cx.emit(CounterEvent::Reset));

                Button::new(cx, |cx| Label::new(cx, "Increment"))
                    .on_press(|cx| cx.emit(CounterEvent::Increment));
            })
            .gap(Pixels(10.0));

            // Show derived state - whether count is even or odd
            let count_signal = self.count; // Copy the signal for use in derived computation
            let parity = cx.derived(move |s| {
                let count = count_signal.get(s);
                if *count % 2 == 0 { "Even" } else { "Odd" }.to_string()
            });

            let parity_text = cx.derived({
                let parity = parity;
                move |store| format!("Parity: {}", parity.get(store))
            });
            Label::new(cx, parity_text).font_size(16.0).color(Color::rgb(128, 128, 128));
        })
        .alignment(Alignment::Center)
        .gap(Pixels(20.0));

        self
    }

    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|counter_event, _| match counter_event {
            CounterEvent::Increment => {
                self.count.update(cx, |count| *count += 1);
            }
            CounterEvent::Decrement => {
                self.count.update(cx, |count| *count -= 1);
            }
            CounterEvent::Reset => {
                self.count.set(cx, 0);
            }
        });
    }

    fn window_config(&self) -> WindowConfig {
        window(|app| app.title("Counter").inner_size((400, 350)))
    }
}

fn main() -> Result<(), ApplicationError> {
    CounterApp::run()
}
