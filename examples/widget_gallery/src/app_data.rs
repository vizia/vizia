use vizia::prelude::*;

pub const CATEGORIES: &[(&str, &[&str])] = &[
    ("Layout", &["Grid", "HStack", "Resizable", "VStack", "ZStack"]),
    (
        "Display",
        &[
            "Avatar",
            "Avatar Group",
            "Badge",
            "Card",
            "Divider",
            "Element",
            "Image",
            "Label",
            "Markdown",
            "Svg",
        ],
    ),
    (
        "Input",
        &[
            "Button",
            "Button Group",
            "Calendar",
            "Checkbox",
            "Chip",
            "Combobox",
            "Dropdown",
            "Knob",
            "Radiobutton",
            "Rating",
            "Select",
            "Slider",
            "Spinbox",
            "Switch",
            "Textbox",
            "ToggleButton",
            "XYPad",
        ],
    ),
    ("Navigation", &["Menu", "MenuBar", "Scrollview", "Tabview"]),
    ("Data", &["List", "Table", "VirtualList", "VirtualTable"]),
    ("Feedback", &["Popup", "Progressbar", "Tooltip"]),
    ("Containers", &["Accordion", "Collapsible"]),
];

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
            selected_view: Signal::new("All"),
            search_text: Signal::new(String::new()),
        }
    }
}
