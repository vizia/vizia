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
//! let text = cx.state("Hello World");
//! let red = cx.state(Color::red());
//! Label::new(cx, text)
//!     .background_color(red);
//! # }).run();
//! ```
//!
//! ```no_run
//! # use vizia_core::prelude::*;
//! # let cx = &mut Context::default();
//! # use vizia_winit::application::Application;
//! # Application::new(|cx|{
//! // Sets the width of the label to be 100 pixels.
//! let text = cx.state("Hello World");
//! let width_100 = cx.state(Pixels(100.0));
//! Label::new(cx, text)
//!     .width(width_100);
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
//! let text = cx.state("Hello World");
//! Label::new(cx, text)
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
        fn $name<U>(mut self, value: Signal<U>) -> Self
        where
            U: Clone + Into<$t> + 'static,
        {
            let entity = self.entity();
            let current = self.current();
            internal::bind_signal(self.context(), current, entity, value, move |cx, v| {
                cx.style.$name.insert(entity, v.clone().into());

                cx.style.system_flags |= $flags;
                cx.set_system_flags(entity, $flags);
            });

            self
        }
    };
}

// Inside private module to hide implementation details.
mod internal {
    use crate::prelude::{Binding, Context, Entity, Handle, Signal};

    // Allows a modifier trait to access to context and entity from `self`.
    pub trait Modifiable: Sized {
        fn context(&mut self) -> &mut Context;
        fn entity(&self) -> Entity;
        fn current(&self) -> Entity;
    }

    impl<V> Modifiable for Handle<'_, V> {
        fn context(&mut self) -> &mut Context {
            self.cx
        }

        fn entity(&self) -> Entity {
            self.entity
        }

        fn current(&self) -> Entity {
            self.current
        }
    }

    pub fn bind_signal<T, F>(
        cx: &mut Context,
        current: Entity,
        entity: Entity,
        signal: Signal<T>,
        f: F,
    )
    where
        T: Clone + 'static,
        F: 'static + Fn(&mut Context, &T),
    {
        cx.with_current(current, move |cx| {
            Binding::new(cx, signal, move |cx| {
                cx.with_current(entity, |cx| {
                    let value = signal.get(cx).clone();
                    f(cx, &value);
                });
            });
        });
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
