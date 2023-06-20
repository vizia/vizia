use chrono::{NaiveDate, Utc};
use vizia::prelude::*;
use vizia_core::icons::{ICON_CHECK, ICON_CLIPBOARD, ICON_COPY, ICON_CUT};

mod helpers;
use helpers::*;

const COLORS: [Color; 3] =
    [Color::rgb(200, 50, 50), Color::rgb(50, 200, 50), Color::rgb(50, 50, 200)];

const STYLE: &str = r#"
    .container {
        child-space: 0px;
    }

    scrollview.widgets > scroll_content {
        child-space: 20px;
        row-between: 15px;
    }

    tabview.widgets > scrollview > scroll_content {
        child-space: 8px;
    }

    tabview.widgets tabheader label {
        width: 120px;
    }

    tabview.widgets tabheader:checked label {
        border-radius: 4px;
        background-color: #51afef;
    }

    vstack.panel {
        height: auto;
        row-between: 12px;
        bottom: 20px;
    }

    label.title {
        font-size: 30;
        font-weight: bold;
        bottom: 20px;
    }
"#;

#[derive(Lens)]
pub struct AppData {
    tabs: Vec<&'static str>,
}

impl Model for AppData {}

fn main() {
    Application::new(|cx| {
        AppData {
            tabs: vec![
                "All",
                "Button",
                "Label",
                "Checkbox",
                "Chip",
                "Combobox",
                "Datepicker",
                "HStack",
                "Knob",
                "List",
                "Menu",
                "Notification",
                "Picklist",
                "Popup",
                "Radiobutton",
                "Rating",
                "Scrollview",
                "Slider",
                "Spinbox",
                "Switch",
                "Tabview",
                "Textbox",
                "Timepicker",
                "Tooltip",
                "VStack",
                "ZStack",
            ],
        }
        .build(cx);

        // ExamplePage::vertical(cx, |cx| {
        cx.add_stylesheet(STYLE).expect("Failed to add stylesheet");

        TabView::new(cx, AppData::tabs, |cx, item| match item.get(cx) {
            "All" => TabPair::new(
                move |cx| {
                    Label::new(cx, item).hoverable(false);
                },
                |cx| {
                    ScrollView::new(cx, 0.0, 0.0, false, true, |cx| {
                        button(cx);
                        checkbox(cx);
                        chip(cx);
                        combobox(cx);
                        datepicker(cx);
                        hstack(cx);
                        knob(cx);
                        label(cx);
                        list(cx);
                        menu(cx);
                        notification(cx);
                        picklist(cx);
                        popup(cx);
                        radiobutton(cx);
                        rating(cx);
                        scrollview(cx);
                        slider(cx);
                        spinbox(cx);
                        switch(cx);
                        tabview(cx);
                        textbox(cx);
                        timepicker(cx);
                        tooltip(cx);
                        vstack(cx);
                        zstack(cx);
                    })
                    .class("widgets");
                },
            ),

            "Button" => TabPair::new(
                move |cx| {
                    Label::new(cx, item).hoverable(false);
                },
                |cx| {
                    ScrollView::new(cx, 0.0, 0.0, false, true, |cx| {
                        button(cx);
                    })
                    .class("widgets");
                },
            ),

            "Checkbox" => TabPair::new(
                move |cx| {
                    Label::new(cx, item).hoverable(false);
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
                    Label::new(cx, item).hoverable(false);
                },
                |cx| {
                    ScrollView::new(cx, 0.0, 0.0, false, true, |cx| {
                        chip(cx);
                    })
                    .class("widgets");
                },
            ),

            "Combobox" => TabPair::new(
                move |cx| {
                    Label::new(cx, item).hoverable(false);
                },
                |cx| {
                    ScrollView::new(cx, 0.0, 0.0, false, true, |cx| {
                        combobox(cx);
                    })
                    .class("widgets");
                },
            ),

            "Datepicker" => TabPair::new(
                move |cx| {
                    Label::new(cx, item).hoverable(false);
                },
                |cx| {
                    ScrollView::new(cx, 0.0, 0.0, false, true, |cx| {
                        datepicker(cx);
                    })
                    .class("widgets");
                },
            ),

            "HStack" => TabPair::new(
                move |cx| {
                    Label::new(cx, item).hoverable(false);
                },
                |cx| {
                    ScrollView::new(cx, 0.0, 0.0, false, true, |cx| {
                        hstack(cx);
                    })
                    .class("widgets");
                },
            ),

            "Knob" => TabPair::new(
                move |cx| {
                    Label::new(cx, item).hoverable(false);
                },
                |cx| {
                    ScrollView::new(cx, 0.0, 0.0, false, true, |cx| {
                        knob(cx);
                    })
                    .class("widgets");
                },
            ),

            "Label" => TabPair::new(
                move |cx| {
                    Label::new(cx, item).hoverable(false);
                },
                |cx| {
                    ScrollView::new(cx, 0.0, 0.0, false, true, |cx| {
                        label(cx);
                    })
                    .class("widgets");
                },
            ),

            "List" => TabPair::new(
                move |cx| {
                    Label::new(cx, item).hoverable(false);
                },
                |cx| {
                    ScrollView::new(cx, 0.0, 0.0, false, true, |cx| {
                        list(cx);
                    })
                    .class("widgets");
                },
            ),

            "Menu" => TabPair::new(
                move |cx| {
                    Label::new(cx, item).hoverable(false);
                },
                |cx| {
                    ScrollView::new(cx, 0.0, 0.0, false, true, |cx| {
                        menu(cx);
                    })
                    .class("widgets");
                },
            ),

            "Notification" => TabPair::new(
                move |cx| {
                    Label::new(cx, item).hoverable(false);
                },
                |cx| {
                    ScrollView::new(cx, 0.0, 0.0, false, true, |cx| {
                        notification(cx);
                    })
                    .class("widgets");
                },
            ),

            "Picklist" => TabPair::new(
                move |cx| {
                    Label::new(cx, item).hoverable(false);
                },
                |cx| {
                    ScrollView::new(cx, 0.0, 0.0, false, true, |cx| {
                        picklist(cx);
                    })
                    .class("widgets");
                },
            ),

            "Popup" => TabPair::new(
                move |cx| {
                    Label::new(cx, item).hoverable(false);
                },
                |cx| {
                    ScrollView::new(cx, 0.0, 0.0, false, true, |cx| {
                        popup(cx);
                    })
                    .class("widgets");
                },
            ),

            "Radiobutton" => TabPair::new(
                move |cx| {
                    Label::new(cx, item).hoverable(false);
                },
                |cx| {
                    ScrollView::new(cx, 0.0, 0.0, false, true, |cx| {
                        radiobutton(cx);
                    })
                    .class("widgets");
                },
            ),

            "Rating" => TabPair::new(
                move |cx| {
                    Label::new(cx, item).hoverable(false);
                },
                |cx| {
                    ScrollView::new(cx, 0.0, 0.0, false, true, |cx| {
                        rating(cx);
                    })
                    .class("widgets");
                },
            ),

            "Scrollview" => TabPair::new(
                move |cx| {
                    Label::new(cx, item).hoverable(false);
                },
                |cx| {
                    ScrollView::new(cx, 0.0, 0.0, false, true, |cx| {
                        scrollview(cx);
                    })
                    .class("widgets");
                },
            ),

            "Slider" => TabPair::new(
                move |cx| {
                    Label::new(cx, item).hoverable(false);
                },
                |cx| {
                    ScrollView::new(cx, 0.0, 0.0, false, true, |cx| {
                        slider(cx);
                    })
                    .class("widgets");
                },
            ),

            "Spinbox" => TabPair::new(
                move |cx| {
                    Label::new(cx, item).hoverable(false);
                },
                |cx| {
                    ScrollView::new(cx, 0.0, 0.0, false, true, |cx| {
                        spinbox(cx);
                    })
                    .class("widgets");
                },
            ),

            "Switch" => TabPair::new(
                move |cx| {
                    Label::new(cx, item).hoverable(false);
                },
                |cx| {
                    ScrollView::new(cx, 0.0, 0.0, false, true, |cx| {
                        switch(cx);
                    })
                    .class("widgets");
                },
            ),

            "Tabview" => TabPair::new(
                move |cx| {
                    Label::new(cx, item).hoverable(false);
                },
                |cx| {
                    ScrollView::new(cx, 0.0, 0.0, false, true, |cx| {
                        tabview(cx);
                    })
                    .class("widgets");
                },
            ),

            "Textbox" => TabPair::new(
                move |cx| {
                    Label::new(cx, item).hoverable(false);
                },
                |cx| {
                    ScrollView::new(cx, 0.0, 0.0, false, true, |cx| {
                        textbox(cx);
                    })
                    .class("widgets");
                },
            ),

            "Timepicker" => TabPair::new(
                move |cx| {
                    Label::new(cx, item).hoverable(false);
                },
                |cx| {
                    ScrollView::new(cx, 0.0, 0.0, false, true, |cx| {
                        timepicker(cx);
                    })
                    .class("widgets");
                },
            ),

            "Tooltip" => TabPair::new(
                move |cx| {
                    Label::new(cx, item).hoverable(false);
                },
                |cx| {
                    ScrollView::new(cx, 0.0, 0.0, false, true, |cx| {
                        tooltip(cx);
                    })
                    .class("widgets");
                },
            ),

            "VStack" => TabPair::new(
                move |cx| {
                    Label::new(cx, item).hoverable(false);
                },
                |cx| {
                    ScrollView::new(cx, 0.0, 0.0, false, true, |cx| {
                        vstack(cx);
                    })
                    .class("widgets");
                },
            ),

            "ZStack" => TabPair::new(
                move |cx| {
                    Label::new(cx, item).hoverable(false);
                },
                |cx| {
                    ScrollView::new(cx, 0.0, 0.0, false, true, |cx| {
                        zstack(cx);
                    })
                    .class("widgets");
                },
            ),

            _ => TabPair::new(|_| {}, |_| {}),
        })
        .class("widgets")
        .vertical();
        // });
    })
    .title("Widget Gallery")
    .run();
}

fn zstack(cx: &mut Context) {
    Label::new(cx, "ZStack").class("title");
}

fn vstack(cx: &mut Context) {
    VStack::new(cx, |cx| {
        Label::new(cx, "VStack").class("title");

        VStack::new(cx, |cx| {
            for i in 0..3 {
                Element::new(cx).size(Pixels(100.0)).background_color(COLORS[i]);
            }
        })
        .size(Auto)
        .child_space(Stretch(1.0));
    })
    .class("panel");
}

fn tooltip(cx: &mut Context) {
    Label::new(cx, "Tooltip").class("title");
}

fn timepicker(cx: &mut Context) {
    Label::new(cx, "Timepicker").class("title");
}

fn spinbox(cx: &mut Context) {
    Label::new(cx, "Spinbox").class("title");
}

fn scrollview(cx: &mut Context) {
    Label::new(cx, "Scrollview").class("title");
}

fn popup(cx: &mut Context) {
    Label::new(cx, "Popup").class("title");
}

fn notification(cx: &mut Context) {
    Label::new(cx, "Notification").class("title");
}

fn menu(cx: &mut Context) {
    Label::new(cx, "Menu").class("title");

    MenuBar::new(cx, |cx| {
        Submenu::new(
            cx,
            |cx| Label::new(cx, "File"),
            |cx| {
                MenuButton::new(
                    cx,
                    |_| println!("File"),
                    |cx| {
                        HStack::new(cx, |cx| {
                            Label::new(cx, "New");
                            Label::new(cx, &format!("Ctrl + N")).class("shortcut");
                        })
                    },
                );
                MenuButton::new(
                    cx,
                    |_| println!("Open"),
                    |cx| {
                        HStack::new(cx, |cx| {
                            Label::new(cx, "Open");
                            Label::new(cx, &format!("Ctrl + O")).class("shortcut");
                        })
                    },
                );
                Submenu::new(
                    cx,
                    |cx| Label::new(cx, "Open Recent"),
                    |cx| {
                        MenuButton::new(cx, |_| println!("Doc 1"), |cx| Label::new(cx, "Doc 1"));
                        Submenu::new(
                            cx,
                            |cx| Label::new(cx, "Doc 2"),
                            |cx| {
                                MenuButton::new(
                                    cx,
                                    |_| println!("Version 1"),
                                    |cx| Label::new(cx, "Version 1"),
                                );
                                MenuButton::new(
                                    cx,
                                    |_| println!("Version 2"),
                                    |cx| Label::new(cx, "Version 2"),
                                );
                                MenuButton::new(
                                    cx,
                                    |_| println!("Version 3"),
                                    |cx| Label::new(cx, "Version 3"),
                                );
                            },
                        );
                        MenuButton::new(cx, |_| println!("Doc 3"), |cx| Label::new(cx, "Doc 3"));
                    },
                );
                MenuDivider::new(cx);
                MenuButton::new(cx, |_| println!("Save"), |cx| Label::new(cx, "Save"));
                MenuButton::new(cx, |_| println!("Save As"), |cx| Label::new(cx, "Save As"));
                MenuDivider::new(cx);
                MenuButton::new(cx, |_| println!("Quit"), |cx| Label::new(cx, "Quit"));
            },
        );

        Submenu::new(
            cx,
            |cx| Label::new(cx, "Edit"),
            |cx| {
                MenuButton::new(
                    cx,
                    |_| println!("Cut"),
                    |cx| {
                        HStack::new(cx, |cx| {
                            Label::new(cx, ICON_CUT).class("icon");
                            Label::new(cx, "Cut");
                        })
                    },
                );
                MenuButton::new(
                    cx,
                    |_| println!("Copy"),
                    |cx| {
                        HStack::new(cx, |cx| {
                            Label::new(cx, ICON_COPY).class("icon");
                            Label::new(cx, "Copy");
                        })
                    },
                );
                MenuButton::new(
                    cx,
                    |_| println!("Paste"),
                    |cx| {
                        HStack::new(cx, |cx| {
                            Label::new(cx, ICON_CLIPBOARD).class("icon");
                            Label::new(cx, "Paste");
                        })
                    },
                );
            },
        );
        Submenu::new(
            cx,
            |cx| Label::new(cx, "View"),
            |cx| {
                MenuButton::new(cx, |_| println!("Zoom In"), |cx| Label::new(cx, "Zoom In"));
                MenuButton::new(cx, |_| println!("Zoom Out"), |cx| Label::new(cx, "Zoom Out"));
                Submenu::new(
                    cx,
                    |cx| Label::new(cx, "Zoom Level"),
                    |cx| {
                        MenuButton::new(cx, |_| println!("10%"), |cx| Label::new(cx, "10%"));
                        MenuButton::new(cx, |_| println!("20%"), |cx| Label::new(cx, "20%"));
                        MenuButton::new(cx, |_| println!("50%"), |cx| Label::new(cx, "50%"));
                        MenuButton::new(cx, |_| println!("100%"), |cx| Label::new(cx, "100%"));
                        MenuButton::new(cx, |_| println!("150%"), |cx| Label::new(cx, "150%"));
                        MenuButton::new(cx, |_| println!("200%"), |cx| Label::new(cx, "200%"));
                    },
                );
            },
        );
        Submenu::new(
            cx,
            |cx| Label::new(cx, "Help"),
            |cx| {
                MenuButton::new(
                    cx,
                    |_| println!("Show License"),
                    |cx| Label::new(cx, "Show License"),
                );
                MenuButton::new(cx, |_| println!("About"), |cx| Label::new(cx, "About"));
            },
        );
    })
    .top(Pixels(0.0));
}

fn list(cx: &mut Context) {
    Label::new(cx, "List").class("title");
}

fn knob(cx: &mut Context) {
    Label::new(cx, "Knob").class("title");
}

fn hstack(cx: &mut Context) {
    VStack::new(cx, |cx| {
        Label::new(cx, "HStack").class("title");

        HStack::new(cx, |cx| {
            for i in 0..3 {
                Element::new(cx).size(Pixels(100.0)).background_color(COLORS[i]);
            }
        })
        .size(Auto)
        .child_space(Stretch(1.0));
    })
    .class("panel");
}

// BUTTON

pub fn button(cx: &mut Context) {
    VStack::new(cx, |cx| {
        Label::new(cx, "Button").class("title");
        HStack::new(cx, |cx| {
            // Basic Button
            Button::new(cx, |_| {}, |cx| Label::new(cx, "Button"));
            // Accent Button
            Button::new(cx, |_| {}, |cx| Label::new(cx, "Accent Button")).class("accent");
            // Outline Button
            Button::new(cx, |_| {}, |cx| Label::new(cx, "Outline Button")).class("outline");
            // Ghost Button
            Button::new(cx, |_| {}, |cx| Label::new(cx, "Ghost Button")).class("ghost");
            // Button with Icon
            Button::new(
                cx,
                |_| {},
                |cx| {
                    HStack::new(cx, |cx| {
                        Label::new(cx, ICON_CHECK).class("icon");
                        Label::new(cx, "Button with Icon");
                    })
                },
            );
        })
        .height(Auto)
        .col_between(Pixels(8.0));
    })
    .class("panel");
}

// CHECKBOX

#[derive(Lens)]
pub struct CheckboxData {
    check: bool,
}

pub enum CheckboxEvent {
    Toggle,
}

impl Model for CheckboxData {
    fn event(&mut self, _: &mut EventContext, event: &mut Event) {
        event.map(|checkbox_event, _| match checkbox_event {
            CheckboxEvent::Toggle => {
                self.check ^= true;
            }
        });
    }
}

pub fn checkbox(cx: &mut Context) {
    CheckboxData { check: false }.build(cx);

    VStack::new(cx, |cx| {
        Label::new(cx, "Checkbox").class("title");

        Checkbox::new(cx, CheckboxData::check).on_toggle(|cx| cx.emit(CheckboxEvent::Toggle));

        HStack::new(cx, |cx| {
            Checkbox::new(cx, CheckboxData::check)
                .id("checky")
                .on_toggle(|cx| cx.emit(CheckboxEvent::Toggle));
            Label::new(cx, "Checkbox with label").describing("checky");
        })
        .size(Auto)
        .child_top(Stretch(1.0))
        .child_bottom(Stretch(1.0))
        .col_between(Pixels(5.0));
    })
    .class("panel");
}

// CHIP

#[derive(Lens)]
struct ChipData {
    chip: String,
    chips: Vec<String>,
}

impl Model for ChipData {
    fn event(&mut self, _cx: &mut EventContext, event: &mut Event) {
        event.map(|app_event, _| match app_event {
            ChipEvent::CloseChip(index) => {
                self.chips.remove(*index);
            }
        })
    }
}

enum ChipEvent {
    CloseChip(usize),
}

pub fn chip(cx: &mut Context) {
    ChipData {
        chip: "Chip".to_string(),
        chips: vec!["red".to_string(), "green".to_string(), "blue".to_string()],
    }
    .build(cx);

    VStack::new(cx, |cx| {
        Label::new(cx, "Chip").class("title");

        Chip::new(cx, ChipData::chip).background_color(Color::from("#ff004444"));

        List::new(cx, ChipData::chips, |cx, index, item| {
            Chip::new(cx, item)
                .on_close(move |cx| cx.emit(ChipEvent::CloseChip(index)))
                .background_color(Color::from("#ff000044"));
        })
        .layout_type(LayoutType::Row)
        .col_between(Pixels(4.0));
    })
    .class("panel");
}

#[derive(Clone, Lens)]
struct ComboBoxData {
    options: Vec<&'static str>,
    selected_option: usize,
}

pub enum ComboBoxEvent {
    SetOption(usize),
}

impl Model for ComboBoxData {
    fn event(&mut self, _: &mut EventContext, event: &mut Event) {
        event.map(|app_event, _| match app_event {
            ComboBoxEvent::SetOption(index) => {
                self.selected_option = *index;
            }
        });
    }
}

pub fn combobox(cx: &mut Context) {
    ComboBoxData {
        options: vec![
            "One", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "Nine", "Ten",
        ],

        selected_option: 0,
    }
    .build(cx);

    VStack::new(cx, |cx| {
        Label::new(cx, "Combobox").class("title");

        ComboBox::new(cx, ComboBoxData::options, ComboBoxData::selected_option)
            .on_select(|cx, index| cx.emit(ComboBoxEvent::SetOption(index)))
            .width(Pixels(140.0));
    })
    .class("panel");
}

// DATEPICKER
#[derive(Lens)]
struct DatepickerData {
    date: NaiveDate,
}

pub enum DatepickerEvent {
    SetDate(NaiveDate),
}

impl Model for DatepickerData {
    fn event(&mut self, _: &mut EventContext, event: &mut Event) {
        event.map(|app_event, _| match app_event {
            DatepickerEvent::SetDate(date) => {
                println!("Date changed to: {}", date);
                self.date = *date;
            }
        });
    }
}

pub fn datepicker(cx: &mut Context) {
    DatepickerData { date: Utc::now().date_naive() }.build(cx);

    VStack::new(cx, |cx| {
        Label::new(cx, "Datepicker").class("title");

        Datepicker::new(cx, DatepickerData::date)
            .on_select(|cx, date| cx.emit(DatepickerEvent::SetDate(date)));
    })
    .class("panel");
}

// LABEL

pub fn label(cx: &mut Context) {
    VStack::new(cx, |cx| {
        Label::new(cx, "Label").font_size(30.0).class("title");

        Label::new(cx, "This is some simple text");
        Label::new(cx, "This is some simple text");
    })
    .class("panel");
}

// PICKLIST

#[derive(Lens)]
struct PicklistData {
    options: Vec<&'static str>,
    selected_option: usize,
}

pub enum PicklistEvent {
    SetOption(usize),
}

impl Model for PicklistData {
    fn event(&mut self, _: &mut EventContext, event: &mut Event) {
        event.map(|app_event, _| match app_event {
            PicklistEvent::SetOption(index) => {
                self.selected_option = *index;
            }
        });
    }
}

pub fn picklist(cx: &mut Context) {
    PicklistData {
        options: vec![
            "One", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "Nine", "Ten",
            "Eleven", "Twelve",
        ],
        selected_option: 0,
    }
    .build(cx);

    VStack::new(cx, |cx| {
        Label::new(cx, "Picklist").class("title");

        PickList::new(cx, PicklistData::options, PicklistData::selected_option, true)
            .on_select(|cx, index| cx.emit(PicklistEvent::SetOption(index)))
            .width(Pixels(140.0));
    })
    .class("panel");
}

// RADIOBUTTON

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Options {
    First,
    Second,
    Third,
}

impl std::fmt::Display for Options {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match *self {
            Options::First => "First",
            Options::Second => "Second",
            Options::Third => "Third",
        };
        write!(f, "{}", str)
    }
}

#[derive(Lens)]
pub struct RadioData {
    option: Options,
}

pub enum RadioEvent {
    SetOption(Options),
}

impl Model for RadioData {
    fn event(&mut self, _: &mut EventContext, event: &mut Event) {
        event.map(|checkbox_event, _| match checkbox_event {
            RadioEvent::SetOption(option) => {
                self.option = *option;
            }
        });
    }
}

pub fn radiobutton(cx: &mut Context) {
    RadioData { option: Options::First }.build(cx);

    VStack::new(cx, |cx| {
        Label::new(cx, "Radiobutton").class("title");

        HStack::new(cx, |cx| {
            for i in 0..3 {
                let current_option = index_to_option(i);
                RadioButton::new(
                    cx,
                    RadioData::option.map(move |option| *option == current_option),
                )
                .on_select(move |cx| cx.emit(RadioEvent::SetOption(current_option)));
            }
        })
        .size(Auto)
        .col_between(Pixels(20.0));

        VStack::new(cx, |cx| {
            for i in 0..3 {
                let current_option = index_to_option(i);
                HStack::new(cx, move |cx| {
                    RadioButton::new(
                        cx,
                        RadioData::option.map(move |option| *option == current_option),
                    )
                    .on_select(move |cx| cx.emit(RadioEvent::SetOption(current_option)))
                    .id(format!("button_{i}"));
                    Label::new(cx, &current_option.to_string()).describing(format!("button_{i}"));
                })
                .size(Auto)
                .child_top(Stretch(1.0))
                .child_bottom(Stretch(1.0))
                .col_between(Pixels(5.0));
            }
        })
        .row_between(Pixels(10.0))
        .size(Auto);
    })
    .class("panel");
}

fn index_to_option(index: usize) -> Options {
    match index {
        0 => Options::First,
        1 => Options::Second,
        2 => Options::Third,
        _ => unreachable!(),
    }
}

// RATING

#[derive(Clone, Lens)]
struct RatingData {
    rating1: u32,
    rating2: u32,
}

impl Model for RatingData {
    fn event(&mut self, _cx: &mut EventContext, event: &mut Event) {
        event.map(|app_event, _| match app_event {
            RatingEvent::SetRating1(val) => self.rating1 = *val,
            RatingEvent::SetRating2(val) => self.rating2 = *val,
        })
    }
}

enum RatingEvent {
    SetRating1(u32),
    SetRating2(u32),
}

pub fn rating(cx: &mut Context) {
    RatingData { rating1: 3, rating2: 7 }.build(cx);

    VStack::new(cx, |cx| {
        Label::new(cx, "Rating").class("title");

        Rating::new(cx, 5, RatingData::rating1)
            .on_change(|ex, rating| ex.emit(RatingEvent::SetRating1(rating)));

        Rating::new(cx, 10, RatingData::rating2)
            .on_change(|ex, rating| ex.emit(RatingEvent::SetRating2(rating)));
    })
    .class("panel");
}

// SLIDER

#[derive(Debug, Lens)]
pub struct SliderData {
    value: f32,
}

pub enum SliderEvent {
    SetValue(f32),
}

impl Model for SliderData {
    fn event(&mut self, _: &mut EventContext, event: &mut Event) {
        event.map(|app_event, _| match app_event {
            SliderEvent::SetValue(val) => {
                self.value = *val;
            }
        });
    }
}

pub fn slider(cx: &mut Context) {
    SliderData { value: 0.0 }.build(cx);

    VStack::new(cx, |cx| {
        Label::new(cx, "Slider").class("title");

        Slider::new(cx, SliderData::value.map(|val| (val + 50.0) / 100.0))
            .range(0.0..1.0)
            .on_changing(move |cx, val| cx.emit(SliderEvent::SetValue(-50.0 + (val * 100.0))));

        HStack::new(cx, |cx| {
            Slider::new(cx, SliderData::value.map(|val| (val + 50.0) / 100.0))
                .range(0.0..1.0)
                .on_changing(move |cx, val| cx.emit(SliderEvent::SetValue(-50.0 + (val * 100.0))));
            Label::new(cx, SliderData::value.map(|val| format!("{:.2}", (val + 50.0) / 100.0)))
                .width(Pixels(50.0));
        })
        .child_top(Stretch(1.0))
        .child_bottom(Stretch(1.0))
        .height(Auto)
        .col_between(Pixels(8.0));
    })
    .class("panel");
}
#[derive(Debug, Lens)]
pub struct SwitchData {
    pub option1: bool,
    pub option2: bool,
}

// SWITCH

#[derive(Debug)]
pub enum SwitchEvent {
    ToggleOption1,
    ToggleOption2,
}

impl Model for SwitchData {
    fn event(&mut self, _cx: &mut EventContext, event: &mut Event) {
        event.map(|app_event, _| match app_event {
            SwitchEvent::ToggleOption1 => {
                self.option1 ^= true;
            }

            SwitchEvent::ToggleOption2 => {
                self.option2 ^= true;
            }
        });
    }
}

pub fn switch(cx: &mut Context) {
    SwitchData { option1: true, option2: false }.build(cx);

    VStack::new(cx, |cx| {
        Label::new(cx, "Switch").class("title");

        Switch::new(cx, SwitchData::option1).on_toggle(|cx| cx.emit(SwitchEvent::ToggleOption1));

        HStack::new(cx, |cx| {
            Switch::new(cx, SwitchData::option2)
                .on_toggle(|cx| cx.emit(SwitchEvent::ToggleOption2))
                .id("Switch_1");
            Label::new(cx, "Switch with label").describing("Switch_1");
        })
        .size(Auto)
        .col_between(Pixels(5.0))
        .child_top(Stretch(1.0))
        .child_bottom(Stretch(1.0));
    })
    .class("panel");
}

// TABVIEW

#[derive(Lens)]
pub struct TabviewData {
    tabs: Vec<&'static str>,
}

impl Model for TabviewData {}

pub fn tabview(cx: &mut Context) {
    TabviewData { tabs: vec!["Tab1", "Tab2"] }.build(cx);
    VStack::new(cx, |cx| {
        TabView::new(cx, TabviewData::tabs, |cx, item| match item.get(cx) {
            "Tab1" => TabPair::new(
                move |cx| {
                    Label::new(cx, item).hoverable(false);
                    Element::new(cx).class("indicator");
                },
                |cx| {
                    Label::new(cx, "Content for first tab");
                },
            ),

            "Tab2" => TabPair::new(
                move |cx| {
                    Label::new(cx, item).hoverable(false);
                    Element::new(cx).class("indicator");
                },
                |cx| {
                    Label::new(cx, "Content for second tab");
                },
            ),

            _ => unreachable!(),
        })
        .height(Pixels(100.0));
    })
    .class("panel");
}

// TEXTBOX

#[derive(Lens)]
pub struct TextboxData {
    editable_text: String,
    multiline_text: String,
    non_editable_text: String,
}

impl Model for TextboxData {
    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|app_event, _| match app_event {
            TextboxEvent::SetEditableText(text) => self.editable_text = text.clone(),
            TextboxEvent::SetMultilineText(text) => self.multiline_text = text.clone(),
        });
    }
}

pub enum TextboxEvent {
    SetEditableText(String),
    SetMultilineText(String),
}

pub fn textbox(cx: &mut Context) {
    VStack::new(cx, |cx| {
        Label::new(cx, "Textbox").class("title");

        TextboxData {
            editable_text: "This is some editable text".to_string(),
            multiline_text: "This is some text which is editable and spans multiple lines"
                .to_string(),
            non_editable_text: "This text can be selected but not edited".to_string(),
        }
        .build(cx);

        Textbox::new(cx, TextboxData::editable_text)
            .width(Pixels(300.0))
            .on_edit(|cx, text| cx.emit(TextboxEvent::SetEditableText(text)));
        Textbox::new_multiline(cx, TextboxData::multiline_text, true)
            .width(Pixels(300.0))
            .height(Pixels(300.0))
            .on_edit(|cx, text| cx.emit(TextboxEvent::SetMultilineText(text)));
        Textbox::new(cx, TextboxData::non_editable_text).width(Pixels(300.0)).read_only(true);
    })
    .class("panel");
}
