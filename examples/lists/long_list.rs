use vizia::prelude::*;

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
        HStack::new(cx, |cx| {
            AppData { long_list: (0..1000).collect() }.build(cx);

            ScrollView::new(cx, 0.0, 0.0, false, true, |cx| {
                List::new(cx, AppData::long_list, |cx, _, item| {
                    Label::new(cx, item).width(Pixels(100.0)).height(Pixels(30.0));
                });
            });
        });
    })
    .run();
}
