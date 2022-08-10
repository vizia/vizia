use vizia::prelude::*;

const THEME: &str = r#"

    label {
        background-color: white;
    }

    label:hover {
        background-color: blue;
    }
"#;

#[derive(Lens)]
pub struct AppData {
    pub long_list: Vec<u32>,
}

impl Model for AppData {
    fn event(&mut self, _: &mut EventContext, event: &mut Event) {
        event.map(|window_event, _| match window_event {
            WindowEvent::KeyDown(code, _) => {
                if *code == Code::Space {
                    println!("Pressed Space key");
                }
            }

            _ => {}
        });
    }
}

fn main() {
    Application::new(|cx| {
        cx.add_theme(THEME);

        HStack::new(cx, |cx| {
            AppData { long_list: (0..10000).collect() }.build(cx);

            ScrollView::new(
                cx,
                ScrollViewSettings { scrollbar_x: false, ..Default::default() },
                |cx| {
                    List::new(cx, AppData::long_list, |cx, _, item| {
                        Label::new(cx, item).width(Pixels(100.0)).height(Pixels(30.0));
                    });
                },
            );
        });
    })
    .run();
}
