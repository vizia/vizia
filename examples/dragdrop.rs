use vizia::prelude::*;

const STYLE: &str = r#"
    :root {
        alignment: center;
    }
"#;

struct DragDropApp {
    size_50: Signal<Units>,
    size_100: Signal<Units>,
    height_100: Signal<Units>,
    auto: Signal<Units>,
    gap_20: Signal<Units>,
    align_center: Signal<Alignment>,
    red: Signal<Color>,
    green: Signal<Color>,
    blue: Signal<Color>,
    gray: Signal<Color>,
}

impl App for DragDropApp {
    fn app_name() -> &'static str {
        "Drag & Drop"
    }

    fn new(cx: &mut Context) -> Self {
        Self {
            size_50: cx.state(Pixels(50.0)),
            size_100: cx.state(Pixels(100.0)),
            height_100: cx.state(Pixels(100.0)),
            auto: cx.state(Auto),
            gap_20: cx.state(Pixels(20.0)),
            align_center: cx.state(Alignment::Center),
            red: cx.state(Color::red()),
            green: cx.state(Color::green()),
            blue: cx.state(Color::blue()),
            gray: cx.state(Color::gray()),
        }
    }

    fn on_build(self, cx: &mut Context) -> Self {
        cx.add_stylesheet(STYLE).expect("Failed to add stylesheet");
        let size_50 = self.size_50;
        let size_100 = self.size_100;
        let height_100 = self.height_100;
        let auto = self.auto;
        let gap_20 = self.gap_20;
        let align_center = self.align_center;
        let red = self.red;
        let green = self.green;
        let blue = self.blue;
        let gray = self.gray;

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
        self
    }
}

fn main() -> Result<(), ApplicationError> {
    DragDropApp::run()
}
