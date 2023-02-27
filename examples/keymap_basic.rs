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
            (
                KeyChord::new(Modifiers::empty(), Code::KeyA),
                KeymapEntry::new(Action::OnA, |_| println!("Action A")),
            ),
            (
                KeyChord::new(Modifiers::empty(), Code::KeyB),
                KeymapEntry::new(Action::OnB, |_| println!("Action B")),
            ),
            (
                KeyChord::new(Modifiers::empty(), Code::KeyC),
                KeymapEntry::new(Action::OnC, |_| println!("Action C")),
            ),
            (
                KeyChord::new(Modifiers::CTRL, Code::KeyA),
                KeymapEntry::new(Action::OnCtrlA, |_| println!("Action OnCtrlA")),
            ),
            (
                KeyChord::new(Modifiers::ALT, Code::KeyA),
                KeymapEntry::new(Action::OnAltA, |_| println!("Action OnAltA")),
            ),
            (
                KeyChord::new(Modifiers::SHIFT, Code::KeyA),
                KeymapEntry::new(Action::OnShiftA, |_| println!("Action OnShiftA")),
            ),
            (
                KeyChord::new(Modifiers::LOGO, Code::KeyA),
                KeymapEntry::new(Action::OnLogoA, |_| println!("Action OnLogoA")),
            ),
            (
                KeyChord::new(Modifiers::ALT | Modifiers::SHIFT, Code::KeyX),
                KeymapEntry::new(Action::OnAltShiftX, |_| println!("Action OnAltShiftX")),
            ),
            (
                KeyChord::new(Modifiers::CTRL | Modifiers::ALT | Modifiers::SHIFT, Code::KeyY),
                KeymapEntry::new(Action::OnCtrlAltShiftY, |_| println!("Action OnCtrlAltShiftY")),
            ),
            (
                KeyChord::new(
                    Modifiers::CTRL | Modifiers::ALT | Modifiers::SHIFT | Modifiers::LOGO,
                    Code::KeyZ,
                ),
                KeymapEntry::new(Action::OnCtrlAltShiftLogoZ, |_| {
                    println!("Action OnCtrlAltShiftLogoZ")
                }),
            ),
        ])
        .build(cx);
    })
    .title("Keymap - Basic")
    .run();
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
