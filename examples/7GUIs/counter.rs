use vizia::{binding::Binding, prelude::*};

#[derive(Lens)]
pub struct AppData {
    count: Signal<i32>,
    other: i32,
}

pub enum AppEvent {
    Increment,
    Decrement,
}

impl Model for AppData {
    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|app_event, _| match app_event {
            AppEvent::Increment => {
                self.other += 1;
                self.count.update(cx, |count| *count += 1);
            }
            AppEvent::Decrement => {
                self.other -= 1;
                self.count.update(cx, |count| *count -= 1);
            }
        });
    }
}

fn main() -> Result<(), ApplicationError> {
    Application::new(|cx| {
        AppData { count: cx.state(0), other: 0 }.build(cx);

        HStack::new(cx, |cx| {
            Button::new(cx, |cx| Label::new(cx, "Increment"))
                .on_press(|cx| cx.emit(AppEvent::Increment));

            Button::new(cx, |cx| Label::new(cx, "Decrement"))
                .on_press(|cx| cx.emit(AppEvent::Decrement));
            // Currently uses a lens to get the atom
            let count = AppData::count.get(cx);

            println!("{:?}", count.id());

            Label::new(cx, count);

            // Derived state - only recomputed when the count changes
            let doubled = cx.derived(move |s| *count.get(s) * 2);

            println!("{:?}", doubled.id());

            Label::new(cx, doubled);

            let current = cx.current();

            Binding::new(cx, count, move |cx| {
                println!("refreshed2");
                Label::new(cx, count);
            });

            Binding::new(cx, AppData::other, move |cx, other| {
                println!("refreshed3");
                Label::new(cx, other);
            });
        })
        .alignment(Alignment::Center)
        .gap(Pixels(50.0));
    })
    .title("Counter")
    .inner_size((400, 100))
    .run()
}
