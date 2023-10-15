use lazy_static::lazy_static;
use vizia::prelude::*;
use vizia_core::icons::ICON_MOON;

lazy_static! {
    pub static ref STATIC_LIST: Vec<&'static str> =
        vec!["Wrapping", "Alignment", "Alignment2", "Alignment3", "Alignment4", "Alignment5"];
}

#[derive(Lens)]
pub struct AppData {
    // text: Vec<String>,
    text: String,
    text2: String,
}

pub enum AppEvent {
    SetText(String),
}

impl Model for AppData {
    fn event(&mut self, _cx: &mut EventContext, event: &mut Event) {
        event.map(|app_event, _| match app_event {
            AppEvent::SetText(txt) => self.text = txt.clone(),
        });
    }
}

fn main() {
    Application::new(|cx| {
        AppData { text: String::from("This is some text"), text2: String::from("سلام") }.build(cx);
        VStack::new(cx, |cx| {
            TabView::new(
                cx,
                StaticLens::<Vec<&'static str>>::new(STATIC_LIST.as_ref()),
                |cx, item| match item.get(cx) {
                    "Wrapping" => TabPair::new(
                        move |cx| {
                            Label::new(cx, item).hoverable(false);
                            Element::new(cx).class("indicator");
                        },
                        |cx| {
                            wrapping(cx);
                        },
                    ),

                    "Alignment" => TabPair::new(
                        move |cx| {
                            Label::new(cx, item).hoverable(false);
                            Element::new(cx).class("indicator");
                        },
                        |cx| {
                            alignment(cx);
                        },
                    ),

                    "Alignment2" => TabPair::new(
                        move |cx| {
                            Label::new(cx, item).hoverable(false);
                            Element::new(cx).class("indicator");
                        },
                        |cx| {
                            alignment2(cx);
                        },
                    ),

                    "Alignment3" => TabPair::new(
                        move |cx| {
                            Label::new(cx, item).hoverable(false);
                            Element::new(cx).class("indicator");
                        },
                        |cx| {
                            alignment3(cx);
                        },
                    ),

                    "Alignment4" => TabPair::new(
                        move |cx| {
                            Label::new(cx, item).hoverable(false);
                            Element::new(cx).class("indicator");
                        },
                        |cx: &mut Context| {
                            alignment4(cx);
                        },
                    ),

                    "Alignment5" => TabPair::new(
                        move |cx| {
                            Label::new(cx, item).hoverable(false);
                            Element::new(cx).class("indicator");
                        },
                        |cx: &mut Context| {
                            alignment5(cx);
                        },
                    ),

                    _ => unreachable!(),
                },
            );

            // Textbox::new(cx, AppData::text.index(0))
            //     .on_submit(|ex, txt, _| ex.emit(AppEvent::SetText(0, txt.clone())));

            // Textbox::new(cx, AppData::text.index(0))
            //     .width(Pixels(200.0))
            //     .on_submit(|ex, txt, _| ex.emit(AppEvent::SetText(0, txt.clone())));

            // Textbox::new_multiline(cx, AppData::text.index(1), false)
            //     .on_submit(|ex, txt, _| ex.emit(AppEvent::SetText(1, txt.clone())));

            // Textbox::new_multiline(cx, AppData::text.index(2), true)
            //     .on_submit(|ex, txt, _| ex.emit(AppEvent::SetText(2, txt.clone())));

            // Textbox::new_multiline(cx, AppData::text.index(3), true)
            // .width(Pixels(200.0))
            // .child_left(Stretch(1.0))
            // .on_submit(|ex, txt, _| ex.emit(AppEvent::SetText(3, txt.clone())));
        })
        .child_space(Pixels(20.0))
        .row_between(Pixels(20.0));
    })
    .title("Text")
    .inner_size((1200, 600))
    .run();
}

fn wrapping(cx: &mut Context) {
    HStack::new(cx, |cx| {
        HStack::new(cx, |cx| {
            Label::new(cx, "This is some non-wrapping text")
                .child_space(Pixels(10.0))
                .background_color(Color::rgb(200, 100, 200));
        })
        .size(Auto)
        .child_space(Pixels(10.0))
        .background_color(Color::rgb(100, 200, 100));

        HStack::new(cx, |cx| {
            Label::new(cx, "This is some text which should wrap on the longest word.")
                .text_wrap(true)
                .child_space(Pixels(10.0))
                .background_color(Color::rgb(200, 100, 100));
        })
        .size(Auto)
        .child_space(Pixels(10.0))
        .background_color(Color::rgb(100, 200, 100));

        HStack::new(cx, |cx| {
            Label::new(
                cx,
                "This is some text which should wrap because its container is too narrow.",
            )
            .text_wrap(true)
            .width(Pixels(100.0))
            .child_space(Pixels(10.0))
            .background_color(Color::rgb(200, 100, 100));
        })
        .size(Auto)
        .child_space(Pixels(10.0))
        .background_color(Color::rgb(100, 200, 100));

        HStack::new(cx, |cx| {
            Label::new(cx, "This is some text which should\nwrap because of a hard break.")
                .text_wrap(false)
                .width(Auto)
                .child_space(Pixels(10.0))
                .background_color(Color::rgb(200, 100, 100));
        })
        .size(Auto)
        .child_space(Pixels(10.0))
        .background_color(Color::rgb(100, 200, 100));

        HStack::new(cx, |cx| {
            Label::new(cx, "This is some text which should\nwrap because of soft and hard breaks.")
                .text_wrap(true)
                .width(Pixels(100.0))
                .child_space(Pixels(10.0))
                .background_color(Color::rgb(200, 100, 100));
        })
        .size(Auto)
        .child_space(Pixels(10.0))
        .background_color(Color::rgb(100, 200, 100));
    })
    .col_between(Pixels(50.0))
    .child_space(Pixels(50.0));
}

fn alignment(cx: &mut Context) {
    HStack::new(cx, |cx| {
        HStack::new(cx, |cx| {
            Label::new(cx, "This is some\ntext aligned to\nthe top left")
                .text_wrap(false)
                .size(Pixels(150.0))
                .background_color(Color::rgb(200, 100, 100));
        })
        .size(Auto)
        .child_space(Pixels(10.0))
        .background_color(Color::rgb(100, 200, 100));

        HStack::new(cx, |cx| {
            Label::new(cx, "This is some\ntext aligned to\nthe top center")
                .text_wrap(false)
                .size(Pixels(150.0))
                .child_left(Stretch(1.0))
                .child_right(Stretch(1.0))
                .background_color(Color::rgb(200, 100, 100));
        })
        .size(Auto)
        .child_space(Pixels(10.0))
        .background_color(Color::rgb(100, 200, 100));

        HStack::new(cx, |cx| {
            Label::new(cx, "This is some\ntext aligned to\nthe top right")
                .size(Pixels(150.0))
                .child_left(Stretch(1.0))
                .child_right(Pixels(0.0))
                .background_color(Color::rgb(200, 100, 100));
        })
        .size(Auto)
        .child_space(Pixels(10.0))
        .background_color(Color::rgb(100, 200, 100));
    })
    .size(Auto)
    .col_between(Pixels(20.0))
    .child_space(Pixels(20.0));

    HStack::new(cx, |cx| {
        HStack::new(cx, |cx| {
            Label::new(cx, "This is some\ntext aligned to\nthe middle left")
                .text_wrap(false)
                .size(Pixels(150.0))
                .child_top(Stretch(1.0))
                .child_bottom(Stretch(1.0))
                .background_color(Color::rgb(200, 100, 100));
        })
        .size(Auto)
        .child_space(Pixels(10.0))
        .background_color(Color::rgb(100, 200, 100));

        HStack::new(cx, |cx| {
            Label::new(cx, "This is some\ntext aligned to\nthe middle center")
                .text_wrap(false)
                .size(Pixels(150.0))
                .child_space(Stretch(1.0))
                .background_color(Color::rgb(200, 100, 100));
        })
        .size(Auto)
        .child_space(Pixels(10.0))
        .background_color(Color::rgb(100, 200, 100));

        HStack::new(cx, |cx| {
            Label::new(cx, "This is some\ntext aligned to\nthe middle right")
                .size(Pixels(150.0))
                .child_top(Stretch(1.0))
                .child_bottom(Stretch(1.0))
                .child_left(Stretch(1.0))
                .child_right(Pixels(0.0))
                .background_color(Color::rgb(200, 100, 100));
        })
        .size(Auto)
        .child_space(Pixels(10.0))
        .background_color(Color::rgb(100, 200, 100));
    })
    .size(Auto)
    .col_between(Pixels(20.0))
    .child_space(Pixels(20.0));

    HStack::new(cx, |cx| {
        HStack::new(cx, |cx| {
            Label::new(cx, "This is some\ntext aligned to\nthe bottom left")
                .text_wrap(false)
                .size(Pixels(150.0))
                .child_top(Stretch(1.0))
                .child_bottom(Pixels(0.0))
                .background_color(Color::rgb(200, 100, 100));
        })
        .size(Auto)
        .child_space(Pixels(10.0))
        .background_color(Color::rgb(100, 200, 100));

        HStack::new(cx, |cx| {
            Label::new(cx, "This is some\ntext aligned to\nthe bottom center")
                .text_wrap(false)
                .size(Pixels(150.0))
                .child_space(Stretch(1.0))
                .child_bottom(Pixels(0.0))
                .background_color(Color::rgb(200, 100, 100));
        })
        .size(Auto)
        .child_space(Pixels(10.0))
        .background_color(Color::rgb(100, 200, 100));

        HStack::new(cx, |cx| {
            Label::new(cx, "This is some\ntext aligned to\nthe bottom right")
                .size(Pixels(150.0))
                .child_top(Stretch(1.0))
                .child_bottom(Pixels(0.0))
                .child_left(Stretch(1.0))
                .child_right(Pixels(0.0))
                .background_color(Color::rgb(200, 100, 100));
        })
        .size(Auto)
        .child_space(Pixels(10.0))
        .background_color(Color::rgb(100, 200, 100));
    })
    .size(Auto)
    .col_between(Pixels(20.0))
    .child_space(Pixels(20.0));
}

fn alignment2(cx: &mut Context) {
    HStack::new(cx, |cx| {
        HStack::new(cx, |cx| {
            Label::new(cx, "سلام")
                .text_wrap(false)
                .size(Pixels(150.0))
                .child_right(Stretch(1.0))
                .background_color(Color::rgb(200, 100, 100));
        })
        .size(Auto)
        .child_space(Pixels(10.0))
        .background_color(Color::rgb(100, 200, 100));

        HStack::new(cx, |cx| {
            Label::new(cx, "سلام")
                .text_wrap(false)
                .size(Pixels(150.0))
                .child_left(Stretch(1.0))
                .child_right(Stretch(1.0))
                .background_color(Color::rgb(200, 100, 100));
        })
        .size(Auto)
        .child_space(Pixels(10.0))
        .background_color(Color::rgb(100, 200, 100));

        HStack::new(cx, |cx| {
            Label::new(cx, "سلام")
                .size(Pixels(150.0))
                .child_left(Stretch(1.0))
                .child_right(Pixels(0.0))
                .background_color(Color::rgb(200, 100, 100));
        })
        .size(Auto)
        .child_space(Pixels(10.0))
        .background_color(Color::rgb(100, 200, 100));
    })
    .size(Auto)
    .col_between(Pixels(20.0))
    .child_space(Pixels(20.0));

    HStack::new(cx, |cx| {
        HStack::new(cx, |cx| {
            Label::new(cx, "سلام")
                .text_wrap(false)
                .size(Pixels(150.0))
                .child_top(Stretch(1.0))
                .child_bottom(Stretch(1.0))
                .child_right(Stretch(1.0))
                .background_color(Color::rgb(200, 100, 100));
        })
        .size(Auto)
        .child_space(Pixels(10.0))
        .background_color(Color::rgb(100, 200, 100));

        HStack::new(cx, |cx| {
            Label::new(cx, "سلام")
                .text_wrap(false)
                .size(Pixels(150.0))
                .child_space(Stretch(1.0))
                .background_color(Color::rgb(200, 100, 100));
        })
        .size(Auto)
        .child_space(Pixels(10.0))
        .background_color(Color::rgb(100, 200, 100));

        HStack::new(cx, |cx| {
            Label::new(cx, "سلام")
                .size(Pixels(150.0))
                .child_top(Stretch(1.0))
                .child_bottom(Stretch(1.0))
                .child_left(Stretch(1.0))
                .child_right(Pixels(0.0))
                .background_color(Color::rgb(200, 100, 100));
        })
        .size(Auto)
        .child_space(Pixels(10.0))
        .background_color(Color::rgb(100, 200, 100));
    })
    .size(Auto)
    .col_between(Pixels(20.0))
    .child_space(Pixels(20.0));

    HStack::new(cx, |cx| {
        HStack::new(cx, |cx| {
            Label::new(cx, "سلام")
                .text_wrap(false)
                .size(Pixels(150.0))
                .child_top(Stretch(1.0))
                .child_bottom(Pixels(0.0))
                .child_right(Stretch(1.0))
                .background_color(Color::rgb(200, 100, 100));
        })
        .size(Auto)
        .child_space(Pixels(10.0))
        .background_color(Color::rgb(100, 200, 100));

        HStack::new(cx, |cx| {
            Label::new(cx, "سلام")
                .text_wrap(false)
                .size(Pixels(150.0))
                .child_space(Stretch(1.0))
                .child_bottom(Pixels(0.0))
                .background_color(Color::rgb(200, 100, 100));
        })
        .size(Auto)
        .child_space(Pixels(10.0))
        .background_color(Color::rgb(100, 200, 100));

        HStack::new(cx, |cx| {
            Label::new(cx, "سلام")
                .size(Pixels(150.0))
                .child_top(Stretch(1.0))
                .child_bottom(Pixels(0.0))
                .child_left(Stretch(1.0))
                .child_right(Pixels(0.0))
                .background_color(Color::rgb(200, 100, 100));
        })
        .size(Auto)
        .child_space(Pixels(10.0))
        .background_color(Color::rgb(100, 200, 100));
    })
    .size(Auto)
    .col_between(Pixels(20.0))
    .child_space(Pixels(20.0));
}

fn alignment3(cx: &mut Context) {
    HStack::new(cx, |cx| {
        HStack::new(cx, |cx| {
            Textbox::new(cx, AppData::text)
                .text_wrap(false)
                .size(Pixels(150.0))
                .child_space(Pixels(0.0))
                .child_right(Stretch(1.0))
                .background_color(Color::rgb(200, 100, 100));
        })
        .size(Auto)
        .child_space(Pixels(10.0))
        .background_color(Color::rgb(100, 200, 100));

        HStack::new(cx, |cx| {
            Textbox::new(cx, AppData::text)
                .text_wrap(false)
                .size(Pixels(150.0))
                .child_top(Pixels(0.0))
                .child_left(Stretch(1.0))
                .child_right(Stretch(1.0))
                .background_color(Color::rgb(200, 100, 100));
        })
        .size(Auto)
        .child_space(Pixels(10.0))
        .background_color(Color::rgb(100, 200, 100));

        HStack::new(cx, |cx| {
            Textbox::new(cx, AppData::text)
                .size(Pixels(150.0))
                .child_top(Pixels(0.0))
                .child_left(Stretch(1.0))
                .child_right(Pixels(0.0))
                .background_color(Color::rgb(200, 100, 100));
        })
        .size(Auto)
        .child_space(Pixels(10.0))
        .background_color(Color::rgb(100, 200, 100));
    })
    .size(Auto)
    .col_between(Pixels(20.0))
    .child_space(Pixels(20.0));

    HStack::new(cx, |cx| {
        HStack::new(cx, |cx| {
            Textbox::new(cx, AppData::text)
                .text_wrap(false)
                .size(Pixels(150.0))
                .child_left(Pixels(0.0))
                .child_top(Stretch(1.0))
                .child_bottom(Stretch(1.0))
                .child_right(Stretch(1.0))
                .background_color(Color::rgb(200, 100, 100));
        })
        .size(Auto)
        .child_space(Pixels(10.0))
        .background_color(Color::rgb(100, 200, 100));

        HStack::new(cx, |cx| {
            Textbox::new(cx, AppData::text)
                .text_wrap(false)
                .size(Pixels(150.0))
                .child_space(Stretch(1.0))
                .background_color(Color::rgb(200, 100, 100));
        })
        .size(Auto)
        .child_space(Pixels(10.0))
        .background_color(Color::rgb(100, 200, 100));

        HStack::new(cx, |cx| {
            Textbox::new(cx, AppData::text)
                .size(Pixels(150.0))
                .child_top(Stretch(1.0))
                .child_bottom(Stretch(1.0))
                .child_left(Stretch(1.0))
                .child_right(Pixels(0.0))
                .background_color(Color::rgb(200, 100, 100));
        })
        .size(Auto)
        .child_space(Pixels(10.0))
        .background_color(Color::rgb(100, 200, 100));
    })
    .size(Auto)
    .col_between(Pixels(20.0))
    .child_space(Pixels(20.0));

    HStack::new(cx, |cx| {
        HStack::new(cx, |cx| {
            Textbox::new(cx, AppData::text)
                .text_wrap(false)
                .size(Pixels(150.0))
                .child_space(Pixels(0.0))
                .child_top(Stretch(1.0))
                .child_right(Stretch(1.0))
                .background_color(Color::rgb(200, 100, 100));
        })
        .size(Auto)
        .child_space(Pixels(10.0))
        .background_color(Color::rgb(100, 200, 100));

        HStack::new(cx, |cx| {
            Textbox::new(cx, AppData::text)
                .text_wrap(false)
                .size(Pixels(150.0))
                .child_space(Stretch(1.0))
                .child_bottom(Pixels(0.0))
                .background_color(Color::rgb(200, 100, 100));
        })
        .size(Auto)
        .child_space(Pixels(10.0))
        .background_color(Color::rgb(100, 200, 100));

        HStack::new(cx, |cx| {
            Textbox::new(cx, AppData::text)
                .size(Pixels(150.0))
                .child_top(Stretch(1.0))
                .child_bottom(Pixels(0.0))
                .child_left(Stretch(1.0))
                .child_right(Pixels(0.0))
                .background_color(Color::rgb(200, 100, 100));
        })
        .size(Auto)
        .child_space(Pixels(10.0))
        .background_color(Color::rgb(100, 200, 100));
    })
    .size(Auto)
    .col_between(Pixels(20.0))
    .child_space(Pixels(20.0));
}

fn alignment4(cx: &mut Context) {
    HStack::new(cx, |cx| {
        HStack::new(cx, |cx| {
            Textbox::new(cx, AppData::text2)
                .text_wrap(false)
                .size(Pixels(150.0))
                .child_space(Pixels(0.0))
                .child_right(Stretch(1.0))
                .background_color(Color::rgb(200, 100, 100));
        })
        .size(Auto)
        .child_space(Pixels(10.0))
        .background_color(Color::rgb(100, 200, 100));

        HStack::new(cx, |cx| {
            Textbox::new(cx, AppData::text2)
                .text_wrap(false)
                .size(Pixels(150.0))
                .child_top(Pixels(0.0))
                .child_left(Stretch(1.0))
                .child_right(Stretch(1.0))
                .background_color(Color::rgb(200, 100, 100));
        })
        .size(Auto)
        .child_space(Pixels(10.0))
        .background_color(Color::rgb(100, 200, 100));

        HStack::new(cx, |cx| {
            Textbox::new(cx, AppData::text2)
                .size(Pixels(150.0))
                .child_top(Pixels(0.0))
                .child_left(Stretch(1.0))
                .child_right(Pixels(0.0))
                .background_color(Color::rgb(200, 100, 100));
        })
        .size(Auto)
        .child_space(Pixels(10.0))
        .background_color(Color::rgb(100, 200, 100));
    })
    .size(Auto)
    .col_between(Pixels(20.0))
    .child_space(Pixels(20.0));

    HStack::new(cx, |cx| {
        HStack::new(cx, |cx| {
            Textbox::new(cx, AppData::text2)
                .text_wrap(false)
                .size(Pixels(150.0))
                .child_left(Pixels(0.0))
                .child_top(Stretch(1.0))
                .child_bottom(Stretch(1.0))
                .child_right(Stretch(1.0))
                .background_color(Color::rgb(200, 100, 100));
        })
        .size(Auto)
        .child_space(Pixels(10.0))
        .background_color(Color::rgb(100, 200, 100));

        HStack::new(cx, |cx| {
            Textbox::new(cx, AppData::text2)
                .text_wrap(false)
                .size(Pixels(150.0))
                .child_space(Stretch(1.0))
                .background_color(Color::rgb(200, 100, 100));
        })
        .size(Auto)
        .child_space(Pixels(10.0))
        .background_color(Color::rgb(100, 200, 100));

        HStack::new(cx, |cx| {
            Textbox::new(cx, AppData::text2)
                .size(Pixels(150.0))
                .child_top(Stretch(1.0))
                .child_bottom(Stretch(1.0))
                .child_left(Stretch(1.0))
                .child_right(Pixels(0.0))
                .background_color(Color::rgb(200, 100, 100));
        })
        .size(Auto)
        .child_space(Pixels(10.0))
        .background_color(Color::rgb(100, 200, 100));
    })
    .size(Auto)
    .col_between(Pixels(20.0))
    .child_space(Pixels(20.0));

    HStack::new(cx, |cx| {
        HStack::new(cx, |cx| {
            Textbox::new(cx, AppData::text2)
                .text_wrap(false)
                .size(Pixels(150.0))
                .child_space(Pixels(0.0))
                .child_top(Stretch(1.0))
                .child_right(Stretch(1.0))
                .background_color(Color::rgb(200, 100, 100));
        })
        .size(Auto)
        .child_space(Pixels(10.0))
        .background_color(Color::rgb(100, 200, 100));

        HStack::new(cx, |cx| {
            Textbox::new(cx, AppData::text2)
                .text_wrap(false)
                .size(Pixels(150.0))
                .child_space(Stretch(1.0))
                .child_bottom(Pixels(0.0))
                .background_color(Color::rgb(200, 100, 100));
        })
        .size(Auto)
        .child_space(Pixels(10.0))
        .background_color(Color::rgb(100, 200, 100));

        HStack::new(cx, |cx| {
            Textbox::new(cx, AppData::text2)
                .size(Pixels(150.0))
                .child_top(Stretch(1.0))
                .child_bottom(Pixels(0.0))
                .child_left(Stretch(1.0))
                .child_right(Pixels(0.0))
                .background_color(Color::rgb(200, 100, 100));
        })
        .size(Auto)
        .child_space(Pixels(10.0))
        .background_color(Color::rgb(100, 200, 100));
    })
    .size(Auto)
    .col_between(Pixels(20.0))
    .child_space(Pixels(20.0));
}

fn alignment5(cx: &mut Context) {
    HStack::new(cx, |cx| {
        HStack::new(cx, |cx| {
            Label::new(cx, ICON_MOON)
                .class("icon")
                .text_wrap(false)
                .size(Pixels(150.0))
                .background_color(Color::rgb(200, 100, 100));
        })
        .size(Auto)
        .child_space(Pixels(10.0))
        .background_color(Color::rgb(100, 200, 100));

        HStack::new(cx, |cx| {
            Label::new(cx, ICON_MOON)
                .class("icon")
                .text_wrap(false)
                .size(Pixels(150.0))
                .child_left(Stretch(1.0))
                .child_right(Stretch(1.0))
                .background_color(Color::rgb(200, 100, 100));
        })
        .size(Auto)
        .child_space(Pixels(10.0))
        .background_color(Color::rgb(100, 200, 100));

        HStack::new(cx, |cx| {
            Label::new(cx, ICON_MOON)
                .class("icon")
                .size(Pixels(150.0))
                .child_left(Stretch(1.0))
                .child_right(Pixels(0.0))
                .background_color(Color::rgb(200, 100, 100));
        })
        .size(Auto)
        .child_space(Pixels(10.0))
        .background_color(Color::rgb(100, 200, 100));
    })
    .size(Auto)
    .col_between(Pixels(20.0))
    .child_space(Pixels(20.0));

    HStack::new(cx, |cx| {
        HStack::new(cx, |cx| {
            Label::new(cx, ICON_MOON)
                .class("icon")
                .text_wrap(false)
                .size(Pixels(150.0))
                .child_top(Stretch(1.0))
                .child_bottom(Stretch(1.0))
                .background_color(Color::rgb(200, 100, 100));
        })
        .size(Auto)
        .child_space(Pixels(10.0))
        .background_color(Color::rgb(100, 200, 100));

        HStack::new(cx, |cx| {
            Label::new(cx, ICON_MOON)
                .class("icon")
                .text_wrap(false)
                .size(Pixels(150.0))
                .child_space(Stretch(1.0))
                .background_color(Color::rgb(200, 100, 100));
        })
        .size(Auto)
        .child_space(Pixels(10.0))
        .background_color(Color::rgb(100, 200, 100));

        HStack::new(cx, |cx| {
            Label::new(cx, ICON_MOON)
                .class("icon")
                .size(Pixels(150.0))
                .child_top(Stretch(1.0))
                .child_bottom(Stretch(1.0))
                .child_left(Stretch(1.0))
                .child_right(Pixels(0.0))
                .background_color(Color::rgb(200, 100, 100));
        })
        .size(Auto)
        .child_space(Pixels(10.0))
        .background_color(Color::rgb(100, 200, 100));
    })
    .size(Auto)
    .col_between(Pixels(20.0))
    .child_space(Pixels(20.0));

    HStack::new(cx, |cx| {
        HStack::new(cx, |cx| {
            Label::new(cx, ICON_MOON)
                .class("icon")
                .text_wrap(false)
                .size(Pixels(150.0))
                .child_top(Stretch(1.0))
                .child_bottom(Pixels(0.0))
                .background_color(Color::rgb(200, 100, 100));
        })
        .size(Auto)
        .child_space(Pixels(10.0))
        .background_color(Color::rgb(100, 200, 100));

        HStack::new(cx, |cx| {
            Label::new(cx, ICON_MOON)
                .class("icon")
                .text_wrap(false)
                .size(Pixels(150.0))
                .child_space(Stretch(1.0))
                .child_bottom(Pixels(0.0))
                .background_color(Color::rgb(200, 100, 100));
        })
        .size(Auto)
        .child_space(Pixels(10.0))
        .background_color(Color::rgb(100, 200, 100));

        HStack::new(cx, |cx| {
            Label::new(cx, ICON_MOON)
                .class("icon")
                .size(Pixels(150.0))
                .child_top(Stretch(1.0))
                .child_bottom(Pixels(0.0))
                .child_left(Stretch(1.0))
                .child_right(Pixels(0.0))
                .background_color(Color::rgb(200, 100, 100));
        })
        .size(Auto)
        .child_space(Pixels(10.0))
        .background_color(Color::rgb(100, 200, 100));
    })
    .size(Auto)
    .col_between(Pixels(20.0))
    .child_space(Pixels(20.0));
}
