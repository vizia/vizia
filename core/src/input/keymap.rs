use crate::{Code, Context, KeyChord, Model};
use std::collections::HashMap;
use std::hash::Hash;

/// A keymap that associates key chords with actions.
///
/// This is useful if you have an application that lets the user configure their key chords.
/// It allows you to check if a particular action is pressed rather than the actual keys.
///
/// # Examples
///
/// First we need to create something that represents an action in our application.
/// This is usually an enum.
///
/// ```
/// #[derive(PartialEq, Eq, Hash, Copy, Clone)]
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
/// # use vizia_core::*;
/// # use std::hash::Hash;
/// #
/// # #[derive(PartialEq, Eq, Hash, Copy, Clone)]
/// # enum Action {
/// #     One,
/// #     Two,
/// #     Three,
/// # }
/// #
/// let keymap = Keymap::new()
///     .insert(Action::One, KeyChord::new(Modifiers::empty(), Code::KeyA))
///     .insert(Action::Two, KeyChord::new(Modifiers::CTRL, Code::KeyB))
///     .insert(Action::Three, KeyChord::new(Modifiers::CTRL | Modifiers::SHIFT, Code::KeyC));
/// ```
///
/// After we've defined our key chords we can now use them inside of a custom view. Here we check if the
/// action `Action::One` is being pressed. If it is we just print a simple message to the console,
/// but here you could do whatever you want to.
///
/// ```
/// # use vizia_core::*;
/// #
/// # #[derive(PartialEq, Eq, Hash, Copy, Clone)]
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
///         if let Some(window_event) = event.message.downcast() {
///             match window_event {
///                 WindowEvent::KeyDown(code, _) => {
///                     if let Some(keymap_data) = cx.data::<Keymap<Action>>() {
///                         if keymap_data.pressed(cx, &Action::One, *code) {
///                             println!("The action One is pressed");
///                         }
///                     }
///                 }
///                 _ => {}
///             }
///         }
///     }
/// }
/// ```
#[derive(Debug)]
pub struct Keymap<T>
where
    T: 'static + Eq + Hash + Send + Sync + Copy + Clone,
{
    chords: HashMap<T, KeyChord>,
}

impl<T> Keymap<T>
where
    T: 'static + Eq + Hash + Send + Sync + Copy + Clone,
{
    /// Creates a new keymap.
    ///
    /// # Examples
    ///
    /// ```
    /// # use vizia_core::*;
    /// # use std::hash::Hash;
    /// #
    /// # #[derive(PartialEq, Eq, Hash, Copy, Clone)]
    /// # enum Action {
    /// #     One,
    /// #     Two,
    /// #     Three,
    /// # }
    /// #
    /// let keymap = Keymap::<Action>::new();
    /// ```
    pub fn new() -> Self {
        Self { chords: HashMap::new() }
    }

    /// Inserts or overwrites a key chord of the keymap.
    ///
    /// # Examples
    ///
    /// ```
    /// # use vizia_core::*;
    /// #
    /// # #[derive(PartialEq, Eq, Hash, Copy, Clone)]
    /// # enum Action {
    /// #     One,
    /// # }
    /// #
    /// let keymap = Keymap::new()
    ///     .insert(Action::One, KeyChord::new(Modifiers::CTRL, Code::KeyA));
    /// ```
    pub fn insert(mut self, key: T, chord: KeyChord) -> Self {
        self.chords.insert(key, chord);
        self
    }

    /// Removes a key chord of the keymap if it exists.
    ///
    /// # Examples
    ///
    /// ```
    /// # use vizia_core::*;
    /// #
    /// # #[derive(PartialEq, Eq, Hash, Copy, Clone)]
    /// # enum Action {
    /// #     One,
    /// # }
    /// #
    /// let keymap = Keymap::new().remove(&Action::One);
    /// ```
    pub fn remove(mut self, key: &T) -> Self {
        self.chords.remove(key);
        self
    }

    /// Returns `true` if the `action` is pressed.
    ///
    /// # Examples
    ///
    /// ```
    /// # use vizia_core::*;
    /// #
    /// # #[derive(PartialEq, Eq, Hash, Copy, Clone)]
    /// # enum Action {
    /// #     One,
    /// # }
    /// #
    /// # let cx = &Context::new();
    /// # let keymap = Keymap::new();
    /// #
    /// if keymap.pressed(cx, &Action::One, Code::KeyA) {
    ///     println!("Action is pressed");
    /// }
    /// ```
    pub fn pressed(&self, cx: &Context, action: &T, button: Code) -> bool {
        self.chords
            .get(action)
            .map(|chord| chord.modifiers == cx.modifiers && chord.button == button)
            .unwrap_or(false)
    }
}

impl<T> Model for Keymap<T>
where
    T: 'static + Eq + Hash + Send + Sync + Copy + Clone,
{
    fn event(&mut self, _: &mut Context, event: &mut crate::Event) {
        if let Some(keymap_event) = event.message.downcast() {
            match keymap_event {
                KeymapEvent::InsertChord(action, chord) => self.chords.insert(*action, *chord),
                KeymapEvent::RemoveChord(action) => self.chords.remove(action),
            };
        }
    }
}

/// An event used to interact with a [`Keymap`] at runtime.
pub enum KeymapEvent<T>
where
    T: 'static + Eq + Hash + Send + Sync + Copy + Clone,
{
    /// Inserts a key chord into the [`Keymap`].
    ///
    /// # Examples
    ///
    /// ```
    /// # use vizia_core::*;
    /// #
    /// # #[derive(PartialEq, Eq, Hash, Copy, Clone)]
    /// # enum Action {
    /// #     One,
    /// # }
    /// #
    /// # let cx = &mut Context::new();
    /// #
    /// cx.emit(KeymapEvent::InsertBinding(
    ///     Action::One,
    ///     KeyChord::new(Modifiers::empty(), Code::KeyA),
    /// ));
    /// ```
    InsertChord(T, KeyChord),
    /// Removes a key chord from the [`Keymap`].
    ///
    /// # Examples
    ///
    /// ```
    /// # use vizia_core::*;
    /// #
    /// # #[derive(PartialEq, Eq, Hash, Copy, Clone)]
    /// # enum Action {
    /// #     One,
    /// # }
    /// #
    /// # let cx = &mut Context::new();
    /// #
    /// cx.emit(KeymapEvent::RemoveBinding(Action::One));
    /// ```
    RemoveChord(T),
}
