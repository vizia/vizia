use crate::prelude::*;

/// A container view used to group related content.
pub struct Card;

impl Card {
    /// Creates a new [Card] view.
    pub fn new(cx: &mut Context, content: impl FnOnce(&mut Context)) -> Handle<Self> {
        Self.build(cx, content)
    }
}

impl View for Card {
    fn element(&self) -> Option<&'static str> {
        Some("card")
    }
}

/// The header section of a [Card].
pub struct CardHeader;

impl CardHeader {
    /// Creates a new [CardHeader] view.
    pub fn new(cx: &mut Context, content: impl FnOnce(&mut Context)) -> Handle<Self> {
        Self.build(cx, content)
    }
}

impl View for CardHeader {
    fn element(&self) -> Option<&'static str> {
        Some("card-header")
    }
}

/// The content section of a [Card].
pub struct CardContent;

impl CardContent {
    /// Creates a new [CardContent] view.
    pub fn new(cx: &mut Context, content: impl FnOnce(&mut Context)) -> Handle<Self> {
        Self.build(cx, content)
    }
}

impl View for CardContent {
    fn element(&self) -> Option<&'static str> {
        Some("card-content")
    }
}

/// The footer section of a [Card].
pub struct CardFooter;

impl CardFooter {
    /// Creates a new [CardFooter] view.
    pub fn new(cx: &mut Context, content: impl FnOnce(&mut Context)) -> Handle<Self> {
        Self.build(cx, content)
    }
}

impl View for CardFooter {
    fn element(&self) -> Option<&'static str> {
        Some("card-footer")
    }
}
