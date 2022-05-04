use crate::prelude::*;

// User Code

// Builder for Scroll
pub struct ScrollBuilder {
    content: Box<dyn Fn(&mut Context)>,
    horizontal_indicator: bool,
    vertical_indicator: bool,
}

// Custom modifiers for ScrollBuilder
impl ScrollBuilder {
    pub fn horizontal_indicator(mut self, flag: bool) -> Self {
        self.horizontal_indicator = flag;

        self
    }

    pub fn vertical_indicator(mut self, flag: bool) -> Self {
        self.vertical_indicator = flag;

        self
    }
}

// What the builder should build
// Maybe this could be simplified with a macro?
impl ViewBuilder for ScrollBuilder {
    fn build(self, cx: &mut Context) -> Entity {
        Scroll::new_with(cx, self.content, self.horizontal_indicator, self.vertical_indicator)
    }
}

// Define the Scroll view
pub struct Scroll {}

// Make it a View
impl View for Scroll {}

// Define two methods:
//      The first creates the builder
//      The second is used by the builder to actually build the view into context
impl Scroll {
    pub fn new(content: impl Fn(&mut Context) + 'static) -> ScrollBuilder {
        ScrollBuilder {
            content: Box::new(content),
            horizontal_indicator: true,
            vertical_indicator: true,
        }
    }

    fn new_with(
        cx: &mut Context,
        content: impl Fn(&mut Context),
        _horizontal_indicator: bool,
        _vertical_indicator: bool,
    ) -> Entity {
        Self {}
            .build(cx, |cx| {
                (content)(cx);
                // Do stuff with horizontal/vertical _indicator here
            })
            .entity()
    }
}

// Internal Vizia Code

// Trait for any type that can build a view
pub trait ViewBuilder {
    // NOTE - This only returns an entity for now. Later that entity can be stored in cx.
    fn build(self, cx: &mut Context) -> Entity;
}

// Trait for style modifiers
pub trait Mod {
    type V: ViewBuilder;
    fn size(self, size: Units) -> SizeMod<Self::V>;
    fn background_color(self, color: Color) -> BackgroundColorMod<Self::V>;
}

// Allow any ViewBuilder to use style modifiers after custom modifiers
// Parts of this could also be replaced by a macro
impl<V: ViewBuilder> Mod for V {
    type V = V;
    fn size(self, size: Units) -> SizeMod<Self::V> {
        SizeMod { inner: self, size }
    }

    fn background_color(self, color: Color) -> BackgroundColorMod<Self::V> {
        BackgroundColorMod { inner: self, color }
    }
}

// Replace with macro
impl<V: ViewBuilder> ViewBuilder for SizeMod<V> {
    fn build(self, cx: &mut Context) -> Entity {
        let size = self.size;
        let current = self.inner.build(cx);
        cx.style().width.insert(current, size);
        cx.style().height.insert(current, size);
        current
    }
}

pub struct SizeMod<V: ViewBuilder> {
    inner: V,
    size: Units,
}

pub struct BackgroundColorMod<V: ViewBuilder> {
    inner: V,
    color: Color,
}

// Replace with macro
impl<V: ViewBuilder> ViewBuilder for BackgroundColorMod<V> {
    fn build(self, cx: &mut Context) -> Entity {
        let color = self.color;
        let current = self.inner.build(cx);
        cx.style().background_color.insert(current, color);
        current
    }
}
