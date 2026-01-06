use vizia::{icons::ICON_CHEVRON_DOWN, prelude::*};

use crate::DemoRegion;

pub fn dropdown(cx: &mut Context) {
    let list = cx.state(vec!["Red".to_string(), "Green".to_string(), "Blue".to_string()]);
    let selected = cx.state(0usize);
    let width_100 = cx.state(Pixels(100.0));
    let stretch_one = cx.state(Stretch(1.0));
    let icon_size = cx.state(Pixels(16.0));
    let selectable_single = cx.state(Selectable::Single);
    let selected_indices = selected.drv(cx, |v, _| vec![*v]);
    let selected_label = selected.drv(cx, move |v, s| {
        list.get(s).get(*v).cloned().unwrap_or_default()
    });

    VStack::new(cx, move |cx| {
        Markdown::new(cx, "# Dropdown");

        Divider::new(cx);

        Markdown::new(cx, "### Basic dropdown");

        DemoRegion::new(
            cx,
            move |cx| {
                Dropdown::new(
                    cx,
                    move |cx| {
                        Button::new(cx, |cx| Label::new(cx, selected_label))
                        .on_press(|cx| cx.emit(PopupEvent::Switch));
                    },
                    move |cx| {
                        List::new(cx, list, move |cx, _, item| {
                            Label::new(cx, item).hoverable(false);
                        })
                        .selectable(selectable_single)
                        .selected(selected_indices)
                        .on_select(move |cx, index| {
                            selected.set(cx, index);
                            cx.emit(PopupEvent::Close);
                        })
                        .focused(true);
                    },
                )
                .width(width_100);
            },
            r#"let list = cx.state(vec!["Red".to_string(), "Green".to_string(), "Blue".to_string()]);
let selected = cx.state(0usize);
let width_100 = cx.state(Pixels(100.0));
let selected_indices = selected.drv(cx, |v, _| vec![*v]);
let selected_label = selected.drv(cx, move |v, s| {
    list.get(s).get(*v).cloned().unwrap_or_default()
});
Dropdown::new(
    cx,
    move |cx| {
        Button::new(cx, |cx| {
            Label::new(cx, selected_label);
        })
        .on_press(|cx| cx.emit(PopupEvent::Switch));
    },
    move |cx| {
        List::new(cx, list, move |cx, _, item| {
            Label::new(cx, item).hoverable(false);
        })
        .selectable(selectable_single)
        .selected(selected_indices)
        .on_select(|cx, index| {
            selected.set(cx, index);
            cx.emit(PopupEvent::Close);
        })
        .focused(true);
    },
)
.width(width_100);"#,
        );

        DemoRegion::new(
            cx,
            move |cx| {
                Dropdown::new(
                    cx,
                    move |cx| {
                        ButtonGroup::new(cx, |cx| {
                            Button::new(cx, |cx| Label::new(cx, "Reply"))
                                .width(stretch_one);

                            Button::new(cx, |cx| {
                                Svg::new(cx, ICON_CHEVRON_DOWN)
                                    .class("icon")
                                    .size(icon_size)
                                    .hoverable(false)
                            })
                            .on_press(|cx| cx.emit(PopupEvent::Switch));
                        });
                    },
                    move |cx| {
                        List::new(cx, list, move |cx, _, item| {
                            Label::new(cx, item).hoverable(false);
                        })
                        .selectable(selectable_single)
                        .selected(selected_indices)
                        .on_select(move |cx, index| {
                            selected.set(cx, index);
                            cx.emit(PopupEvent::Close);
                        })
                        .focused(true);
                    },
                )
                .width(width_100);
            },
            r#"let list = cx.state(vec!["Red".to_string(), "Green".to_string(), "Blue".to_string()]);
let selected = cx.state(0usize);
let width_100 = cx.state(Pixels(100.0));
let stretch_one = cx.state(Stretch(1.0));
let icon_size = cx.state(Pixels(16.0));
let selected_indices = selected.drv(cx, |v, _| vec![*v]);
Dropdown::new(
    cx,
    move |cx| {
        ButtonGroup::new(cx, |cx| {
            Button::new(cx, |cx| Label::new(cx, "Reply")).width(stretch_one);

            Button::new(cx, |cx| {
                Svg::new(cx, ICON_CHEVRON_DOWN)
                    .class("icon")
                    .size(icon_size)
                    .hoverable(false)
            })
            .on_press(|cx| cx.emit(PopupEvent::Switch));
        });
    },
    move |cx| {
        List::new(cx, list, move |cx, _, item| {
            Label::new(cx, item).hoverable(false);
        })
        .selectable(selectable_single)
        .selected(selected_indices)
        .on_select(|cx, index| {
            selected.set(cx, index);
            cx.emit(PopupEvent::Close);
        })
        .focused(true);
    },
)
.width(width_100);"#,
        );
    })
    .class("panel");
}
