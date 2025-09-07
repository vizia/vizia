use crate::{prelude::*, views::combo_box_derived_lenses::p};
use indexmap::IndexMap;

/// A keymap that associates key chords with actions.
///
/// This is useful if you have an application that lets the user configure their key chords.
/// It allows you to check if a particular action is pressed rather than the actual keys.
/// The relationship between a key chord and an action is a many-to-many relationship.
///
/// # Examples
///
/// First we need to create something that represents an action in our application.
/// This is usually an enum.
///
/// ```
/// #[derive(PartialEq, Copy, Clone)]
/// enum Action {
///     One,
///     Two,
///     Three,
/// }
/// ```
///
/// Now we can create a new keymap inside of our application and configure our key chords.
/// We will bind `Action::One` to the key chord `A`, `Action::Two` to the key chord `CTRL+B`
/// and `Action::Three` to the key chord `CTRL+SHIFT+C`. Every action has an associated callback
/// function that gets triggered when the key chord is pressed.
///
/// ```
/// # use vizia_core::prelude::*;
/// #
/// # #[derive(PartialEq, Copy, Clone)]
/// # enum Action {
/// #     One,
/// #     Two,
/// #     Three,
/// # }
/// #
/// let keymap = Keymap::from(vec![
///     (KeyChord::new(Modifiers::empty(), Code::KeyA), KeymapEntry::new(Action::One, |_| debug!("Action One"))),
///     (KeyChord::new(Modifiers::CTRL, Code::KeyB), KeymapEntry::new(Action::Two, |_| debug!("Action Two"))),
///     (KeyChord::new(Modifiers::CTRL | Modifiers::SHIFT, Code::KeyC), KeymapEntry::new(Action::Three, |_| debug!("Action Three"))),
/// ]);
/// ```
#[derive(Default)]
pub struct Keymap<T>
where
    T: 'static + Clone + PartialEq + Send + Sync,
{
    entries: IndexMap<KeyChord, Vec<KeymapEntry<T>>>,
}

impl<T> Keymap<T>
where
    T: 'static + Clone + PartialEq + Send + Sync,
{
    /// Creates a new keymap.
    ///
    /// # Examples
    ///
    /// ```
    /// # use vizia_core::prelude::*;
    /// #
    /// # #[derive(Debug, PartialEq, Copy, Clone)]
    /// # enum Action {
    /// #     One,
    /// #     Two,
    /// #     Three,
    /// # }
    /// #
    /// let keymap = Keymap::<Action>::new();
    /// ```
    pub fn new() -> Self {
        Self { entries: IndexMap::new() }
    }

    /// Inserts an entry into the keymap.
    ///
    /// This method is for internal use only.
    /// To insert an entry into the keymap at runtime use the [`KeymapEvent::InsertAction`] event.
    fn insert(&mut self, chord: KeyChord, keymap_entry: KeymapEntry<T>) {
        if let Some(actions) = self.entries.get_mut(&chord) {
            if !actions.contains(&keymap_entry) {
                actions.push(keymap_entry);
            }
        } else {
            self.entries.insert(chord, vec![keymap_entry]);
        }
    }

    /// Removes an entry of the keymap.
    ///
    /// This method is for internal use only.
    /// To remove an entry of the keymap at runtime use the [`KeymapEvent::RemoveAction`] event.
    fn remove(&mut self, chord: &KeyChord, action: &T) {
        if let Some(actions) = self.entries.get_mut(chord) {
            if let Some(index) = actions.iter().position(|x| x == action) {
                if actions.len() == 1 {
                    self.entries.swap_remove(chord);
                } else {
                    actions.swap_remove(index);
                }
            }
        }
    }

    /// Returns an iterator over every pressed keymap entry.
    ///
    /// # Examples
    ///
    /// ```
    /// # use vizia_core::prelude::*;
    /// #
    /// # #[derive(Debug, PartialEq, Copy, Clone)]
    /// # enum Action {
    /// #     One,
    /// # }
    /// #
    /// # let cx = &Context::default();
    /// # let keymap = Keymap::<Action>::new();
    /// #
    /// for entry in keymap.pressed_actions(cx, Code::KeyA) {
    ///     debug!("The action {:?} is being pressed!", entry.action());
    /// };
    pub fn pressed_actions(
        &self,
        cx: &Context,
        code: Code,
    ) -> impl Iterator<Item = &KeymapEntry<T>> {
        if let Some(actions) = self.entries.get(&KeyChord::new(cx.modifiers, code)) {
            actions.iter()
        } else {
            [].iter()
        }
    }

    /// Exports all keymap entries and their associated key chords.
    ///
    /// This is useful if you want to have a settings window and need to access every key chord
    /// and keymap entry of a keymap.
    ///
    /// # Examples
    ///
    /// ```
    /// # use vizia_core::prelude::*;
    /// #
    /// # #[derive(Debug, PartialEq, Copy, Clone)]
    /// # enum Action {
    /// #     One,
    /// # }
    /// #
    /// # let keymap = Keymap::<Action>::new();
    /// #
    /// let actions_chords = keymap.export();
    ///
    /// for (chord, entry) in actions_chords {
    ///     debug!("The key chord {:?} triggers the action {:?}!", chord, entry.action());
    /// }
    /// ```
    pub fn export(&self) -> Vec<(&KeyChord, &KeymapEntry<T>)> {
        let mut vec = Vec::new();
        for (chord, entries) in self.entries.iter() {
            for entry in entries {
                vec.push((chord, entry));
            }
        }
        vec
    }
}

impl<T> Model for Keymap<T>
where
    T: 'static + Clone + PartialEq + Send + Sync,
{
    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.take(|keymap_event, _| match keymap_event {
            KeymapEvent::InsertAction(chord, entry) => self.insert(chord, entry.clone()),
            KeymapEvent::RemoveAction(chord, action) => self.remove(&chord, &action),
        });
        event.map(|window_event, _| match window_event {
            WindowEvent::KeyDown(code, _) => {
                if let Some(entries) = self.entries.get(&KeyChord::new(*cx.modifiers, *code)) {
                    for entry in entries {
                        (entry.on_action())(cx)
                    }
                }
            }
            _ => {}
        })
    }
}

impl<T> From<Vec<(KeyChord, KeymapEntry<T>)>> for Keymap<T>
where
    T: 'static + Clone + PartialEq + Send + Sync,
{
    fn from(vec: Vec<(KeyChord, KeymapEntry<T>)>) -> Self {
        let mut keymap = Self::new();
        for (chord, entry) in vec {
            keymap.insert(chord, entry);
        }
        keymap
    }
}

/// An event used to interact with a [`Keymap`] at runtime.
pub enum KeymapEvent<T>
where
    T: 'static + Clone + PartialEq + Send + Sync,
{
    /// Inserts an entry into the [`Keymap`].
    ///
    /// # Examples
    ///
    /// ```
    /// # use vizia_core::prelude::*;
    /// #
    /// # #[derive(PartialEq, Copy, Clone)]
    /// # enum Action {
    /// #     One,
    /// # }
    /// #
    /// # let cx = &mut Context::default();
    /// #
    /// cx.emit(KeymapEvent::InsertAction(
    ///     KeyChord::new(Modifiers::empty(), Code::KeyA),
    ///     KeymapEntry::new(Action::One, |_| debug!("Action One")),
    /// ));
    /// ```
    InsertAction(KeyChord, KeymapEntry<T>),
    /// Removes an entry from the [`Keymap`].
    ///
    /// # Examples
    ///
    /// ```
    /// # use vizia_core::prelude::*;
    /// #
    /// # #[derive(PartialEq, Copy, Clone)]
    /// # enum Action {
    /// #     One,
    /// # }
    /// #
    /// # let cx = &mut Context::default();
    /// #
    /// cx.emit(KeymapEvent::RemoveAction(
    ///     KeyChord::new(Modifiers::empty(), Code::KeyA),
    ///     Action::One,
    /// ));
    /// ```
    RemoveAction(KeyChord, T),
}
