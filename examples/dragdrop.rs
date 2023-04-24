use vizia::prelude::*;

const STYLE: &str = r#"

    :root {
        child-space: 1s;
    }

    .drop {
        background-color: grey;
    }

    .drop:hover {
        background-color: red;
    }
"#;

fn main() {
    Application::new(|cx| {
        cx.add_theme(STYLE);
        HStack::new(cx, |cx| {
            Element::new(cx).size(Pixels(50.0)).background_color(Color::red()).on_drag_start(
                |ex| {
                    ex.cx.set_drop_data(ex.current());
                },
            );

            Element::new(cx).size(Pixels(50.0)).background_color(Color::green()).on_drag_start(
                |ex| {
                    ex.cx.set_drop_data(ex.current());
                },
            );

            Element::new(cx).size(Pixels(50.0)).background_color(Color::blue()).on_drag_start(
                |ex| {
                    ex.cx.set_drop_data(ex.current());
                },
            );
        })
        .height(Pixels(100.0))
        .width(Auto)
        .col_between(Pixels(20.0))
        .child_space(Stretch(1.0));

        HStack::new(cx, |cx| {})
            .size(Pixels(100.0))
            .background_color(Color::beige())
            .on_drop(|ex, data| {
                if let DropData::Id(id) = data {
                    let bg = ex.cx.style.background_color.get(id).cloned().unwrap_or_default();
                    ex.cx.set_background_color(bg);
                    ex.emit(WindowEvent::SetCursor(CursorIcon::Default));
                }
            })
            .on_hover(|ex| {
                if ex.cx.has_drop_data() {
                    ex.emit(WindowEvent::SetCursor(CursorIcon::Copy));
                } else {
                    ex.emit(WindowEvent::SetCursor(CursorIcon::Default));
                }
            });
    })
    .run();
}

pub struct CustomView {}

impl CustomView {
    pub fn new(cx: &mut Context) -> Handle<Self> {
        Self {}.build(cx, |_| {})
    }
}

impl View for CustomView {
    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|window_event, _| match window_event {
            WindowEvent::Drop(data) => {
                println!("File dropped: {:?}", data);
            }

            _ => {}
        })
    }
}
