//! This example showcases how to use a keymap.
//!
//! Key chords:
//! `A`                     => `Action::OnA`
//! `B`                     => `Action::OnB`
//! `C`                     => `Action::OnC`
//! `CTRL+A`                => `Action::OnCtrlA`
//! `ALT+A`                 => `Action::OnAltA`
//! `SHIFT+A`               => `Action::OnShiftA`
//! `LOGO+A`                => `Action::OnLogoA`
//! `ALT+SHIFT+X`           => `Action::OnAltShiftX`
//! `CTRL+ALT+SHIFT+Y`      => `Action::OnCtrlAltShiftY`
//! `CTRL+ALT+SHIFT+LOGO+Z` => `Action::OnCtrlAltShiftLogoZ`

use vizia::*;

fn main() {
    Application::new(WindowDescription::new().with_title("Keymap - Basic"), |cx| {
        // Build the keymap
        Keymap::new()
            .insert(Action::OnA, KeyChord::new(Modifiers::empty(), Code::KeyA))
            .insert(Action::OnB, KeyChord::new(Modifiers::empty(), Code::KeyB))
            .insert(Action::OnC, KeyChord::new(Modifiers::empty(), Code::KeyC))
            .insert(Action::OnCtrlA, KeyChord::new(Modifiers::CTRL, Code::KeyA))
            .insert(Action::OnAltA, KeyChord::new(Modifiers::ALT, Code::KeyA))
            .insert(Action::OnShiftA, KeyChord::new(Modifiers::SHIFT, Code::KeyA))
            .insert(Action::OnLogoA, KeyChord::new(Modifiers::LOGO, Code::KeyA))
            .insert(
                Action::OnAltShiftX,
                KeyChord::new(Modifiers::ALT | Modifiers::SHIFT, Code::KeyX),
            )
            .insert(
                Action::OnCtrlAltShiftY,
                KeyChord::new(Modifiers::CTRL | Modifiers::ALT | Modifiers::SHIFT, Code::KeyY),
            )
            .insert(
                Action::OnCtrlAltShiftLogoZ,
                KeyChord::new(
                    Modifiers::CTRL | Modifiers::ALT | Modifiers::SHIFT | Modifiers::LOGO,
                    Code::KeyZ,
                ),
            )
            .build(cx);

        // Create a custom view
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
    OnA,
    OnB,
    OnC,
    OnCtrlA,
    OnAltA,
    OnShiftA,
    OnLogoA,
    OnAltShiftX,
    OnCtrlAltShiftY,
    OnCtrlAltShiftLogoZ,
}

const ACTIONS: [Action; 10] = [
    Action::OnA,
    Action::OnB,
    Action::OnC,
    Action::OnCtrlA,
    Action::OnAltA,
    Action::OnShiftA,
    Action::OnLogoA,
    Action::OnAltShiftX,
    Action::OnCtrlAltShiftY,
    Action::OnCtrlAltShiftLogoZ,
];
