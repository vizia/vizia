use bitflags::bitflags;

bitflags! {
    /// The state of the modifier keys.
    #[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Hash)]
    pub struct Modifiers: u8 {
        const SHIFT = 1;
        const CTRL = 1<<1;
        const ALT = 1<<2;
        const SUPER = 1<<3;
    }
}

impl Modifiers {
    pub fn shift(&self) -> bool {
        self.contains(Modifiers::SHIFT)
    }

    pub fn alt(&self) -> bool {
        self.contains(Modifiers::ALT)
    }

    pub fn ctrl(&self) -> bool {
        self.contains(Modifiers::CTRL)
    }

    pub fn logo(&self) -> bool {
        self.contains(Modifiers::SUPER)
    }
}
