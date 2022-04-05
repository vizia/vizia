//! This example showcases how to change the key chords of a keymap at runtime.
//!
//! Key chords before pressing the button:
//! `A` => `Action::One`
//! `B` => `Action::Two`
//! `C` => `Action::Three`
//!
//! Key chords after pressing the button:
//! `X` => `Action::One`
//! `Z` => `Action::Three`

use vizia::*;

fn main() {
    Application::new(WindowDescription::new().with_title("Keymap - Change Key Chords"), |cx| {
        // Build the keymap
        Keymap::new()
            .insert(Action::One, KeyChord::new(Modifiers::empty(), Code::KeyA))
            .insert(Action::Two, KeyChord::new(Modifiers::empty(), Code::KeyB))
            .insert(Action::Three, KeyChord::new(Modifiers::empty(), Code::KeyC))
            .build(cx);

        // Create a new button that changes our key chords.
        Button::new(
            cx,
            |cx| {
                // Change the first chord to trigger when pressing `X` instead of `A`.
                cx.emit(KeymapEvent::InsertChord(
                    Action::One,
                    KeyChord::new(Modifiers::empty(), Code::KeyX),
                ));

                // Remove the second chord.
                cx.emit(KeymapEvent::RemoveChord(Action::Two));

                // Change the third chord to trigger when pressing `Z` instead of `C`.
                cx.emit(KeymapEvent::InsertChord(
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
                        // Loop through every action in our `Action` enum.
                        for action in ACTIONS {
                            // Check if the action is being pressed.
                            if keymap_data.pressed(cx, &action, *code) {
                                println!("The action {:?} is being pressed!", action);
                            }
                        }
                    }
                }
                _ => {}
            }
        }
    }
}

// The actions that are associated with the key chords.
#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
enum Action {
    One,
    Two,
    Three,
}

const ACTIONS: [Action; 3] = [Action::One, Action::Two, Action::Three];
