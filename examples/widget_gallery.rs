use vizia::{
    fonts::vizia_icons::{CHECK, MOON, PLUS, SUN},
    prelude::*,
};

const STYLE: &str = r#"

    .title {
        space: 0px;
        child-space: 16px;
        background-color: red;
        height: 48px;
        child-top: 1s;
        child-bottom: 1s;
    }
    
    .title > label {
        width: 1s;
        font-size: 30px;
        font-weight: 600;
    }

    .heading {
        font-size: 20.0;
        top: 10px;
        bottom: 6px;
    }

    .tabview-tabheader-wrapper {
        width: 200px;
        child-top: 4px;
        child-bottom: 4px;
    }

    tabheader {
        width: 1s;
        height: 24px;
        child-top: 1s;
        child-bottom: 1s;
    }

    tabheader label {
        width: 1s;
        color: #ffffff88;
    }
    
    tabheader:checked {
        background-color: #51afef22;
    }

    tabheader:hover {
        background-color: #51afef22;
    }

    .indicator {
        width: 2px;
        top: 0px;
        bottom: 0px;
        border-top-left-radius: 2px;
        border-bottom-left-radius: 2px;
    }

    .row-wrapper {
        width: auto;
    }

    .wrapper {
        width: auto;
        height: auto;
        background-color: #282828;
        border-radius: 4px;
        child-space: 8px;
        row-between: 8px;
    }

    .wrapper > hstack {
        col-between: 8px;
    }

    .bg-darker {
        child-space: 16px;
        row-between: 8px;
    }
"#;

#[derive(Lens)]
pub struct AppData {
    items: Vec<&'static str>,
    disabled: bool,
    theme: ThemeMode,
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

fn title(cx: &mut Context, title: &str) {
    HStack::new(cx, |cx| {
        Label::new(cx, title);
        HStack::new(cx, |cx| {
            Switch::new(cx, AppData::disabled).on_toggle(|cx| cx.emit(AppEvent::ToggleDisabled));
            Label::new(cx, "Disabled");
        })
        .size(Auto)
        .col_between(Pixels(5.0))
        .child_top(Stretch(1.0))
        .child_bottom(Stretch(1.0));
        Button::new(
            cx,
            |cx| cx.emit(AppEvent::ToggleTheme),
            |cx| {
                Label::new(
                    cx,
                    match AppData::theme.get(cx) {
                        ThemeMode::DarkMode => SUN,
                        _ => MOON,
                    },
                )
                .class("icon")
            },
        );
    })
    .class("title");
}

fn tab<T: ToString + Data>(cx: &mut Context, item: impl Lens<Target = T>) {
    Label::new(cx, item).hoverable(false);
    Element::new(cx).class("indicator").hoverable(false);
}

fn main() {
    Application::new(|cx| {
        cx.add_theme(STYLE);
        cx.emit(EnvironmentEvent::SetThemeMode(ThemeMode::DarkMode));

        AppData {
            items: vec![
                "Label", "Button", "Checkbox", "Slider", "Switch", "Spinbox", "Tabs", "Textbox",
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
        title(cx, "Button");

        // Basic Buttons
        VStack::new(cx, |cx| {
            Label::new(cx, "Basic Buttons").class("heading");
            HStack::new(cx, |cx| {
                Button::new(cx, |_| {}, |cx| Label::new(cx, "Simple Button"));
                Button::new(cx, |_| {}, |cx| Label::new(cx, "Accent Button")).class("accent");
                Button::new(cx, |_| {}, |cx| Label::new(cx, "Outline Button")).class("outline");
                Button::new(cx, |_| {}, |cx| Label::new(cx, "Ghost Button")).class("ghost");
            });
        })
        .disabled(AppData::disabled)
        .class("wrapper");

        // Icon Buttons
        VStack::new(cx, |cx| {
            Label::new(cx, "Icon Buttons").class("heading");
            HStack::new(cx, |cx| {
                Button::new(cx, |_| {}, |cx| Label::new(cx, PLUS)).class("icon");
                Button::new(cx, |_| {}, |cx| Label::new(cx, PLUS)).class("icon").class("accent");
                Button::new(cx, |_| {}, |cx| Label::new(cx, PLUS)).class("icon").class("outline");
                Button::new(cx, |_| {}, |cx| Label::new(cx, PLUS)).class("icon").class("ghost");
            });
        })
        .disabled(AppData::disabled)
        .class("wrapper");

        // Icon & Label Buttons
        VStack::new(cx, |cx| {
            Label::new(cx, "Icon & Label Buttons").class("heading");
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
        .class("wrapper");

        VStack::new(cx, |cx| {
            Label::new(cx, "Label & Icon Buttons").class("heading");
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
        .class("wrapper");
    })
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
        Label::new(cx, "Checkbox").class("title");

        VStack::new(cx, |cx| {
            Label::new(cx, "Simple Checkbox").class("heading");
            Checkbox::new(cx, CheckboxData::check).on_toggle(|cx| cx.emit(CheckboxEvent::Toggle));
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
    title(cx, "Label");

    VStack::new(cx, |cx| {
        Label::new(cx, "A simple label").class("heading");

        Label::new(cx, "This is some simple text");

        Label::new(cx, "A styled label").class("heading");

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
