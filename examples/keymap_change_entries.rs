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

use vizia::*;

fn main() {
    Application::new(WindowDescription::new().with_title("Keymap - Change Key Chords"), |cx| {
        // Build the keymap.
        Keymap::from(vec![
            (Action::One, KeyChord::new(Modifiers::empty(), Code::KeyA)),
            (Action::Two, KeyChord::new(Modifiers::empty(), Code::KeyB)),
            (Action::Three, KeyChord::new(Modifiers::empty(), Code::KeyC)),
        ])
        .build(cx);

        // Create a new button that changes our key chords.
        Button::new(
            cx,
            |cx| {
                // Insert `Action::One` that triggers on `Code::KeyX`.
                cx.emit(KeymapEvent::InsertAction(
                    Action::One,
                    KeyChord::new(Modifiers::empty(), Code::KeyX),
                ));

                // Remove `Action::Two` that triggers on `Code::KeyB`.
                cx.emit(KeymapEvent::RemoveAction(
                    Action::Two,
                    KeyChord::new(Modifiers::empty(), Code::KeyB),
                ));

                // Remove `Action::Three` that triggers on `Code::KeyC`.
                cx.emit(KeymapEvent::RemoveAction(
                    Action::Three,
                    KeyChord::new(Modifiers::empty(), Code::KeyC),
                ));

                // Insert `Action::Three` that triggers on `Code::KeyZ`.
                cx.emit(KeymapEvent::InsertAction(
                    Action::Three,
                    KeyChord::new(Modifiers::empty(), Code::KeyZ),
                ))
            },
            |cx| Label::new(cx, "Change key chords"),
        );

        // Create a custom view that prints a message every time one of our actions is pressed.
        CustomView::new(cx);
    })
    .run();
}

struct CustomView;

impl CustomView {
    fn new(cx: &mut Context) -> Handle<Self> {
        Self.build2(cx, |_| {})
    }
}

impl View for CustomView {
    fn event(&mut self, cx: &mut Context, event: &mut Event) {
        if let Some(window_event) = event.message.downcast() {
            match window_event {
                WindowEvent::KeyDown(code, _) => {
                    // Retrieve our keymap data containing all of our key chords.
                    if let Some(keymap_data) = cx.data::<Keymap<Action>>() {
                        // Loop through every action that is being pressed.
                        for action in keymap_data.pressed_actions(cx, *code) {
                            println!("The action {:?} is being pressed!", action);
                        }
                    }
                }
                _ => {}
            }
        }
    }
}

// The actions that are associated with the key chords.
#[derive(Debug, PartialEq, Copy, Clone)]
enum Action {
    One,
    Two,
    Three,
}
