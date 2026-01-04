use vizia::prelude::*;

use log::LevelFilter;

mod app_data;
use app_data::*;

mod views;
use views::*;

mod components;
use components::*;

pub fn setup_logging() -> Result<(), ApplicationError> {
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
        .level_for("selectors::matching", LevelFilter::Warn)
        // Output to stdout
        .chain(std::io::stdout())
        // Apply globally
        .apply()
        .map_err(|_| ApplicationError::LogError)?;

    Ok(())
}

fn theme_selection_dropdown(
    cx: &mut Context,
    theme_options: Signal<Vec<&'static str>>,
    selected_theme: Signal<usize>,
) {
    let width_100 = cx.state(Pixels(100.0));

    PickList::new(cx, theme_options, selected_theme, true)
        .on_select(move |cx, index| {
            selected_theme.set(cx, index);
            cx.emit(EnvironmentEvent::SetThemeMode(match index {
                0 /* system */ => AppTheme::System,
                1 /* Dark */ => AppTheme::BuiltIn(ThemeMode::DarkMode),
                2 /* Light */ => AppTheme::BuiltIn(ThemeMode::LightMode),
                _ => unreachable!(),
            }));
        })
        .width(width_100)
        .tooltip(|cx| {
            Tooltip::new(cx, |cx| {
                Label::new(cx, "Select Theme Mode");
            })
        });
}

fn main() -> Result<(), ApplicationError> {
    setup_logging()?;
    WidgetGalleryApp::run()
}

struct WidgetGalleryApp {
    app_data: AppData,
    header_padding: Signal<Units>,
    header_gap: Signal<Units>,
    align_right: Signal<Alignment>,
    auto: Signal<Units>,
}

impl App for WidgetGalleryApp {
    fn new(cx: &mut Context) -> Self {
        Self {
            app_data: AppData::new(cx),
            header_padding: cx.state(Pixels(8.0)),
            header_gap: cx.state(Pixels(20.0)),
            align_right: cx.state(Alignment::Right),
            auto: cx.state(Auto),
        }
    }

    fn on_build(self, cx: &mut Context) -> Self {
        cx.add_stylesheet(include_style!("src/style.css")).expect("Failed to add stylesheet");

        let app_data = self.app_data;
        let header_padding = self.header_padding;
        let header_gap = self.header_gap;
        let align_right = self.align_right;
        let auto = self.auto;

        VStack::new(cx, |cx| {
            // Header
            HStack::new(cx, |cx| {
                // toggle_disabled_switch(cx);
                theme_selection_dropdown(cx, app_data.theme_options, app_data.selected_theme);
            })
            .padding(header_padding)
            .alignment(align_right)
            .horizontal_gap(header_gap)
            .height(auto);

            Divider::new(cx);

            TabView::new(cx, app_data.tabs, |cx, item| match item.get(cx) {
                "Avatar" => TabPair::new(
                    move |cx| {
                        Label::new(cx, item).class("tab-name").hoverable(false);
                    },
                    |cx| {
                        ScrollView::new(cx, |cx| {
                            avatar(cx);
                        })
                        .class("widgets");
                    },
                ),

                "Avatar Group" => TabPair::new(
                    move |cx| {
                        Label::new(cx, item).class("tab-name").hoverable(false);
                    },
                    |cx| {
                        ScrollView::new(cx, |cx| {
                            avatar_group(cx);
                        })
                        .class("widgets");
                    },
                ),

                "Badge" => TabPair::new(
                    move |cx| {
                        Label::new(cx, item).class("tab-name").hoverable(false);
                    },
                    |cx| {
                        ScrollView::new(cx, |cx| {
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
                        ScrollView::new(cx, |cx| {
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
                        ScrollView::new(cx, |cx| {
                            button_group(cx);
                        })
                        .class("widgets");
                    },
                ),

                "Checkbox" => TabPair::new(
                    move |cx| {
                        Label::new(cx, item).class("tab-name").hoverable(false);
                    },
                    |cx| {
                        ScrollView::new(cx, |cx| {
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
                        ScrollView::new(cx, |cx| {
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
                        ScrollView::new(cx, |cx| {
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
                        ScrollView::new(cx, |cx| {
                            datepicker(cx);
                        })
                        .class("widgets");
                    },
                ),

                "Divider" => TabPair::new(
                    move |cx| {
                        Label::new(cx, item).class("tab-name").hoverable(false);
                    },
                    |cx| {
                        ScrollView::new(cx, |cx| {
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
                        ScrollView::new(cx, |cx| {
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
                        ScrollView::new(cx, |cx| {
                            element(cx);
                        })
                        .class("widgets");
                    },
                ),

                "HStack" => TabPair::new(
                    move |cx| {
                        Label::new(cx, item).class("tab-name").hoverable(false);
                    },
                    |cx| {
                        ScrollView::new(cx, |cx| {
                            hstack(cx);
                        })
                        .class("widgets");
                    },
                ),

                "Svg" => TabPair::new(
                    move |cx| {
                        Label::new(cx, item).class("tab-name").hoverable(false);
                    },
                    |cx| {
                        ScrollView::new(cx, |cx| {
                            svg(cx);
                        })
                        .class("widgets");
                    },
                ),

                "Image" => TabPair::new(
                    move |cx| {
                        Label::new(cx, item).class("tab-name").hoverable(false);
                    },
                    |cx| {
                        ScrollView::new(cx, |cx| {
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
                        ScrollView::new(cx, |cx| {
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
                        ScrollView::new(cx, |cx| {
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
                        ScrollView::new(cx, |cx| {
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
                        ScrollView::new(cx, |cx| {
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
                        ScrollView::new(cx, |cx| {
                            menu_bar(cx);
                        })
                        .class("widgets");
                    },
                ),

                "Picklist" => TabPair::new(
                    move |cx| {
                        Label::new(cx, item).class("tab-name").hoverable(false);
                    },
                    |cx| {
                        ScrollView::new(cx, |cx| {
                            picklist(cx);
                        })
                        .class("widgets");
                    },
                ),

                "Progressbar" => TabPair::new(
                    move |cx| {
                        Label::new(cx, item).class("tab-name").hoverable(false);
                    },
                    |cx| {
                        ScrollView::new(cx, |cx| {
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
                        ScrollView::new(cx, |cx| {
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
                        ScrollView::new(cx, |cx| {
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
                        ScrollView::new(cx, |cx| {
                            scrollview(cx);
                        })
                        .class("widgets");
                    },
                ),

                "Slider" => TabPair::new(
                    move |cx| {
                        Label::new(cx, item).class("tab-name").hoverable(false);
                    },
                    |cx| {
                        ScrollView::new(cx, |cx| {
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
                        ScrollView::new(cx, |cx| {
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
                        ScrollView::new(cx, |cx| {
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
                        ScrollView::new(cx, |cx| {
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
                        ScrollView::new(cx, |cx| {
                            textbox(cx);
                        })
                        .class("widgets");
                    },
                ),

                "ToggleButton" => TabPair::new(
                    move |cx| {
                        Label::new(cx, item).class("tab-name").hoverable(false);
                    },
                    |cx| {
                        ScrollView::new(cx, |cx| {
                            toggle_button(cx);
                        })
                        .class("widgets");
                    },
                ),

                "Tooltip" => TabPair::new(
                    move |cx| {
                        Label::new(cx, item).class("tab-name").hoverable(false);
                    },
                    |cx| {
                        ScrollView::new(cx, |cx| {
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
                        ScrollView::new(cx, |cx| {
                            virtual_list(cx);
                        })
                        .class("widgets");
                    },
                ),

                "VStack" => TabPair::new(
                    move |cx| {
                        Label::new(cx, item).class("tab-name").hoverable(false);
                    },
                    |cx| {
                        ScrollView::new(cx, |cx| {
                            vstack(cx);
                        })
                        .class("widgets");
                    },
                ),

                "ZStack" => TabPair::new(
                    move |cx| {
                        Label::new(cx, item).class("tab-name").hoverable(false);
                    },
                    |cx| {
                        ScrollView::new(cx, |cx| {
                            zstack(cx);
                        })
                        .class("widgets");
                    },
                ),

                _ => TabPair::new(|_| {}, |_| {}),
            })
            .class("widgets")
            .vertical();
        });
        self
    }

    fn window_config(&self) -> WindowConfig {
        window(|app| {
            app.title("Widget Gallery")
               .inner_size((1400, 600))
               .min_inner_size(Some((900, 300)))
        })
    }
}
