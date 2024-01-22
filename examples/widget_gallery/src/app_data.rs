use vizia::prelude::*;

#[derive(Lens)]
pub struct AppData {
    pub theme_options: Vec<&'static str>,
    pub selected_theme: usize,
    pub disabled: bool,
    pub tabs: Vec<&'static str>,
}

pub enum AppEvent {
    ToggleDisabled,
    SetThemeMode(usize),
}

impl Model for AppData {
    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|app_event, _| match app_event {
            AppEvent::ToggleDisabled => {
                self.disabled ^= true;
            }
            AppEvent::SetThemeMode(theme_mode) => {
                self.selected_theme = *theme_mode;
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
            theme_options: vec!["System", "Dark", "Light"],
            selected_theme: 0,
            disabled: false,
            tabs: vec![
                "All",
                "Avatar",
                "Badge",
                "Button",
                "Button Group",
                "Checkbox",
                "Chip",
                "Combobox",
                "Datepicker",
                "Dialog",
                "Divider",
                "Dropdown",
                "Element",
                "Form",
                "HStack",
                "Icon",
                "Image",
                "Knob",
                "Label",
                "List",
                "Menu",
                "MenuBar",
                "Notification",
                "Picklist",
                "Popup",
                "Progressbar",
                "Radiobutton",
                "Rating",
                "Scrollview",
                "Slider",
                "Spinbox",
                "Switch",
                "Tabview",
                "Textbox",
                "Timepicker",
                "ToggleButton",
                "Tooltip",
                "VirtualList",
                "VStack",
                "ZStack",
            ],
        }
    }
}
