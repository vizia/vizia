use crate::context::EventContext;

/// An entry inside of a [`Keymap`](crate::prelude::Keymap).
///
/// It consists of an action which is usually just an enum variant
/// and a callback function that gets called if the action got triggered.
#[derive(Copy, Clone)]
pub struct KeymapEntry<T>
where
    T: 'static + Clone + PartialEq + Send + Sync,
{
    action: T,
    on_action: fn(&mut EventContext),
}

impl<T> KeymapEntry<T>
where
    T: 'static + Clone + PartialEq + Send + Sync,
{
    /// Creates a new keymap entry.
    ///
    /// # Examples
    ///
    /// ```
    /// # use vizia_core::prelude::*;
    /// #
    /// # #[derive(Copy, Clone, PartialEq)]
    /// # enum Action {
    /// #     One,
    /// # }
    /// #
    /// KeymapEntry::new(Action::One, |_| println!("Action One"));
    /// ```
    pub fn new(action: T, on_action: fn(&mut EventContext)) -> Self {
        Self { action, on_action }
    }

    /// Returns the action of the keymap entry.
    pub fn action(&self) -> &T {
        &self.action
    }

    /// Returns the `on_action` callback function of the keymap entry.
    pub fn on_action(&self) -> &fn(&mut EventContext) {
        &self.on_action
    }
}

impl<T> PartialEq for KeymapEntry<T>
where
    T: 'static + Clone + PartialEq + Send + Sync,
{
    fn eq(&self, other: &Self) -> bool {
        self.action == other.action
    }
}

impl<T> PartialEq<T> for KeymapEntry<T>
where
    T: 'static + Clone + PartialEq + Send + Sync,
{
    fn eq(&self, other: &T) -> bool {
        self.action == *other
    }
}
