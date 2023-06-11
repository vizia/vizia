use vizia::prelude::*;

const STYLE: &str = r#"
    
    label {
        background-color: white;
    }

    label:checked {
        background-color: blue;
    }
"#;

#[derive(Lens)]
pub struct AppData {
    list: Vec<u32>,
}

#[derive(Debug)]
pub enum AppEvent {
    Sort,
}

impl Model for AppData {
    fn event(&mut self, _: &mut EventContext, event: &mut Event) {
        event.map(|app_event, _| match app_event {
            AppEvent::Sort => {
                self.list.sort();
            }
        });
    }
}

fn main() {
    Application::new(|cx| {
        cx.add_stylesheet(STYLE).expect("Failed to add stylesheet");

        AppData { list: vec![12, 5, 65, 31, 18, 7] }.build(cx);

        VStack::new(cx, |cx| {
            Button::new(cx, |cx| cx.emit(AppEvent::Sort), |cx| Label::new(cx, "Sort"));

            List::new(cx, AppData::list, move |cx, _, item| {
                Label::new(cx, item)
                    .width(Pixels(100.0))
                    .height(Pixels(30.0))
                    .border_color(Color::black())
                    .border_width(Pixels(1.0));
            })
            .row_between(Pixels(5.0));
        })
        .row_between(Pixels(5.0))
        .size(Auto)
        .space(Stretch(1.0))
        .top(Pixels(100.0))
        .child_space(Stretch(1.0));
    })
    .title("Sortable List")
    .run();
}
