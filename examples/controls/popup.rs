use vizia::*;

fn main() {
    let window_description = WindowDescription::new();
    Application::new(window_description, |cx| {
        PopupData::default().build(cx);

        Button::new(cx, |cx| cx.emit(PopupEvent::Switch), |cx| Label::new(cx, "Open"));

        // Label::new(cx, "Press Me")
        //     .on_checked(cx, |cx| cx.emit(PopupEvent::Open))
        //     .on_unchecked(cx, |cx| cx.emit(PopupEvent::Close));

        Popup::new(cx, |_| {})
            .space(Pixels(100.0))
            .size(Pixels(200.0))
            .background_color(Color::red());
    })
    .run();
}
