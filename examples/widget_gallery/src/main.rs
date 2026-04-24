use vizia::{icons::ICON_CLOCK, prelude::*};

use chrono::Utc;
use log::LevelFilter;
use vizia::icons::{ICON_BOLD, ICON_ITALIC, ICON_SETTINGS, ICON_UNDERLINE, ICON_USER};

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

const THEME_MODE_LABELS: [&str; 3] = ["System", "Dark", "Light"];
const LANGUAGE_LABELS: [&str; 3] = ["English", "French", "Arabic"];
const DIRECTION_LABELS: [&str; 2] = ["LTR", "RTL"];
const ACCENT_COLOR_LABELS: [&str; 6] = ["Default", "Blue", "Emerald", "Crimson", "Amber", "Violet"];

fn selected_theme_label(selected: Option<usize>) -> &'static str {
    selected.and_then(|index| THEME_MODE_LABELS.get(index).copied()).unwrap_or("System")
}

fn selected_language_label(selected: Option<usize>) -> &'static str {
    selected.and_then(|index| LANGUAGE_LABELS.get(index).copied()).unwrap_or("English")
}

fn selected_direction_label(direction: Direction) -> &'static str {
    match direction {
        Direction::RightToLeft => "RTL",
        Direction::Auto | Direction::LeftToRight => "LTR",
    }
}

fn selected_accent_label(selected: Option<usize>) -> &'static str {
    selected.and_then(|index| ACCENT_COLOR_LABELS.get(index).copied()).unwrap_or("Default")
}

fn controls_menu_row<'a, R>(
    cx: &'a mut Context,
    title: &'static str,
    value: R,
) -> Handle<'a, HStack>
where
    R: Res<String> + Clone + 'static,
{
    HStack::new(cx, move |cx| {
        Label::new(cx, title);
        Label::new(cx, value).class("control-menu-value");
    })
    .class("control-menu-row")
}

fn controls_menu(cx: &mut Context, app_data: AppData) {
    Submenu::new(
        cx,
        |cx| {
            HStack::new(cx, |cx| {
                Svg::new(cx, ICON_SETTINGS).class("icon");
                Label::new(cx, "Controls");
            })
        },
        move |cx| {
            MenuButton::new(
                cx,
                |cx| cx.emit(AppEvent::ToggleDisabled),
                move |cx| {
                    controls_menu_row(
                        cx,
                        "Disabled",
                        app_data.disabled.map(|disabled| {
                            if *disabled { "On".to_string() } else { "Off".to_string() }
                        }),
                    )
                },
            );

            Divider::new(cx);

            Submenu::new(
                cx,
                move |cx| {
                    controls_menu_row(
                        cx,
                        "Theme",
                        app_data
                            .selected_theme
                            .map(|selected| selected_theme_label(*selected).to_string()),
                    )
                },
                move |cx| {
                    for (index, label) in THEME_MODE_LABELS.iter().copied().enumerate() {
                        let selected_theme = app_data.selected_theme;
                        MenuButton::new(
                            cx,
                            move |cx| cx.emit(AppEvent::SetThemeMode(index)),
                            move |cx| {
                                controls_menu_row(
                                    cx,
                                    label,
                                    selected_theme.map(move |selected| {
                                        if *selected == Some(index) {
                                            "Current".to_string()
                                        } else {
                                            String::new()
                                        }
                                    }),
                                )
                            },
                        );
                    }
                },
            );

            Submenu::new(
                cx,
                move |cx| {
                    controls_menu_row(
                        cx,
                        "Language",
                        app_data
                            .selected_language
                            .map(|selected| selected_language_label(*selected).to_string()),
                    )
                },
                move |cx| {
                    for (index, label) in LANGUAGE_LABELS.iter().copied().enumerate() {
                        let selected_language = app_data.selected_language;
                        MenuButton::new(
                            cx,
                            move |cx| cx.emit(AppEvent::SetLanguage(index)),
                            move |cx| {
                                controls_menu_row(
                                    cx,
                                    label,
                                    selected_language.map(move |selected| {
                                        if *selected == Some(index) {
                                            "Current".to_string()
                                        } else {
                                            String::new()
                                        }
                                    }),
                                )
                            },
                        );
                    }
                },
            );

            Submenu::new(
                cx,
                move |cx| {
                    controls_menu_row(
                        cx,
                        "Direction",
                        cx.environment()
                            .direction
                            .map(|direction| selected_direction_label(*direction).to_string()),
                    )
                },
                move |cx| {
                    for (index, label) in DIRECTION_LABELS.iter().copied().enumerate() {
                        MenuButton::new(
                            cx,
                            move |cx| cx.emit(AppEvent::SetDirection(index)),
                            move |cx| {
                                controls_menu_row(
                                    cx,
                                    label,
                                    cx.environment().direction.map(move |direction| {
                                        let selected_index = match direction {
                                            Direction::RightToLeft => 1,
                                            Direction::Auto | Direction::LeftToRight => 0,
                                        };

                                        if selected_index == index {
                                            "Current".to_string()
                                        } else {
                                            String::new()
                                        }
                                    }),
                                )
                            },
                        );
                    }
                },
            );

            Submenu::new(
                cx,
                move |cx| {
                    controls_menu_row(
                        cx,
                        "Accent Color",
                        app_data
                            .selected_primary_color
                            .map(|selected| selected_accent_label(*selected).to_string()),
                    )
                },
                move |cx| {
                    for (index, label) in ACCENT_COLOR_LABELS.iter().copied().enumerate() {
                        let selected_primary_color = app_data.selected_primary_color;
                        MenuButton::new(
                            cx,
                            move |cx| cx.emit(AppEvent::SetPrimaryThemeColor(index)),
                            move |cx| {
                                controls_menu_row(
                                    cx,
                                    label,
                                    selected_primary_color.map(move |selected| {
                                        if *selected == Some(index) {
                                            "Current".to_string()
                                        } else {
                                            String::new()
                                        }
                                    }),
                                )
                            },
                        );
                    }
                },
            );
        },
    )
    .class("controls-menu-button")
    .width(Auto)
    .tooltip(|cx| {
        Tooltip::new(cx, |cx| {
            Label::new(cx, "Gallery controls");
        })
    });
}

fn build_sidebar_content(
    cx: &mut Context,
    selected_view: Signal<&'static str>,
    search_text: Signal<String>,
) {
    Binding::new(cx, search_text, move |cx| {
        let query = search_text.get().to_lowercase();
        let query = query.trim().to_string();

        let mut matching_items: Vec<&'static str> = std::iter::once("All")
            .chain(CATEGORIES.iter().flat_map(|(_, items)| items.iter().copied()))
            .filter(|item| query.is_empty() || item.to_lowercase().contains(&query))
            .collect();

        matching_items.sort_unstable();

        for item in matching_items.iter().copied() {
            let selected_view = selected_view;
            Button::new(cx, move |cx| Label::new(cx, item).hoverable(false))
                .class("sidebar-menu-button")
                .toggle_class(
                    "sidebar-menu-button-active",
                    selected_view.map(move |sv| *sv == item),
                )
                .on_press(move |cx| cx.emit(AppEvent::SelectView(item)));
        }

        if matching_items.is_empty() {
            Label::new(cx, "No widgets match your search.").class("sidebar-empty-state");
        }
    });
}

fn all_views_page(cx: &mut Context) {
    let mut all_views: Vec<(&'static str, &'static str)> = CATEGORIES
        .iter()
        .flat_map(|(category, items)| {
            items.iter().copied().map(move |view_name| (*category, view_name))
        })
        .collect();

    all_views.sort_unstable_by_key(|(_, view_name)| *view_name);

    VStack::new(cx, |cx| {
        HStack::new(cx, move |cx| {
            for (category, view_name) in all_views.iter().copied() {
                Card::new(cx, move |cx| {
                    CardHeader::new(cx, |cx| {
                        Label::new(cx, view_name).class("all-view-card-title");
                        Label::new(cx, category).class("all-view-card-category");
                    })
                    .height(Auto);

                    CardContent::new(cx, move |cx| {
                        render_view_preview(cx, view_name);
                    })
                    .size(Stretch(1.0))
                    .alignment(Alignment::Center);
                })
                .class("all-view-card")
                .width(Pixels(180.0))
                .height(Pixels(180.0));
            }
        })
        .class("all-view-grid")
        .height(Auto)
        .width(Stretch(1.0))
        .wrap(LayoutWrap::Wrap)
        .gap(Pixels(12.0));
    })
    .max_width(Pixels(2000.0))
    .class("panel");
}

#[derive(Clone, PartialEq)]
struct PreviewTableRow {
    id: u32,
    name: String,
}

fn render_view_preview(cx: &mut Context, view_name: &'static str) {
    VStack::new(cx, |cx| match view_name {
        "Accordion" => {
            let items = Signal::new(vec![
                ("Section 1".to_string(), "Accordion content 1".to_string()),
                ("Section 2".to_string(), "Accordion content 2".to_string()),
                ("Section 3".to_string(), "Accordion content 3".to_string()),
            ]);
            let open_indices = Signal::new(vec![0usize]);
            Accordion::new(cx, items, |_cx, _index, item| {
                let header = item.0;
                let content = item.1;
                AccordionPair::new(
                    move |cx| {
                        Label::new(cx, header.clone()).hoverable(false);
                    },
                    move |cx| {
                        Label::new(cx, content.clone()).hoverable(false);
                    },
                )
            })
            .open(open_indices)
            .width(Stretch(1.0));
        }
        "Avatar" => {
            Avatar::new(cx, |cx| {
                Image::new(cx, "vizia.png");
            });
        }
        "Avatar Group" => {
            AvatarGroup::new(cx, |cx| {
                Avatar::new(cx, |cx| {
                    Label::new(cx, "A");
                });
                Avatar::new(cx, |cx| {
                    Label::new(cx, "B");
                });
                Avatar::new(cx, |cx| {
                    Label::new(cx, "C");
                });
                Avatar::new(cx, |cx| {
                    Label::new(cx, "D");
                });
            })
            .max_visible(3);
        }
        "Badge" => {
            Avatar::new(cx, |cx| {
                Svg::new(cx, ICON_USER);
            })
            .badge(|cx| {
                Badge::new(cx, |cx| Svg::new(cx, ICON_CLOCK))
                    .class("warning")
                    .placement(BadgePlacement::BottomRight)
            });
        }
        "Button" => {
            Button::new(cx, |cx| Label::new(cx, "Button"));
        }
        "Button Group" => {
            let one = Signal::new(false);
            let two = Signal::new(false);
            let three = Signal::new(false);

            ButtonGroup::new(cx, |cx| {
                ToggleButton::new(cx, one, |cx| {
                    Svg::new(cx, ICON_BOLD).direction(Direction::LeftToRight)
                })
                .on_toggle(move |_cx| one.update(|value| *value ^= true));
                ToggleButton::new(cx, two, |cx| {
                    Svg::new(cx, ICON_ITALIC).direction(Direction::LeftToRight)
                })
                .on_toggle(move |_cx| two.update(|value| *value ^= true));
                ToggleButton::new(cx, three, |cx| {
                    Svg::new(cx, ICON_UNDERLINE).direction(Direction::LeftToRight)
                })
                .on_toggle(move |_cx| three.update(|value| *value ^= true));
            });
        }
        "Calendar" => {
            //let date = Signal::new(Utc::now().date_naive());
            //Calendar::new(cx, date).on_select(move |_cx, d| date.set(d));
        }
        "Card" => {
            Card::new(cx, |cx| {
                CardHeader::new(cx, |cx| {
                    Label::new(cx, "Card Header");
                });
                CardContent::new(cx, |cx| {
                    Label::new(cx, "Card content");
                });
            })
            .width(Stretch(1.0));
        }
        "Checkbox" => {
            let checked = Signal::new(true);
            Checkbox::new(cx, checked).on_toggle(move |_cx| checked.update(|v| *v ^= true));
        }
        "Chip" => {
            Chip::new(cx, "Chip");
        }
        "Collapsible" => {
            Collapsible::new(
                cx,
                |cx| {
                    Label::new(cx, "Section").hoverable(false);
                },
                |cx| {
                    Label::new(cx, "Collapsible content").hoverable(false);
                },
            )
            .width(Stretch(1.0));
        }
        "Combobox" => {
            let options = Signal::new(vec!["One", "Two", "Three"]);
            let selected = Signal::new(0usize);
            ComboBox::new(cx, options, selected)
                .on_select(move |_cx, index| selected.set(index))
                .width(Pixels(120.0));
        }
        "Divider" => {
            Divider::new(cx).width(Stretch(1.0));
        }
        "Dropdown" => {
            let list =
                Signal::new(vec![Signal::new("Red".to_string()), Signal::new("Blue".to_string())]);
            let selected = Signal::new(0usize);
            let choice = Signal::new("Red".to_string());
            Dropdown::new(
                cx,
                move |cx| {
                    Button::new(cx, |cx| Label::new(cx, choice))
                        .on_press(|cx| cx.emit(PopupEvent::Switch));
                },
                move |cx| {
                    List::new(cx, list, |cx, _, item| {
                        Label::new(cx, item).hoverable(false);
                    })
                    .selectable(Selectable::Single)
                    .on_select(move |cx, index| {
                        selected.set(index);
                        if let Some(current) = list.get().get(index).map(|s| s.get()) {
                            choice.set(current);
                        }
                        cx.emit(PopupEvent::Close);
                    });
                },
            )
            .width(Pixels(120.0));
            let _ = selected;
        }
        "Element" => {
            Element::new(cx).size(Pixels(64.0)).background_color(Color::rgb(58, 134, 255));
        }
        "Grid" => {
            Grid::new(cx, vec![Stretch(1.0), Stretch(1.0)], vec![Pixels(60.0)], |cx| {
                Label::new(cx, "A")
                    .column_start(0)
                    .row_start(0)
                    .background_color(Color::red())
                    .color(Color::white())
                    .alignment(Alignment::Center);
                Label::new(cx, "B")
                    .column_start(1)
                    .row_start(0)
                    .background_color(Color::blue())
                    .color(Color::white())
                    .alignment(Alignment::Center);
            })
            .width(Stretch(1.0))
            .height(Pixels(60.0));
        }
        "HStack" => {
            HStack::new(cx, |cx| {
                Element::new(cx).size(Pixels(30.0)).background_color(Color::red());
                Element::new(cx).size(Pixels(30.0)).background_color(Color::green());
            })
            .height(Auto)
            .gap(Pixels(6.0));
        }
        "Image" => {
            Label::new(cx, "Coming soon...");
        }
        "Knob" => {
            let value = Signal::new(0.3f32);
            Knob::new(cx, 0.5, value, false).on_change(move |_cx, v| value.set(v));
        }
        "Label" => {
            Label::new(cx, "Hello Vizia");
        }
        "List" => {
            let list = Signal::new(vec![Signal::new(1u32), Signal::new(2u32), Signal::new(3u32)]);
            List::new(cx, list, |cx, _, item| {
                Label::new(cx, item).width(Stretch(1.0)).height(Pixels(24.0));
            })
            .size(Pixels(160.0));
        }
        "Markdown" => {
            Markdown::new(cx, "**Markdown** preview");
        }
        "Menu" => {
            Submenu::new(
                cx,
                |cx| Label::new(cx, "Menu"),
                |cx| {
                    MenuButton::new(cx, |_| {}, |cx| Label::new(cx, "Item"));
                },
            )
            .width(Pixels(120.0));
        }
        "MenuBar" => {
            MenuBar::new(cx, |cx| {
                Submenu::new(
                    cx,
                    |cx| Label::new(cx, "File"),
                    |cx| {
                        MenuButton::new(cx, |_| {}, |cx| Label::new(cx, "Open"));
                    },
                );
            })
            .width(Stretch(1.0));
        }
        "Popup" => {
            let is_open = Signal::new(false);
            HStack::new(cx, move |cx| {
                Button::new(cx, |cx| Label::new(cx, "Open")).on_press(move |_cx| is_open.set(true));
                Binding::new(cx, is_open, move |cx| {
                    if is_open.get() {
                        Popover::new(cx, |cx| {
                            Label::new(cx, "Popup Content").padding(Pixels(8.0));
                        })
                        .on_blur(move |_cx| is_open.set(false))
                        .placement(Placement::Bottom)
                        .show_arrow(true);
                    }
                });
            })
            .size(Auto);
        }
        "Progressbar" => {
            let progress = Signal::new(0.6f32);
            ProgressBar::horizontal(cx, progress).width(Pixels(180.0));
        }
        "Radiobutton" => {
            let selected = Signal::new(true);
            RadioButton::new(cx, selected).on_select(move |_cx| selected.set(true));
        }
        "Rating" => {
            let rating = Signal::new(3u32);
            Rating::new(cx, 5, rating).on_change(move |_cx, v| rating.set(v));
        }
        "Resizable" => {
            let width = Signal::new(Pixels(140.0));
            Resizable::new(
                cx,
                width,
                ResizeStackDirection::Right,
                move |_cx, w| width.set(Pixels(w)),
                |cx| {
                    Element::new(cx).size(Stretch(1.0)).background_color(Color::rgb(120, 193, 243));
                },
            )
            .height(Pixels(60.0));
        }
        "Scrollview" => {
            ScrollView::new(cx, |cx| {
                Label::new(cx, "Scrollable").height(Pixels(220.0)).width(Stretch(1.0));
            })
            .size(Pixels(160.0));
        }
        "Select" => {
            let options = Signal::new(["Red", "Green", "Blue"].map(Signal::new).to_vec());
            let selected = Signal::new(Some(0usize));
            Select::new(cx, options, selected, true)
                .on_select(move |_cx, index| selected.set(Some(index)))
                .width(Pixels(130.0));
        }
        "Slider" => {
            let value = Signal::new(0.5f32);
            Slider::new(cx, value).on_change(move |_cx, v| value.set(v)).width(Pixels(180.0));
        }
        "Spinbox" => {
            let value = Signal::new(10.0f64);
            Spinbox::new(cx, value)
                .on_increment(move |_cx| value.update(|v| *v += 1.0))
                .on_decrement(move |_cx| value.update(|v| *v -= 1.0))
                .width(Pixels(110.0));
        }
        "Svg" => {
            Svg::new(cx, ICON_USER);
        }
        "Switch" => {
            let enabled = Signal::new(true);
            Switch::new(cx, enabled).on_toggle(move |_cx| enabled.update(|v| *v ^= true));
        }
        "Table" => {
            let rows = Signal::new(vec![PreviewTableRow { id: 1, name: "Button".to_string() }]);
            let columns: Signal<Vec<TableColumn<PreviewTableRow, TableHeader>>> =
                Signal::new(vec![
                    TableColumn::new(
                        "name",
                        |cx, sort_dir| TableHeader::new(cx, "Name", sort_dir),
                        |cx, row| {
                            let text = row.map(|r: &PreviewTableRow| r.name.clone());
                            Label::new(cx, text);
                        },
                    )
                    .width(160.0),
                ]);
            Table::new(cx, rows, columns, |row: &PreviewTableRow| row.id)
                .width(Stretch(1.0))
                .height(Pixels(120.0));
        }
        "Tabview" => {
            let tabs = Signal::new(vec!["Tab1", "Tab2"]);
            let selected = Signal::new(0usize);
            TabView::new(cx, tabs, |_, _, item| match item {
                "Tab1" => TabPair::new(
                    move |cx| {
                        Label::new(cx, item).hoverable(false);
                    },
                    |cx| {
                        Label::new(cx, "Tab one").height(Stretch(1.0)).alignment(Alignment::Center);
                    },
                ),
                "Tab2" => TabPair::new(
                    move |cx| {
                        Label::new(cx, item).hoverable(false);
                    },
                    |cx| {
                        Label::new(cx, "Tab two").height(Stretch(1.0)).alignment(Alignment::Center);
                    },
                ),
                _ => unreachable!(),
            })
            .with_selected(selected)
            .on_select(move |_cx, index| selected.set(index))
            .width(Pixels(220.0))
            .height(Pixels(120.0));
        }
        "Textbox" => {
            let text = Signal::new("Hello".to_string());
            Textbox::new(cx, text)
                .on_submit(move |_cx, value, _| text.set(value.clone()))
                .width(Pixels(150.0));
        }
        "ToggleButton" => {
            let toggled = Signal::new(false);
            ToggleButton::new(cx, toggled, |cx| Label::new(cx, "Bold"))
                .on_toggle(move |_cx| toggled.update(|v| *v ^= true));
        }
        "Tooltip" => {
            Button::new(cx, |cx| Label::new(cx, "Hover")).tooltip(|cx| {
                Tooltip::new(cx, |cx| {
                    Label::new(cx, "Tooltip");
                })
            });
        }
        "VirtualList" => {
            let list = Signal::new((1..40u32).collect::<Vec<_>>());
            let selected = Signal::new(vec![0usize]);
            VirtualList::new(cx, list, 24.0, |cx, _, item| Label::new(cx, item))
                .selection(selected)
                .on_select(move |_cx, index| selected.set(vec![index]))
                .size(Pixels(180.0));
        }
        "VirtualTable" => {
            let rows = Signal::new(
                (0u32..50)
                    .map(|id| PreviewTableRow { id, name: format!("Widget {}", id) })
                    .collect::<Vec<_>>(),
            );
            let columns: Signal<Vec<TableColumn<PreviewTableRow, TableHeader>>> =
                Signal::new(vec![
                    TableColumn::new(
                        "name",
                        |cx, sort_dir| TableHeader::new(cx, "Name", sort_dir),
                        |cx, row| {
                            let text = row.map(|r: &PreviewTableRow| r.name.clone());
                            Label::new(cx, text);
                        },
                    )
                    .width(180.0),
                ]);
            VirtualTable::new(cx, rows, columns, 24.0, |row: &PreviewTableRow| row.id)
                .width(Stretch(1.0))
                .height(Pixels(120.0));
        }
        "VStack" => {
            VStack::new(cx, |cx| {
                Element::new(cx).size(Pixels(24.0)).background_color(Color::red());
                Element::new(cx).size(Pixels(24.0)).background_color(Color::green());
            })
            .height(Auto)
            .gap(Pixels(6.0));
        }
        "XYPad" => {
            let xy = Signal::new((0.5f32, 0.5f32));
            XYPad::new(cx, xy).on_change(move |_cx, x, y| xy.set((x, y))).size(Pixels(140.0));
        }
        "ZStack" => {
            ZStack::new(cx, |cx| {
                Element::new(cx).size(Pixels(44.0)).background_color(Color::red());
                Element::new(cx).size(Pixels(36.0)).background_color(Color::blue());
            })
            .size(Auto);
        }
        _ => {
            Label::new(cx, "No preview");
        }
    })
    .class("all-view-preview")
    .alignment(Alignment::Center)
    .height(Auto)
    .width(Stretch(1.0));
}

fn render_view_page(cx: &mut Context, view_name: &'static str) {
    match view_name {
        "All" => all_views_page(cx),
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
    }
}

fn content_area(cx: &mut Context, selected_view: Signal<&'static str>) {
    ScrollView::new(cx, move |cx| {
        Binding::new(cx, selected_view, move |cx| {
            let current_view = selected_view.get();
            VStack::new(cx, |cx| {
                render_view_page(cx, current_view);
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

        cx.load_image(
            "vizia.png",
            include_bytes!("../resources/images/vizia-logo-01.png"),
            ImageRetentionPolicy::Forever,
        );

        let app_data = AppData::new();
        app_data.build(cx);

        cx.add_stylesheet(include_style!("src/style.css")).expect("Failed to add stylesheet");

        VStack::new(cx, |cx| {
            HStack::new(cx, |cx| {
                Sidebar::new(
                    cx,
                    move |cx| {
                        VStack::new(cx, |cx| {
                            HStack::new(cx, |cx| {
                                Image::new(cx, "vizia.png").class("sidebar-logo");
                                Label::new(cx, "Widget Gallery").class("sidebar-title");
                            })
                            .size(Auto)
                            .alignment(Alignment::Left);

                            Textbox::new(cx, app_data.search_text)
                                .on_edit(|cx, text| cx.emit(AppEvent::SetSearchText(text)))
                                .placeholder("Search widgets...")
                                .class("sidebar-search");
                        })
                        .height(Auto)
                        .gap(Pixels(8.0));
                    },
                    move |cx| {
                        build_sidebar_content(cx, app_data.selected_view, app_data.search_text)
                    },
                    move |_cx| {},
                );
                VStack::new(cx, |cx| {
                    HStack::new(cx, |cx| {
                        controls_menu(cx, app_data);
                    })
                    .class("gallery-toolbar")
                    .alignment(Alignment::Right)
                    .padding(Pixels(12.0))
                    .width(Stretch(1.0))
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
