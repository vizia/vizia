use vizia::prelude::*;

use crate::categories::Category;

#[derive(Debug, Lens)]
pub struct AppData {
    pub categories: Vec<Category>,
}

impl AppData {
    pub fn new() -> Self {
        Self {
            categories: vec![
                Category::All,
                Category::Latest,
                Category::Animals,
                Category::Arrows,
                Category::Brand,
                Category::Buildings,
                Category::Charts,
                Category::Communication,
                Category::Computers,
                Category::Currencies,
                Category::Database,
                Category::Design,
                Category::Devices,
                Category::Document,
                Category::ECommerce,
                Category::Electrical,
                Category::Extensions,
                Category::Filled,
                Category::Food,
                Category::Gender,
                Category::Gestures,
                Category::Health,
                Category::Laundry,
                Category::Letters,
                Category::Logic,
                Category::Map,
                Category::Math,
                Category::Media,
                Category::Mood,
                Category::Nature,
                Category::Numbers,
                Category::Photography,
                Category::Shapes,
                Category::Sport,
                Category::Symbols,
                Category::System,
                Category::Text,
                Category::Vehicles,
                Category::VersionControl,
                Category::Weather,
                Category::Zodiac,
            ],
        }
    }
}

impl Model for AppData {}
