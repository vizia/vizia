mod helpers;
use helpers::*;
use vizia::prelude::*;

const STYLE: &str = r#"
    virtual_list label {
        child-top: 1s;
        child-bottom: 1s;
        child-left: 5px;
        width: 1s;
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

impl Model for AppData {}

fn main() {
    Application::new(|cx| {
        cx.add_theme(STYLE);

        let list: Vec<u32> = (1..100000u32).collect();
        AppData { list, selected: 312 }.build(cx);

        ExamplePage::new(cx, |cx| {
            VirtualList::new(cx, AppData::list, 30.0, |cx, index, item| {
                Label::new(cx, item).toggle_class("light", index % 2 == 0)
            })
            .space(Pixels(0.0))
            .background_color(Color::from("#202020"));
        });
    })
    .title("Virtual List")
    .run();
}
