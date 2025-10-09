use vizia::prelude::*;

// Simple counter application using the App trait
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
            Label::new(cx, self.count.map(|count| format!("Count: {}", count))).font_size(32.0);

            // Counter controls
            HStack::new(cx, |cx| {
                Button::new(cx, |cx| Label::new(cx, "Decrement")).on_press(move |cx| {
                    self.count.update(cx, |count| *count -= 1);
                });

                Button::new(cx, |cx| Label::new(cx, "Reset")).on_press(move |cx| {
                    self.count.set(cx, 0);
                });

                Button::new(cx, |cx| Label::new(cx, "Increment")).on_press(move |cx| {
                    self.count.update(cx, |count| *count += 1);
                });
            })
            .gap(Pixels(10.0));

            // Show derived state - whether count is even or odd
            let parity = cx.derived(move |s| {
                let count = self.count.get(s);
                if *count % 2 == 0 { "Even" } else { "Odd" }.to_string()
            });

            Label::new(cx, parity.map(|p| format!("Parity: {}", p)))
                .font_size(16.0)
                .color(Color::rgb(128, 128, 128));
        })
        .alignment(Alignment::Center)
        .gap(Pixels(20.0));

        self
    }
}

fn main() -> Result<(), ApplicationError> {
    CounterApp::build().title("Simple Counter - App Trait Demo").inner_size((400, 300)).run()
}

// Show that the old approach still works for comparison
#[allow(dead_code)]
fn old_style_counter() -> Result<(), ApplicationError> {
    Application::new(|cx| {
        let count = cx.state(0);

        VStack::new(cx, |cx| {
            Label::new(cx, "Old Style Counter");
            Label::new(cx, count);

            HStack::new(cx, |cx| {
                Button::new(cx, |cx| Label::new(cx, "-"))
                    .on_press(move |cx| count.update(cx, |c| *c -= 1));

                Button::new(cx, |cx| Label::new(cx, "+"))
                    .on_press(move |cx| count.update(cx, |c| *c += 1));
            });
        });
    })
    .title("Old Style Counter")
    .run()
}
