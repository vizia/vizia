use vizia::prelude::*;

fn main() -> Result<(), ApplicationError> {
    let (app, (title, size)) = Application::new_with_state(|cx| {
        let count = cx.state(0i32);
        let title = cx.state("Counter".to_string());
        let size = cx.state((400, 100));
        let align_center = cx.state(Alignment::Center);
        let gap_50 = cx.state(Pixels(50.0));

        HStack::new(cx, move |cx| {
            Button::new(cx, |cx| Label::static_text(cx, "Increment"))
                .on_press(move |cx| count.update(cx, |value| *value += 1));

            Button::new(cx, |cx| Label::static_text(cx, "Decrement"))
                .on_press(move |cx| count.update(cx, |value| *value -= 1));

            Label::new(cx, count);
        })
        .alignment(align_center)
        .gap(gap_50);

        (title, size)
    });

    app.title(title).inner_size(size).run()
}
