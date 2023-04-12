use vizia::{prelude::*, style::FontWeightKeyword};

const ICON_PLUS: &str = "\u{2b}";
const ICON_STAR: &str = "\u{2605}";

const STYLE: &str = r#"

    .title {
        font-size: 30.0;
        font: "roboto-bold";
        top: 10px;
        bottom: 10px;
        space: 0px;
        child-space: 40px;
        child-top: 20px;
        background-color: red;
        height: 100px;
        width: 1s;
    }

    .heading {
        font-size: 20.0;
        font: "roboto-bold";
        top: 10px;
        bottom: 6px;
    }

    .tabview-tabheader-wrapper {
        width: 200px;
    }

    tabheader {
        width: 1s;
    }

    tabheader label {
        width: 1s;
    }

"#;

#[derive(Lens)]
pub struct AppData {
    items: Vec<&'static str>,
}

impl Model for AppData {}

fn tab<T: ToString + Data>(cx: &mut Context, item: impl Lens<Target = T>) {
    Element::new(cx).class("indicator").width(Pixels(5.0));
    Label::new(cx, item);
}

fn main() {
    Application::new(|cx| {
        cx.add_theme(STYLE);
        AppData {
            items: vec![
                "Label", "Button", "Checkbox", "Slider", "Switch", "Spinbox", "Tabs", "Textbox",
            ],
        }
        .build(cx);
        //cx.add_stylesheet("examples/test_style.css").unwrap();

        TabView::new(cx, AppData::items, |cx, item| match item.get(cx) {
            "Label" => TabPair::new(move |cx| tab(cx, item), label),

            "Button" => TabPair::new(
                move |cx| {
                    tab(cx, item);
                },
                button,
            ),

            "Checkbox" => TabPair::new(
                move |cx| {
                    tab(cx, item);
                },
                checkbox,
            ),

            "Slider" => TabPair::new(
                move |cx| {
                    tab(cx, item);
                },
                slider,
            ),

            "Switch" => TabPair::new(
                move |cx| {
                    tab(cx, item);
                },
                switch,
            ),

            "Spinbox" => TabPair::new(
                move |cx| {
                    tab(cx, item);
                },
                switch,
            ),

            "Tabs" => TabPair::new(
                move |cx| {
                    tab(cx, item);
                },
                switch,
            ),

            "Textbox" => TabPair::new(
                move |cx| {
                    tab(cx, item);
                },
                textbox,
            ),
            _ => TabPair::new(|_| {}, |_| {}),
        })
        .class("vertical");

        //buttons(cx)
        //    .space(Pixels(30.0));
        // checkbox(cx).space(Pixels(30.0));
        // label(cx);
    })
    .title("Widget Gallery")
    .run();
}

pub fn button(cx: &mut Context) {
    VStack::new(cx, |cx| {
        Label::new(cx, "Button").font_size(30.0).font_weight(FontWeightKeyword::Bold);

        Label::new(cx, "A simple Button with a text label")
            .font_size(24.0)
            .font_weight(FontWeightKeyword::Bold);

        // Icon Buttons
        VStack::new(cx, |cx| {
            Label::new(cx, "Icon Buttons").class("heading");
            HStack::new(cx, |cx| {
                Button::new(cx, |_| {}, |cx| Label::new(cx, ICON_PLUS)).class("icon");
                Button::new(cx, |_| {}, |cx| Label::new(cx, ICON_PLUS))
                    .class("icon")
                    .class("accent");
                Button::new(cx, |_| {}, |cx| Label::new(cx, ICON_PLUS))
                    .class("icon")
                    .class("outline");
                Button::new(cx, |_| {}, |cx| Label::new(cx, ICON_PLUS))
                    .class("icon")
                    .class("ghost");
            })
            .col_between(Pixels(20.0));
        })
        .height(Auto)
        .child_space(Pixels(20.0))
        .background_color(Color::rgb(40, 40, 40));

        Label::new(cx, "A simple Button with an icon label")
            .font_size(24.0)
            .font_weight(FontWeightKeyword::Bold);

                Button::new(
                    cx,
                    |_| {},
                    |cx| {
                        HStack::new(cx, |cx| {
                            Label::new(cx, ICON_STAR).class("icon");
                            Label::new(cx, "Icon before");
                        })
                        .size(Auto)
                        .child_space(Stretch(1.0))
                        .col_between(Pixels(4.0))
                    },
                )
                .class("accent");

        Label::new(cx, "A simple Button with icon and text labels")
            .font_size(24.0)
            .font_weight(FontWeightKeyword::Bold);

                Button::new(
                    cx,
                    |_| {},
                    |cx| {
                        HStack::new(cx, |cx| {
                            Label::new(cx, ICON_STAR).class("icon");
                            Label::new(cx, "Icon before");
                        })
                        .size(Auto)
                        .child_space(Stretch(1.0))
                        .col_between(Pixels(4.0))
                    },
                )
                .class("ghost");
            })
            .col_between(Pixels(20.0));
        })
        .height(Auto)
        .child_space(Pixels(20.0))
        .background_color(Color::rgb(40, 40, 40));

        Label::new(cx, "An accented Button with a text label")
            .font_size(24.0)
            .font_weight(FontWeightKeyword::Bold);

                Button::new(
                    cx,
                    |_| {},
                    |cx| {
                        HStack::new(cx, |cx| {
                            Label::new(cx, "Icon after");
                            Label::new(cx, ICON_STAR).class("icon");
                        })
                        .size(Auto)
                        .child_space(Stretch(1.0))
                        .col_between(Pixels(4.0))
                    },
                )
                .class("accent");

                Button::new(
                    cx,
                    |_| {},
                    |cx| {
                        HStack::new(cx, |cx| {
                            Label::new(cx, "Icon after");
                            Label::new(cx, ICON_STAR).class("icon");
                        })
                        .size(Auto)
                        .child_space(Stretch(1.0))
                        .col_between(Pixels(4.0))
                    },
                )
                .class("outline");

                Button::new(
                    cx,
                    |_| {},
                    |cx| {
                        HStack::new(cx, |cx| {
                            Label::new(cx, "Icon after");
                            Label::new(cx, ICON_STAR).class("icon");
                        })
                        .size(Auto)
                        .child_space(Stretch(1.0))
                        .col_between(Pixels(4.0))
                    },
                )
                .class("ghost");
            })
            .col_between(Pixels(20.0));
        })
        .height(Auto)
        .child_space(Pixels(20.0))
        .background_color(Color::rgb(40, 40, 40));
    })
    .child_space(Pixels(20.0))
    .row_between(Pixels(20.0))
    .class("bg-darker");
}

#[derive(Lens)]
pub struct CheckboxData {
    check: bool,

    items: Vec<bool>,
}

pub enum CheckboxEvent {
    Toggle,
    ToggleItem(usize),
    ToggleAll,
}

impl Model for CheckboxData {
    fn event(&mut self, _: &mut EventContext, event: &mut Event) {
        event.map(|checkbox_event, _| match checkbox_event {
            CheckboxEvent::Toggle => {
                self.check ^= true;
            }

            CheckboxEvent::ToggleItem(index) => {
                self.items[*index] ^= true;
            }

            CheckboxEvent::ToggleAll => {
                let any = self.items.iter().any(|b| *b);
                if any {
                    self.items.iter_mut().for_each(|b| *b = false);
                } else {
                    self.items.iter_mut().for_each(|b| *b = true);
                }
            }
        });
    }
}

// Checkbox
pub fn checkbox(cx: &mut Context) {
    CheckboxData { check: false, items: vec![false, false, false] }.build(cx);

    VStack::new(cx, |cx| {
        Label::new(cx, "Checkbox").font_size(30.0).font_weight(FontWeightKeyword::Bold);

        Label::new(cx, "A simple 2-state checkbox")
            .font_size(24.0)
            .font_weight(FontWeightKeyword::Bold);

        Checkbox::new(cx, CheckboxData::check).on_toggle(|cx| cx.emit(CheckboxEvent::Toggle));

        Label::new(cx, "A simple 2-state checkbox with a text label")
            .font_size(24.0)
            .font_weight(FontWeightKeyword::Bold);

        HStack::new(cx, |cx| {
            Checkbox::new(cx, CheckboxData::check);
            Label::new(cx, "Two-state checkbox");
        })
        .height(Auto)
        .child_space(Pixels(20.0))
        .background_color(Color::rgb(40, 40, 40));

        VStack::new(cx, |cx| {
            Label::new(cx, "Checkbox and Label").class("heading");
            HStack::new(cx, |cx| {
                Checkbox::new(cx, CheckboxData::check);
                Label::new(cx, "Two-state checkbox");
            })
            .size(Auto)
            .child_top(Stretch(1.0))
            .child_bottom(Stretch(1.0))
            .col_between(Pixels(5.0))
            .on_press(|cx| cx.emit(CheckboxEvent::Toggle));
        })
        .height(Auto)
        .child_space(Pixels(20.0))
        .background_color(Color::rgb(40, 40, 40));

        VStack::new(cx, |cx| {
            Label::new(cx, "Intermediate Checkbox").class("heading");
            HStack::new(cx, |cx| {
                Checkbox::intermediate(
                    cx,
                    CheckboxData::items.map(|items| items.iter().all(|b| *b)),
                    CheckboxData::items.map(|items| items.iter().any(|b| *b)),
                );
                Label::new(cx, "All items");
            })
            .size(Auto)
            .child_top(Stretch(1.0))
            .child_bottom(Stretch(1.0))
            .col_between(Pixels(5.0))
            .on_press(|cx| cx.emit(CheckboxEvent::ToggleAll));
            List::new(cx, CheckboxData::items, |cx, index, item| {
                HStack::new(cx, |cx| {
                    Checkbox::new(cx, item);
                    Label::new(cx, "Item 1");
                })
                .size(Auto)
                .child_top(Stretch(1.0))
                .child_bottom(Stretch(1.0))
                .col_between(Pixels(5.0))
                .on_press(move |cx| cx.emit(CheckboxEvent::ToggleItem(index)));
            })
            .child_left(Pixels(25.0))
            .row_between(Pixels(5.0));
        })
        .height(Auto)
        .child_space(Pixels(20.0))
        .row_between(Pixels(5.0))
        .background_color(Color::rgb(40, 40, 40));
    })
    .child_space(Pixels(20.0))
    .row_between(Pixels(20.0))
    .class("bg-darker");
}

pub fn label(cx: &mut Context) {
    VStack::new(cx, |cx| {
        Label::new(cx, "A simple label").font_size(20.0).font_weight(FontWeightKeyword::Bold);

        Label::new(cx, "This is some simple text");

        Label::new(cx, "A styled label").font_size(20.0).font_weight(FontWeightKeyword::Bold);

        Label::new(cx, "This is some styled text").color(Color::red());

        Label::new(cx, "A multiline label").class("heading");

        Label::new(cx, "This is some text which is wrapped")
            .width(Pixels(100.0))
            .bottom(Pixels(10.0));
    })
    .child_space(Pixels(10.0))
    .class("bg-darker");
}

#[derive(Lens)]
pub struct SliderData {
    val: f32,
}

pub enum SliderEvent {
    SetValue(f32),
}

impl Model for SliderData {
    fn event(&mut self, _cx: &mut EventContext, event: &mut Event) {
        event.map(|slider_event, _| match slider_event {
            SliderEvent::SetValue(val) => {
                self.val = *val;
            }
        });
    }
}

pub fn slider(cx: &mut Context) {
    SliderData { val: 0.5 }.build(cx);

    VStack::new(cx, |cx| {
        Label::new(cx, "Label").class("title");

        Label::new(cx, "A simple slider").class("heading");

        Slider::new(cx, SliderData::val).on_changing(|cx, val| cx.emit(SliderEvent::SetValue(val)));

        Label::new(cx, "A slider and label").class("heading");

        HStack::new(cx, |cx| {
            Slider::new(cx, SliderData::val)
                .on_changing(|cx, val| cx.emit(SliderEvent::SetValue(val)));
            Label::new(cx, SliderData::val.map(|val| format!("{:.2}", val))).width(Pixels(50.0));
        })
        .height(Auto)
        .child_top(Stretch(1.0))
        .child_bottom(Stretch(1.0))
        .col_between(Pixels(10.0));
    })
    .child_space(Pixels(10.0))
    .class("bg-darker");
}

#[derive(Lens)]
pub struct SwitchData {
    flag: bool,
}

pub enum SwitchEvent {
    Toggle,
}

impl Model for SwitchData {
    fn event(&mut self, _cx: &mut EventContext, event: &mut Event) {
        event.map(|switch_event, _| match switch_event {
            SwitchEvent::Toggle => {
                self.flag ^= true;
            }
        });
    }
}

pub fn switch(cx: &mut Context) {
    SwitchData { flag: false }.build(cx);

    VStack::new(cx, |cx| {
        Label::new(cx, "Switch").class("title");

        Label::new(cx, "A simple switch").class("heading");

        Switch::new(cx, SwitchData::flag).on_toggle(|cx| cx.emit(SwitchEvent::Toggle));
        // Slider::new(cx, SliderData::val).on_changing(|cx, val| cx.emit(SliderEvent::SetValue(val)));

        // Label::new(cx, "A slider and label").class("heading");

        // HStack::new(cx, |cx| {
        //     Slider::new(cx, SliderData::val)
        //         .on_changing(|cx, val| cx.emit(SliderEvent::SetValue(val)));
        //     Label::new(cx, SliderData::val.map(|val| format!("{:.2}", val))).width(Pixels(50.0));
        // })
        // .height(Auto)
        // .child_top(Stretch(1.0))
        // .child_bottom(Stretch(1.0))
        // .col_between(Pixels(10.0));
    })
    .child_space(Pixels(10.0))
    .class("bg-darker");
}

#[derive(Lens)]
pub struct TextboxData {
    text: String,
    multiline_text: String,
}

pub enum TextboxEvent {
    SetText(String),
    SetMultiline(String),
}

impl Model for TextboxData {
    fn event(&mut self, _cx: &mut EventContext, event: &mut Event) {
        event.map(|textbox_event, _| match textbox_event {
            TextboxEvent::SetText(text) => {
                self.text = text.clone();
            }

            TextboxEvent::SetMultiline(text) => {
                self.multiline_text = text.clone();
            }
        });
    }
}

pub fn textbox(cx: &mut Context) {
    TextboxData {
        text: String::from("Some text..."),
        multiline_text: String::from("This text spans \n multiple lines."),
    }
    .build(cx);

    VStack::new(cx, |cx| {
        Label::new(cx, "Switch").class("title");
        VStack::new(cx, |cx| {
            Label::new(cx, "Single Line Textbox").class("heading");
            Textbox::new(cx, TextboxData::text)
                .width(Stretch(1.0))
                .on_submit(|cx, text, _| cx.emit(TextboxEvent::SetText(text)));
        })
        .height(Auto)
        .child_space(Pixels(20.0))
        .background_color(Color::rgb(36, 36, 36));

        VStack::new(cx, |cx| {
            Label::new(cx, "Single Line Textbox").class("heading");
            Textbox::new_multiline(cx, TextboxData::multiline_text, false)
                .width(Stretch(1.0))
                .height(Pixels(100.0))
                .on_submit(|cx, text, _| cx.emit(TextboxEvent::SetMultiline(text)));
        })
        .height(Auto)
        .child_space(Pixels(20.0))
        .background_color(Color::rgb(36, 36, 36));
    })
    .child_space(Pixels(20.0))
    .row_between(Pixels(20.0))
    .class("bg-darker");
}
