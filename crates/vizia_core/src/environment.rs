use crate::prelude::{LensValue, Wrapper};
use unic_langid::LanguageIdentifier;
use vizia_derive::Lens;

use crate::{
    context::EventContext,
    events::Event,
    state::Model,
    state::{Lens, StatelessLens},
};

#[derive(Lens)]
pub struct Environment {
    pub locale: LanguageIdentifier,
}

impl Default for Environment {
    fn default() -> Self {
        Environment::new()
    }
}

impl Environment {
    pub fn new() -> Self {
        let locale = sys_locale::get_locale().and_then(|l| l.parse().ok()).unwrap_or_default();

        Self { locale }
    }
}

pub enum EnvironmentEvent {
    SetLocale(LanguageIdentifier),
    UseSystemLocale,
}

impl Model for Environment {
    fn event(&mut self, _: &mut EventContext, event: &mut Event) {
        event.map(|event, _| match event {
            EnvironmentEvent::SetLocale(locale) => {
                self.locale = locale.clone();
            }

            EnvironmentEvent::UseSystemLocale => {
                self.locale =
                    sys_locale::get_locale().map(|l| l.parse().unwrap()).unwrap_or_default();
            }
        });
    }
}
