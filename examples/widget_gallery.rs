use chrono::{NaiveDateTime, Utc};
use vizia::prelude::*;

const THEME: &str = "\u{25d1}";
const PLUS: &str = "+";
const CHECK: &str = "\u{2713}";

#[derive(Lens)]
pub struct AppData {
    items: Vec<&'static str>,
    disabled: bool,
    theme: ThemeMode,
}

fn section_title(cx: &mut Context, section_title: &str) {
    HStack::new(cx, |cx| {
        Label::new(cx, section_title);
        HStack::new(cx, |cx| {
            Switch::new(cx, AppData::disabled).on_toggle(|cx| cx.emit(AppEvent::ToggleDisabled));
            Label::new(cx, "Disabled");
        });
        Button::new(cx, |cx| cx.emit(AppEvent::ToggleTheme), |cx| Label::new(cx, THEME))
            .class("icon")
            .class("ghost");
    })
    .class("section-title")
    .class("bg-main");
}

fn wrapper_heading(cx: &mut Context, section_title: &str) {
    Label::new(cx, &section_title.to_uppercase()).class("heading").class("text-disabled");
}

fn tab<T: ToString + Data>(cx: &mut Context, item: impl Lens<Target = T>) {
    Label::new(cx, item).hoverable(false).class("text-disabled");
    Element::new(cx).class("indicator").hoverable(false);
}

fn main() {
    Application::new(|cx| {
        cx.emit(EnvironmentEvent::SetThemeMode(ThemeMode::DarkMode));
        cx.add_stylesheet("examples/widget_gallery.css").unwrap();

        AppData {
            items: vec![
                "Label",
                "Button",
                "Checkbox",
                "Slider",
                "Switch",
                "Spinbox",
                "Dropdown",
                "Tabs",
                "Textbox",
                "Date & Time Picker",
                "Knob",
            ],
            disabled: false,
            theme: ThemeMode::DarkMode,
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
                spinbox,
            ),

            "Dropdown" => TabPair::new(
                move |cx| {
                    tab(cx, item);
                },
                dropdown,
            ),

            "Tabs" => TabPair::new(
                move |cx| {
                    tab(cx, item);
                },
                tabs,
            ),

            "Textbox" => TabPair::new(
                move |cx| {
                    tab(cx, item);
                },
                textbox,
            ),

            "Date & Time Picker" => TabPair::new(
                move |cx| {
                    tab(cx, item);
                },
                datetimepicker,
            ),

            "Knob" => TabPair::new(
                move |cx| {
                    tab(cx, item);
                },
                knob,
            ),
            _ => TabPair::new(|_| {}, |_| {}),
        })
        .class("tabview-main");
    })
    .title("Widget Gallery")
    .run();
}

pub fn label(cx: &mut Context) {
    section_title(cx, "Label");

    VStack::new(cx, |cx| {
        HStack::new(cx, |cx| {
            VStack::new(cx, |cx| {
                wrapper_heading(cx, "A simple label");
                Label::new(cx, "This is some simple text");
            })
            .disabled(AppData::disabled)
            .class("wrapper")
            .class("bg-secondary");

            VStack::new(cx, |cx| {
                wrapper_heading(cx, "A styled label");
                Label::new(cx, "This is some styled text").color(Color::red());
            })
            .disabled(AppData::disabled)
            .class("wrapper")
            .class("bg-secondary");
        })
        .class("row-wrapper");

        VStack::new(cx, |cx| {
            wrapper_heading(cx, "A multiline label");
            Label::new(cx, "This is some text which is wrapped")
                .width(Pixels(100.0))
                .bottom(Pixels(10.0));
        })
        .disabled(AppData::disabled)
        .class("wrapper")
        .class("bg-secondary");
    })
    .class("bg-main");
}

pub fn button(cx: &mut Context) {
    section_title(cx, "Button");

    VStack::new(cx, |cx| {
        // Basic Buttons
        VStack::new(cx, |cx| {
            wrapper_heading(cx, "Basic Buttons");
            HStack::new(cx, |cx| {
                Button::new(cx, |_| {}, |cx| Label::new(cx, "Simple Button"));
                Button::new(cx, |_| {}, |cx| Label::new(cx, "Accent Button")).class("accent");
                Button::new(cx, |_| {}, |cx| Label::new(cx, "Outline Button")).class("outline");
                Button::new(cx, |_| {}, |cx| Label::new(cx, "Ghost Button")).class("ghost");
            });
        })
        .disabled(AppData::disabled)
        .class("wrapper")
        .class("bg-secondary");

        // Icon Buttons
        VStack::new(cx, |cx| {
            wrapper_heading(cx, "Icon Buttons");
            HStack::new(cx, |cx| {
                Button::new(cx, |_| {}, |cx| Label::new(cx, PLUS)).class("icon");
                Button::new(cx, |_| {}, |cx| Label::new(cx, PLUS)).class("icon").class("accent");
                Button::new(cx, |_| {}, |cx| Label::new(cx, PLUS)).class("icon").class("outline");
                Button::new(cx, |_| {}, |cx| Label::new(cx, PLUS)).class("icon").class("ghost");
            });
        })
        .disabled(AppData::disabled)
        .class("wrapper")
        .class("bg-secondary");

        // Icon & Label Buttons
        VStack::new(cx, |cx| {
            wrapper_heading(cx, "Icon & Label Buttons");
            HStack::new(cx, |cx| {
                Button::new(
                    cx,
                    |_| {},
                    |cx| {
                        HStack::new(cx, |cx| {
                            Label::new(cx, CHECK).class("icon");
                            Label::new(cx, "Icon before");
                        })
                        .size(Auto)
                        .child_space(Stretch(1.0))
                        .col_between(Pixels(4.0))
                    },
                );

                Button::new(
                    cx,
                    |_| {},
                    |cx| {
                        HStack::new(cx, |cx| {
                            Label::new(cx, CHECK).class("icon");
                            Label::new(cx, "Icon before");
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
                            Label::new(cx, CHECK).class("icon");
                            Label::new(cx, "Icon before");
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
                            Label::new(cx, CHECK).class("icon");
                            Label::new(cx, "Icon before");
                        })
                        .size(Auto)
                        .child_space(Stretch(1.0))
                        .col_between(Pixels(4.0))
                    },
                )
                .class("ghost");
            });
        })
        .disabled(AppData::disabled)
        .class("wrapper")
        .class("bg-secondary");

        VStack::new(cx, |cx| {
            wrapper_heading(cx, "Label & Icon Buttons");
            HStack::new(cx, |cx| {
                Button::new(
                    cx,
                    |_| {},
                    |cx| {
                        HStack::new(cx, |cx| {
                            Label::new(cx, "Icon after");
                            Label::new(cx, CHECK).class("icon");
                        })
                        .size(Auto)
                        .child_space(Stretch(1.0))
                        .col_between(Pixels(4.0))
                    },
                );

                Button::new(
                    cx,
                    |_| {},
                    |cx| {
                        HStack::new(cx, |cx| {
                            Label::new(cx, "Icon after");
                            Label::new(cx, CHECK).class("icon");
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
                            Label::new(cx, CHECK).class("icon");
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
                            Label::new(cx, CHECK).class("icon");
                        })
                        .size(Auto)
                        .child_space(Stretch(1.0))
                        .col_between(Pixels(4.0))
                    },
                )
                .class("ghost");
            });
        })
        .disabled(AppData::disabled)
        .class("wrapper")
        .class("bg-secondary");
    })
    .class("bg-main");
}

#[derive(Lens)]
pub struct CheckboxData {
    check: bool,

    items: Vec<bool>,
}

pub fn checkbox(cx: &mut Context) {
    CheckboxData { check: false, items: vec![false, false, false] }.build(cx);

    section_title(cx, "Checkbox");

    VStack::new(cx, |cx| {
        HStack::new(cx, |cx| {
            VStack::new(cx, |cx| {
                wrapper_heading(cx, "Simple Checkbox");
                Checkbox::new(cx, CheckboxData::check)
                    .on_toggle(|cx| cx.emit(CheckboxEvent::Toggle));
            })
            .disabled(AppData::disabled)
            .class("wrapper")
            .class("bg-secondary");
            VStack::new(cx, |cx| {
                wrapper_heading(cx, "Checkbox and Label");
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
            .disabled(AppData::disabled)
            .class("wrapper")
            .class("bg-secondary");
        })
        .class("row-wrapper");

        VStack::new(cx, |cx| {
            wrapper_heading(cx, "Intermediate Checkbox");
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
        .disabled(AppData::disabled)
        .class("wrapper")
        .class("bg-secondary");
    })
    .class("bg-main");
}

#[derive(Lens)]
pub struct SliderData {
    val: f32,
}

pub fn slider(cx: &mut Context) {
    SliderData { val: 0.5 }.build(cx);

    section_title(cx, "Slider");

    VStack::new(cx, |cx| {
        HStack::new(cx, |cx| {
            VStack::new(cx, |cx| {
                wrapper_heading(cx, "A simple slider");
                HStack::new(cx, |cx| {
                    Slider::new(cx, SliderData::val)
                        .on_changing(|cx, val| cx.emit(SliderEvent::SetValue(val)));
                })
                .height(Pixels(32.0))
                .child_top(Stretch(1.0))
                .child_bottom(Stretch(1.0))
                .col_between(Pixels(10.0));
            })
            .disabled(AppData::disabled)
            .class("wrapper")
            .class("bg-secondary");

            VStack::new(cx, |cx| {
                wrapper_heading(cx, "A slider and label");
                HStack::new(cx, |cx| {
                    Slider::new(cx, SliderData::val)
                        .on_changing(|cx, val| cx.emit(SliderEvent::SetValue(val)));
                    Label::new(cx, SliderData::val.map(|val| format!("{:.2}", val)))
                        .width(Pixels(50.0));
                })
                .height(Pixels(32.0))
                .child_top(Stretch(1.0))
                .child_bottom(Stretch(1.0))
                .col_between(Pixels(10.0));
            })
            .disabled(AppData::disabled)
            .class("wrapper")
            .class("bg-secondary");
        })
        .class("row-wrapper");
    })
    .class("bg-main");
}

#[derive(Lens)]
pub struct SwitchData {
    flag: bool,
}

pub fn switch(cx: &mut Context) {
    SwitchData { flag: false }.build(cx);

    section_title(cx, "Switch");

    VStack::new(cx, |cx| {
        VStack::new(cx, |cx| {
            wrapper_heading(cx, "A simple switch");
            Switch::new(cx, SwitchData::flag).on_toggle(|cx| cx.emit(SwitchEvent::Toggle));
        })
        .disabled(AppData::disabled)
        .class("wrapper")
        .class("bg-secondary");

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
    .class("bg-main");
}

#[derive(Lens)]
struct DropdownData {
    list: Vec<String>,
    choice: String,
}

pub fn dropdown(cx: &mut Context) {
    DropdownData {
        list: vec!["Red".to_string(), "Green".to_string(), "Blue".to_string()],
        choice: "Red".to_string(),
    }
    .build(cx);

    section_title(cx, "Dropdown");

    VStack::new(cx, |cx| {
        VStack::new(cx, |cx| {
            wrapper_heading(cx, "A simple dropdown");
            Dropdown::new(
                cx,
                move |cx| Label::new(cx, DropdownData::choice),
                move |cx| {
                    List::new(cx, DropdownData::list, |cx, _, item| {
                        Label::new(cx, item)
                            .width(Stretch(1.0))
                            //.child_top(Stretch(1.0))
                            //.child_bottom(Stretch(1.0))
                            .cursor(CursorIcon::Hand)
                            .bind(DropdownData::choice, move |handle, selected| {
                                if item.get(handle.cx) == selected.get(handle.cx) {
                                    handle.checked(true);
                                }
                            })
                            .on_press(move |cx| {
                                cx.emit(DropdownEvent::SetChoice(item.get(cx).clone()));
                                cx.emit(PopupEvent::Close);
                            });
                    });
                },
            )
            .width(Pixels(100.0));
        })
        .disabled(AppData::disabled)
        .class("wrapper")
        .class("bg-secondary");
    })
    .class("bg-main");
}

#[derive(Lens)]
struct TabData {
    list: Vec<&'static str>,
}

pub fn tabs(cx: &mut Context) {
    TabData { list: vec!["Tab1", "Tab2"] }.build(cx);

    section_title(cx, "Tabs");

    VStack::new(cx, |cx| {
        VStack::new(cx, |cx| {
            wrapper_heading(cx, "Simple tabs");
            TabView::new(cx, TabData::list, |cx, item| match item.get(cx) {
                "Tab1" => TabPair::new(
                    move |cx| {
                        Label::new(cx, item).hoverable(false);
                        Element::new(cx).class("indicator");
                    },
                    |cx| {
                        Element::new(cx).size(Pixels(200.0)).background_color(Color::red());
                    },
                ),

                "Tab2" => TabPair::new(
                    move |cx| {
                        Label::new(cx, item).hoverable(false);
                        Element::new(cx).class("indicator");
                    },
                    |cx| {
                        Element::new(cx).size(Pixels(200.0)).background_color(Color::blue());
                    },
                ),

                _ => TabPair::new(|_| {}, |_| {}),
            })
            .size(Auto);
        })
        .disabled(AppData::disabled)
        .class("wrapper")
        .class("bg-secondary");
    })
    .class("bg-main");
}

#[derive(Lens)]
struct SpinboxData {
    spinbox_value_1: i64,
    spinbox_value_2: usize,
    spinbox_value_3_choices: Vec<Spinbox3Values>,
    spinbox_value_3: Spinbox3Values,
}

#[derive(Clone, PartialEq, Copy, Eq, Data)]
enum Spinbox3Values {
    One,
    Two,
    Three,
}

pub fn spinbox(cx: &mut Context) {
    SpinboxData {
        spinbox_value_1: 99,
        spinbox_value_2: 0,
        spinbox_value_3: Spinbox3Values::One,
        spinbox_value_3_choices: Spinbox3Values::values(),
    }
    .build(cx);

    section_title(cx, "Spinbox");

    VStack::new(cx, |cx| {
        VStack::new(cx, |cx| {
            wrapper_heading(cx, "A Basic Numeric Spinbox");
            Spinbox::new(
                cx,
                SpinboxData::spinbox_value_1,
                SpinboxKind::Horizontal,
                SpinboxIcons::Math,
            )
            .on_increment(|ex| ex.emit(SpinboxEvent::Increment1))
            .on_decrement(|ex| ex.emit(SpinboxEvent::Decrement1))
            .width(Pixels(120.0));
        })
        .disabled(AppData::disabled)
        .class("wrapper")
        .class("bg-secondary");

        HStack::new(cx, |cx| {
            VStack::new(cx, |cx| {
                wrapper_heading(cx, "Spinbox With Textbox");
                Spinbox::custom(
                    cx,
                    |cx| {
                        Textbox::new(cx, SpinboxData::spinbox_value_2)
                            .on_edit(|ex, v| ex.emit(SpinboxEvent::Set2(v)))
                    },
                    SpinboxKind::Vertical,
                    SpinboxIcons::Math,
                )
                .on_increment(|ex| ex.emit(SpinboxEvent::Increment2))
                .on_decrement(|ex| ex.emit(SpinboxEvent::Decrement2));
            })
            .disabled(AppData::disabled)
            .class("wrapper")
            .class("bg-secondary");

            VStack::new(cx, |cx| {
                wrapper_heading(cx, "Spinbox With Dropdown");
                Spinbox::custom(
                    cx,
                    |cx| {
                        Dropdown::new(
                            cx,
                            |cx| {
                                HStack::new(cx, move |cx| {
                                    Label::new(cx, SpinboxData::spinbox_value_3);
                                })
                                .child_left(Pixels(5.0))
                                .child_right(Pixels(5.0))
                                .col_between(Stretch(1.0))
                            },
                            |cx| {
                                List::new(
                                    cx,
                                    SpinboxData::spinbox_value_3_choices,
                                    |cx, _, item| {
                                        Label::new(cx, &format!("{}", item.get(cx).to_string()))
                                            .on_press(move |cx| {
                                                cx.emit(SpinboxEvent::Set3(item.get(cx).clone()));
                                                cx.emit(PopupEvent::Close);
                                            });
                                    },
                                )
                                .child_right(Pixels(4.0));
                            },
                        )
                        .width(Pixels(50.0))
                    },
                    SpinboxKind::Horizontal,
                    SpinboxIcons::Chevrons,
                )
                .on_increment(|ex| ex.emit(SpinboxEvent::Increment3))
                .on_decrement(|ex| ex.emit(SpinboxEvent::Decrement3))
                .width(Pixels(120.0));
            })
            .disabled(AppData::disabled)
            .class("wrapper")
            .class("bg-secondary");
        })
        .class("row-wrapper");
    })
    .class("bg-main");
}

#[derive(Lens)]
pub struct TextboxData {
    text: String,
    multiline_text: String,
}

pub fn textbox(cx: &mut Context) {
    TextboxData {
        text: String::from("Some text..."),
        multiline_text: String::from("This text spans \n multiple lines."),
    }
    .build(cx);

    section_title(cx, "Textbox");

    VStack::new(cx, |cx| {
        VStack::new(cx, |cx| {
            wrapper_heading(cx, "Single Line Textbox");
            Textbox::new(cx, TextboxData::text)
                .width(Stretch(1.0))
                .on_submit(|cx, text, _| cx.emit(TextboxEvent::SetText(text)));
        })
        .disabled(AppData::disabled)
        .class("wrapper")
        .class("bg-secondary");

        VStack::new(cx, |cx| {
            wrapper_heading(cx, "Multi Line Textbox");
            Textbox::new_multiline(cx, TextboxData::multiline_text, false)
                .width(Stretch(1.0))
                .height(Pixels(100.0))
                .on_submit(|cx, text, _| cx.emit(TextboxEvent::SetMultiline(text)));
        })
        .disabled(AppData::disabled)
        .class("wrapper")
        .class("bg-secondary");

        VStack::new(cx, |cx| {
            wrapper_heading(cx, "Error Textbox");
            Textbox::new(cx, TextboxData::text)
                .width(Stretch(1.0))
                .class("error")
                .on_submit(|cx, text, _| cx.emit(TextboxEvent::SetText(text)));
        })
        .disabled(AppData::disabled)
        .class("wrapper")
        .class("bg-secondary");
    })
    .class("bg-main");
}

#[derive(Clone, Lens)]
struct DateTimePickerData {
    datetime: NaiveDateTime,
    show_popup: bool,
}

const ICON_CALENDAR: &str = "\u{1f4c5}";

pub fn datetimepicker(cx: &mut Context) {
    DateTimePickerData { datetime: Utc::now().naive_utc(), show_popup: false }.build(cx);

    PopupData::default().build(cx);

    section_title(cx, "Date & Time Picker");

    VStack::new(cx, |cx| {
        VStack::new(cx, |cx| {
            ZStack::new(cx, |cx| {
                wrapper_heading(cx, "Datetimepicker");
                Textbox::new(
                    cx,
                    DateTimePickerData::datetime
                        .map(|datetime| format!("{}", datetime.format("%d/%m/%Y  %H:%M:%S"))),
                )
                .child_top(Stretch(1.0))
                .child_bottom(Stretch(1.0))
                .width(Pixels(252.0))
                .height(Pixels(32.0));

                Label::new(cx, ICON_CALENDAR)
                    .height(Pixels(32.0))
                    .width(Pixels(32.0))
                    .left(Stretch(1.0))
                    .right(Pixels(0.0))
                    .child_space(Stretch(1.0))
                    .class("icon")
                    .cursor(CursorIcon::Hand)
                    .on_press(|cx| cx.emit(PopupEvent::Switch));
            })
            .width(Pixels(252.0))
            .height(Pixels(32.0));

            Popup::new(cx, PopupData::is_open, false, |cx| {
                DatetimePicker::new(cx, DateTimePickerData::datetime)
                    .on_change(|cx, datetime| cx.emit(DateTimePickerEvent::SetDateTime(datetime)));
            })
            .on_blur(|cx| cx.emit(PopupEvent::Close))
            .top(Pixels(36.0));
        })
        .disabled(AppData::disabled)
        .row_between(Pixels(8.0))
        .class("wrapper")
        .class("bg-secondary");
    })
    .class("bg-main");
}

#[derive(Lens)]
struct KnobData {
    knobs: Vec<f32>,
}

pub fn knob(cx: &mut Context) {
    KnobData { knobs: vec![0.5; 5] }.build(cx);

    section_title(cx, "Knob");

    VStack::new(cx, |cx| {
        HStack::new(cx, |cx| {
            // default knob
            VStack::new(cx, move |cx| {
                wrapper_heading(cx, "Default knob");

                Knob::new(cx, 0.5, KnobData::knobs.map(|knobs| knobs[0]), false)
                    .on_changing(move |cx, val| cx.emit(KnobEvent::SetKnob(0, val)))
                    .color(Color::red());

                Label::new(cx, KnobData::knobs.map(|knobs| format!("{:.3}", knobs[0])));
            })
            .disabled(AppData::disabled)
            .class("wrapper")
            .class("bg-secondary");

            // simple tick knob
            VStack::new(cx, move |cx| {
                wrapper_heading(cx, "Tick knob");

                Knob::custom(cx, 0.5, KnobData::knobs.map(|knobs| knobs[1]), move |cx, lens| {
                    TickKnob::new(
                        cx,
                        Percentage(100.0),
                        Percentage(20.0),
                        Percentage(50.0),
                        300.0,
                        KnobMode::Continuous,
                    )
                    .value(lens)
                    .class("track")
                })
                .on_changing(move |cx, val| cx.emit(KnobEvent::SetKnob(1, val)));
                Label::new(cx, KnobData::knobs.map(|knobs| format!("{:.3}", knobs[1])));
            })
            .disabled(AppData::disabled)
            .class("wrapper")
            .class("bg-secondary");

            // steppy knob
            VStack::new(cx, move |cx| {
                wrapper_heading(cx, "Steppy knob");

                Knob::custom(cx, 0.5, KnobData::knobs.map(|knobs| knobs[2]), move |cx, lens| {
                    let mode = KnobMode::Discrete(5);
                    TickKnob::new(
                        cx,
                        Percentage(60.0),
                        Percentage(20.0),
                        Percentage(50.0),
                        300.0,
                        mode,
                    )
                    .value(lens)
                    .class("track");
                    Ticks::new(cx, Percentage(100.0), Percentage(25.0), Pixels(5.0), 300.0, mode)
                        .class("track")
                })
                .on_changing(move |cx, val| cx.emit(KnobEvent::SetKnob(2, val)));
                Label::new(
                    cx,
                    KnobData::knobs.map(|knobs| format!("{:.3}", (knobs[2] * 4.0).floor() / 4.0)),
                );
            })
            .disabled(AppData::disabled)
            .class("wrapper")
            .class("bg-secondary");
        })
        .class("row-wrapper");

        HStack::new(cx, |cx| {
            // Arc+tick knob knob
            VStack::new(cx, move |cx| {
                wrapper_heading(cx, "Arc knob");

                Knob::custom(cx, 0.5, KnobData::knobs.map(|knobs| knobs[3]), move |cx, lens| {
                    TickKnob::new(
                        cx,
                        Percentage(90.0),
                        // setting tick_width to 0 to make the tick invisible
                        Percentage(0.0),
                        Percentage(0.0),
                        300.0,
                        KnobMode::Continuous,
                    )
                    .value(lens)
                    .class("track");
                    ArcTrack::new(
                        cx,
                        false,
                        Percentage(100.0),
                        Percentage(10.),
                        -150.,
                        150.,
                        KnobMode::Continuous,
                    )
                    .value(lens)
                    .class("track")
                })
                .on_changing(move |cx, val| cx.emit(KnobEvent::SetKnob(3, val)));
                Label::new(cx, KnobData::knobs.map(|knobs| format!("{:.3}", knobs[3])));
            })
            .disabled(AppData::disabled)
            .class("wrapper")
            .class("bg-secondary");

            // drag-able label
            VStack::new(cx, move |cx| {
                wrapper_heading(cx, "Label \"knob\"");

                Knob::custom(cx, 0.5, KnobData::knobs.map(|knobs| knobs[4]), move |cx, val| {
                    HStack::new(cx, move |cx| {
                        Label::new(cx, "val:").width(Pixels(40.0));
                        Label::new(cx, val.map(|val| format!("{:.2}", val))).width(Pixels(40.0));
                    })
                    .class("label_knob")
                })
                .on_changing(move |cx, val| cx.emit(KnobEvent::SetKnob(4, val)));
                Label::new(cx, KnobData::knobs.map(|knobs| format!("{:.3}", knobs[4])));
            })
            .disabled(AppData::disabled)
            .class("wrapper")
            .class("bg-secondary");
        })
        .class("row-wrapper");
    })
    .class("bg-main");
}

pub enum AppEvent {
    ToggleDisabled,
    ToggleTheme,
}

impl Model for AppData {
    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|e, _| match e {
            AppEvent::ToggleDisabled => {
                self.disabled = !self.disabled;
            }

            AppEvent::ToggleTheme => {
                self.theme = match self.theme {
                    ThemeMode::DarkMode => ThemeMode::LightMode,
                    ThemeMode::LightMode => ThemeMode::DarkMode,
                };

                cx.emit(EnvironmentEvent::SetThemeMode(self.theme));
            }
        })
    }
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

enum DropdownEvent {
    SetChoice(String),
}

impl Model for DropdownData {
    fn event(&mut self, _: &mut EventContext, event: &mut Event) {
        event.map(|dropdown_event, _| match dropdown_event {
            DropdownEvent::SetChoice(choice) => self.choice = choice.clone(),
        })
    }
}

impl Model for TabData {}

enum SpinboxEvent {
    Increment1,
    Decrement1,

    Increment2,
    Decrement2,
    Set2(String),

    Increment3,
    Decrement3,
    Set3(Spinbox3Values),
}

impl Spinbox3Values {
    pub fn from_number(num: usize) -> Result<Self, ()> {
        match num {
            0 => Ok(Spinbox3Values::One),
            1 => Ok(Spinbox3Values::Two),
            2 => Ok(Spinbox3Values::Three),
            _ => Err(()),
        }
    }

    pub fn values() -> Vec<Self> {
        vec![Spinbox3Values::One, Spinbox3Values::Two, Spinbox3Values::Three]
    }
}

impl Model for SpinboxData {
    fn event(&mut self, _cx: &mut EventContext, event: &mut Event) {
        event.map(|e, _| match e {
            SpinboxEvent::Decrement1 => {
                self.spinbox_value_1 -= 1;
            }

            SpinboxEvent::Increment1 => {
                self.spinbox_value_1 += 1;
            }

            SpinboxEvent::Decrement2 => {
                if self.spinbox_value_2 != 0 {
                    self.spinbox_value_2 -= 1;
                }
            }

            SpinboxEvent::Increment2 => {
                self.spinbox_value_2 += 1;
            }

            SpinboxEvent::Set2(v) => {
                self.spinbox_value_2 = match v.parse::<usize>() {
                    Ok(number) => number,
                    Err(_) => self.spinbox_value_2,
                }
            }

            SpinboxEvent::Increment3 => {
                let index = self.spinbox_value_3 as usize;
                self.spinbox_value_3 = Spinbox3Values::from_number((index + 1) % 3).unwrap();
            }

            SpinboxEvent::Decrement3 => {
                let mut index = self.spinbox_value_3 as usize;
                if index == 0 {
                    index = 3
                }
                self.spinbox_value_3 = Spinbox3Values::from_number(index - 1).unwrap();
            }

            SpinboxEvent::Set3(v) => self.spinbox_value_3 = v.clone(),
        })
    }
}

impl core::fmt::Display for Spinbox3Values {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Spinbox3Values::One => "one",
            Spinbox3Values::Two => "two",
            Spinbox3Values::Three => "three",
        })
    }
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

enum DateTimePickerEvent {
    SetDateTime(NaiveDateTime),
    ToggleDatetimePicker,
}

impl Model for DateTimePickerData {
    fn event(&mut self, _: &mut EventContext, event: &mut Event) {
        event.map(|app_event, _| match app_event {
            DateTimePickerEvent::SetDateTime(datetime) => {
                self.datetime = *datetime;
            }

            DateTimePickerEvent::ToggleDatetimePicker => {
                self.show_popup ^= true;
            }
        });
    }
}

enum KnobEvent {
    SetKnob(usize, f32),
}

impl Model for KnobData {
    fn event(&mut self, _cx: &mut EventContext, event: &mut Event) {
        event.map(|knob_change_event, _| match knob_change_event {
            KnobEvent::SetKnob(idx, new_val) => {
                self.knobs[*idx] = *new_val;
            }
        });
    }
}
