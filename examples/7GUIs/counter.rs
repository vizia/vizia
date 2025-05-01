use vizia::{binding::SignalBinding, prelude::*};

#[derive(Lens)]
pub struct AppData {
    count: Signal<i32>,
    other: Signal<i32>,
}

pub enum AppEvent {
    Increment,
    Decrement,
}

impl Model for AppData {
    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|app_event, _| match app_event {
            AppEvent::Increment => self.count.update(cx, |count| *count += 1),
            AppEvent::Decrement => self.count.update(cx, |count| *count -= 1),
        });
    }
}

fn main() -> Result<(), ApplicationError> {
    Application::new(|cx| {
        AppData { count: cx.state(0), other: cx.state(0) }.build(cx);

        HStack::new(cx, |cx| {
            Button::new(cx, |cx| Label::new(cx, "Increment"))
                .on_press(|cx| cx.emit(AppEvent::Increment));

            Button::new(cx, |cx| Label::new(cx, "Decrement"))
                .on_press(|cx| cx.emit(AppEvent::Decrement));
            // Currently uses a lens to get the atom
            let count = AppData::count.get(cx);
            let other = AppData::other.get(cx);

            println!("{:?}", count.id());

            Label::new(cx, count);

            // Derived state - only recomputed when the count changes
            let doubled = cx.derived(move |s| *count.get(s) * 2);

            println!("{:?}", doubled.id());

            Label::new(cx, doubled);

            let current = cx.current();

            // Binding::new(cx, AppData::count, move |cx, count| {
            //     let current = cx.current();
            //     doubled.observe(cx.data.get_store_mut(), current);
            //     Label::new(cx, doubled);
            // });

            // Binding::new(cx, AppData::count, move |cx, _| {
            //     let current = cx.current();
            //     count.observe(cx.data.get_store_mut(), current);
            //     other.observe(cx.data.get_store_mut(), current);
            //     println!("refreshed");
            //     Label::new(cx, doubled);
            // });

            SignalBinding::new(cx, count, move |cx| {
                println!("refreshed2");
                Label::new(cx, "test");
            });

            // doubled.subscribe(cx.data.get_store_mut(), move |cx| {
            //     cx.with_current(current, |cx| {
            //         Label::new(cx, doubled);
            //     })
            // });
        })
        .alignment(Alignment::Center)
        .gap(Pixels(50.0));
    })
    .title("Counter")
    .inner_size((400, 100))
    .run()
}
