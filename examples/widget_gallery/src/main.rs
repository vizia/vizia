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
    theme_options: Signal<Vec<Localized>>,
    selected_theme: Signal<Option<usize>>,
) {
    VStack::new(cx, |cx| {
        Label::new(cx, "Theme Mode").class("dropdown-label");
        Select::new(cx, theme_options, selected_theme, true)
            .min_selected(1)
            .on_select(|cx, index| cx.emit(AppEvent::SetThemeMode(index)))
            .width(Pixels(100.0))
            .tooltip(|cx| {
                Tooltip::new(cx, |cx| {
                    Label::new(cx, "Select Theme Mode");
                })
            });
    })
    .alignment(Alignment::Left)
    .gap(Pixels(4.0))
    .size(Auto);
}

fn primary_color_selection_dropdown(
    cx: &mut Context,
    color_options: Signal<Vec<Localized>>,
    selected_color: Signal<Option<usize>>,
) {
    VStack::new(cx, |cx| {
        Label::new(cx, "Primary Color").class("dropdown-label");
        Select::new(cx, color_options, selected_color, true)
            .min_selected(1)
            .on_select(|cx, index| cx.emit(AppEvent::SetPrimaryThemeColor(index)))
            .width(Pixels(120.0))
            .tooltip(|cx| {
                Tooltip::new(cx, |cx| {
                    Label::new(cx, "Select Primary Color");
                })
            });
    })
    .alignment(Alignment::Left)
    .gap(Pixels(4.0))
    .size(Auto);
}

fn direction_selection_dropdown(cx: &mut Context, direction_options: Signal<Vec<&'static str>>) {
    let selected_direction = cx.environment().direction.map(|direction| match direction {
        Direction::LeftToRight => Some(0),
        Direction::RightToLeft => Some(1),
    });

    VStack::new(cx, |cx| {
        Label::new(cx, "Direction").class("dropdown-label");
        Select::new(cx, direction_options, selected_direction, true)
            .min_selected(1)
            .on_select(|cx, index| cx.emit(AppEvent::SetDirection(index)))
            .width(Pixels(100.0))
            .tooltip(|cx| {
                Tooltip::new(cx, |cx| {
                    Label::new(cx, "Select Direction");
                })
            });
    })
    .alignment(Alignment::Left)
    .gap(Pixels(4.0))
    .size(Auto);
}

fn language_selection_dropdown(
    cx: &mut Context,
    language_options: Signal<Vec<Localized>>,
    selected_language: Signal<Option<usize>>,
) {
    VStack::new(cx, |cx| {
        Label::new(cx, "Language").class("dropdown-label");
        Select::new(cx, language_options, selected_language, true)
            .min_selected(1)
            .on_select(|cx, index| cx.emit(AppEvent::SetLanguage(index)))
            .width(Pixels(110.0))
            .tooltip(|cx| {
                Tooltip::new(cx, |cx| {
                    Label::new(cx, "Select Language");
                })
            });
    })
    .alignment(Alignment::Left)
    .gap(Pixels(4.0))
    .size(Auto);
}

fn build_sidebar_content(
    cx: &mut Context,
    selected_view: Signal<&'static str>,
    search_text: Signal<String>,
    open_categories: Signal<Vec<bool>>,
) {
    Binding::new(cx, search_text, move |cx| {
        let query = search_text.get().to_lowercase();
        let query = query.trim().to_string();

        //Binding::new(cx, open_categories, move |cx| {
        let mut visible_item_count = 0usize;

        for (index, (category, items)) in CATEGORIES.iter().enumerate() {
            let matching: Vec<&'static str> = items
                .iter()
                .copied()
                .filter(|item| query.is_empty() || item.to_lowercase().contains(&query))
                .collect();

            if matching.is_empty() {
                continue;
            }

            visible_item_count += matching.len();

            let query_for_open = query.clone();
            let open_categories_for_open = open_categories;
            let category_open = Memo::new(move |_| {
                if query_for_open.is_empty() {
                    open_categories_for_open.get().get(index).copied().unwrap_or(true)
                } else {
                    true
                }
            });

            let open_categories_for_toggle = open_categories;
            Collapsible::new(
                cx,
                move |cx| {
                    Label::new(cx, *category).class("sidebar-group-label").hoverable(false);
                },
                move |cx| {
                    VStack::new(cx, |cx| {
                        for item in matching.clone() {
                            let selected_view = selected_view;
                            Button::new(cx, move |cx| Label::new(cx, item).hoverable(false))
                                .class("sidebar-menu-button")
                                .toggle_class(
                                    "sidebar-menu-button-active",
                                    selected_view.map(move |sv| *sv == item),
                                )
                                .on_press(move |cx| cx.emit(AppEvent::SelectView(item)));
                        }
                    })
                    .class("sidebar-menu")
                    .height(Auto);
                },
            )
            .class("sidebar-group")
            .open(category_open)
            .on_toggle(move |_cx, is_open| {
                open_categories_for_toggle.update(|open_states| {
                    if let Some(state) = open_states.get_mut(index) {
                        *state = is_open;
                    }
                });
            });
        }

        if visible_item_count == 0 {
            Label::new(cx, "No widgets match your search.").class("sidebar-empty-state");
        }
        //});
    });
}

fn content_area(cx: &mut Context, selected_view: Signal<&'static str>) {
    ScrollView::new(cx, move |cx| {
        Binding::new(cx, selected_view, move |cx| {
            let current_view = selected_view.get();
            VStack::new(cx, |cx| match current_view {
                "Accordion" => accordion(cx),
                "Avatar" => avatar(cx),
                "Avatar Group" => avatar_group(cx),
                "Badge" => badge(cx),
                "Button" => button(cx),
                "Button Group" => button_group(cx),
                "Calendar" => calendar(cx),
                "Card" => card(cx),
                "Checkbox" => checkbox(cx),
                "Chip" => chip(cx),
                "Collapsible" => collapsible(cx),
                "Combobox" => combobox(cx),
                "Divider" => divider(cx),
                "Dropdown" => dropdown(cx),
                "Element" => element(cx),
                "Grid" => grid(cx),
                "HStack" => hstack(cx),
                "Image" => image(cx),
                "Knob" => knob(cx),
                "Label" => label(cx),
                "List" => list(cx),
                "Markdown" => markdown_panel(cx),
                "Menu" => menu(cx),
                "MenuBar" => menu_bar(cx),
                "Popup" => popup(cx),
                "Progressbar" => progressbar(cx),
                "Radiobutton" => radiobutton(cx),
                "Rating" => rating(cx),
                "Resizable" => resizable(cx),
                "Scrollview" => scrollview(cx),
                "Select" => select(cx),
                "Slider" => slider(cx),
                "Spinbox" => spinbox(cx),
                "Svg" => svg(cx),
                "Switch" => switch(cx),
                "Table" => table(cx),
                "Tabview" => tabview(cx),
                "Textbox" => textbox(cx),
                "ToggleButton" => toggle_button(cx),
                "Tooltip" => tooltip(cx),
                "VirtualList" => virtual_list(cx),
                "VirtualTable" => virtual_table(cx),
                "VStack" => vstack(cx),
                "XYPad" => xypad(cx),
                "ZStack" => zstack(cx),
                _ => {}
            })
            .class("content-area")
            .alignment(Alignment::TopCenter)
            .height(Auto)
            .width(Stretch(1.0));
        });
    })
    .class("widgets")
    .width(Stretch(1.0));
}

fn main() -> Result<(), ApplicationError> {
    setup_logging()?;

    Application::new(|cx: &mut Context| {
        cx.add_translation(
            langid!("en-US"),
            include_str!("../../resources/translations/en-US/helper.ftl"),
        )
        .unwrap();

        cx.add_translation(
            langid!("fr"),
            include_str!("../../resources/translations/fr/helper.ftl"),
        )
        .unwrap();

        cx.add_translation(
            langid!("ar"),
            include_str!("../../resources/translations/ar/helper.ftl"),
        )
        .unwrap();

        let app_data = AppData::new();
        app_data.build(cx);

        cx.add_stylesheet(include_style!("src/style.css")).expect("Failed to add stylesheet");

        VStack::new(cx, |cx| {
            HStack::new(cx, |cx| {
                Sidebar::new(
                    cx,
                    move |cx| {
                        Label::new(cx, "Widget Gallery").class("sidebar-title");

                        Textbox::new(cx, app_data.search_text)
                            .on_edit(|cx, text| cx.emit(AppEvent::SetSearchText(text)))
                            .placeholder("Search widgets...")
                            .class("sidebar-search");
                    },
                    move |cx| {
                        build_sidebar_content(
                            cx,
                            app_data.selected_view,
                            app_data.search_text,
                            app_data.open_categories,
                        )
                    },
                    move |cx| {},
                );
                VStack::new(cx, |cx| {
                    HStack::new(cx, |cx| {
                        HStack::new(cx, |cx| {
                            Switch::new(cx, app_data.disabled)
                                .on_toggle(|cx| cx.emit(AppEvent::ToggleDisabled))
                                .id("disabled_toggle")
                                .tooltip(|cx| {
                                    Tooltip::new(cx, |cx| {
                                        Label::new(cx, Localized::new("toggle-disabled"));
                                    })
                                });
                            Label::new(cx, Localized::new("toggle-disabled"))
                                .describing("disabled_toggle");
                        })
                        .alignment(Alignment::Center)
                        .gap(Pixels(4.0))
                        .size(Auto);

                        theme_selection_dropdown(
                            cx,
                            app_data.theme_options,
                            app_data.selected_theme,
                        );
                        language_selection_dropdown(
                            cx,
                            app_data.language_options,
                            app_data.selected_language,
                        );
                        direction_selection_dropdown(cx, app_data.direction_options);
                        primary_color_selection_dropdown(
                            cx,
                            app_data.primary_color_options,
                            app_data.selected_primary_color,
                        );
                    })
                    .alignment(Alignment::Center)
                    .gap(Pixels(12.0))
                    .padding(Pixels(12.0))
                    .height(Auto);
                    content_area(cx, app_data.selected_view);
                });
            })
            .height(Stretch(1.0));
        });
    })
    .title("Widget Gallery")
    .inner_size((1000, 760))
    .min_inner_size(Some((800, 400)))
    .run()
}
