//! This example showcases how to change the entries of a keymap at runtime.
//!
//! Key chords before pressing the button:
//! `A` => `Action::One`
//! `B` => `Action::Two`
//! `C` => `Action::Three`
//!
//! Key chords after pressing the button:
//! `A` => `Action::One`
//! `X` => `Action::One`
//! `Z` => `Action::Three`

use vizia::prelude::*;

fn main() {
    Application::new(|cx| {
        // Build the keymap.
        Keymap::from(vec![
            (
                KeyChord::new(Modifiers::empty(), Code::KeyA),
                KeymapEntry::new(Action::One, |_| println!("Action One using A")),
            ),
            (
                KeyChord::new(Modifiers::empty(), Code::KeyB),
                KeymapEntry::new(Action::Two, |_| println!("Action Two using B")),
            ),
            (
                KeyChord::new(Modifiers::empty(), Code::KeyC),
                KeymapEntry::new(Action::Three, |_| println!("Action Three using C")),
            ),
        ])
        .build(cx);

        // Create a new button that changes our key chords.
        Button::new(
            cx,
            |cx| {
                // Insert `Action::One` that triggers on `Code::KeyX`.
                cx.emit(KeymapEvent::InsertAction(
                    KeyChord::new(Modifiers::empty(), Code::KeyX),
                    KeymapEntry::new(Action::One, |_| println!("Action One using X")),
                ));

                // Remove `Action::Two` that triggers on `Code::KeyB`.
                cx.emit(KeymapEvent::RemoveAction(
                    KeyChord::new(Modifiers::empty(), Code::KeyB),
                    Action::Two,
                ));

                // Remove `Action::Three` that triggers on `Code::KeyC`.
                cx.emit(KeymapEvent::RemoveAction(
                    KeyChord::new(Modifiers::empty(), Code::KeyC),
                    Action::Three,
                ));

                // Insert `Action::Three` that triggers on `Code::KeyZ`.
                cx.emit(KeymapEvent::InsertAction(
                    KeyChord::new(Modifiers::empty(), Code::KeyZ),
                    KeymapEntry::new(Action::Three, |_| println!("Action Three using Z")),
                ))
            },
            |cx| Label::new(cx, "Change key chords"),
        )
        .space(Pixels(10.0));
    })
    .title("Keymap - Change Key Chords")
    .run();
}

// The actions that are associated with the key chords.
#[derive(Debug, PartialEq, Copy, Clone)]
enum Action {
    One,
    Two,
    Three,
}
