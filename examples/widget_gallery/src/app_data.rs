use vizia::prelude::*;

pub const ALL_VIEW_ID: &str = "All";
pub const ALL_VIEW_KEY: &str = "all";

pub const CATEGORIES: &[(&str, &[(&str, &str)])] = &[
    (
        "category-layout",
        &[
            ("Grid", "grid"),
            ("HStack", "hstack"),
            ("Resizable", "resizable"),
            ("VStack", "vstack"),
            ("ZStack", "zstack"),
        ],
    ),
    (
        "category-display",
        &[
            ("Avatar", "avatar"),
            ("Avatar Group", "avatar-group"),
            ("Badge", "badge"),
            ("Card", "card"),
            ("Divider", "divider"),
            ("Element", "element"),
            ("Image", "image"),
            ("Label", "label"),
            ("Markdown", "markdown"),
            ("Svg", "svg"),
        ],
    ),
    (
        "category-input",
        &[
            ("Button", "button-title"),
            ("Button Group", "button-group"),
            ("Calendar", "calendar"),
            ("Checkbox", "checkbox"),
            ("Chip", "chip"),
            ("Combobox", "combobox"),
            ("Dropdown", "dropdown"),
            ("Knob", "knob"),
            ("Radiobutton", "radiobutton"),
            ("Rating", "rating"),
            ("Select", "select"),
            ("Slider", "slider"),
            ("Spinbox", "spinbox"),
            ("Switch", "switch"),
            ("Textbox", "textbox"),
            ("ToggleButton", "toggle-button"),
            ("XYPad", "xypad"),
        ],
    ),
    (
        "category-navigation",
        &[
            ("Menu", "menu"),
            ("MenuBar", "menu-bar"),
            ("Scrollview", "scrollview"),
            ("Tabview", "tabview"),
        ],
    ),
    (
        "category-data",
        &[
            ("List", "list"),
            ("Table", "table"),
            ("VirtualList", "virtual-list"),
            ("VirtualTable", "virtual-table"),
        ],
    ),
    (
        "category-feedback",
        &[("Popup", "popup"), ("Progressbar", "progress-bar"), ("Tooltip", "tooltip")],
    ),
    ("category-containers", &[("Accordion", "accordion"), ("Collapsible", "collapsible")]),
];

pub fn localized_view_key(view_id: &str) -> &'static str {
    if view_id == ALL_VIEW_ID {
        return ALL_VIEW_KEY;
    }

    for (_, items) in CATEGORIES.iter() {
        for (id, key) in items.iter().copied() {
            if id == view_id {
                return key;
            }
        }
    }

    ALL_VIEW_KEY
}

const PRIMARY_COLOR_CLASSES: [&str; 6] = [
    "default",
    "primary-blue",
    "primary-emerald",
    "primary-crimson",
    "primary-amber",
    "primary-violet",
];

#[derive(Clone, Copy)]
pub struct AppData {
    pub disabled: Signal<bool>,
    pub selected_theme: Signal<Option<usize>>,
    pub selected_language: Signal<Option<usize>>,
    pub selected_primary_color: Signal<Option<usize>>,
    pub selected_view: Signal<&'static str>,
    pub search_text: Signal<String>,
}

pub enum AppEvent {
    ToggleDisabled,
    SetThemeMode(usize),
    SetLanguage(usize),
    SetDirection(usize),
    SetPrimaryThemeColor(usize),
    SelectView(&'static str),
    SetSearchText(String),
}

impl Model for AppData {
    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|app_event, _| match app_event {
            AppEvent::ToggleDisabled => {
                self.disabled.update(|disabled| *disabled ^= true);
            }
            AppEvent::SetThemeMode(theme_mode) => {
                self.selected_theme.set(Some(*theme_mode));
                cx.emit(EnvironmentEvent::SetThemeMode(match theme_mode {
                    0 /* system */ => ThemeMode::System,
                    1 /* Dark */ => ThemeMode::DarkMode,
                    2 /* Light */ => ThemeMode::LightMode,
                    _ => unreachable!(),
                }));
            }
            AppEvent::SetLanguage(language) => {
                self.selected_language.set(Some(*language));
                cx.emit(EnvironmentEvent::SetLocale(match language {
                    0 /* English */ => langid!("en-US"),
                    1 /* French */ => langid!("fr"),
                    2 /* Arabic */ => langid!("ar"),
                    _ => unreachable!(),
                }));
            }
            AppEvent::SetDirection(direction) => {
                cx.emit(EnvironmentEvent::SetDirection(match direction {
                    0 /* LTR */ => Direction::LeftToRight,
                    1 /* RTL */ => Direction::RightToLeft,
                    _ => unreachable!(),
                }));
            }
            AppEvent::SetPrimaryThemeColor(color) => {
                self.selected_primary_color.set(Some(*color));

                cx.with_current(Entity::root(), |cx| {
                    for (index, class) in PRIMARY_COLOR_CLASSES.iter().enumerate() {
                        cx.toggle_class(class, index == *color);
                    }
                });
            }
            AppEvent::SelectView(name) => {
                self.selected_view.set(name);
            }
            AppEvent::SetSearchText(text) => {
                self.search_text.set(text.clone());
            }
        });
    }
}

impl AppData {
    pub fn new() -> Self {
        AppData {
            disabled: Signal::new(false),
            selected_theme: Signal::new(Some(0)),
            selected_language: Signal::new(Some(0)),
            selected_primary_color: Signal::new(Some(0)),
            selected_view: Signal::new(ALL_VIEW_ID),
            search_text: Signal::new(String::new()),
        }
    }
}
