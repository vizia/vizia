use vizia_derive::Lens;

#[cfg(feature = "localization")]
use unic_langid::LanguageIdentifier;

use crate::{context::EventContext, events::Event, state::Lens, state::Model};

#[derive(Lens)]
pub struct Environment {
    #[cfg(feature = "localization")]
    pub locale: LanguageIdentifier,
}

impl Default for Environment {
    fn default() -> Self {
        Environment::new()
    }
}

impl Environment {
    pub fn new() -> Self {
        #[cfg(feature = "localization")]
        let locale = sys_locale::get_locale().map(|l| l.parse().ok()).flatten().unwrap_or_default();

        Self {
            #[cfg(feature = "localization")]
            locale,
        }
    }
}

pub enum EnvironmentEvent {
    #[cfg(feature = "localization")]
    SetLocale(LanguageIdentifier),
    #[cfg(feature = "localization")]
    UseSystemLocale,
    None,
}

impl Model for Environment {
    fn event(&mut self, _: &mut EventContext, event: &mut Event) {
        event.map(|event, _| match event {
            #[cfg(feature = "localization")]
            EnvironmentEvent::SetLocale(locale) => {
                self.locale = locale.clone();
            }

            #[cfg(feature = "localization")]
            EnvironmentEvent::UseSystemLocale => {
                self.locale =
                    sys_locale::get_locale().map(|l| l.parse().unwrap()).unwrap_or_default();
            }

            EnvironmentEvent::None => {}
        });
    }
}
