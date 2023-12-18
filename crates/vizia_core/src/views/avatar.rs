use crate::prelude::*;

/// Enum which represents the geometric variants of an avatar view.
#[derive(Debug, Default, Clone, Copy, Data, PartialEq)]
pub enum AvatarVariant {
    #[default]
    Circle,
    Square,
    Rounded,
}

/// An avatar is used to visually represent a person or entity and can contain text, an icon, or an image.
///
/// # Example
/// ```
/// # use vizia_core::prelude::*;
/// # let cx = &mut Context::default();
/// Avatar::new(cx, |cx|{
///     Icon::new(cx, ICON_USER);
/// });
/// ```
pub struct Avatar {}

impl Avatar {
    pub fn new<F>(cx: &mut Context, content: F) -> Handle<Self>
    where
        F: FnOnce(&mut Context),
    {
        Self {}.build(cx, content).class("circle")
    }
}

impl View for Avatar {
    fn element(&self) -> Option<&'static str> {
        Some("avatar")
    }
}

impl<'a> Handle<'a, Avatar> {
    /// Selects the style variant for the Avatar.
    pub fn variant<U: Into<AvatarVariant>>(mut self, variant: impl Res<U>) -> Self {
        let entity = self.entity();
        variant.set_or_bind(self.context(), entity, |cx, val| {
            let var: AvatarVariant = val.get(cx).into();
            match var {
                AvatarVariant::Circle => {
                    cx.toggle_class("circle", true);
                    cx.toggle_class("square", false);
                    cx.toggle_class("rounded", false);
                }

                AvatarVariant::Square => {
                    cx.toggle_class("circle", false);
                    cx.toggle_class("square", true);
                    cx.toggle_class("rounded", false);
                }

                AvatarVariant::Rounded => {
                    cx.toggle_class("circle", false);
                    cx.toggle_class("square", false);
                    cx.toggle_class("rounded", true);
                }
            }
        });

        self
    }

    /// Adds a badge to the Avatar.
    pub fn badge<F>(mut self, content: F) -> Self
    where
        F: FnOnce(&mut Context) -> Handle<'_, Badge>,
    {
        let entity = self.entity();

        self.context().with_current(entity, |cx| {
            (content)(cx);
        });

        self
    }
}
