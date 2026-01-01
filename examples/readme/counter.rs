use vizia::prelude::*;

fn main() -> Result<(), ApplicationError> {
    let (app, (title, size)) = Application::new_with_state(|cx| {
        let count = cx.state(0i32);

        HStack::new(cx, move |cx| {
            Button::new(cx, |cx| Label::static_text(cx, "-"))
                .on_press(move |cx| count.update(cx, |n| *n -= 1));

            Label::new(cx, count);

            Button::new(cx, |cx| Label::static_text(cx, "+"))
                .on_press(move |cx| count.update(cx, |n| *n += 1));
        });

        (cx.state("Counter"), cx.state((200, 100)))
    });

    app.title(title).inner_size(size).run()
}
