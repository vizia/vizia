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

        // if let Some(list_event) = event.message.downcast() {
        //     match list_event {
        //         ListEvent::SetSelected(index) => {
        //             self.choice = self.list.get(*index).unwrap().to_owned();
        //         }

        //         _ => {}
        //     }
        // }
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
        Binding::new(cx, ThemeData::choice, |cx, choice|{
            HStack::new(cx, move |cx|{
                // Dropdown List
                Dropdown::new(cx, move |cx|
                    // A Label and an Icon
                    HStack::new(cx, move |cx|{
                        let choice = choice.get(cx).clone();
                        Label::new(cx, &choice).left(Pixels(5.0));
                        Label::new(cx, ICON_DOWN_OPEN).font("icons").left(Stretch(1.0)).right(Pixels(5.0));
                    }).child_space(Auto), 
                    move |cx|{
                    // List of options
                    List::new(cx, ThemeData::list, move |cx, item|{
                        // Need this because of a bug to do ith bindings inside a list
                        VStack::new(cx, move |cx|{
                                let option = item.get(cx).clone();
                                let is_selected = item.get(cx) == choice.get(cx);
                                // Button which updates the chosen option
                                Button::new(cx, move |cx| {
                                    cx.emit(ThemeEvent::SetTheme(option.clone()));
                                    cx.emit(PopupEvent::Close);
                                }, move |cx|{
                                    let opt = item.get(cx).clone();
                                    Label::new(cx, &opt.clone()).width(Stretch(1.0)).height(Pixels(20.0))
                                }).width(Stretch(1.0)).background_color(if is_selected {Color::from("#f8ac14")} else {Color::transparent()});
                        }).width(Stretch(1.0));
                    });
                });
            });
        });
    }).size(Auto).overflow(Overflow::Visible)
}
