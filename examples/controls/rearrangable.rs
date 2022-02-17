use vizia::*;
use vizia_glutin::application::Application;

const STYLE: &'static str = r##"
rearrangable {
    width: 100px;
    height: auto;
    space: 1s;
}

label {
    color: white;
    background-color: #0000ff;
    child-space: 1s;
    width: 100%;
    height: 30px;
    border-color: black;
    border-width: 1px;
}

.dragging label {
    background-color: #3030ff;
}
"##;

#[derive(Lens)]
pub struct AppState {
    pub items: Vec<i32>,
}

impl Model for AppState {
    fn event(&mut self, _cx: &mut Context, event: &mut Event) {
        match event.message.downcast() {
            Some(AppEvent::Swap(a, b)) => {
                self.items.swap(*a, *b);
            }
            _ => {}
        }
    }
}

#[derive(Debug)]
pub enum AppEvent {
    Swap(usize, usize),
}

fn main() {
    Application::new(WindowDescription::new().with_title("Rearrangable"), |cx| {
        cx.add_theme(STYLE);
        AppState {
            items: (10..15).collect()
        }.build(cx);

        // TODO: bind only to the relevant data (items.len(), items[idx]) instead of the whole list
        Binding::new(cx, AppState::items, move |cx, items| {
            Rearrangable::new(cx, items.get(cx).len(), move |cx, idx| {
                Label::new(cx, &format!("Item {}", items.get(cx)[idx]));
            }, move |cx, a, b| {
                cx.emit(AppEvent::Swap(a, b));
            });
        });
    })
        .run();
}
