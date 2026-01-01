pub use vizia::prelude::*;

#[cfg(feature = "baseview")]
fn main() {
    panic!("This example is not supported on baseview");
}

#[cfg(not(feature = "baseview"))]
fn main() -> Result<(), ApplicationError> {
    let (app, (title, position)) = Application::new_with_state(|cx| {
        // Color component signals
        let red = cx.state(1.0f32);
        let green = cx.state(1.0f32);
        let blue = cx.state(1.0f32);
        let show_popup = cx.state(false);
        let title = cx.state("Main".to_string());
        let position = cx.state((100, 100));
        let popup_title = cx.state("Set color...".to_string());
        let popup_size = cx.state((400, 200));
        let popup_anchor = cx.state(Anchor::Center);
        let padding_20 = cx.state(Pixels(20.0));
        let gap_12 = cx.state(Pixels(12.0));
        let align_center = cx.state(Alignment::Center);

        Binding::new(cx, show_popup, move |cx| {
            if *show_popup.get(cx) {
                Window::popup(cx, false, move |cx| {
                    VStack::new(cx, move |cx: &mut Context| {
                        Slider::new(cx, red).on_change(move |cx, val| red.set(cx, val));
                        Slider::new(cx, green).on_change(move |cx, val| green.set(cx, val));
                        Slider::new(cx, blue).on_change(move |cx, val| blue.set(cx, val));
                    })
                    .padding(padding_20)
                    .alignment(align_center)
                    .vertical_gap(gap_12);
                })
                .on_close(move |cx| show_popup.set(cx, false))
                .title(popup_title)
                .inner_size(popup_size)
                .anchor(popup_anchor);
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
            Button::new(cx, |cx| Label::static_text(cx, "Show Popup"))
                .on_press(move |cx| show_popup.set(cx, true));
        })
        .padding(padding_20)
        .background_color(color);
        (title, position)
    });

    app.title(title).position(position).run()
}
