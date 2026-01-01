use vizia::prelude::*;

const STYLE: &str = r#"
    :root {
        alignment: center;
    }
"#;

fn main() -> Result<(), ApplicationError> {
    Application::new(|cx| {
        cx.add_stylesheet(STYLE).expect("Failed to add stylesheet");
        let size_50 = cx.state(Pixels(50.0));
        let size_100 = cx.state(Pixels(100.0));
        let height_100 = cx.state(Pixels(100.0));
        let auto = cx.state(Auto);
        let gap_20 = cx.state(Pixels(20.0));
        let align_center = cx.state(Alignment::Center);
        let red = cx.state(Color::red());
        let green = cx.state(Color::green());
        let blue = cx.state(Color::blue());
        let gray = cx.state(Color::gray());

        HStack::new(cx, |cx| {
            Element::new(cx).size(size_50).background_color(red).on_drag(|ex| {
                ex.set_drop_data(ex.current());
            });

            Element::new(cx).size(size_50).background_color(green).on_drag(|ex| {
                ex.set_drop_data(ex.current());
            });

            Element::new(cx).size(size_50).background_color(blue).on_drag(|ex| {
                ex.set_drop_data(ex.current());
            });
        })
        .height(height_100)
        .width(auto)
        .horizontal_gap(gap_20)
        .alignment(align_center);

        Element::new(cx)
            .size(size_100)
            .background_color(gray)
            .on_drop(|ex, data| {
                if let DropData::Id(id) = data {
                    let bg = ex.with_current(id, |ex| ex.background_color());
                    ex.set_background_color(bg);
                    ex.emit(WindowEvent::SetCursor(CursorIcon::Default));
                }
                if let DropData::File(file) = data {
                    println!("Dropped File: {:?}", file);
                }
            })
            .on_hover(|ex| {
                if ex.has_drop_data() {
                    ex.emit(WindowEvent::SetCursor(CursorIcon::Copy));
                } else {
                    ex.emit(WindowEvent::SetCursor(CursorIcon::Default));
                }
            });
    })
    .run()
}
