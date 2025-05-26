use vizia::prelude::*;

const STYLE: &str = r#"
    resizable-stack {
        background-color: #b1b1b1;
    }

    resizable-stack.vertical {
        background-color: #878787;
        min-width: 100px;
        max-width: 500px;
    }

    resizable-stack.horizontal {
        min-height: 100px;
        max-height: 500px;
    }

    resizable-stack > resize-handle {
        background-color: #73a3cd;
        opacity: 0;
    }

    resizable-stack:active > resize-handle,
    resizable-stack > resize-handle:hover {
        opacity: 1;
        transition: opacity 200ms 200ms ease-in-out;
    }

    
"#;

#[derive(Lens)]
pub struct AppData {
    width: Units,
    height: Units,
}

pub enum AppEvent {
    SetWidth(Units),
    SetHeight(Units),
    ResetWidth,
    ResetHeight,
}

impl Model for AppData {
    fn event(&mut self, _cx: &mut EventContext, event: &mut Event) {
        event.map(|app_event, _| match app_event {
            AppEvent::SetWidth(width) => {
                self.width = *width;
            }
            AppEvent::SetHeight(height) => {
                self.height = *height;
            }
            AppEvent::ResetWidth => {
                self.width = Pixels(200.0);
            }
            AppEvent::ResetHeight => {
                self.height = Pixels(200.0);
            }
        });
    }
}

fn main() -> Result<(), ApplicationError> {
    Application::new(|cx| {
        cx.add_stylesheet(STYLE).expect("Failed to add stylesheet");

        AppData { width: Pixels(200.0), height: Pixels(200.0) }.build(cx);

        ResizableStack::new(
            cx,
            AppData::height,
            ResizeStackDirection::Bottom,
            |cx, h| cx.emit(AppEvent::SetHeight(Pixels(h))),
            |cx| {
                ResizableStack::new(
                    cx,
                    AppData::width,
                    ResizeStackDirection::Right,
                    |cx, w| cx.emit(AppEvent::SetWidth(Pixels(w))),
                    |_cx| {},
                )
                .on_reset(|cx| {
                    cx.emit(AppEvent::ResetWidth);
                });
            },
        )
        .on_reset(|cx| {
            cx.emit(AppEvent::ResetHeight);
        });
    })
    .title("Resizable Stack")
    .inner_size((800, 600))
    .run()
}
