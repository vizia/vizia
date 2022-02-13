// Support file for the control examples

use vizia::*;

const DIFFERENT: &str = r#"
    checkbox {
        background-color: white;
        border-color: blue;
        border-width: 1px;
        border-radius: 2px;
    }

    checkbox:checked {
        background-color: blue;
        border-width: 0px;
        color: white;
    }

    radiobutton {
        background-color: white;
        border-color: blue;
        border-width: 1px;
    }

    radiobutton:checked {
        background-color: blue;
        color: white;
    }
"#;

#[derive(Lens)]
pub struct ThemeData {
    list: Vec<String>,
    choice: String,
}

#[derive(Debug)]
pub enum ThemeEvent {
    SetTheme(String),
}

impl Model for ThemeData {
    fn event(&mut self, cx: &mut Context, event: &mut Event) {
        if let Some(app_event) = event.message.downcast() {
            match app_event {
                ThemeEvent::SetTheme(choice) => {
                    self.choice = choice.clone();

                    cx.remove_user_themes();

                    if self.choice.as_str() == "Material" {
                        cx.add_theme(DIFFERENT);
                    }
                }
            }
        }
    }
}

const ICON_DOWN_OPEN: &str = "\u{e75c}";

pub fn style_dropdown(cx: &mut Context) -> Handle<ZStack> {
    if cx.data::<ThemeData>().is_none() {
        ThemeData {
            list: vec!["Default".to_string(), "Material".to_string()],
            choice: "Default".to_string(),
        }
        .build(cx);
    }

    ZStack::new(cx, |cx|{
        //Binding::new(cx, ThemeData::choice, |cx, choice|{
            HStack::new(cx, move |cx|{
                // Dropdown List
                Dropdown::new(cx, move |cx|
                    // A Label and an Icon
                    HStack::new(cx, move |cx|{
                        Binding::new(cx, ThemeData::choice, |cx, choice|{
                            Label::new(cx, &choice.get(cx).to_string());
                        });
                        Label::new(cx, ICON_DOWN_OPEN).font("icons").left(Stretch(1.0)).right(Pixels(5.0));
                    }),
                    move |cx|{
                    // List of options
                    List::new(cx, ThemeData::list, |cx, _, item|{
                        VStack::new(cx, move |cx|{
                            Binding::new(cx, ThemeData::choice, move |cx, choice|{
                                let selected = *item.get(cx) == *choice.get(cx);
                                let item = item.clone();
                                Label::new(cx, &item.get(cx).to_string())
                                    .width(Stretch(1.0))
                                    .background_color(if selected {Color::from("#f8ac14")} else {Color::white()})
                                    .on_press(move |cx| {
                                        cx.emit(ThemeEvent::SetTheme(item.get(cx).clone()));
                                        cx.emit(PopupEvent::Close);
                                    });
                            });
                        }).height(Auto);
                    });
                }).width(Pixels(100.0));
            });
        //});
    }).size(Auto).overflow(Overflow::Visible)
}
