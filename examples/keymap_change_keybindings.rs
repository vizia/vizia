//! This example showcases how to change the keybindings of a keymap at runtime.
//!
//! Keybindings before pressing the button:
//! `A` => `Action::One`
//! `B` => `Action::Two`
//! `C` => `Action::Three`
//!
//! Keybindings after pressing the button:
//! `X` => `Action::One`
//! `Z` => `Action::Three`

use vizia::*;

fn main() {
    Application::new(WindowDescription::new().with_title("Keymap - Change Keybindings"), |cx| {
        // Build the keymap
        Keymap::new()
            .insert(Action::One, KeyBinding::new(Modifiers::empty(), Code::KeyA))
            .insert(Action::Two, KeyBinding::new(Modifiers::empty(), Code::KeyB))
            .insert(Action::Three, KeyBinding::new(Modifiers::empty(), Code::KeyC))
            .build(cx);

        // Create a new button that changes our keybindings.
        Button::new(
            cx,
            |cx| {
                // Change the first binding to trigger when pressing `X` instead of `A`.
                cx.emit(KeymapEvent::InsertBinding(
                    Action::One,
                    KeyBinding::new(Modifiers::empty(), Code::KeyX),
                ));

                // Remove the second binding.
                cx.emit(KeymapEvent::RemoveBinding(Action::Two));

                // Change the third binding to trigger when pressing `Z` instead of `C`.
                cx.emit(KeymapEvent::InsertBinding(
                    Action::Three,
                    KeyBinding::new(Modifiers::empty(), Code::KeyZ),
                ))
            },
            |cx| Label::new(cx, "Change keybindings"),
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
                    // Retrieve our keymap data containing all of our keybindings.
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

// The actions that are associated with the keybindings.
#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
enum Action {
    One,
    Two,
    Three,
}

const ACTIONS: [Action; 3] = [Action::One, Action::Two, Action::Three];
