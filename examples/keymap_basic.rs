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

use vizia::prelude::*;

fn main() {
    Application::new(|cx| {
        // Build the keymap.
        Keymap::from(vec![
            (Action::OnA, KeyChord::new(Modifiers::empty(), Code::KeyA)),
            (Action::OnB, KeyChord::new(Modifiers::empty(), Code::KeyB)),
            (Action::OnC, KeyChord::new(Modifiers::empty(), Code::KeyC)),
            (Action::OnCtrlA, KeyChord::new(Modifiers::CTRL, Code::KeyA)),
            (Action::OnAltA, KeyChord::new(Modifiers::ALT, Code::KeyA)),
            (Action::OnShiftA, KeyChord::new(Modifiers::SHIFT, Code::KeyA)),
            (Action::OnLogoA, KeyChord::new(Modifiers::LOGO, Code::KeyA)),
            (Action::OnAltShiftX, KeyChord::new(Modifiers::ALT | Modifiers::SHIFT, Code::KeyX)),
            (
                Action::OnCtrlAltShiftY,
                KeyChord::new(Modifiers::CTRL | Modifiers::ALT | Modifiers::SHIFT, Code::KeyY),
            ),
            (
                Action::OnCtrlAltShiftLogoZ,
                KeyChord::new(
                    Modifiers::CTRL | Modifiers::ALT | Modifiers::SHIFT | Modifiers::LOGO,
                    Code::KeyZ,
                ),
            ),
        ])
        .build(cx);

        // Create a custom view.
        CustomView::new(cx);
    })
    .title("Keymap - Basic")
    .run();
}

struct CustomView;

impl CustomView {
    fn new(cx: &mut Context) -> Handle<Self> {
        Self.build(cx, |_| {}).on_build(|cx| {
            cx.focus();
        })
    }
}

impl View for CustomView {
    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|window_event, _| match window_event {
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
        });
    }
}

// The actions that are associated with the key chords.
#[derive(Debug, PartialEq, Copy, Clone)]
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
