use crate::prelude::*;

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

    pub fn build(self, cx: &mut Context) -> Handle<Scroll> {
        Scroll {}.build(cx, |cx| {
            // do something with horizontal and vertical indicator here
            (self.content)(cx);
        })
    }
}

// Define the Scroll view
pub struct Scroll {}

// Make it a View
impl View for Scroll {}

// Define two methods:
//      The first creates the builder
//      The second uses the builder with all its explicit parameters
impl Scroll {
    pub fn builder(content: impl Fn(&mut Context) + 'static) -> ScrollBuilder {
        ScrollBuilder {
            content: Box::new(content),
            horizontal_indicator: true,
            vertical_indicator: true,
        }
    }

    pub fn new(
        cx: &mut Context,
        content: impl Fn(&mut Context) + 'static,
        horizontal_indicator: bool,
        vertical_indicator: bool,
    ) -> Handle<Self> {
        Self::builder(content)
            .horizontal_indicator(horizontal_indicator)
            .vertical_indicator(vertical_indicator)
            .build(cx)
    }
}
