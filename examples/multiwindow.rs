pub use vizia::prelude::*;

#[cfg(feature = "baseview")]
fn main() {
    panic!("This example is not supported on baseview");
}

#[cfg(not(feature = "baseview"))]
fn main() -> Result<(), ApplicationError> {
    let (app, title) = Application::new_with_state(|cx| {
        // Color component signals
        let red = cx.state(1.0f32);
        let green = cx.state(1.0f32);
        let blue = cx.state(1.0f32);
        let show_window = cx.state(false);
        let padding_20 = cx.state(Pixels(20.0));
        let align_center = cx.state(Alignment::Center);
        let gap_12 = cx.state(Pixels(12.0));
        let auto = cx.state(Auto);
        let window_title = cx.state("Set color...");
        let window_size = cx.state((400, 200));
        let window_anchor = cx.state(Anchor::Center);

        Binding::new(cx, show_window, move |cx| {
            if *show_window.get(cx) {
                Window::new(cx, move |cx| {
                    VStack::new(cx, move |cx: &mut Context| {
                        Slider::new(cx, red).two_way();
                        Slider::new(cx, green).two_way();
                        Slider::new(cx, blue).two_way();
                    })
                    .padding(padding_20)
                    .alignment(align_center)
                    .vertical_gap(gap_12);
                })
                .on_close(move |cx| show_window.set(cx, false))
                .title(window_title)
                .inner_size(window_size)
                .anchor(window_anchor);
            }
        });

        // Derive color from RGB signals
        let color = cx.derived(move |cx| {
            let r = (*red.get(cx) * 255.0) as u8;
            let g = (*green.get(cx) * 255.0) as u8;
            let b = (*blue.get(cx) * 255.0) as u8;
            Color::rgb(r, g, b)
        });

        HStack::new(cx, move |cx| {
            Button::new(cx, |cx| Label::static_text(cx, "Show Window"))
                .on_press(move |cx| show_window.set(cx, true));
        })
        .size(auto)
        .padding(padding_20)
        .background_color(color);
        cx.state("Main")
    });

    app.title(title).run()
}
