use vizia::prelude::*;

const STYLE: &str = r#"

    .container:over .linear {
        left: 300px;
        transition: left 1s linear;
    }

    .container:over .ease {
        left: 300px;
        transition: left 1s ease;
    }

    .container:over .easein {
        left: 300px;
        transition: left 1s ease-in;
    }

    .container:over .easeout {
        left: 300px;
        transition: left 1s ease-out;
    }

    .container:over .easeinout {
        left: 300px;
        transition: left 1s ease-in-out;
    }

    element {
        size: 100px;
        left: 0px;
        background-color: #606060;
        child-space: 1s;
    }

    .container {
        height: auto;
        background-color: #303030;
    }

    .row {
        height: auto;
    }
"#;

#[derive(Lens)]
pub struct AppData {
    skew: f32,
}

pub enum AppEvent {
    SetSkew(f32),
}

impl Model for AppData {
    fn event(&mut self, _cx: &mut EventContext, event: &mut Event) {
        event.map(|app_event, _| match app_event {
            AppEvent::SetSkew(val) => {
                self.skew = *val;
            }
        });
    }
}

fn main() {
    Application::new(|cx| {
        cx.add_stylesheet(STYLE).expect("Failed to add stylesheet");

        VStack::new(cx, |cx| {
            HStack::new(cx, |cx| {
                Element::new(cx).class("linear").text("linear");
            })
            .class("row");

            HStack::new(cx, |cx| {
                Element::new(cx).class("ease").text("ease");
            })
            .class("row");

            HStack::new(cx, |cx| {
                Element::new(cx).class("easein").text("ease-in");
            })
            .class("row");

            HStack::new(cx, |cx| {
                Element::new(cx).class("easeout").text("ease-out");
            })
            .class("row");

            HStack::new(cx, |cx| {
                Element::new(cx).class("easeinout").text("ease-in-out");
            })
            .class("row");
        })
        .class("container");
    })
    .title("Overlflow and Clipping")
    .inner_size((800, 400))
    .run();
}
