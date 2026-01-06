use vizia::prelude::*;

#[derive(Clone, Copy)]
pub struct AppData {
    pub theme_options: Signal<Vec<&'static str>>,
    pub selected_theme: Signal<usize>,
    pub tabs: Signal<Vec<&'static str>>,
}

impl AppData {
    pub fn new(cx: &mut Context) -> Self {
        Self {
            theme_options: cx.state(vec!["System", "Dark", "Light"]),
            selected_theme: cx.state(0usize),
            tabs: cx.state(vec![
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
