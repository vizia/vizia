use vizia::prelude::*;

use crate::DemoRegion;

#[derive(Lens)]
pub struct DropdownData {
    list: Vec<String>,
    choice: String,
}

pub enum DropdownEvent {
    SetChoice(String),
}

impl Model for DropdownData {
    fn event(&mut self, _cx: &mut EventContext, event: &mut Event) {
        event.map(|app_event, _| match app_event {
            DropdownEvent::SetChoice(choice) => {
                self.choice = choice.clone();
            }
        })
    }
}

pub fn dropdown(cx: &mut Context) {
    DropdownData {
        list: vec!["Red".to_string(), "Green".to_string(), "Blue".to_string()],
        choice: "Red".to_string(),
    }
    .build(cx);

    VStack::new(cx, |cx| {
        Label::new(cx, "Dropdown").class("title");
        Label::new(cx, "").class("paragraph");

        Divider::new(cx).top(Pixels(12.0)).bottom(Pixels(12.0));

        Label::new(cx, "Basic dropdown").class("header");

        DemoRegion::new(
            cx,
            |cx| {
                Dropdown::new(
                    cx,
                    move |cx| {
                        Button::new(cx, |cx| Label::new(cx, DropdownData::choice))
                            .on_press(|cx| cx.emit(PopupEvent::Switch));
                    },
                    move |cx| {
                        List::new(cx, DropdownData::list, |cx, _, item| {
                            Label::new(cx, item)
                                .cursor(CursorIcon::Hand)
                                .bind(DropdownData::choice, move |handle, selected| {
                                    if item.get(&handle) == selected.get(&handle) {
                                        handle.checked(true);
                                    }
                                })
                                .on_press(move |cx| {
                                    cx.emit(DropdownEvent::SetChoice(item.get(cx)));
                                    cx.emit(PopupEvent::Close);
                                });
                        });
                    },
                )
                .top(Pixels(40.0))
                .width(Pixels(100.0));
            },
            r#"Dropdown::new(
        cx,
        move |cx| Label::new(cx, AppData::choice),
        move |cx| {
            List::new(cx, AppData::list, |cx, _, item| {
                Label::new(cx, item)
                    .cursor(CursorIcon::Hand)
                    .bind(AppData::choice, move |handle, selected| {
                        if item.get(&handle) == selected.get(&handle) {
                            handle.checked(true);
                        }
                    })
                    .on_press(move |cx| {
                        cx.emit(AppEvent::SetChoice(item.get(cx)));
                        cx.emit(PopupEvent::Close);
                    });
            });
        },
    )
    .top(Pixels(40.0))
    .width(Pixels(100.0));"#,
        );
    })
    .class("panel");
}
