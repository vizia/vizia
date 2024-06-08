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

use log::debug;
use vizia::prelude::*;

fn main() -> Result<(), ApplicationError> {
    Application::new(|cx| {
        // Build the keymap.
        Keymap::from(vec![
            (
                KeyChord::new(Modifiers::empty(), Code::KeyA),
                KeymapEntry::new(Action::OnA, |_| debug!("Action A")),
            ),
            (
                KeyChord::new(Modifiers::empty(), Code::KeyB),
                KeymapEntry::new(Action::OnB, |_| debug!("Action B")),
            ),
            (
                KeyChord::new(Modifiers::empty(), Code::KeyC),
                KeymapEntry::new(Action::OnC, |_| debug!("Action C")),
            ),
            (
                KeyChord::new(Modifiers::CTRL, Code::KeyA),
                KeymapEntry::new(Action::OnCtrlA, |_| debug!("Action OnCtrlA")),
            ),
            (
                KeyChord::new(Modifiers::ALT, Code::KeyA),
                KeymapEntry::new(Action::OnAltA, |_| debug!("Action OnAltA")),
            ),
            (
                KeyChord::new(Modifiers::SHIFT, Code::KeyA),
                KeymapEntry::new(Action::OnShiftA, |_| debug!("Action OnShiftA")),
            ),
            (
                KeyChord::new(Modifiers::SUPER, Code::KeyA),
                KeymapEntry::new(Action::OnLogoA, |_| debug!("Action OnLogoA")),
            ),
            (
                KeyChord::new(Modifiers::ALT | Modifiers::SHIFT, Code::KeyX),
                KeymapEntry::new(Action::OnAltShiftX, |_| debug!("Action OnAltShiftX")),
            ),
            (
                KeyChord::new(Modifiers::CTRL | Modifiers::ALT | Modifiers::SHIFT, Code::KeyY),
                KeymapEntry::new(Action::OnCtrlAltShiftY, |_| debug!("Action OnCtrlAltShiftY")),
            ),
            (
                KeyChord::new(
                    Modifiers::CTRL | Modifiers::ALT | Modifiers::SHIFT | Modifiers::SUPER,
                    Code::KeyZ,
                ),
                KeymapEntry::new(Action::OnCtrlAltShiftLogoZ, |_| {
                    debug!("Action OnCtrlAltShiftLogoZ")
                }),
            ),
        ])
        .build(cx);
    })
    .title("Keymap")
    .run()
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
