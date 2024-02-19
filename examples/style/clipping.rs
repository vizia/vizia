use vizia::prelude::*;

const STYLE: &str = r#"
    .container {
        size: 100px;
        border-radius: 10px;
        background-color: rgb(200, 200, 200);
    }

    .row {
        child-space: 1s;
        col-between: 40px;
    }

    element {
        size: 75px;
        left: 50px;
        top: 50px;
        background-color: red;
    }
    
    .overflow {
        overflow: hidden;
    }

    .overflow:over {
        overflow: visible;
    }

    .overflowx {
        overflow-x: hidden;
    }

    .overflowx:over {
        overflow-x: visible;
    }

    .overflowy {
        overflow-y: hidden;
    }

    .overflowy:over {
        overflow-y: visible;
    }
    
    .clipping {
        clip-path: inset(30px);
        overflow: hidden;
    }

    .clipping:over {
        clip-path: inset(0px);
        transition: clip-path 100ms;
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

fn main() -> Result<(), ApplicationError> {
    Application::new(|cx| {
        cx.add_stylesheet(STYLE).expect("Failed to add stylesheet");

        HStack::new(cx, |cx| {
            HStack::new(cx, |cx| {
                Element::new(cx);
            })
            .class("container")
            .class("overflow");

            HStack::new(cx, |cx| {
                Element::new(cx);
            })
            .class("container")
            .class("overflowx");

            HStack::new(cx, |cx| {
                Element::new(cx);
            })
            .class("container")
            .class("overflowy");

            HStack::new(cx, |cx| {
                Element::new(cx);
            })
            .class("container")
            .class("clipping");
        })
        .class("row");
    })
    .title("Overflow and Clipping")
    .inner_size((800, 400))
    .run()
}
