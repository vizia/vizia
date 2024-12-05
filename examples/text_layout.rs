use vizia::prelude::*;

static STATIC_LIST: &[&str] = &["Wrapping", "Alignment", "Alignment2", "Alignment3", "Alignment4"];

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

fn main() -> Result<(), ApplicationError> {
    Application::new(|cx| {
        AppData {
            text: String::from("This is some text\nwhich can be edited"),
            text2: String::from("سلام"),
        }
        .build(cx);
        VStack::new(cx, |cx| {
            TabView::new(cx, &STATIC_LIST, |cx, item| match *item.get_ref(cx).unwrap() {
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

                _ => unreachable!(),
            });

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
            // .alignment(Alignment::Center)
            // .on_submit(|ex, txt, _| ex.emit(AppEvent::SetText(3, txt.clone())));
        })
        .padding(Pixels(20.0))
        .vertical_gap(Pixels(20.0));
    })
    .title("Text")
    .inner_size((1200, 600))
    .run()
}

fn wrapping(cx: &mut Context) {
    HStack::new(cx, |cx| {
        HStack::new(cx, |cx| {
            Label::new(cx, "This is some non-wrapping text")
                .padding(Pixels(10.0))
                .background_color(Color::rgb(200, 100, 200));
        })
        .size(Auto)
        .padding(Pixels(10.0))
        .background_color(Color::rgb(100, 200, 100));

        HStack::new(cx, |cx| {
            Label::new(cx, "This is some text which should wrap on the longest word.")
                .text_wrap(true)
                .padding(Pixels(10.0))
                .background_color(Color::rgb(200, 100, 100));
        })
        .size(Auto)
        .padding(Pixels(50.0))
        .background_color(Color::rgb(100, 200, 100));

        HStack::new(cx, |cx| {
            Label::new(
                cx,
                "This is some text which should wrap because its container is too narrow.",
            )
            .text_wrap(true)
            .width(Pixels(100.0))
            .padding(Pixels(10.0))
            .background_color(Color::rgb(200, 100, 100));
        })
        .size(Auto)
        .padding(Pixels(10.0))
        .background_color(Color::rgb(100, 200, 100));

        HStack::new(cx, |cx| {
            Label::new(cx, "This is some text which should\nwrap because of a hard break.")
                .text_wrap(false)
                .width(Auto)
                .padding(Pixels(10.0))
                .background_color(Color::rgb(200, 100, 100));
        })
        .size(Auto)
        .padding(Pixels(10.0))
        .background_color(Color::rgb(100, 200, 100));

        HStack::new(cx, |cx| {
            Label::new(cx, "This is some text which should\nwrap because of soft and hard breaks.")
                .text_wrap(true)
                .width(Pixels(100.0))
                .padding(Pixels(10.0))
                .background_color(Color::rgb(200, 100, 100));
        })
        .size(Auto)
        .padding(Pixels(10.0))
        .background_color(Color::rgb(100, 200, 100));
    })
    .horizontal_gap(Pixels(50.0))
    .padding(Pixels(50.0));
}

fn alignment(cx: &mut Context) {
    HStack::new(cx, |cx| {
        HStack::new(cx, |cx| {
            Label::new(cx, "This is some\ntext aligned to\nthe top left")
                .text_wrap(false)
                .size(Pixels(150.0))
                .alignment(Alignment::TopLeft)
                .padding(Pixels(10.0))
                .background_color(Color::rgb(200, 100, 100));
        })
        .size(Auto)
        .padding(Pixels(10.0))
        .background_color(Color::rgb(100, 200, 100));

        HStack::new(cx, |cx| {
            Label::new(cx, "This is some\ntext aligned to\nthe top center")
                .text_wrap(false)
                .size(Pixels(150.0))
                .alignment(Alignment::TopCenter)
                .padding(Pixels(10.0))
                .background_color(Color::rgb(200, 100, 100));
        })
        .size(Auto)
        .padding(Pixels(10.0))
        .background_color(Color::rgb(100, 200, 100));

        HStack::new(cx, |cx| {
            Label::new(cx, "This is some\ntext aligned to\nthe top right")
                .size(Pixels(150.0))
                .alignment(Alignment::TopRight)
                .padding(Pixels(10.0))
                .background_color(Color::rgb(200, 100, 100));
        })
        .size(Auto)
        .padding(Pixels(10.0))
        .background_color(Color::rgb(100, 200, 100));
    })
    .size(Auto)
    .horizontal_gap(Pixels(20.0))
    .padding(Pixels(20.0));

    HStack::new(cx, |cx| {
        HStack::new(cx, |cx| {
            Label::new(cx, "This is some\ntext aligned to\nthe middle left")
                .text_wrap(false)
                .size(Pixels(150.0))
                .alignment(Alignment::Left)
                .padding(Pixels(10.0))
                .background_color(Color::rgb(200, 100, 100));
        })
        .size(Auto)
        .padding(Pixels(10.0))
        .background_color(Color::rgb(100, 200, 100));

        HStack::new(cx, |cx| {
            Label::new(cx, "This is some\ntext aligned to\nthe middle center")
                .text_wrap(false)
                .size(Pixels(150.0))
                .alignment(Alignment::Center)
                .padding(Pixels(10.0))
                .background_color(Color::rgb(200, 100, 100));
        })
        .size(Auto)
        .padding(Pixels(10.0))
        .background_color(Color::rgb(100, 200, 100));

        HStack::new(cx, |cx| {
            Label::new(cx, "This is some\ntext aligned to\nthe middle right")
                .size(Pixels(150.0))
                .alignment(Alignment::Right)
                .padding(Pixels(10.0))
                .background_color(Color::rgb(200, 100, 100));
        })
        .size(Auto)
        .padding(Pixels(10.0))
        .background_color(Color::rgb(100, 200, 100));
    })
    .size(Auto)
    .horizontal_gap(Pixels(20.0))
    .padding(Pixels(20.0));

    HStack::new(cx, |cx| {
        HStack::new(cx, |cx| {
            Label::new(cx, "This is some\ntext aligned to\nthe bottom left")
                .text_wrap(false)
                .size(Pixels(150.0))
                .alignment(Alignment::BottomLeft)
                .padding(Pixels(10.0))
                .background_color(Color::rgb(200, 100, 100));
        })
        .size(Auto)
        .padding(Pixels(10.0))
        .background_color(Color::rgb(100, 200, 100));

        HStack::new(cx, |cx| {
            Label::new(cx, "This is some\ntext aligned to\nthe bottom center")
                .text_wrap(false)
                .size(Pixels(150.0))
                .alignment(Alignment::BottomCenter)
                .padding(Pixels(10.0))
                .background_color(Color::rgb(200, 100, 100));
        })
        .size(Auto)
        .padding(Pixels(10.0))
        .background_color(Color::rgb(100, 200, 100));

        HStack::new(cx, |cx| {
            Label::new(cx, "This is some\ntext aligned to\nthe bottom right")
                .size(Pixels(150.0))
                .alignment(Alignment::BottomRight)
                .padding(Pixels(10.0))
                .background_color(Color::rgb(200, 100, 100));
        })
        .size(Auto)
        .padding(Pixels(10.0))
        .background_color(Color::rgb(100, 200, 100));
    })
    .size(Auto)
    .horizontal_gap(Pixels(20.0))
    .padding(Pixels(20.0));
}

fn alignment2(cx: &mut Context) {
    HStack::new(cx, |cx| {
        HStack::new(cx, |cx| {
            Label::new(cx, "سلام")
                .text_wrap(false)
                .size(Pixels(150.0))
                .background_color(Color::rgb(200, 100, 100));
        })
        .size(Auto)
        .padding(Pixels(10.0))
        .background_color(Color::rgb(100, 200, 100));

        HStack::new(cx, |cx| {
            Label::new(cx, "سلام")
                .text_wrap(false)
                .size(Pixels(150.0))
                .alignment(Alignment::Center)
                .background_color(Color::rgb(200, 100, 100));
        })
        .size(Auto)
        .padding(Pixels(10.0))
        .background_color(Color::rgb(100, 200, 100));

        HStack::new(cx, |cx| {
            Label::new(cx, "سلام")
                .size(Pixels(150.0))
                .alignment(Alignment::Center)
                .padding_right(Pixels(0.0))
                .background_color(Color::rgb(200, 100, 100));
        })
        .size(Auto)
        .padding(Pixels(10.0))
        .background_color(Color::rgb(100, 200, 100));
    })
    .size(Auto)
    .horizontal_gap(Pixels(20.0))
    .padding(Pixels(20.0));

    HStack::new(cx, |cx| {
        HStack::new(cx, |cx| {
            Label::new(cx, "سلام")
                .text_wrap(false)
                .size(Pixels(150.0))
                .alignment(Alignment::Center)
                .background_color(Color::rgb(200, 100, 100));
        })
        .size(Auto)
        .padding(Pixels(10.0))
        .background_color(Color::rgb(100, 200, 100));

        HStack::new(cx, |cx| {
            Label::new(cx, "سلام")
                .text_wrap(false)
                .size(Pixels(150.0))
                .alignment(Alignment::Center)
                .background_color(Color::rgb(200, 100, 100));
        })
        .size(Auto)
        .padding(Pixels(10.0))
        .background_color(Color::rgb(100, 200, 100));

        HStack::new(cx, |cx| {
            Label::new(cx, "سلام")
                .size(Pixels(150.0))
                .alignment(Alignment::Center)
                .alignment(Alignment::Center)
                .padding_right(Pixels(0.0))
                .background_color(Color::rgb(200, 100, 100));
        })
        .size(Auto)
        .padding(Pixels(10.0))
        .background_color(Color::rgb(100, 200, 100));
    })
    .size(Auto)
    .horizontal_gap(Pixels(20.0))
    .padding(Pixels(20.0));

    HStack::new(cx, |cx| {
        HStack::new(cx, |cx| {
            Label::new(cx, "سلام")
                .text_wrap(false)
                .size(Pixels(150.0))
                .alignment(Alignment::Center)
                .padding_bottom(Pixels(0.0))
                .background_color(Color::rgb(200, 100, 100));
        })
        .size(Auto)
        .padding(Pixels(10.0))
        .background_color(Color::rgb(100, 200, 100));

        HStack::new(cx, |cx| {
            Label::new(cx, "سلام")
                .text_wrap(false)
                .size(Pixels(150.0))
                .alignment(Alignment::Center)
                .padding_bottom(Pixels(0.0))
                .background_color(Color::rgb(200, 100, 100));
        })
        .size(Auto)
        .padding(Pixels(10.0))
        .background_color(Color::rgb(100, 200, 100));

        HStack::new(cx, |cx| {
            Label::new(cx, "سلام")
                .size(Pixels(150.0))
                .alignment(Alignment::Center)
                .padding_bottom(Pixels(0.0))
                .alignment(Alignment::Center)
                .padding_right(Pixels(0.0))
                .background_color(Color::rgb(200, 100, 100));
        })
        .size(Auto)
        .padding(Pixels(10.0))
        .background_color(Color::rgb(100, 200, 100));
    })
    .size(Auto)
    .horizontal_gap(Pixels(20.0))
    .padding(Pixels(20.0));
}

fn alignment3(cx: &mut Context) {
    HStack::new(cx, |cx| {
        HStack::new(cx, |cx| {
            Textbox::new(cx, AppData::text)
                .text_wrap(false)
                .size(Pixels(150.0))
                .alignment(Alignment::TopLeft)
                .padding(Pixels(10.0))
                .background_color(Color::rgb(200, 100, 100));
        })
        .size(Auto)
        .padding(Pixels(10.0))
        .background_color(Color::rgb(100, 200, 100));

        HStack::new(cx, |cx| {
            Textbox::new(cx, AppData::text)
                .text_wrap(false)
                .size(Pixels(150.0))
                .alignment(Alignment::TopCenter)
                .padding(Pixels(10.0))
                .background_color(Color::rgb(200, 100, 100));
        })
        .size(Auto)
        .padding(Pixels(10.0))
        .background_color(Color::rgb(100, 200, 100));

        HStack::new(cx, |cx| {
            Textbox::new(cx, AppData::text)
                .size(Pixels(150.0))
                .alignment(Alignment::TopRight)
                .padding(Pixels(10.0))
                .background_color(Color::rgb(200, 100, 100));
        })
        .size(Auto)
        .padding(Pixels(10.0))
        .background_color(Color::rgb(100, 200, 100));
    })
    .size(Auto)
    .horizontal_gap(Pixels(20.0))
    .padding(Pixels(20.0));

    HStack::new(cx, |cx| {
        HStack::new(cx, |cx| {
            Textbox::new(cx, AppData::text)
                .text_wrap(false)
                .size(Pixels(150.0))
                .alignment(Alignment::Left)
                .padding(Pixels(10.0))
                .background_color(Color::rgb(200, 100, 100));
        })
        .size(Auto)
        .padding(Pixels(10.0))
        .background_color(Color::rgb(100, 200, 100));

        HStack::new(cx, |cx| {
            Textbox::new(cx, AppData::text)
                .text_wrap(false)
                .size(Pixels(150.0))
                .alignment(Alignment::Center)
                .padding(Pixels(10.0))
                .background_color(Color::rgb(200, 100, 100));
        })
        .size(Auto)
        .padding(Pixels(10.0))
        .background_color(Color::rgb(100, 200, 100));

        HStack::new(cx, |cx| {
            Textbox::new(cx, AppData::text)
                .size(Pixels(150.0))
                .alignment(Alignment::Right)
                .padding(Pixels(10.0))
                .background_color(Color::rgb(200, 100, 100));
        })
        .size(Auto)
        .padding(Pixels(10.0))
        .background_color(Color::rgb(100, 200, 100));
    })
    .size(Auto)
    .horizontal_gap(Pixels(20.0))
    .padding(Pixels(20.0));

    HStack::new(cx, |cx| {
        HStack::new(cx, |cx| {
            Textbox::new(cx, AppData::text)
                .text_wrap(false)
                .size(Pixels(150.0))
                .alignment(Alignment::BottomLeft)
                .padding(Pixels(10.0))
                .background_color(Color::rgb(200, 100, 100));
        })
        .size(Auto)
        .padding(Pixels(10.0))
        .background_color(Color::rgb(100, 200, 100));

        HStack::new(cx, |cx| {
            Textbox::new(cx, AppData::text)
                .text_wrap(false)
                .size(Pixels(150.0))
                .alignment(Alignment::BottomCenter)
                .padding(Pixels(10.0))
                .background_color(Color::rgb(200, 100, 100));
        })
        .size(Auto)
        .padding(Pixels(10.0))
        .background_color(Color::rgb(100, 200, 100));

        HStack::new(cx, |cx| {
            Textbox::new(cx, AppData::text)
                .size(Pixels(150.0))
                .alignment(Alignment::BottomRight)
                .padding(Pixels(10.0))
                .background_color(Color::rgb(200, 100, 100));
        })
        .size(Auto)
        .padding(Pixels(10.0))
        .background_color(Color::rgb(100, 200, 100));
    })
    .size(Auto)
    .horizontal_gap(Pixels(20.0))
    .padding(Pixels(20.0));
}

fn alignment4(cx: &mut Context) {
    HStack::new(cx, |cx| {
        HStack::new(cx, |cx| {
            Textbox::new(cx, AppData::text2)
                .text_wrap(false)
                .size(Pixels(150.0))
                .padding(Pixels(0.0))
                .background_color(Color::rgb(200, 100, 100));
        })
        .size(Auto)
        .padding(Pixels(10.0))
        .background_color(Color::rgb(100, 200, 100));

        HStack::new(cx, |cx| {
            Textbox::new(cx, AppData::text2)
                .text_wrap(false)
                .size(Pixels(150.0))
                .padding_top(Pixels(0.0))
                .alignment(Alignment::Center)
                .background_color(Color::rgb(200, 100, 100));
        })
        .size(Auto)
        .padding(Pixels(10.0))
        .background_color(Color::rgb(100, 200, 100));

        HStack::new(cx, |cx| {
            Textbox::new(cx, AppData::text2)
                .size(Pixels(150.0))
                .padding_top(Pixels(0.0))
                .alignment(Alignment::Center)
                .padding_right(Pixels(0.0))
                .background_color(Color::rgb(200, 100, 100));
        })
        .size(Auto)
        .padding(Pixels(10.0))
        .background_color(Color::rgb(100, 200, 100));
    })
    .size(Auto)
    .horizontal_gap(Pixels(20.0))
    .padding(Pixels(20.0));

    HStack::new(cx, |cx| {
        HStack::new(cx, |cx| {
            Textbox::new(cx, AppData::text2)
                .text_wrap(false)
                .size(Pixels(150.0))
                .padding_left(Pixels(0.0))
                .alignment(Alignment::Center)
                .background_color(Color::rgb(200, 100, 100));
        })
        .size(Auto)
        .padding(Pixels(10.0))
        .background_color(Color::rgb(100, 200, 100));

        HStack::new(cx, |cx| {
            Textbox::new(cx, AppData::text2)
                .text_wrap(false)
                .size(Pixels(150.0))
                .alignment(Alignment::Center)
                .background_color(Color::rgb(200, 100, 100));
        })
        .size(Auto)
        .padding(Pixels(10.0))
        .background_color(Color::rgb(100, 200, 100));

        HStack::new(cx, |cx| {
            Textbox::new(cx, AppData::text2)
                .size(Pixels(150.0))
                .alignment(Alignment::Center)
                .alignment(Alignment::Center)
                .padding_right(Pixels(0.0))
                .background_color(Color::rgb(200, 100, 100));
        })
        .size(Auto)
        .padding(Pixels(10.0))
        .background_color(Color::rgb(100, 200, 100));
    })
    .size(Auto)
    .horizontal_gap(Pixels(20.0))
    .padding(Pixels(20.0));

    HStack::new(cx, |cx| {
        HStack::new(cx, |cx| {
            Textbox::new(cx, AppData::text2)
                .text_wrap(false)
                .size(Pixels(150.0))
                .padding(Pixels(0.0))
                .alignment(Alignment::Center)
                .background_color(Color::rgb(200, 100, 100));
        })
        .size(Auto)
        .padding(Pixels(10.0))
        .background_color(Color::rgb(100, 200, 100));

        HStack::new(cx, |cx| {
            Textbox::new(cx, AppData::text2)
                .text_wrap(false)
                .size(Pixels(150.0))
                .alignment(Alignment::Center)
                .padding_bottom(Pixels(0.0))
                .background_color(Color::rgb(200, 100, 100));
        })
        .size(Auto)
        .padding(Pixels(10.0))
        .background_color(Color::rgb(100, 200, 100));

        HStack::new(cx, |cx| {
            Textbox::new(cx, AppData::text2)
                .size(Pixels(150.0))
                .alignment(Alignment::Center)
                .padding_bottom(Pixels(0.0))
                .alignment(Alignment::Center)
                .padding_right(Pixels(0.0))
                .background_color(Color::rgb(200, 100, 100));
        })
        .size(Auto)
        .padding(Pixels(10.0))
        .background_color(Color::rgb(100, 200, 100));
    })
    .size(Auto)
    .horizontal_gap(Pixels(20.0))
    .padding(Pixels(20.0));
}
