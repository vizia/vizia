use crate::prelude::*;

/// A view which arranges its children into a grid.
///
///
pub struct Grid {}

impl Grid {
    /// Creates a new [Grid].
    pub fn new<F>(
        cx: &mut Context,
        grid_columns: Vec<Units>,
        grid_rows: Vec<Units>,
        content: F,
    ) -> Handle<Self>
    where
        F: FnOnce(&mut Context),
    {
        Self {}
            .build(cx, |cx| {
                (content)(cx);
            })
            .layout_type(LayoutType::Grid)
            .grid_columns(grid_columns)
            .grid_rows(grid_rows)
            .role(Role::GenericContainer)
    }
}

impl View for Grid {
    fn element(&self) -> Option<&'static str> {
        Some("grid")
    }
}
