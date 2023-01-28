//! Methods on views for changing their style properties or for adding actions.

// Macro used within modifier traits to set style properties.
macro_rules! modifier {
    (
        $(#[$meta:meta])*
        $name:ident, $t:ty, $flags:expr
    ) => {
        $(#[$meta])*
        #[allow(unused_variables)]
        fn $name<U: Into<$t>>(mut self, value: impl Res<U>) -> Self {
            let entity = self.entity();
            value.set_or_bind(self.context(), entity, |cx, entity, v| {
                cx.style.$name.insert(entity, v.into());

                cx.style.system_flags |= $flags;
            });

            self
        }
    };
}

// Inside private module to hide implementation details.
mod internal {
    use crate::prelude::{Context, Entity, Handle};

    // Allows a modifier trait to access to context and entity from `self`.
    pub trait Modifiable: Sized {
        fn context(&mut self) -> &mut Context;
        fn entity(&self) -> Entity;
    }

    impl<'a, V> Modifiable for Handle<'a, V> {
        fn context(&mut self) -> &mut Context {
            self.cx
        }

        fn entity(&self) -> Entity {
            self.entity
        }
    }
}

mod actions;
pub use actions::*;

mod layout;
pub use layout::*;

mod style;
pub use style::*;

mod text;
pub use text::*;

mod abilities;
pub use abilities::*;
