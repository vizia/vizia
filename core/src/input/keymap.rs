use super::KeymapEntry;
use crate::prelude::*;
use std::collections::HashMap;

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
/// and `Action::Three` to the key chord `CTRL+SHIFT+C`.
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
///     (Action::One, KeyChord::new(Modifiers::empty(), Code::KeyA)),
///     (Action::Two, KeyChord::new(Modifiers::CTRL, Code::KeyB)),
///     (Action::Three, KeyChord::new(Modifiers::CTRL | Modifiers::SHIFT, Code::KeyC)),
/// ]);
/// ```
///
/// After we've defined our key chords we can now use them inside of a custom view. Here we check if the
/// action `Action::One` is being pressed. If it is we just print a simple message to the console,
/// but here you could do whatever you want to.
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
/// struct CustomView;
///
/// impl View for CustomView {
///     fn event(&mut self, cx: &mut Context, event: &mut Event) {
///         event.map(|window_event, _| match window_event {
///             WindowEvent::KeyDown(code, _) => {
///                 if let Some(keymap_data) = cx.data::<Keymap<Action>>() {
///                     for action in keymap_data.pressed_actions(cx, *code) {
///                         println!("The action {:?} is being pressed!", action);
///                     }
///                 }
///             }
///             _ => {}
///         });
///     }
/// }
/// ```
///
/// This type is part of the prelude.
pub struct Keymap<T>
where
    T: 'static + PartialEq + Send + Sync + Copy + Clone,
{
    entries: HashMap<KeyChord, Vec<KeymapEntry<T>>>,
}

impl<T> Keymap<T>
where
    T: 'static + PartialEq + Send + Sync + Copy + Clone,
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
        Self { entries: HashMap::new() }
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
                    self.entries.remove(chord);
                } else {
                    actions.swap_remove(index);
                }
            }
        }
    }

    /// Returns an iterator over every pressed action or `None` if there are no actions for that key chord.
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
    /// # let cx = &Context::new();
    /// # let keymap = Keymap::<Action>::new();
    /// #
    /// for action in keymap.pressed_actions(cx, Code::KeyA) {
    ///     println!("The action {:?} is being pressed!", action);
    /// };
    pub fn pressed_actions(
        &self,
        cx: &Context,
        code: Code,
    ) -> impl Iterator<Item = &KeymapEntry<T>> {
        if let Some(actions) = self.entries.get(&KeyChord::new(cx.modifiers(), code)) {
            actions.iter()
        } else {
            [].iter()
        }
    }

    /// Exports all actions and their associated key chords.
    ///
    /// This is useful if you want to have a settings window and need every to access every key chord
    /// and action of a keymap.
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
    /// for (action, chord) in actions_chords {
    ///     println!("The key chord {:?} triggers the action {:?}!", chord, action);
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
    T: 'static + PartialEq + Send + Sync + Copy + Clone,
{
    fn event(&mut self, cx: &mut Context, event: &mut Event) {
        event.map(|keymap_event, meta| {
            match keymap_event {
                KeymapEvent::InsertAction(chord, entry) => self.insert(*chord, *entry),
                KeymapEvent::RemoveAction(chord, action) => self.remove(chord, action),
                _ => {}
            }
            meta.consume();
        });
        event.map(|window_event, _| match window_event {
            WindowEvent::KeyDown(code, _) => {
                if let Some(entries) = self.entries.get(&KeyChord::new(cx.modifiers(), *code)) {
                    for entry in entries {
                        (entry.callback())(cx)
                    }
                }
            }
            _ => {}
        })
    }
}

impl<T> From<Vec<(KeyChord, KeymapEntry<T>)>> for Keymap<T>
where
    T: 'static + PartialEq + Send + Sync + Copy + Clone,
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
///
/// This type is part of the prelude.
pub enum KeymapEvent<T>
where
    T: 'static + PartialEq + Send + Sync + Copy + Clone,
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
    /// # let cx = &mut Context::new();
    /// #
    /// cx.emit(KeymapEvent::InsertAction(
    ///     Action::One,
    ///     KeyChord::new(Modifiers::empty(), Code::KeyA),
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
    /// # let cx = &mut Context::new();
    /// #
    /// cx.emit(KeymapEvent::RemoveAction(
    ///     Action::One,
    ///     KeyChord::new(Modifiers::empty(), Code::KeyA),
    /// ));
    /// ```
    RemoveAction(KeyChord, T),
}
