use vizia::*;

const ICON_DOWN_OPEN: &str = "\u{e75c}";

const STYLE: &str = r#"
    dropdown .title {
        background-color: #FFFFFF;
        height: 30px;
        width: 100px;
        child-space: 1s;
        child-left: 5px;
    }

    dropdown>popup {
        background-color: #FFFFFF;
    }

    dropdown>popup>list {
        width: 1s;
    }

    dropdown list label {
        width: 1s;
        height: 30px;
        child-left: 6px;
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
    }
}

fn main() {
    let window_description = WindowDescription::new();
    Application::new(window_description, |cx|{

        cx.add_theme(STYLE);

        AppData {
            list: vec!["Red".to_string(), "Green".to_string(), "Blue".to_string()],
            choice: "Red".to_string(),
        }.build(cx);


        Binding::new(cx, AppData::choice, |cx, choice|{
            let option = choice.get(cx).clone();
            HStack::new(cx, move |cx|{
                // Dropdown List
                Dropdown::new(cx, move |cx|
                    // A Label and an Icon
                    HStack::new(cx, move |cx|{
                        Binding::new(cx, AppData::choice, |cx, choice|{
                            Label::new(cx, choice);
                        });
                        Label::new(cx, ICON_DOWN_OPEN).font("icons").left(Stretch(1.0)).right(Pixels(5.0));
                    }),
                    move |cx|{
                    List::new(cx, AppData::list, |cx, _, item|{
                        VStack::new(cx, move |cx|{
                            Binding::new(cx, AppData::choice, move |cx, choice|{
                                let selected = *item.get(cx) == *choice.get(cx);
                                let item = item.clone();
                                Label::new(cx, item.clone())
                                    .width(Stretch(1.0))
                                    .background_color(if selected {Color::from("#f8ac14")} else {Color::white()})
                                    .on_press(move |cx| {
                                        cx.emit(AppEvent::SetChoice(item.get(cx).clone()));
                                        cx.emit(PopupEvent::Close);
                                    });
                            });
                        }).height(Auto);
                    });
                }).width(Pixels(100.0));
            }).background_color(choice_to_color(&option)).child_space(Stretch(1.0));
        });

    }).run();
}

fn choice_to_color(name: &str) -> Color {
    match name {
        "Red" => Color::rgb(200, 100, 100),
        "Green" => Color::rgb(100, 200, 100),
        "Blue" => Color::rgb(100, 100, 200),
        _ => Color::red(),
    }
}
