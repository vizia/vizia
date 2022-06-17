use crate::context::Context;

/// An entry inside of a [`Keymap`](crate::prelude::Keymap).
///
/// It consists of an action which is usually just an enum variant
/// and a callback function that gets called if the action got triggered.
///
/// This type is part of the prelude.
#[derive(Copy, Clone)]
pub struct KeymapEntry<T>
where
    T: 'static + Copy + Clone + PartialEq + Send + Sync,
{
    action: T,
    on_action: fn(&mut Context),
}

impl<T> KeymapEntry<T>
where
    T: 'static + Copy + Clone + PartialEq + Send + Sync,
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
    pub fn new(action: T, on_action: fn(&mut Context)) -> Self {
        Self { action, on_action }
    }

    /// Returns the action of the keymap entry.
    pub fn action(&self) -> &T {
        &self.action
    }

    /// Returns the `on_action` callback function of the keymap entry.
    pub fn on_action(&self) -> &fn(&mut Context) {
        &self.on_action
    }
}

impl<T> PartialEq for KeymapEntry<T>
where
    T: 'static + Copy + Clone + PartialEq + Send + Sync,
{
    fn eq(&self, other: &Self) -> bool {
        self.action == other.action
    }
}

impl<T> PartialEq<T> for KeymapEntry<T>
where
    T: 'static + Copy + Clone + PartialEq + Send + Sync,
{
    fn eq(&self, other: &T) -> bool {
        self.action == *other
    }
}
