use vizia::*;

const ICON_DOWN_OPEN: &str = "\u{e75c}";

const STYLE: &str = r#"
    dropdown .title {
        background-color: #101010;
        height: 30px;
        width: 100px;
        child-space: 1s;
        child-left: 5px;
    }

    dropdown>popup {
        background-color: #141414;
    }

    button {
        width: auto;
        height: auto;
        child-space: 5px;
        background-color: gray;
    }

    label {
        width: auto;
        height: auto;
        color: white;
    }
"#;

#[derive(Lens)]
pub struct AppData {
    list: Vec<String>,
    choice: String,
}

#[derive(Debug)]
pub enum AppEvent {
    SetChoice(String),
}

impl Model for AppData {
    fn event(&mut self, _: &mut Context, event: &mut Event) {
        if let Some(app_event) = event.message.downcast() {
            match app_event {
                AppEvent::SetChoice(choice) => {
                    self.choice = choice.clone();
                }
            }
        }

        if let Some(list_event) = event.message.downcast() {
            match list_event {
                ListEvent::SetSelected(index) => {
                    self.choice = self.list.get(*index).unwrap().to_owned();
                }

                _ => {}
            }
        }
    }
}

fn main() {
    let window_description = WindowDescription::new();
    Application::new(window_description, |cx|{

        cx.add_theme(STYLE);

        AppData {
            list: vec!["Red".to_string(), "Green".to_string(), "Blue".to_string()],
            choice: "Color".to_string(),
        }.build(cx);


        Binding::new(cx, AppData::choice, |cx, choice|{
            let option = choice.get(cx).clone();
            HStack::new(cx, move |cx|{
                // Dropdown List
                Dropdown::new(cx, move |cx|
                    // A Label and an Icon
                    HStack::new(cx, move |cx|{
                        let choice = choice.get(cx).clone();
                        Label::new(cx, &choice);
                        Label::new(cx, ICON_DOWN_OPEN).font("icons").left(Stretch(1.0)).right(Pixels(5.0));
                    }), 
                    move |cx|{
                    // List of options
                    List::new(cx, AppData::list, move |cx, item|{
                        // Need this because of a bug to do ith bindings inside a list
                        VStack::new(cx, move |cx|{
                                let option = item.get(cx).clone();
                                let is_selected = item.get(cx) == choice.get(cx);
                                // Button which updates the chosen option
                                Button::new(cx, move |cx| {
                                    cx.emit(AppEvent::SetChoice(option.clone()));
                                    cx.emit(PopupEvent::Close);
                                }, move |cx|{
                                    let opt = item.get(cx).clone();
                                    Label::new(cx, &opt.clone()).width(Stretch(1.0)).height(Pixels(20.0))
                                }).width(Stretch(1.0)).background_color(if is_selected {Color::from("#f8ac14")} else {Color::transparent()});
                        }).width(Stretch(1.0));
                    });
                });
                // Set background color based on the chosen value of the dropdown
            }).background_color(choice_to_color(&option));
        });

    }).run();
}

fn choice_to_color(name: &str) -> Color {
    match name {
        "Red" => Color::red(),
        "Green" => Color::green(),
        "Blue" => Color::blue(),
        _ => Color::red(),
    }
}
