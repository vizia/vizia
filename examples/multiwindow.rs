pub use vizia::prelude::*;

const STYLE: &str = r#"
    element.one {
        background-color: red;
    }

    element.one:hover {
        background-color: green;
    }

    element.two {
        background-color: blue;
    }

    element.two:hover {
        background-color: yellow;
    }
"#;

#[derive(Lens)]
struct AppData {
    text: String,
    show_subwindow: bool,
}

impl Model for AppData {
    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|app_event, meta| match app_event {
            AppEvent::ToggleShowSubwindow => self.show_subwindow ^= true,
        })
    }
}

pub enum AppEvent {
    ToggleShowSubwindow,
}

#[derive(Lens)]
struct AppData2 {
    text: String,
}

impl Model for AppData2 {}

fn main() {
    Application::new(|cx| {
        AppData { text: String::from("Some text"), show_subwindow: true }.build(cx);

        cx.add_stylesheet(STYLE);
        Element::new(cx).size(Pixels(100.0)).class("one");
        Button::new(cx, |cx| cx.emit(WindowEvent::WindowClose), |cx| Label::new(cx, "Close"));

        // Window::new(cx, |cx| {
        //     AppData2 { text: String::from("more text") }.build(cx);
        //     Element::new(cx).size(Pixels(50.0)).class("two");
        //     Button::new(cx, |cx| cx.emit(WindowEvent::WindowClose), |cx| Label::new(cx, "Close"));
        //     Label::new(cx, "Subwindow");
        //     Label::new(cx, AppData::text);
        //     Label::new(cx, AppData2::text);
        // })
        // .title("Secondary")
        // .inner_size((400, 400));

        // Window::new(cx, |cx| {
        //     Element::new(cx).size(Pixels(50.0)).class("two");
        //     Button::new(cx, |cx| cx.emit(WindowEvent::WindowClose), |cx| Label::new(cx, "Close"));
        //     Label::new(cx, "Another");
        //     Label::new(cx, AppData::text);
        // })
        // .title("Secondary")
        // .inner_size((400, 400));

        Binding::new(cx, AppData::show_subwindow, |cx, show_subwindow| {
            if show_subwindow.get(cx) {
                Window::new(cx, |cx| {
                    Element::new(cx).size(Pixels(50.0)).class("two");
                    Button::new(
                        cx,
                        |cx| cx.emit(WindowEvent::WindowClose),
                        |cx| Label::new(cx, "Close"),
                    );
                    Label::new(cx, "Subwindow");
                    Label::new(cx, AppData::text);
                })
                .title("Secondary")
                .inner_size((400, 400));
            }
        });

        Label::new(cx, "Main window");
        Label::new(cx, AppData::text);
        Checkbox::new(cx, AppData::show_subwindow)
            .on_toggle(|cx| cx.emit(AppEvent::ToggleShowSubwindow));
    })
    .title("Main")
    .run();
}
