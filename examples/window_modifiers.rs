use vizia::prelude::*;

#[cfg(feature = "baseview")]
fn main() {
    panic!("This example is not supported on baseview");
}

#[cfg(not(feature = "baseview"))]
fn main() -> Result<(), ApplicationError> {
    let (app, (title, size)) = Application::new_with_state(|cx| {
        let title = cx.state("Window Modifiers".to_string());
        let size = cx.state((400, 100));
        let stretch_one = cx.state(Stretch(1.0));
        let padding_8 = cx.state(Pixels(8.0));
        let gap_8 = cx.state(Pixels(8.0));

        VStack::new(cx, |cx| {
            Label::static_text(cx, "Window title:");
            Textbox::new(cx, title)
                .on_edit(move |cx, txt| title.set(cx, txt))
                .width(stretch_one);
        })
        .padding(padding_8)
        .gap(gap_8);

        (title, size)
    });

    app.title(title).inner_size(size).run()
}
