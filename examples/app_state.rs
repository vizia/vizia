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

    fn view(self, cx: &mut Context) -> Self {
        let title_size = cx.state(24.0);
        let title_weight = cx.state(FontWeightKeyword::Bold);
        let count_size = cx.state(32.0);
        let gap_10 = cx.state(Pixels(10.0));
        let parity_size = cx.state(16.0);
        let parity_color = cx.state(Color::rgb(128, 128, 128));
        let align_center = cx.state(Alignment::Center);
        let gap_20 = cx.state(Pixels(20.0));
        VStack::new(cx, |cx| {
            Label::static_text(cx, "Counter App")
                .font_size(title_size)
                .font_weight(title_weight);

            // Display the current count
            let count_text = cx.derived({
                let count = self.count;
                move |store| format!("Count: {}", count.get(store))
            });
            Label::new(cx, count_text).font_size(count_size);

            // Counter controls - emit events instead of directly updating
            HStack::new(cx, |cx| {
                Button::new(cx, |cx| Label::static_text(cx, "Decrement"))
                    .on_press(|cx| cx.emit(CounterEvent::Decrement));

                Button::new(cx, |cx| Label::static_text(cx, "Reset"))
                    .on_press(|cx| cx.emit(CounterEvent::Reset));

                Button::new(cx, |cx| Label::static_text(cx, "Increment"))
                    .on_press(|cx| cx.emit(CounterEvent::Increment));
            })
            .gap(gap_10);

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
            Label::new(cx, parity_text)
                .font_size(parity_size)
                .color(parity_color);
        })
        .alignment(align_center)
        .gap(gap_20);

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
}

fn main() -> Result<(), ApplicationError> {
    let (app, (title, size)) = Application::new_with_state(|cx| {
        let mut app = CounterApp::new(cx);
        app = app.view(cx);
        app.build(cx);
        (cx.state("Counter"), cx.state((400, 350)))
    });

    app.title(title).inner_size(size).run()
}
