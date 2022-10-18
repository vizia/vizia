use vizia::prelude::*;

fn main() {
    Application::new(|cx| {
        MousePopupData { is_open: false, x: 50.0, y: 50.0 }.build(cx);

        Button::new(cx, |cx| cx.emit(MousePopupEvent::Switch), |cx| Label::new(cx, "Open"))
            .size(Pixels(200.0));

        MousePopup::new(
            cx,
            MousePopupData::is_open,
            MousePopupData::x,
            MousePopupData::y,
            false,
            |_| {},
        )
        .on_blur(|cx| cx.emit(MousePopupEvent::Close))
        .size(Pixels(200.0))
        .background_color(Color::red());
    })
    .title("Popup")
    .run();
}
