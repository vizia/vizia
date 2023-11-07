use bitflags::bitflags;

bitflags! {
    /// The state of the modifier keys.
    #[derive(Default)]
    pub struct Modifiers: u8 {
        const SHIFT    = 0b0000_0011;
        const LSHIFT   = 0b0000_0001;
        const RSHIFT   = 0b0000_0010;

        const CONTROL  = 0b0000_1100;
        const LCONTROL = 0b0000_0100;
        const RCONTROL = 0b0000_1000;

        const ALT      = 0b0011_0000;
        const LALT     = 0b0001_0000;
        const RALT     = 0b0010_0000;

        const SUPER    = 0b1100_0000;
        const LSUPER   = 0b0100_0000;
        const RSUPER   = 0b1000_0000;
    }
}
