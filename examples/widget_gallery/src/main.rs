use vizia::prelude::*;

mod app_data;
use app_data::*;

mod views;
use views::*;

fn main() {
    Application::new(|cx: &mut Context| {
        AppData::new().build(cx);

        cx.add_stylesheet(include_style!("src/style.css")).expect("Failed to add stylesheet");

        TabView::new(cx, AppData::tabs, |cx, item| match item.get(cx) {
            "All" => TabPair::new(
                move |cx| {
                    Label::new(cx, item).class("tab-name").hoverable(false);
                },
                |cx| {
                    ScrollView::new(cx, 0.0, 0.0, false, true, |cx| {
                        button(cx);
                        checkbox(cx);
                        // chip(cx);
                        // combobox(cx);
                        // datepicker(cx);
                        // hstack(cx);
                        // knob(cx);
                        // label(cx);
                        // list(cx);
                        // menu(cx);
                        // notification(cx);
                        // picklist(cx);
                        // popup(cx);
                        // radiobutton(cx);
                        // rating(cx);
                        // scrollview(cx);
                        // slider(cx);
                        // spinbox(cx);
                        // switch(cx);
                        // tabview(cx);
                        // textbox(cx);
                        // timepicker(cx);
                        // tooltip(cx);
                        // vstack(cx);
                        // zstack(cx);
                    })
                    .class("widgets");
                },
            ),

            "Button" => TabPair::new(
                move |cx| {
                    Label::new(cx, item).class("tab-name").hoverable(false);
                },
                |cx| {
                    ScrollView::new(cx, 0.0, 0.0, false, true, |cx| {
                        button2(cx);
                    })
                    .class("widgets");
                },
            ),

            "Checkbox" => TabPair::new(
                move |cx| {
                    Label::new(cx, item).class("tab-name").hoverable(false);
                },
                |cx| {
                    ScrollView::new(cx, 0.0, 0.0, false, true, |cx| {
                        checkbox(cx);
                    })
                    .class("widgets");
                },
            ),

            "Chip" => TabPair::new(
                move |cx| {
                    Label::new(cx, item).class("tab-name").hoverable(false);
                },
                |cx| {
                    ScrollView::new(cx, 0.0, 0.0, false, true, |cx| {
                        // chip(cx);
                    })
                    .class("widgets");
                },
            ),

            "Combobox" => TabPair::new(
                move |cx| {
                    Label::new(cx, item).class("tab-name").hoverable(false);
                },
                |cx| {
                    ScrollView::new(cx, 0.0, 0.0, false, true, |cx| {
                        // combobox(cx);
                    })
                    .class("widgets");
                },
            ),

            "Datepicker" => TabPair::new(
                move |cx| {
                    Label::new(cx, item).class("tab-name").hoverable(false);
                },
                |cx| {
                    ScrollView::new(cx, 0.0, 0.0, false, true, |cx| {
                        // datepicker(cx);
                    })
                    .class("widgets");
                },
            ),

            "HStack" => TabPair::new(
                move |cx| {
                    Label::new(cx, item).class("tab-name").hoverable(false);
                },
                |cx| {
                    ScrollView::new(cx, 0.0, 0.0, false, true, |cx| {
                        // hstack(cx);
                    })
                    .class("widgets");
                },
            ),

            "Knob" => TabPair::new(
                move |cx| {
                    Label::new(cx, item).class("tab-name").hoverable(false);
                },
                |cx| {
                    ScrollView::new(cx, 0.0, 0.0, false, true, |cx| {
                        // knob(cx);
                    })
                    .class("widgets");
                },
            ),

            "Label" => TabPair::new(
                move |cx| {
                    Label::new(cx, item).class("tab-name").hoverable(false);
                },
                |cx| {
                    ScrollView::new(cx, 0.0, 0.0, false, true, |cx| {
                        // label(cx);
                    })
                    .class("widgets");
                },
            ),

            "List" => TabPair::new(
                move |cx| {
                    Label::new(cx, item).class("tab-name").hoverable(false);
                },
                |cx| {
                    ScrollView::new(cx, 0.0, 0.0, false, true, |cx| {
                        // list(cx);
                    })
                    .class("widgets");
                },
            ),

            "Menu" => TabPair::new(
                move |cx| {
                    Label::new(cx, item).class("tab-name").hoverable(false);
                },
                |cx| {
                    ScrollView::new(cx, 0.0, 0.0, false, true, |cx| {
                        // menu(cx);
                    })
                    .class("widgets");
                },
            ),

            "Notification" => TabPair::new(
                move |cx| {
                    Label::new(cx, item).class("tab-name").hoverable(false);
                },
                |cx| {
                    ScrollView::new(cx, 0.0, 0.0, false, true, |cx| {
                        // notification(cx);
                    })
                    .class("widgets");
                },
            ),

            "Picklist" => TabPair::new(
                move |cx| {
                    Label::new(cx, item).class("tab-name").hoverable(false);
                },
                |cx| {
                    ScrollView::new(cx, 0.0, 0.0, false, true, |cx| {
                        // picklist(cx);
                    })
                    .class("widgets");
                },
            ),

            "Popup" => TabPair::new(
                move |cx| {
                    Label::new(cx, item).class("tab-name").hoverable(false);
                },
                |cx| {
                    ScrollView::new(cx, 0.0, 0.0, false, true, |cx| {
                        // popup(cx);
                    })
                    .class("widgets");
                },
            ),

            "Radiobutton" => TabPair::new(
                move |cx| {
                    Label::new(cx, item).class("tab-name").hoverable(false);
                },
                |cx| {
                    ScrollView::new(cx, 0.0, 0.0, false, true, |cx| {
                        // radiobutton(cx);
                    })
                    .class("widgets");
                },
            ),

            "Rating" => TabPair::new(
                move |cx| {
                    Label::new(cx, item).class("tab-name").hoverable(false);
                },
                |cx| {
                    ScrollView::new(cx, 0.0, 0.0, false, true, |cx| {
                        // rating(cx);
                    })
                    .class("widgets");
                },
            ),

            "Scrollview" => TabPair::new(
                move |cx| {
                    Label::new(cx, item).class("tab-name").hoverable(false);
                },
                |cx| {
                    ScrollView::new(cx, 0.0, 0.0, false, true, |cx| {
                        // scrollview(cx);
                    })
                    .class("widgets");
                },
            ),

            "Slider" => TabPair::new(
                move |cx| {
                    Label::new(cx, item).class("tab-name").hoverable(false);
                },
                |cx| {
                    ScrollView::new(cx, 0.0, 0.0, false, true, |cx| {
                        // slider(cx);
                    })
                    .class("widgets");
                },
            ),

            "Spinbox" => TabPair::new(
                move |cx| {
                    Label::new(cx, item).class("tab-name").hoverable(false);
                },
                |cx| {
                    ScrollView::new(cx, 0.0, 0.0, false, true, |cx| {
                        // spinbox(cx);
                    })
                    .class("widgets");
                },
            ),

            "Switch" => TabPair::new(
                move |cx| {
                    Label::new(cx, item).class("tab-name").hoverable(false);
                },
                |cx| {
                    ScrollView::new(cx, 0.0, 0.0, false, true, |cx| {
                        // switch(cx);
                    })
                    .class("widgets");
                },
            ),

            "Tabview" => TabPair::new(
                move |cx| {
                    Label::new(cx, item).class("tab-name").hoverable(false);
                },
                |cx| {
                    ScrollView::new(cx, 0.0, 0.0, false, true, |cx| {
                        // tabview(cx);
                    })
                    .class("widgets");
                },
            ),

            "Textbox" => TabPair::new(
                move |cx| {
                    Label::new(cx, item).class("tab-name").hoverable(false);
                },
                |cx| {
                    ScrollView::new(cx, 0.0, 0.0, false, true, |cx| {
                        // textbox(cx);
                    })
                    .class("widgets");
                },
            ),

            "Timepicker" => TabPair::new(
                move |cx| {
                    Label::new(cx, item).class("tab-name").hoverable(false);
                },
                |cx| {
                    ScrollView::new(cx, 0.0, 0.0, false, true, |cx| {
                        // timepicker(cx);
                    })
                    .class("widgets");
                },
            ),

            "Tooltip" => TabPair::new(
                move |cx| {
                    Label::new(cx, item).class("tab-name").hoverable(false);
                },
                |cx| {
                    ScrollView::new(cx, 0.0, 0.0, false, true, |cx| {
                        // tooltip(cx);
                    })
                    .class("widgets");
                },
            ),

            "VStack" => TabPair::new(
                move |cx| {
                    Label::new(cx, item).class("tab-name").hoverable(false);
                },
                |cx| {
                    ScrollView::new(cx, 0.0, 0.0, false, true, |cx| {
                        // vstack(cx);
                    })
                    .class("widgets");
                },
            ),

            "ZStack" => TabPair::new(
                move |cx| {
                    Label::new(cx, item).class("tab-name").hoverable(false);
                },
                |cx| {
                    ScrollView::new(cx, 0.0, 0.0, false, true, |cx| {
                        // zstack(cx);
                    })
                    .class("widgets");
                },
            ),

            _ => TabPair::new(|_| {}, |_| {}),
        })
        .class("widgets")
        .vertical();
    })
    .title("Widget Gallery")
    .inner_size((1100, 600))
    .run();
}
