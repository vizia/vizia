use super::internal;
use crate::prelude::*;

/// Modifiers for changing the abilities of a view.
pub trait AbilityModifiers: internal::Modifiable {
    /// Sets whether the view can be hovered by the mouse.
    ///
    /// Views which cannot be hovered will not receive mouse input events unless
    /// the view has captured the mouse input, see [`cx.capture()`](crate::prelude::EventContext::capture).
    fn hoverable<U: Into<bool>>(mut self, state: impl Res<U>) -> Self {
        let entity = self.entity();
        state.set_or_bind(self.context(), entity, |cx, entity, v| {
            if let Some(abilities) = cx.style.abilities.get_mut(entity) {
                abilities.set(Abilities::HOVERABLE, v.into());
                cx.needs_restyle();
            }
        });

        self
    }

    /// Sets whether the view can be focused to receive keyboard input events.
    fn focusable<U: Into<bool>>(mut self, state: impl Res<U>) -> Self {
        let entity = self.entity();
        state.set_or_bind(self.context(), entity, |cx, entity, v| {
            if let Some(abilities) = cx.style.abilities.get_mut(entity) {
                let state = v.into();
                abilities.set(Abilities::FOCUSABLE, state);

                // If an element is not focusable then it can't be keyboard navigable.
                if !state {
                    abilities.set(Abilities::NAVIGABLE, false);
                }

                cx.needs_restyle();
            }
        });

        self
    }

    /// Sets whether the view can be checked.
    fn checkable<U: Into<bool>>(mut self, state: impl Res<U>) -> Self {
        let entity = self.entity();
        state.set_or_bind(self.context(), entity, |cx, entity, v| {
            if let Some(abilities) = cx.style.abilities.get_mut(entity) {
                let state = v.into();
                abilities.set(Abilities::CHECKABLE, state);

                cx.need_restyle();
            }
        });

        self
    }

    /// Sets whether the view can be navigated to, i.e. focused, by the keyboard.
    ///
    /// Navigating to a view with the keyboard gives the view keyboard focus and is typically done with `tab` and `shift + tab` key combinations.
    fn navigable<U: Into<bool>>(mut self, state: impl Res<U>) -> Self {
        let entity = self.entity();
        state.set_or_bind(self.context(), entity, |cx, entity, v| {
            if let Some(abilities) = cx.style.abilities.get_mut(entity) {
                abilities.set(Abilities::NAVIGABLE, v.into());
                cx.needs_restyle();
            }
        });

        self
    }
}

impl<'a, V> AbilityModifiers for Handle<'a, V> {}
