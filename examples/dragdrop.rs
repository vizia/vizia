use vizia::prelude::*;

const STYLE: &str = r#"
    :root {
        alignment: center;
    }

    .drop-zone {
        border-width: 2px;
        border-color: transparent;
    }

    .drop-zone.drag-over {
        border-color: white;
        cursor: copy;
    }

    .drag-preview {
        size: 28px;
        border-radius: 6px;
        border-width: 1px;
        border-color: #ffffff66;
        background-color: #ffffff22;
        backdrop-filter: blur(2px);
    }
"#;

#[derive(Clone, Copy)]
struct AppData {
    drop_zone_color: Signal<Color>,
}

enum AppEvent {
    SetDropZoneColor(Color),
}

impl Model for AppData {
    fn event(&mut self, _cx: &mut EventContext, event: &mut Event) {
        event.map(|app_event, _| match app_event {
            AppEvent::SetDropZoneColor(color) => {
                self.drop_zone_color.set(*color);
            }
        });
    }
}

fn main() -> Result<(), ApplicationError> {
    Application::new(|cx| {
        cx.add_stylesheet(STYLE).expect("Failed to add stylesheet");

        let drop_zone_color = Signal::new(Color::gray());
        AppData { drop_zone_color }.build(cx);

        HStack::new(cx, |cx| {
            Element::new(cx)
                .size(Pixels(50.0))
                .background_color(Color::red())
                .on_drag(|ex| {
                    ex.set_drop_data(ex.current());
                })
                .on_drag_view(|cx| {
                    Element::new(cx)
                        .class("drag-preview")
                        .size(Pixels(50.0))
                        .background_color(Color::red())
                });

            Element::new(cx)
                .size(Pixels(50.0))
                .background_color(Color::green())
                .on_drag(|ex| {
                    ex.set_drop_data(ex.current());
                })
                .on_drag_view(|cx| {
                    Element::new(cx)
                        .class("drag-preview")
                        .size(Pixels(50.0))
                        .background_color(Color::green())
                });

            Element::new(cx)
                .size(Pixels(50.0))
                .background_color(Color::blue())
                .on_drag(|ex| {
                    ex.set_drop_data(ex.current());
                })
                .on_drag_view(|cx| {
                    Element::new(cx)
                        .class("drag-preview")
                        .size(Pixels(50.0))
                        .background_color(Color::blue())
                });
        })
        .height(Pixels(100.0))
        .width(Auto)
        .horizontal_gap(Pixels(20.0))
        .alignment(Alignment::Center);

        Element::new(cx)
            .size(Pixels(100.0))
            .background_color(drop_zone_color)
            .class("drop-zone")
            .on_drag_enter(|ex| {
                println!("Drag entered drop zone");
                ex.toggle_class("drag-over", true);
            })
            .on_drag_leave(|ex| {
                println!("Drag left drop zone");
                ex.toggle_class("drag-over", false);
            })
            .on_drag_move(|_ex, x, y| {
                println!("Drag move over drop zone: ({x}, {y})");
            })
            .on_drop(|ex, data| {
                ex.toggle_class("drag-over", false);
                if let DropData::Id(id) = data {
                    let bg = ex.with_current(id, |ex| ex.background_color());
                    ex.emit(AppEvent::SetDropZoneColor(bg));
                }
                if let DropData::File(file) = data {
                    println!("Dropped File: {:?}", file);
                }
            });
    })
    .run()
}
