use vizia::prelude::*;

pub struct AppData {
    pub theme_options: Signal<Vec<Signal<&'static str>>>,
    pub selected_theme: Signal<usize>,
    // pub disabled: bool,
    pub tabs: Signal<Vec<&'static str>>,
}

pub enum AppEvent {
    SetThemeMode(usize),
}

impl Model for AppData {
    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|app_event, _| match app_event {
            AppEvent::SetThemeMode(theme_mode) => {
                self.selected_theme.set(*theme_mode);
                cx.emit(EnvironmentEvent::SetThemeMode(match theme_mode {
                    0 /* system */ => AppTheme::System,
                    1 /* Dark */ => AppTheme::BuiltIn(ThemeMode::DarkMode),
                    2 /* Light */ => AppTheme::BuiltIn(ThemeMode::LightMode),
                    _ => unreachable!(),
                }));
            }
        });
    }
}

impl AppData {
    pub fn new() -> Self {
        AppData {
            theme_options: Signal::new(["System", "Dark", "Light"].map(Signal::new).to_vec()),
            selected_theme: Signal::new(0),
            // disabled: false,
            tabs: Signal::new(vec![
                "Avatar",
                "Avatar Group",
                "Badge",
                "Button",
                "Button Group",
                "Checkbox",
                "Chip",
                "Combobox",
                "Datepicker",
                "Divider",
                "Dropdown",
                "Element",
                "HStack",
                "Image",
                "Knob",
                "Label",
                "List",
                "Menu",
                "MenuBar",
                "Picklist",
                "Progressbar",
                "Radiobutton",
                "Rating",
                "Scrollview",
                "Slider",
                "Spinbox",
                "Svg",
                "Switch",
                "Tabview",
                "Textbox",
                "ToggleButton",
                "Tooltip",
                "VirtualList",
                "VStack",
                "ZStack",
            ]),
        }
    }
}
