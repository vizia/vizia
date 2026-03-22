mod helpers;
use helpers::*;
use vizia::prelude::*;

pub struct AppData {
    list: Signal<Vec<String>>,
    selected: Signal<usize>,
    choice: Signal<String>,
}

pub enum AppEvent {
    SetSelected(usize),
}

impl Model for AppData {
    fn event(&mut self, _cx: &mut EventContext, event: &mut Event) {
        event.map(|app_event, _| match app_event {
            AppEvent::SetSelected(selected) => {
                self.selected.set(*selected);
                let options = self.list.get();
                if let Some(choice) = options.as_slice().get(*selected).cloned() {
                    self.choice.set(choice);
                }
            }
        })
    }
}

fn main() -> Result<(), ApplicationError> {
    Application::new(|cx| {
        let list = Signal::new(vec!["Red".to_string(), "Green".to_string(), "Blue".to_string()]);
        let selected = Signal::new(0usize);
        let choice = Signal::new("Red".to_string());

        AppData { list, selected, choice }.build(cx);

        ExamplePage::new(cx, |cx| {
            Dropdown::new(
                cx,
                move |cx| {
                    Button::new(cx, |cx| Label::new(cx, choice))
                        .on_press(|cx| cx.emit(PopupEvent::Switch));
                },
                move |cx| {
                    Binding::new(cx, list, move |cx, options| {
                        let options = options.clone();

                        Binding::new(cx, selected, move |cx, selected_index| {
                            let options = options.clone();

                            VStack::new(cx, move |cx| {
                                for (index, item) in options.iter().enumerate() {
                                    let item = item.clone();

                                    Button::new(cx, move |cx| {
                                        Label::new(cx, item.clone()).hoverable(false)
                                    })
                                    .width(Stretch(1.0))
                                    .checked(selected_index == index)
                                    .on_press(move |cx| {
                                        cx.emit(AppEvent::SetSelected(index));
                                        cx.emit(PopupEvent::Close);
                                    });
                                }
                            })
                            .width(Stretch(1.0));
                        });
                    })
                },
            )
            .width(Pixels(100.0));
        });
    })
    .title("Dropdown")
    .inner_size((350, 300))
    .run()
}
