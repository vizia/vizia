use vizia::icons::{ICON_MOON, ICON_SUN};
use vizia::prelude::*;

use log::LevelFilter;
use std::error::Error;

mod app_data;
use app_data::*;

mod views;
use views::*;

mod components;
use components::*;

pub fn setup_logging() -> Result<(), Box<dyn Error>> {
    #[cfg(debug_assertions)]
    const MAIN_LOG_LEVEL: LevelFilter = LevelFilter::Debug;
    #[cfg(not(debug_assertions))]
    const MAIN_LOG_LEVEL: LevelFilter = LevelFilter::Info;

    fern::Dispatch::new()
        // Perform allocation-free log formatting
        .format(move |out, message, record| {
            out.finish(format_args!("[{}][{}] {}", record.target(), record.level(), message))
        })
        // Add blanket level filter
        .level(MAIN_LOG_LEVEL)
        .level_for("cosmic_text::buffer", LevelFilter::Warn)
        .level_for("selectors::matching", LevelFilter::Warn)
        .level_for("cosmic_text::font::system::std", LevelFilter::Warn)
        // Output to stdout
        .chain(std::io::stdout())
        // Apply globally
        .apply()?;

    Ok(())
}

fn theme_selection_dropdown(cx: &mut Context) {
    PickList::new(cx, AppData::theme_options, AppData::selected_theme, true)
        .on_select(|cx, index| cx.emit(AppEvent::SetThemeMode(index)))
        .width(Pixels(85.0))
        .tooltip(|cx| {
            Tooltip::new(cx, |cx| {
                Label::new(cx, "Select Theme Mode");
            })
        });
}

fn toggle_disabled_switch(cx: &mut Context) {
    HStack::new(cx, |cx| {
        Switch::new(cx, AppData::disabled)
            .on_toggle(|cx| cx.emit(AppEvent::ToggleDisabled))
            .tooltip(|cx| {
                Tooltip::new(cx, |cx| {
                    Label::new(cx, "Toggle disabled");
                })
            });
        Label::new(cx, "Toggle Disabled");
    })
    .child_top(Stretch(1.0))
    .child_bottom(Stretch(1.0))
    .col_between(Pixels(5.0))
    .top(Stretch(1.0))
    .bottom(Stretch(1.0))
    .size(Auto);
}

fn main() -> Result<(), ApplicationError> {
    setup_logging();

    Application::new(|cx: &mut Context| {
        AppData::new().build(cx);

        cx.add_stylesheet(include_style!("src/style.css")).expect("Failed to add stylesheet");

        VStack::new(cx, |cx| {
            // Header
            HStack::new(cx, |cx| {
                toggle_disabled_switch(cx);
                theme_selection_dropdown(cx);
            })
            .child_space(Pixels(8.0))
            .child_left(Stretch(1.0))
            .col_between(Pixels(20.0))
            .height(Auto);

            Divider::new(cx);

            TabView::new(cx, AppData::tabs, |cx, item| match item.get(cx) {
                "All" => TabPair::new(
                    move |cx| {
                        Label::new(cx, item).class("tab-name").hoverable(false);
                    },
                    |cx| {
                        ScrollView::new(cx, 0.0, 0.0, false, true, |cx| {
                            button(cx);
                            // checkbox(cx);
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

                "Avatar" => TabPair::new(
                    move |cx| {
                        Label::new(cx, item).class("tab-name").hoverable(false);
                    },
                    |cx| {
                        ScrollView::new(cx, 0.0, 0.0, false, true, |cx| {
                            avatar(cx);
                        })
                        .class("widgets");
                    },
                ),

                "Badge" => TabPair::new(
                    move |cx| {
                        Label::new(cx, item).class("tab-name").hoverable(false);
                    },
                    |cx| {
                        ScrollView::new(cx, 0.0, 0.0, false, true, |cx| {
                            badge(cx);
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
                            button(cx);
                        })
                        .class("widgets");
                    },
                ),

                "Button Group" => TabPair::new(
                    move |cx| {
                        Label::new(cx, item).class("tab-name").hoverable(false);
                    },
                    |cx| {
                        ScrollView::new(cx, 0.0, 0.0, false, true, |cx| {
                            button_group(cx);
                        })
                        .class("widgets");
                    },
                ),

                "Floating Action Button" => TabPair::new(
                    move |cx| {
                        Label::new(cx, item).class("tab-name").hoverable(false);
                    },
                    |cx| {
                        ScrollView::new(cx, 0.0, 0.0, false, true, |cx| {
                            floating_action_button(cx);
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
                            chip(cx);
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
                            combobox(cx);
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
                            datepicker(cx);
                        })
                        .class("widgets");
                    },
                ),

                "Dialog" => TabPair::new(
                    move |cx| {
                        Label::new(cx, item).class("tab-name").hoverable(false);
                    },
                    |cx| {
                        ScrollView::new(cx, 0.0, 0.0, false, true, |cx| {
                            dialog(cx);
                        })
                        .class("widgets");
                    },
                ),

                "Divider" => TabPair::new(
                    move |cx| {
                        Label::new(cx, item).class("tab-name").hoverable(false);
                    },
                    |cx| {
                        ScrollView::new(cx, 0.0, 0.0, false, true, |cx| {
                            divider(cx);
                        })
                        .class("widgets");
                    },
                ),

                "Dropdown" => TabPair::new(
                    move |cx| {
                        Label::new(cx, item).class("tab-name").hoverable(false);
                    },
                    |cx| {
                        ScrollView::new(cx, 0.0, 0.0, false, true, |cx| {
                            dropdown(cx);
                        })
                        .class("widgets");
                    },
                ),

                "Element" => TabPair::new(
                    move |cx| {
                        Label::new(cx, item).class("tab-name").hoverable(false);
                    },
                    |cx| {
                        ScrollView::new(cx, 0.0, 0.0, false, true, |cx| {
                            // element(cx);
                        })
                        .class("widgets");
                    },
                ),

                "Form" => TabPair::new(
                    move |cx| {
                        Label::new(cx, item).class("tab-name").hoverable(false);
                    },
                    |cx| {
                        ScrollView::new(cx, 0.0, 0.0, false, true, |cx| {
                            // form(cx);
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
                            hstack(cx);
                        })
                        .class("widgets");
                    },
                ),

                "Icon" => TabPair::new(
                    move |cx| {
                        Label::new(cx, item).class("tab-name").hoverable(false);
                    },
                    |cx| {
                        ScrollView::new(cx, 0.0, 0.0, false, true, |cx| {
                            icon(cx);
                        })
                        .class("widgets");
                    },
                ),

                "Image" => TabPair::new(
                    move |cx| {
                        Label::new(cx, item).class("tab-name").hoverable(false);
                    },
                    |cx| {
                        ScrollView::new(cx, 0.0, 0.0, false, true, |cx| {
                            image(cx);
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
                            knob(cx);
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
                            label(cx);
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
                            list(cx);
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
                            menu(cx);
                        })
                        .class("widgets");
                    },
                ),

                "MenuBar" => TabPair::new(
                    move |cx| {
                        Label::new(cx, item).class("tab-name").hoverable(false);
                    },
                    |cx| {
                        ScrollView::new(cx, 0.0, 0.0, false, true, |cx| {
                            menu_bar(cx);
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
                            notification(cx);
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
                            picklist(cx);
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
                            popup(cx);
                        })
                        .class("widgets");
                    },
                ),

                "Progressbar" => TabPair::new(
                    move |cx| {
                        Label::new(cx, item).class("tab-name").hoverable(false);
                    },
                    |cx| {
                        ScrollView::new(cx, 0.0, 0.0, false, true, |cx| {
                            progressbar(cx);
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
                            radiobutton(cx);
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
                            rating(cx);
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
                            slider(cx);
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
                            spinbox(cx);
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
                            switch(cx);
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
                            tabview(cx);
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
                            textbox(cx);
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
                            timepicker(cx);
                        })
                        .class("widgets");
                    },
                ),

                "ToggleButton" => TabPair::new(
                    move |cx| {
                        Label::new(cx, item).class("tab-name").hoverable(false);
                    },
                    |cx| {
                        ScrollView::new(cx, 0.0, 0.0, false, true, |cx| {
                            // toggle_button(cx);
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
                            tooltip(cx);
                        })
                        .class("widgets");
                    },
                ),

                "VirtualList" => TabPair::new(
                    move |cx| {
                        Label::new(cx, item).class("tab-name").hoverable(false);
                    },
                    |cx| {
                        ScrollView::new(cx, 0.0, 0.0, false, true, |cx| {
                            // virtual_list(cx);
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
        });
    })
    .title("Widget Gallery")
    .inner_size((1400, 600))
    .min_inner_size(Some((900, 300)))
    .run()
}
