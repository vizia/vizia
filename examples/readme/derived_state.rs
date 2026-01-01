use vizia::prelude::*;

fn main() -> Result<(), ApplicationError> {
    let (app, (title, size)) = Application::new_with_state(|cx| {
        let number = cx.state(5i32);

        // Derived signals recompute automatically
        let squared = cx.derived(move |s| number.get(s) * number.get(s));
        let is_even = cx.derived(move |s| number.get(s) % 2 == 0);
        let parity = cx.derived(move |s| {
            if *is_even.get(s) { "even" } else { "odd" }
        });

        VStack::new(cx, move |cx| {
            Label::new(cx, number);
            Label::new(cx, squared);
            Label::new(cx, parity);

            HStack::new(cx, move |cx| {
                Button::new(cx, |cx| Label::static_text(cx, "-"))
                    .on_press(move |cx| number.update(cx, |n| *n -= 1));
                Button::new(cx, |cx| Label::static_text(cx, "+"))
                    .on_press(move |cx| number.update(cx, |n| *n += 1));
            });
        });

        (cx.state("Derived State"), cx.state((300, 200)))
    });

    app.title(title).inner_size(size).run()
}
