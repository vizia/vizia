use vizia::prelude::*;

#[derive(Lens)]
pub struct AppData {
    pub tabs: Vec<&'static str>,
}

impl Model for AppData {}

impl AppData {
    pub fn new() -> Self {
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
    }
}
