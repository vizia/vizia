pub use vizia::prelude::*;

#[cfg(feature = "baseview")]
fn main() {
    panic!("This example is not supported on baseview");
}

enum AppEvent {
    ShowWindow,
    WindowClosed,
}

#[cfg(not(feature = "baseview"))]
fn main() -> Result<(), ApplicationError> {
    Application::new(|cx| {
        // Color component signals
        let red = cx.state(1.0f32);
        let green = cx.state(1.0f32);
        let blue = cx.state(1.0f32);
        let show_window = cx.state(false);

        Binding::new(cx, show_window, move |cx| {
            if *show_window.get(cx) {
                Window::new(cx, move |cx| {
                    VStack::new(cx, move |cx: &mut Context| {
                        Slider::new(cx, red)
                            .on_change(move |cx, val| red.set(cx, val));
                        Slider::new(cx, green)
                            .on_change(move |cx, val| green.set(cx, val));
                        Slider::new(cx, blue)
                            .on_change(move |cx, val| blue.set(cx, val));
                    })
                    .padding(Pixels(20.0))
                    .alignment(Alignment::Center)
                    .vertical_gap(Pixels(12.0));
                })
                .on_close(|cx| {
                    cx.emit(AppEvent::WindowClosed);
                })
                .title("Set color...")
                .inner_size((400, 200))
                .anchor(Anchor::Center);
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
            Button::new(cx, |cx| Label::new(cx, "Show Window"))
                .on_press(move |cx| show_window.set(cx, true));
        })
        .size(Auto)
        .padding(Pixels(20.0))
        .background_color(color);
    })
    .title("Main")
    .run()
}
