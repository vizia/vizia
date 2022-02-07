use std::collections::HashMap;

use vizia::*;

const STYLE: &str = r#"

"#;

#[derive(Clone)]
pub struct MyData {
    text: String,
}

#[derive(Clone)]
pub struct CustomCollection {
    data: HashMap<String, MyData>,
}

impl Data for CustomCollection {
    fn same(&self, other: &Self) -> bool {

        if self.data.len() != other.data.len() {
            return false;
        }

        for ((key1,_),(key2,_)) in self.data.iter().zip(other.data.iter()) {
            if key1 != key2 {
                return false;
            }
        }

        return true;
    }
}

#[derive(Lens)]
pub struct AppData {
    collection: CustomCollection,
}

impl Model for AppData {
    fn event(&mut self, _: &mut Context, event: &mut Event) {

    }
}

#[derive(Debug)]
pub enum AppEvent {

}

fn main() {
    Application::new(WindowDescription::new().with_title("Test"), |cx| {
        cx.add_theme(STYLE);

        let mut data = HashMap::new();
        data.insert("First".to_string(), MyData{text: "one".to_string()});
        data.insert("Second".to_string(), MyData{text: "two".to_string()});
        data.insert("Third".to_string(), MyData{text: "three".to_string()});

        AppData {
            collection: CustomCollection {
                data,
            }
        }.build(cx);

        List::new(cx, AppData::collection, |cx, item|{
            Label::new(cx, &item.get(cx).clone());
        });
    })
    .run();
}

impl ListIter<String> for CustomCollection {
    fn len(&self) -> usize {
        self.data.len()
    }

    fn get_value(&self, index: usize) -> Option<&String> {

        for (q, (key, _)) in self.data.iter().enumerate() {
            if q == index {
                return Some(key);
            }
        }

        None
    }
}