use crate::prelude::*;

/// Enum which represents the geometric variants of an avatar view.
#[derive(Debug, Default, Clone, Copy, Data, PartialEq)]
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

impl Handle<'_, Avatar> {
    /// Selects the geometric variant of the Avatar. Accepts a `Signal<AvatarVariant>`.
    ///
    /// ```
    /// # use vizia_core::prelude::*;
    /// # let cx = &mut Context::default();
    /// Avatar::new(cx, |cx|{
    ///     Svg::new(cx, ICON_USER);
    /// })
    /// .variant(cx.state(AvatarVariant::Rounded));
    /// ```
    pub fn variant(mut self, variant: Signal<AvatarVariant>) -> Self {
        let is_circle = self.context().derived({
            let variant = variant;
            move |store| *variant.get(store) == AvatarVariant::Circle
        });
        let is_square = self.context().derived({
            let variant = variant;
            move |store| *variant.get(store) == AvatarVariant::Square
        });
        let is_rounded = self.context().derived({
            let variant = variant;
            move |store| *variant.get(store) == AvatarVariant::Rounded
        });

        self.toggle_class("circle", is_circle)
            .toggle_class("square", is_square)
            .toggle_class("rounded", is_rounded)
    }

    /// Adds a badge to the Avatar.
    ///
    /// ```
    /// # use vizia_core::prelude::*;
    /// # let cx = &mut Context::default();
    /// Avatar::new(cx, |cx|{
    ///     Svg::new(cx, ICON_USER);
    /// })
    /// .badge(|cx| Badge::empty(cx).class("error"));
    /// ```
    pub fn badge<F>(mut self, content: F) -> Self
    where
        F: FnOnce(&mut Context) -> Handle<'_, Badge>,
    {
        let entity = self.entity();

        self.context().with_current(entity, |cx| {
            let mut badge = (content)(cx);
            let badge_entity = badge.entity();
            let placement = badge
                .context()
                .with_current(badge_entity, |cx| cx.state(BadgePlacement::default()));
            badge.placement(placement);
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
        let auto = cx.state(Auto);
        let overlap_gap = cx.state(Pixels(-20.0));
        let layout_row = cx.state(LayoutType::Row);
        Self {}.build(cx, content).size(auto).gap(overlap_gap).layout_type(layout_row)
    }
}

impl View for AvatarGroup {
    fn element(&self) -> Option<&'static str> {
        Some("avatar-group")
    }
}
