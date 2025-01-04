use crate::prelude::*;

/// Enum which represents the geometric variants of an avatar view.
#[derive(Debug, Default, Clone, Copy, Data, PartialEq)]
pub enum AvatarVariant {
    #[default]
    Circle,
    Square,
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
    /// Selects the geometric variant of the Avatar. Accepts a value of, or lens to, an [AvatarVariant].
    ///
    /// ```
    /// # use vizia_core::prelude::*;
    /// # let cx = &mut Context::default();
    /// Avatar::new(cx, |cx|{
    ///     Svg::new(cx, ICON_USER);
    /// })
    /// .variant(AvatarVariant::Rounded);
    /// ```
    pub fn variant<U: Into<AvatarVariant>>(self, variant: impl Res<U>) -> Self {
        self.bind(variant, |handle, val| {
            let var: AvatarVariant = val.get(&handle).into();
            match var {
                AvatarVariant::Circle => {
                    handle
                        .toggle_class("circle", true)
                        .toggle_class("square", false)
                        .toggle_class("rounded", false);
                }

                AvatarVariant::Square => {
                    handle
                        .toggle_class("circle", false)
                        .toggle_class("square", true)
                        .toggle_class("rounded", false);
                }

                AvatarVariant::Rounded => {
                    handle
                        .toggle_class("circle", false)
                        .toggle_class("square", false)
                        .toggle_class("rounded", true);
                }
            }
        })
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
            (content)(cx).placement(BadgePlacement::default());
        });

        self
    }
}

pub struct AvatarGroup {}

impl AvatarGroup {
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
