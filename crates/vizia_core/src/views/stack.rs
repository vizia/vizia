use crate::prelude::*;

/// A view which arranges its children into a vertical stack (column).
///
///
pub struct VStack {}

impl VStack {
    /// Creates a new [VStack].
    pub fn new<F>(cx: &mut Context, content: F) -> Handle<Self>
    where
        F: FnOnce(&mut Context),
    {
        Self {}
            .build(cx, |cx| {
                (content)(cx);
            })
            .role(Role::GenericContainer)
    }
}

impl View for VStack {
    fn element(&self) -> Option<&'static str> {
        Some("vstack")
    }
}

/// A view which arranges its children into a horizontal stack (row).
pub struct HStack {}

impl HStack {
    /// Creates a new [HStack].
    pub fn new<F>(cx: &mut Context, content: F) -> Handle<Self>
    where
        F: FnOnce(&mut Context),
    {
        let layout_row = cx.state(LayoutType::Row);
        Self {}
            .build(cx, |cx| {
                (content)(cx);
            })
            .layout_type(layout_row)
            .role(Role::GenericContainer)
    }
}

impl View for HStack {
    fn element(&self) -> Option<&'static str> {
        Some("hstack")
    }
}

/// A view which overlays its children on top of each other.
pub struct ZStack {}

impl ZStack {
    /// Creates a new [ZStack].
    pub fn new<F>(cx: &mut Context, content: F) -> Handle<Self>
    where
        F: FnOnce(&mut Context),
    {
        Self {}.build(cx, |cx| {
            (content)(cx);
        })
    }
}

impl View for ZStack {
    fn element(&self) -> Option<&'static str> {
        Some("zstack")
    }
}
