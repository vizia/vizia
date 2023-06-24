//! Methods on views for changing their properties or for adding actions.
//!
//! # Examples
//! Modifiers can be used to apply inline [style](StyleModifiers) and [layout](LayoutModifiers) properties to a view:
//! ```no_run
//! # use vizia_core::prelude::*;
//! # let cx = &mut Context::default();
//! # use vizia_winit::application::Application;
//! # Application::new(|cx|{
//! // Sets the background color of the label to red.
//! Label::new(cx, "Hello World")
//!     .background_color(Color::red());
//! # }).run();
//! ```
//!
//! ```no_run
//! # use vizia_core::prelude::*;
//! # let cx = &mut Context::default();
//! # use vizia_winit::application::Application;
//! # Application::new(|cx|{
//! // Sets the width of the label to be 100 pixels.
//! Label::new(cx, "Hello World")
//!     .width(Pixels(100.0));
//! # }).run();
//! ```
//!
//! Modifiers can also be used to add [actions](ActionModifiers) to a view:
//! ```no_run
//! # use vizia_core::prelude::*;
//! # let cx = &mut Context::default();
//! # use vizia_winit::application::Application;
//! # Application::new(|cx|{
//! // Closes the window when the label is pressed.
//! Label::new(cx, "Hello World")
//!     .on_press(|cx| cx.emit(WindowEvent::WindowClose));
//! # }).run();
//! ```

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
            value.set_or_bind(self.context(), entity, |cx, v| {
                cx.style.$name.insert(cx.current, v.into());

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

mod accessibility;
pub use accessibility::*;

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

// Re-export here for docs
pub use crate::window::WindowModifiers;
