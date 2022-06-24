use std::collections::HashMap;

use crate::prelude::*;

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
///     fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
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
#[derive(Debug)]
pub struct Keymap<T>
where
    T: 'static + PartialEq + Send + Sync + Copy + Clone,
{
    actions: HashMap<KeyChord, Vec<T>>,
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
        Self { actions: HashMap::new() }
    }

    /// Inserts an entry into the keymap.
    ///
    /// This method is for internal use only.
    /// To insert an entry into the keymap at runtime use the [`KeymapEvent::InsertAction`] event.
    fn insert(&mut self, action: T, chord: KeyChord) {
        if let Some(actions) = self.actions.get_mut(&chord) {
            if !actions.contains(&action) {
                actions.push(action);
            }
        } else {
            self.actions.insert(chord, vec![action]);
        }
    }

    /// Removes an entry of the keymap.
    ///
    /// This method is for internal use only.
    /// To remove an entry of the keymap at runtime use the [`KeymapEvent::RemoveAction`] event.
    fn remove(&mut self, action: &T, chord: &KeyChord) {
        if let Some(actions) = self.actions.get_mut(chord) {
            if let Some(index) = actions.iter().position(|x| x == action) {
                if actions.len() == 1 {
                    self.actions.remove(chord);
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
    /// # let mut cx = &mut Context::new();
    /// # let cx = &mut EventContext::new(cx);
    /// # let keymap = Keymap::<Action>::new();
    /// #
    /// for action in keymap.pressed_actions(cx, Code::KeyA) {
    ///     println!("The action {:?} is being pressed!", action);
    /// };
    /// ```
    pub fn pressed_actions(&self, cx: &EventContext, code: Code) -> impl Iterator<Item = &T> {
        if let Some(actions) = self.actions.get(&KeyChord::new(*cx.modifiers, code)) {
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
    pub fn export(&self) -> Vec<(T, KeyChord)> {
        let mut vec = Vec::new();
        for (chord, actions) in self.actions.iter() {
            for action in actions {
                vec.push((*action, *chord));
            }
        }
        vec
    }
}

impl<T> Model for Keymap<T>
where
    T: 'static + PartialEq + Send + Sync + Copy + Clone,
{
    fn event(&mut self, _: &mut EventContext, event: &mut Event) {
        event.map(|keymap_event, _| match keymap_event {
            KeymapEvent::InsertAction(action, chord) => self.insert(*action, *chord),
            KeymapEvent::RemoveAction(action, chord) => self.remove(action, chord),
        });
    }
}

impl<T> From<Vec<(T, KeyChord)>> for Keymap<T>
where
    T: 'static + PartialEq + Send + Sync + Copy + Clone,
{
    fn from(vec: Vec<(T, KeyChord)>) -> Self {
        let mut keymap = Self::new();
        for (action, chord) in vec {
            keymap.insert(action, chord);
        }
        keymap
    }
}

/// An event used to interact with a [`Keymap`] at runtime.
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
    InsertAction(T, KeyChord),
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
    RemoveAction(T, KeyChord),
}
