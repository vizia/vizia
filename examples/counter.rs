use vizia::*;

const STYLE: &str = r#"
    button {
        width: 100px;
        height: 30px;
    }

    label {
        width: 100px;
        height: 30px;
    }
"#;

// Define some data which the counter will use
#[derive(Lens)]
pub struct CounterData {
    count: i32,
}

// Define some events
#[derive(Debug)]
pub enum CounterEvent {
    Increment,
    Decrement,
}

// Respond to events and mutate the data
impl Model for CounterData {
    fn event(&mut self, _: &mut Context, event: &mut Event) {
        if let Some(counter_event) = event.message.downcast() {
            match counter_event {
                CounterEvent::Increment => self.count += 1,
                CounterEvent::Decrement => self.count -= 1,
            }
        }
    }
}

fn main() {
    let window_description =
        WindowDescription::new().with_title("Counter").with_inner_size(500, 100);

    Application::new(window_description, |cx| {
        // Add the stylesheet to the app
        cx.add_theme(STYLE);

        // Add the data to the app
        CounterData { count: 0 }.build(cx);

        HStack::new(cx, |cx| {
            // Button for incrementing the counter
            Button::new(
                cx,
                |cx| cx.emit(CounterEvent::Increment),
                |cx| Label::new(cx, "Increment"),
            );

            // Button for decrementing the counter
            Button::new(
                cx,
                |cx| cx.emit(CounterEvent::Decrement),
                |cx| Label::new(cx, "Decrement"),
            );

            // Label bound to the counter value
            Binding::new(cx, CounterData::count, |cx, count| {
                Label::new(cx, count);
            });

            // Label bound to the counter value displaying the value as english text
            Binding::new(cx, CounterData::count, |cx, count| {
                Label::new(cx, &english_numbers::convert_all_fmt(*count.get(cx) as i64));
            });
        })
        .child_space(Stretch(1.0))
        .col_between(Pixels(10.0));
    })
    .run();
}
