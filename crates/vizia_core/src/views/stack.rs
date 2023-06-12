use morphorm::LayoutType;

use crate::prelude::*;

/// A view which arranges its children into a vertical stack (column).
///
///
pub struct VStack {}

impl VStack {
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
    pub fn new<F>(cx: &mut Context, content: F) -> Handle<Self>
    where
        F: FnOnce(&mut Context),
    {
        Self {}
            .build(cx, |cx| {
                (content)(cx);
            })
            .layout_type(LayoutType::Row)
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
