use vizia::*;


fn main() {
    Application::new(WindowDescription::new().with_title("Cursor Icon"), |cx|{
        Element::new(cx).width(Pixels(100.0)).height(Pixels(100.0)).background_color(Color::red()).on_hover(cx, |cx|{
            println!("Hovered");
            cx.emit(WindowEvent::SetCursor(CursorIcon::Text));
        });
    }).run();
}