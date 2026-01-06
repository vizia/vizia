use super::internal;
use crate::prelude::*;

/// Modifiers for changing the abilities of a view.
pub trait AbilityModifiers: internal::Modifiable {
    /// Sets whether the view can be hovered by the mouse and receive mouse events.
    ///
    /// Accepts a signal to some boolean state.
    /// Views which cannot be hovered will not receive mouse input events unless
    /// the view has captured the mouse input, see [`cx.capture()`](crate::prelude::EventContext::capture).
    ///
    /// # Example
    /// ```
    /// # use vizia_core::prelude::*;
    /// # let cx = &mut Context::default();
    /// let text = cx.state("Hello Vizia");
    /// Label::new(cx, text)
    ///     .hoverable(false);
    /// ```
    fn hoverable(mut self, state: impl Res<bool> + 'static) -> Self {
        let entity = self.entity();
        let current = self.entity();
        internal::bind_res(self.context(), current, entity, state, move |cx, val| {
            if let Some(abilities) = cx.style.abilities.get_mut(entity) {
                abilities.set(Abilities::HOVERABLE, *val);
                cx.needs_restyle(entity);
            }
        });

        self
    }

    /// Sets whether the view can be focused to receive keyboard input events.
    ///
    /// Accepts a signal to some boolean state.
    /// # Example
    /// ```
    /// # use vizia_core::prelude::*;
    /// # let cx = &mut Context::default();
    /// let text = cx.state("Hello Vizia");
    /// Label::new(cx, text)
    ///     .focusable(false);
    /// ```
    fn focusable(mut self, state: impl Res<bool> + 'static) -> Self {
        let entity = self.entity();
        let current = self.current();
        internal::bind_res(self.context(), current, entity, state, move |cx, val| {
            if let Some(abilities) = cx.style.abilities.get_mut(entity) {
                abilities.set(Abilities::FOCUSABLE, *val);

                // If an element is not focusable then it can't be keyboard navigable.
                if !*val {
                    abilities.set(Abilities::NAVIGABLE, false);
                }

                cx.needs_restyle(entity);
            }
        });

        self
    }

    /// Sets whether the view can be checked.
    ///
    /// Accepts a signal to some boolean state.
    /// # Example
    /// ```
    /// # use vizia_core::prelude::*;
    /// # let cx = &mut Context::default();
    /// let text = cx.state("Hello Vizia");
    /// Label::new(cx, text)
    ///     .checkable(false);
    /// ```
    fn checkable(mut self, state: impl Res<bool> + 'static) -> Self {
        let entity = self.entity();
        let current = self.current();
        internal::bind_res(self.context(), current, entity, state, move |cx, val| {
            if let Some(abilities) = cx.style.abilities.get_mut(cx.current) {
                abilities.set(Abilities::CHECKABLE, *val);

                cx.needs_restyle(entity);
            }
        });

        self
    }

    /// Sets whether the view can be navigated to, i.e. focused, by the keyboard.
    ///
    /// Accepts a signal to some boolean state.
    /// Navigating to a view with the keyboard gives the view keyboard focus and is typically done with `tab` and `shift + tab` key combinations.
    /// # Example
    /// ```
    /// # use vizia_core::prelude::*;
    /// # let cx = &mut Context::default();
    /// let text = cx.state("Hello Vizia");
    /// Label::new(cx, text)
    ///     .checkable(false);
    /// ```
    fn navigable(mut self, state: impl Res<bool> + 'static) -> Self {
        let entity = self.entity();
        let current = self.current();
        internal::bind_res(self.context(), current, entity, state, move |cx, val| {
            if let Some(abilities) = cx.style.abilities.get_mut(entity) {
                abilities.set(Abilities::NAVIGABLE, *val);
                cx.needs_restyle(entity);
            }
        });

        self
    }
}

impl<V> AbilityModifiers for Handle<'_, V> {}
