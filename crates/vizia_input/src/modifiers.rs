use bitflags::bitflags;

bitflags! {
    /// The state of the modifier keys.
    #[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Hash)]
    pub struct Modifiers: u8 {
        const LSHIFT = 1;
        const LCTRL = 1<<1;
        const LALT = 1<<2;
        const LSUPER = 1<<3;
        const RSHIFT = 1<<4;
        const RCTRL = 1<<5;
        const RALT = 1<<6;
        const RSUPER = 1<<7;
    }
}

impl Modifiers {
    pub fn shift(&self) -> bool {
        self.contains(Modifiers::LSHIFT) | self.contains(Modifiers::RSHIFT)
    }

    pub fn alt(&self) -> bool {
        self.contains(Modifiers::LALT) | self.contains(Modifiers::RALT)
    }

    pub fn ctrl(&self) -> bool {
        self.contains(Modifiers::LCTRL) | self.contains(Modifiers::RCTRL)
    }

    pub fn logo(&self) -> bool {
        self.contains(Modifiers::LSUPER) | self.contains(Modifiers::RSUPER)
    }
}
