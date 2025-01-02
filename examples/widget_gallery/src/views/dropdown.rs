use vizia::{icons::ICON_CHEVRON_DOWN, prelude::*};

use crate::DemoRegion;

#[derive(Lens)]
pub struct DropdownData {
    list: Vec<String>,
    selected: usize,
}

pub enum DropdownEvent {
    SetSelected(usize),
}

impl Model for DropdownData {
    fn event(&mut self, _cx: &mut EventContext, event: &mut Event) {
        event.map(|app_event, _| match app_event {
            DropdownEvent::SetSelected(selected) => {
                self.selected = *selected;
            }
        })
    }
}

pub fn dropdown(cx: &mut Context) {
    DropdownData {
        list: vec!["Red".to_string(), "Green".to_string(), "Blue".to_string()],
        selected: 0,
    }
    .build(cx);

    VStack::new(cx, |cx| {
        Markdown::new(cx, "# Dropdown");

        Divider::new(cx);

        Markdown::new(cx, "### Basic dropdown");

        DemoRegion::new(
            cx,
            |cx| {
                Dropdown::new(
                    cx,
                    move |cx| {
                        Button::new(cx, |cx| {
                            Label::new(cx, "").bind(DropdownData::list, move |handle, list| {
                                handle.bind(DropdownData::selected, move |handle, sel| {
                                    let selected_index = sel.get(&handle);
                                    handle.text(list.idx(selected_index));
                                });
                            })
                        })
                        .on_press(|cx| cx.emit(PopupEvent::Switch));
                    },
                    move |cx| {
                        List::new(cx, DropdownData::list, |cx, _, item| {
                            Label::new(cx, item).hoverable(false);
                        })
                        .selectable(Selectable::Single)
                        .selected(DropdownData::selected.map(|s| vec![*s]))
                        .on_select(|cx, index| {
                            cx.emit(DropdownEvent::SetSelected(index));
                            cx.emit(PopupEvent::Close);
                        })
                        .focused(true);
                    },
                )
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

        DemoRegion::new(
            cx,
            |cx| {
                Dropdown::new(
                    cx,
                    move |cx| {
                        ButtonGroup::new(cx, |cx| {
                            Button::new(cx, |cx| Label::new(cx, "Reply")).width(Stretch(1.0));

                            Button::new(cx, |cx| {
                                Svg::new(cx, ICON_CHEVRON_DOWN)
                                    .class("icon")
                                    .size(Pixels(16.0))
                                    .hoverable(false)
                            })
                            .on_press(|cx| cx.emit(PopupEvent::Switch));
                        });
                    },
                    move |cx| {
                        List::new(cx, DropdownData::list, |cx, _, item| {
                            Label::new(cx, item).hoverable(false);
                        })
                        .selectable(Selectable::Single)
                        .selected(DropdownData::selected.map(|s| vec![*s]))
                        .on_select(|cx, index| {
                            cx.emit(DropdownEvent::SetSelected(index));
                            cx.emit(PopupEvent::Close);
                        })
                        .focused(true);
                    },
                )
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
