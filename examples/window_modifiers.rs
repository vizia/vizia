use vizia::prelude::*;
use vizia_winit::GetRawWindowHandle;

#[cfg(feature = "baseview")]
fn main() {
    panic!("This example is not supported on baseview");
}

#[derive(Lens)]
pub struct AppData {
    title: String,
}

impl Model for AppData {
    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|window_event, _| match window_event {
            WindowEvent::MouseMove(_, _) => {
                let raw = cx.raw_window_handle();

                cx.mutate_window(|window| {
                    // Do stuff with window here
                });
            }

            _ => {}
        })
    }
}

#[cfg(all(not(feature = "baseview")))]
fn main() {
    Application::new(|cx| {
        AppData { title: "Window Modifiers".to_string() }.build(cx);

        let raw = cx.raw_window_handle();

        Label::new(cx, "Hello Vizia");
        Textbox::new(cx, AppData::title)
            .on_edit(|ex, txt| ex.emit(WindowEvent::SetTitle(txt)))
            .width(Stretch(1.0));
    })
    .title("Window Modifiers")
    .inner_size((400, 100))
    .run();
}
