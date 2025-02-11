use super::internal;
use crate::prelude::*;

/// Modifiers for changing the layout properties of a view.
pub trait LayoutModifiers: internal::Modifiable {
    modifier!(
        /// Sets the layout type of the view.
        ///
        /// The layout type controls how a parent will position any children which have `Position::Relative`.
        /// Accepts any value, or lens to a target, with a type which can be converted into `LayoutType`.
        ///
        /// There are three variants:
        /// - `LayoutType::Row` - Parent will stack its children horizontally.
        /// - `LayoutType::Column` - (default) Parent will stack its children vertically.
        /// - `LayoutType::Grid` - The position of children is determine by the grid properties.
        ///
        /// # Example
        /// ```
        /// # use vizia_core::prelude::*;
        /// # let cx = &mut Context::default();
        /// #[derive(Lens, Model, Setter)]
        /// pub struct AppData {
        ///     layout_type: LayoutType,
        /// }
        ///
        /// # AppData {
        /// #   layout_type: LayoutType::Row,
        /// # }.build(cx);
        ///
        /// Element::new(cx).layout_type(LayoutType::Row);  // Value of type `LayoutType`.
        /// Element::new(cx).layout_type(AppData::layout_type); // Lens to target of type `LayoutType`.
        /// ```
        layout_type,
        LayoutType,
        SystemFlags::RELAYOUT
    );

    modifier!(
        /// Sets the position type of the view.
        ///
        /// The position type determines how a child will be positioned within a parent.
        ///
        /// - `Position::Relative` - The child will be positioned relative to its siblings in a stack
        /// (if parent layout type is `Row` or `Column`), or relative to its grid position (if parent layout type is `Grid`).
        /// - `Position::Absolute` - The child will be positioned relative to the top-left corner of its parents bounding box
        /// and will ignore its siblings or grid position. This is approximately equivalent to absolute positioning.
        ///
        /// # Example
        /// ```
        /// # use vizia_core::prelude::*;
        /// # let cx = &mut Context::default();
        /// Element::new(cx).position_type(PositionType::Absolute);
        /// ```
        position_type,
        PositionType,
        SystemFlags::RELAYOUT
    );

    modifier!(
        /// Sets the space on the left side of the view.
        ///
        /// The left space, along with the right space, determines the horizontal position of a view.
        ///
        /// - `Units::Pixels(...)` - The left space will be a fixed number of points. This will scale with the DPI of the target display.
        /// - `Units::Percentage(...)` - The left space will be a proportion of the parent width.
        /// - `Units::Stretch(...)` - The left space will be a ratio of the remaining free space, see [`Units`](crate::prelude::Units).
        /// - `Units::Auto` - The left space will be determined by the parent `padding-left`, see [`padding_left`](crate::prelude::LayoutModifiers::left).
        ///
        /// # Example
        /// ```
        /// # use vizia_core::prelude::*;
        /// # let cx = &mut Context::default();
        /// Element::new(cx).left(Units::Pixels(100.0));
        /// ```
        left,
        Units,
        SystemFlags::RELAYOUT
    );

    modifier!(
        /// Sets the space on the right side of the view.
        ///
        /// The right space, along with the left space, determines the horizontal position of a view.
        ///
        /// - `Units::Pixels(...)` - The right space will be a fixed number of points. This will scale with the DPI of the target display.
        /// - `Units::Percentage(...)` - The right space will be a proportion of the parent width.
        /// - `Units::Stretch(...)` - The right space will be a ratio of the remaining free space, see [`Units`](crate::prelude::Units).
        /// - `Units::Auto` - The right space will be determined by the parent `padding-left`, see [`padding_left`](crate::prelude::LayoutModifiers::left).
        ///
        /// # Example
        /// ```
        /// # use vizia_core::prelude::*;
        /// # let cx = &mut Context::default();
        /// Element::new(cx).right(Units::Pixels(100.0));
        /// ```
        right,
        Units,
        SystemFlags::RELAYOUT
    );

    modifier!(
        /// Sets the space on the top side of the view.
        ///
        /// The top space, along with the bottom space, determines the vertical position of a view.
        ///
        /// - `Units::Pixels(...)` - The top space will be a fixed number of points. This will scale with the DPI of the target display.
        /// - `Units::Percentage(...)` - The top space will be a proportion of the parent width.
        /// - `Units::Stretch(...)` - The top space will be a ratio of the remaining free space, see [`Units`](crate::prelude::Units).
        /// - `Units::Auto` - The top space will be determined by the parent `padding-left`, see [`padding_left`](crate::prelude::LayoutModifiers::left).
        ///
        /// # Example
        /// ```
        /// # use vizia_core::prelude::*;
        /// # let cx = &mut Context::default();
        /// Element::new(cx).top(Units::Pixels(100.0));
        /// ```
        top,
        Units,
        SystemFlags::RELAYOUT
    );

    modifier!(
        /// Sets the space on the bottom side of the view.
        ///
        /// The bottom space, along with the top space, determines the vertical position of a view.
        ///
        /// - `Units::Pixels(...)` - The bottom space will be a fixed number of points. This will scale with the DPI of the target display.
        /// - `Units::Percentage(...)` - The bottom space will be a proportion of the parent width.
        /// - `Units::Stretch(...)` - The bottom space will be a ratio of the remaining free space, see [`Units`](crate::prelude::Units).
        /// - `Units::Auto` - The bottom space will be determined by the parent `padding-left`, see [`padding_left`](crate::prelude::LayoutModifiers::left).
        ///
        /// # Example
        /// ```
        /// # use vizia_core::prelude::*;
        /// # let cx = &mut Context::default();
        /// Element::new(cx).bottom(Units::Pixels(100.0));
        /// ```
        bottom,
        Units,
        SystemFlags::RELAYOUT
    );

    /// Sets the space for all sides of the view.
    fn space<U: Into<Units>>(mut self, value: impl Res<U>) -> Self {
        let entity = self.entity();
        let current = self.current();
        self.context().with_current(current, |cx| {
            value.set_or_bind(cx, entity, |cx, v| {
                let value = v.get(cx).into();
                cx.style.left.insert(cx.current, value);
                cx.style.right.insert(cx.current, value);
                cx.style.top.insert(cx.current, value);
                cx.style.bottom.insert(cx.current, value);

                cx.style.needs_relayout();
            });
        });

        self
    }

    modifier!(
        /// Sets the width of the view.
        width,
        Units,
        SystemFlags::RELAYOUT
    );

    modifier!(
        /// Sets the height of the view.
        height,
        Units,
        SystemFlags::RELAYOUT
    );

    /// Sets the width and height of the view.
    fn size<U: Into<Units>>(mut self, value: impl Res<U>) -> Self {
        let entity = self.entity();
        let current = self.current();
        self.context().with_current(current, |cx| {
            value.set_or_bind(cx, entity, move |cx, v| {
                let value = v.get(cx).into();
                cx.style.width.insert(cx.current, value);
                cx.style.height.insert(cx.current, value);

                cx.style.needs_relayout();
            });
        });

        self
    }

    modifier!(
        /// Sets the space between the left side of the view and the left side of its children.
        ///
        /// Applies only to child views which have a `left` property set to `Auto`.
        padding_left,
        Units,
        SystemFlags::RELAYOUT
    );

    modifier!(
        /// Sets the space between the right side of the view and the right side of its children.
        ///
        /// Applies only to child views which have a `right` property set to `Auto`.
        padding_right,
        Units,
        SystemFlags::RELAYOUT
    );

    modifier!(
        /// Sets the space between the top side of the view and the top side of its children.
        ///
        /// Applies only to child views which have a `top` property set to `Auto`.
        padding_top,
        Units,
        SystemFlags::RELAYOUT
    );

    modifier!(
        /// Sets the space between the bottom side of the view and the bottom side of its children.
        ///
        /// Applies only to child views which have a `bottom` property set to `Auto`.
        padding_bottom,
        Units,
        SystemFlags::RELAYOUT
    );

    modifier!(
        /// Set the alignment of the view.
        alignment,
        Alignment,
        SystemFlags::RELAYOUT
    );

    /// Sets the space between the vew and its children.
    ///
    /// The child_space works by overriding the `Auto` space properties of its children.
    fn padding<U: Into<Units>>(mut self, value: impl Res<U>) -> Self {
        let entity = self.entity();
        let current = self.current();
        self.context().with_current(current, |cx| {
            value.set_or_bind(cx, entity, move |cx, v| {
                let value = v.get(cx).into();
                cx.style.padding_left.insert(cx.current, value);
                cx.style.padding_right.insert(cx.current, value);
                cx.style.padding_top.insert(cx.current, value);
                cx.style.padding_bottom.insert(cx.current, value);

                cx.style.needs_relayout();
            });
        });

        self
    }

    modifier!(
        /// Sets the space between the views children in the vertical direction.
        vertical_gap,
        Units,
        SystemFlags::RELAYOUT
    );

    modifier!(
        /// Sets the space between the views children in the horizontal direction.
        horizontal_gap,
        Units,
        SystemFlags::RELAYOUT
    );

    /// Sets the space between the views children in both the horizontal and vertical directions.
    fn gap<U: Into<Units>>(mut self, value: impl Res<U>) -> Self {
        let entity = self.entity();
        let current = self.current();
        self.context().with_current(current, |cx| {
            value.set_or_bind(cx, entity, move |cx, v| {
                let value = v.get(cx).into();
                cx.style.horizontal_gap.insert(cx.current, value);
                cx.style.vertical_gap.insert(cx.current, value);

                cx.style.needs_relayout();
            });
        });

        self
    }

    modifier!(
        /// Set the vertical scroll position of the view.
        vertical_scroll,
        f32,
        SystemFlags::RELAYOUT
    );

    modifier!(
        /// Set the horizontal scroll position of the view.
        horizontal_scroll,
        f32,
        SystemFlags::RELAYOUT
    );

    modifier!(
        /// Sets the minimum width of the view.
        min_width,
        Units,
        SystemFlags::RELAYOUT
    );

    modifier!(
        /// Sets the minimum height of the view.
        min_height,
        Units,
        SystemFlags::RELAYOUT
    );

    /// Sets the minimum width and minimum height of the view.
    fn min_size<U: Into<Units>>(mut self, value: impl Res<U>) -> Self {
        let entity = self.entity();
        let current = self.current();
        self.context().with_current(current, |cx| {
            value.set_or_bind(cx, entity, move |cx, v| {
                let value = v.get(cx).into();
                cx.style.min_width.insert(cx.current, value);
                cx.style.min_height.insert(cx.current, value);

                cx.needs_relayout();
            });
        });

        self
    }

    modifier!(
        /// Sets the maximum width of the view.
        max_width,
        Units,
        SystemFlags::RELAYOUT
    );

    modifier!(
        /// Sets the maximum height of the view.
        max_height,
        Units,
        SystemFlags::RELAYOUT
    );

    /// Sets the maximum width and maximum height of the view.
    fn max_size<U: Into<Units>>(mut self, value: impl Res<U>) -> Self {
        let entity = self.entity();
        let current = self.current();
        self.context().with_current(entity, |cx| {
            value.set_or_bind(cx, current, move |cx, v| {
                let value = v.get(cx).into();
                cx.style.max_width.insert(cx.current, value);
                cx.style.max_height.insert(cx.current, value);

                cx.needs_relayout();
            });
        });

        self
    }

    modifier!(
        /// Sets the minimum horizontal space between the children of the view.
        min_horizontal_gap,
        Units,
        SystemFlags::RELAYOUT
    );

    modifier!(
        /// Sets the minimum vertical space between the children of the view.
        min_vertical_gap,
        Units,
        SystemFlags::RELAYOUT
    );

    /// Sets the minimum horizontal and minimum vertical space between the children of the view.
    fn min_gap<U: Into<Units>>(mut self, value: impl Res<U>) -> Self {
        let entity = self.entity();
        let current = self.current();
        self.context().with_current(current, |cx| {
            value.set_or_bind(cx, entity, move |cx, v| {
                let value = v.get(cx).into();
                cx.style.min_horizontal_gap.insert(cx.current, value);
                cx.style.min_vertical_gap.insert(cx.current, value);

                cx.needs_relayout();
            });
        });

        self
    }

    modifier!(
        /// Sets the maximum horizontal space between the children of the view.
        max_horizontal_gap,
        Units,
        SystemFlags::RELAYOUT
    );

    modifier!(
        /// Sets the maximum vertical space between the children of the view.
        max_vertical_gap,
        Units,
        SystemFlags::RELAYOUT
    );

    /// Sets the maximum horizontal and maximum vertical space between the children of the view.
    fn max_gap<U: Into<Units>>(mut self, value: impl Res<U>) -> Self {
        let entity = self.entity();
        let current = self.current();
        self.context().with_current(current, |cx| {
            value.set_or_bind(cx, entity, move |cx, v| {
                let value = v.get(cx).into();
                cx.style.max_horizontal_gap.insert(cx.current, value);
                cx.style.max_vertical_gap.insert(cx.current, value);

                cx.needs_relayout();
            });
        });

        self
    }

    modifier!(
        /// Sets the grid columns for a grid layout.
        grid_columns,
        Vec<Units>,
        SystemFlags::RELAYOUT
    );

    modifier!(
        /// Sets the grid rows for a grid layout.
        grid_rows,
        Vec<Units>,
        SystemFlags::RELAYOUT
    );

    /// Sets the grid column start for a grid layout.
    fn column_start(mut self, value: impl Res<usize>) -> Self {
        let entity = self.entity();
        let current = self.current();
        self.context().with_current(current, |cx| {
            value.set_or_bind(cx, entity, move |cx, v| {
                let value = v.get(cx);
                cx.style.column_start.insert(cx.current, value);

                cx.needs_relayout();
            });
        });

        self
    }

    /// Sets the grid column span for a grid layout.
    fn column_span(mut self, value: impl Res<usize>) -> Self {
        let entity = self.entity();
        let current = self.current();
        self.context().with_current(current, |cx| {
            value.set_or_bind(cx, entity, move |cx, v| {
                let value = v.get(cx);
                cx.style.column_span.insert(cx.current, value);

                cx.needs_relayout();
            });
        });

        self
    }

    /// Sets the grid row start for a grid layout.
    fn row_start(mut self, value: impl Res<usize>) -> Self {
        let entity = self.entity();
        let current = self.current();
        self.context().with_current(current, |cx| {
            value.set_or_bind(cx, entity, move |cx, v| {
                let value = v.get(cx);
                cx.style.row_start.insert(cx.current, value);

                cx.needs_relayout();
            });
        });

        self
    }

    /// Sets the grid row span for a grid layout.
    fn row_span(mut self, value: impl Res<usize>) -> Self {
        let entity = self.entity();
        let current = self.current();
        self.context().with_current(current, |cx| {
            value.set_or_bind(cx, entity, move |cx, v| {
                let value = v.get(cx);
                cx.style.row_span.insert(cx.current, value);

                cx.needs_relayout();
            });
        });

        self
    }
}

impl<V: View> LayoutModifiers for Handle<'_, V> {}
