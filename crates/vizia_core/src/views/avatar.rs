use crate::prelude::*;

/// Enum which represents the geometric variants of an avatar view.
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub enum AvatarVariant {
    #[default]
    /// Represents a circular avatar shape.
    Circle,
    /// Represents a square avatar shape.
    Square,
    /// Represents a  rounded rectangle avatar shape.
    Rounded,
}

/// An avatar view is used to visually represent a person or entity and can contain text, an icon, or an image.
///
/// # Example
/// ```
/// # use vizia_core::prelude::*;
/// # use vizia_core::icons::ICON_USER;
/// # let cx = &mut Context::default();
/// Avatar::new(cx, |cx|{
///     Svg::new(cx, ICON_USER);
/// });
/// ```
pub struct Avatar {}

impl Avatar {
    /// Creates a new avatar with the given content.
    ///
    /// ```
    /// # use vizia_core::prelude::*;
    /// # use vizia_core::icons::ICON_USER;
    /// # let cx = &mut Context::default();
    /// Avatar::new(cx, |cx|{
    ///     Svg::new(cx, ICON_USER);
    /// });
    /// ```
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

/// Modifiers for changing the appearance and content of an [Avatar].
pub trait AvatarModifiers: Sized {
    /// Selects the geometric variant of the Avatar. Accepts a value or signal of type [AvatarVariant].
    ///
    /// ```
    /// # use vizia_core::prelude::*;
    /// # use vizia_core::icons::ICON_USER;
    /// # let cx = &mut Context::default();
    /// Avatar::new(cx, |cx|{
    ///     Svg::new(cx, ICON_USER);
    /// })
    /// .variant(AvatarVariant::Rounded);
    /// ```
    fn variant<U: Into<AvatarVariant> + Clone + PartialEq + 'static>(
        self,
        variant: impl Res<U> + 'static,
    ) -> Self;

    /// Adds a badge to the Avatar.
    ///
    /// ```
    /// # use vizia_core::prelude::*;
    /// # use vizia_core::icons::ICON_USER;
    /// # let cx = &mut Context::default();
    /// Avatar::new(cx, |cx|{
    ///     Svg::new(cx, ICON_USER);
    /// })
    /// .badge(|cx| Badge::empty(cx).class("error"));
    /// ```
    #[allow(unused_variables)]
    fn badge<F>(self, content: F) -> Self
    where
        F: FnOnce(&mut Context) -> Handle<'_, Badge>,
    {
        self
    }
}

impl AvatarModifiers for Handle<'_, Avatar> {
    fn variant<U: Into<AvatarVariant> + Clone + PartialEq + 'static>(
        mut self,
        variant: impl Res<U> + 'static,
    ) -> Self {
        let avatar_variant = variant.to_signal(self.context()).map(|value| value.clone().into());

        let is_circle = Memo::new(move |_| avatar_variant.get() == AvatarVariant::Circle);

        let is_square = Memo::new(move |_| avatar_variant.get() == AvatarVariant::Square);

        let is_rounded = Memo::new(move |_| avatar_variant.get() == AvatarVariant::Rounded);

        self.toggle_class("circle", is_circle)
            .toggle_class("square", is_square)
            .toggle_class("rounded", is_rounded)
    }

    fn badge<F>(mut self, content: F) -> Self
    where
        F: FnOnce(&mut Context) -> Handle<'_, Badge>,
    {
        let entity = self.entity();

        self.context().with_current(entity, |cx| {
            (content)(cx).placement(BadgePlacement::default());
        });

        self
    }
}

/// The [AvatarGroup] view can be used to group a series of avatars together.
pub struct AvatarGroup {}

impl AvatarGroup {
    /// Create a new [AvatarGroup]. The content should be a series of [Avatar] views.
    pub fn new<F>(cx: &mut Context, content: F) -> Handle<Self>
    where
        F: FnOnce(&mut Context),
    {
        Self {}.build(cx, content).size(Auto).gap(Pixels(-20.0)).layout_type(LayoutType::Row)
    }
}

impl View for AvatarGroup {
    fn element(&self) -> Option<&'static str> {
        Some("avatar-group")
    }
}

impl AvatarModifiers for Handle<'_, AvatarGroup> {
    fn variant<U: Into<AvatarVariant> + Clone + PartialEq + 'static>(
        mut self,
        variant: impl Res<U> + 'static,
    ) -> Self {
        let avatar_variant = variant.to_signal(self.context()).map(|value| value.clone().into());

        let is_circle = Memo::new(move |_| avatar_variant.get() == AvatarVariant::Circle);

        let is_square = Memo::new(move |_| avatar_variant.get() == AvatarVariant::Square);

        let is_rounded = Memo::new(move |_| avatar_variant.get() == AvatarVariant::Rounded);

        self.toggle_class("circle", is_circle)
            .toggle_class("square", is_square)
            .toggle_class("rounded", is_rounded)
    }
}
