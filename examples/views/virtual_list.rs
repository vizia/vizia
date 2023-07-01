mod helpers;
use helpers::*;
use vizia::prelude::*;

const STYLE: &str = r#"
    virtual_list label {
        child-top: 1s;
        child-bottom: 1s;
        child-left: 5px;
        width: 1s;
        color: white;
    }

    virtual_list label.light {
        background-color: #242424;
    }

    virtual_list .scroll_content {
        width: 1s;
    }

"#;

#[derive(Lens)]
pub struct AppData {
    list: Vec<u32>,
    selected: usize,
}

pub enum AppEvent {
    SetValue(usize, u32),
}

impl Model for AppData {
    fn event(&mut self, _cx: &mut EventContext, event: &mut Event) {
        event.map(|app_event, _| match app_event {
            AppEvent::SetValue(index, value) => {
                self.list[*index] = *value;
            }
        });
    }
}

fn main() {
    Application::new(|cx| {
        cx.add_stylesheet(STYLE).expect("Failed to add stylesheet");

        let list: Vec<u32> = (1..100u32).collect();
        AppData { list, selected: 312 }.build(cx);

        ExamplePage::new(cx, |cx| {
            VirtualList::new(cx, AppData::list, 40.0, |cx, index, item| {
                Label::new(cx, item).toggle_class("light", index % 2 == 0)
                // HStack::new(cx, |cx| {
                //     Textbox::new(cx, item).width(Pixels(100.0)).height(Pixels(32.0)).on_submit(
                //         move |cx, txt, _| {
                //             if let Ok(val) = txt.parse() {
                //                 cx.emit(AppEvent::SetValue(index, val));
                //             }
                //         },
                //     );
                // })
                // .child_top(Stretch(1.0))
                // .child_bottom(Stretch(1.0))
                // .child_left(Pixels(10.0))
                // .toggle_class("light", index % 2 == 0)
            })
            .space(Pixels(0.0))
            .background_color(Color::from("#202020"));
        });
    })
    .title("Virtual List")
    .run();
}
