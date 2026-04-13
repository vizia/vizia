use vizia::{icons::ICON_CHEVRON_DOWN, prelude::*};

use crate::DemoRegion;

pub struct DropdownData {
    list: Signal<Vec<Signal<String>>>,
    selected: Signal<usize>,
    choice: Signal<String>,
}

pub enum DropdownEvent {
    SetSelected(usize),
}

impl Model for DropdownData {
    fn event(&mut self, _cx: &mut EventContext, event: &mut Event) {
        event.map(|app_event, _| match app_event {
            DropdownEvent::SetSelected(selected) => {
                self.selected.set(*selected);
                if let Some(choice) = self.list.get().as_slice().get(*selected).map(|s| s.get()) {
                    self.choice.set(choice);
                }
            }
        })
    }
}

pub fn dropdown(cx: &mut Context) {
    let list = Signal::new(vec![
        Signal::new("Red".to_string()),
        Signal::new("Green".to_string()),
        Signal::new("Blue".to_string()),
    ]);
    let selected = Signal::new(0usize);
    let choice = Signal::new("Red".to_string());

    DropdownData { list, selected, choice }.build(cx);

    VStack::new(cx, |cx| {
        Markdown::new(cx, "# Dropdown");

        Divider::new(cx);

        Markdown::new(cx, "### Basic dropdown");

        DemoRegion::new(cx, "Basic Dropdown", move |cx| {
            Dropdown::new(
                cx,
                move |cx| {
                    Button::new(cx, |cx| Label::new(cx, choice))
                        .on_press(|cx| cx.emit(PopupEvent::Switch));
                },
                move |cx| {
                    List::new(cx, list, |cx, _, item| {
                        Label::new(cx, item).hoverable(false);
                    })
                    .selectable(Selectable::Single)
                    .on_select(|cx, index| {
                        cx.emit(DropdownEvent::SetSelected(index));
                        cx.emit(PopupEvent::Close);
                    })
                    .focused(true);
                },
            )
            .width(Pixels(100.0));
        });

        DemoRegion::new(cx, "Custom Dropdown", move |cx| {
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
                    List::new(cx, list, |cx, _, item| {
                        Label::new(cx, item).hoverable(false);
                    })
                    .selectable(Selectable::Single)
                    .on_select(|cx, index| {
                        cx.emit(DropdownEvent::SetSelected(index));
                        cx.emit(PopupEvent::Close);
                    })
                    .focused(true);
                },
            )
            .width(Pixels(100.0));
        });
    })
    .class("panel");
}
