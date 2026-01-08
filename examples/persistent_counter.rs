//! Persistent Counter Example
//!
//! Demonstrates `cx.state_persists()` - the counter value survives app restarts.
//! Data is saved to the platform's app data directory (e.g., ~/Library/Application Support/Persistent_Counter/signals/).
//!
//! Run with: cargo run --example persistent_counter

use vizia::prelude::*;

pub enum CounterEvent {
    Increment,
    Decrement,
    Reset,
}

struct PersistentCounterApp {
    count: Signal<i32>,
}

impl App for PersistentCounterApp {
    fn new(cx: &mut Context) -> Self {
        // This counter persists across app restarts!
        // The value is saved to disk (debounced 500ms) and loaded on startup.
        Self { count: cx.state_persists("counter", 0) }
    }

    fn on_build(self, cx: &mut Context) -> Self {
        let count = self.count;

        VStack::new(cx, move |cx| {
            Label::new(cx, "Persistent Counter")
                .font_size(24.0)
                .font_weight(FontWeightKeyword::Bold);

            Label::new(cx, "This value persists across app restarts!")
                .font_size(12.0)
                .color(Color::gray());

            Label::new(cx, count).font_size(48.0);

            HStack::new(cx, |cx| {
                Button::new(cx, |cx| Label::new(cx, "-"))
                    .on_press(|cx| cx.emit(CounterEvent::Decrement))
                    .width(Pixels(60.0));

                Button::new(cx, |cx| Label::new(cx, "+"))
                    .on_press(|cx| cx.emit(CounterEvent::Increment))
                    .width(Pixels(60.0));

                Button::new(cx, |cx| Label::new(cx, "Reset"))
                    .on_press(|cx| cx.emit(CounterEvent::Reset))
                    .width(Pixels(80.0));
            })
            .gap(Pixels(10.0));
        })
        .alignment(Alignment::Center)
        .gap(Pixels(20.0));

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
            CounterEvent::Reset => {
                self.count.set(cx, 0);
            }
        });
    }

    fn window_config(&self) -> WindowConfig {
        // Title defaults to app_name(), only need to set size
        window(|app| app.inner_size((300, 250)))
    }
}

fn main() -> Result<(), ApplicationError> {
    PersistentCounterApp::run()
}
