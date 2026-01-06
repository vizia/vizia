use super::internal;
use crate::prelude::*;

/// Modifiers for changing the layout properties of a view.
pub trait LayoutModifiers: internal::Modifiable {
    modifier!(
        /// Sets the layout type of the view.
        ///
        /// The layout type controls how a parent will position any children which have `Position::Relative`.
        /// Accepts a `LayoutType` or `Signal<LayoutType>`.
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
        /// let layout_row = cx.state(LayoutType::Row);
        ///
        /// Element::new(cx).layout_type(layout_row);
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
        /// let position_absolute = cx.state(PositionType::Absolute);
        /// Element::new(cx).position_type(position_absolute);
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
        /// let left_100 = cx.state(Units::Pixels(100.0));
        /// Element::new(cx).left(left_100);
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
        /// let right_100 = cx.state(Units::Pixels(100.0));
        /// Element::new(cx).right(right_100);
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
        /// let top_100 = cx.state(Units::Pixels(100.0));
        /// Element::new(cx).top(top_100);
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
        /// let bottom_100 = cx.state(Units::Pixels(100.0));
        /// Element::new(cx).bottom(bottom_100);
        /// ```
        bottom,
        Units,
        SystemFlags::RELAYOUT
    );

    /// Sets the space for all sides of the view.
    fn space<U>(mut self, value: impl Res<U> + 'static) -> Self
    where
        U: Clone + Into<Units> + 'static,
    {
        let entity = self.entity();
        let current = self.current();
        internal::bind_res(self.context(), current, entity, value, move |cx, v| {
            let value = v.clone().into();
            cx.style.left.insert(cx.current, value);
            cx.style.right.insert(cx.current, value);
            cx.style.top.insert(cx.current, value);
            cx.style.bottom.insert(cx.current, value);

            cx.style.needs_relayout();
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
    fn size<U>(mut self, value: impl Res<U> + 'static) -> Self
    where
        U: Clone + Into<Units> + 'static,
    {
        let entity = self.entity();
        let current = self.current();
        internal::bind_res(self.context(), current, entity, value, move |cx, v| {
            let value = v.clone().into();
            cx.style.width.insert(cx.current, value);
            cx.style.height.insert(cx.current, value);

            cx.style.needs_relayout();
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
    fn padding<U>(mut self, value: impl Res<U> + 'static) -> Self
    where
        U: Clone + Into<Units> + 'static,
    {
        let entity = self.entity();
        let current = self.current();
        internal::bind_res(self.context(), current, entity, value, move |cx, v| {
            let value = v.clone().into();
            cx.style.padding_left.insert(cx.current, value);
            cx.style.padding_right.insert(cx.current, value);
            cx.style.padding_top.insert(cx.current, value);
            cx.style.padding_bottom.insert(cx.current, value);

            cx.style.needs_relayout();
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
    fn gap<U>(mut self, value: impl Res<U> + 'static) -> Self
    where
        U: Clone + Into<Units> + 'static,
    {
        let entity = self.entity();
        let current = self.current();
        internal::bind_res(self.context(), current, entity, value, move |cx, v| {
            let value = v.clone().into();
            cx.style.horizontal_gap.insert(cx.current, value);
            cx.style.vertical_gap.insert(cx.current, value);

            cx.style.needs_relayout();
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
    fn min_size<U>(mut self, value: impl Res<U> + 'static) -> Self
    where
        U: Clone + Into<Units> + 'static,
    {
        let entity = self.entity();
        let current = self.current();
        internal::bind_res(self.context(), current, entity, value, move |cx, v| {
            let value = v.clone().into();
            cx.style.min_width.insert(cx.current, value);
            cx.style.min_height.insert(cx.current, value);

            cx.needs_relayout();
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
    fn max_size<U>(mut self, value: impl Res<U> + 'static) -> Self
    where
        U: Clone + Into<Units> + 'static,
    {
        let entity = self.entity();
        let current = self.current();
        internal::bind_res(self.context(), current, entity, value, move |cx, v| {
            let value = v.clone().into();
            cx.style.max_width.insert(cx.current, value);
            cx.style.max_height.insert(cx.current, value);

            cx.needs_relayout();
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
    fn min_gap<U>(mut self, value: impl Res<U> + 'static) -> Self
    where
        U: Clone + Into<Units> + 'static,
    {
        let entity = self.entity();
        let current = self.current();
        internal::bind_res(self.context(), current, entity, value, move |cx, v| {
            let value = v.clone().into();
            cx.style.min_horizontal_gap.insert(cx.current, value);
            cx.style.min_vertical_gap.insert(cx.current, value);

            cx.needs_relayout();
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
    fn max_gap<U>(mut self, value: impl Res<U> + 'static) -> Self
    where
        U: Clone + Into<Units> + 'static,
    {
        let entity = self.entity();
        let current = self.current();
        internal::bind_res(self.context(), current, entity, value, move |cx, v| {
            let value = v.clone().into();
            cx.style.max_horizontal_gap.insert(cx.current, value);
            cx.style.max_vertical_gap.insert(cx.current, value);

            cx.needs_relayout();
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

    fn column_start(mut self, value: impl Res<usize> + 'static) -> Self {
        let entity = self.entity();
        let current = self.current();
        internal::bind_res(self.context(), current, entity, value, move |cx, v| {
            cx.style.column_start.insert(cx.current, *v);
            cx.needs_relayout();
        });

        self
    }

    fn column_span(mut self, value: impl Res<usize> + 'static) -> Self {
        let entity = self.entity();
        let current = self.current();
        internal::bind_res(self.context(), current, entity, value, move |cx, v| {
            cx.style.column_span.insert(cx.current, *v);
            cx.needs_relayout();
        });

        self
    }

    fn row_start(mut self, value: impl Res<usize> + 'static) -> Self {
        let entity = self.entity();
        let current = self.current();
        internal::bind_res(self.context(), current, entity, value, move |cx, v| {
            cx.style.row_start.insert(cx.current, *v);
            cx.needs_relayout();
        });

        self
    }

    fn row_span(mut self, value: impl Res<usize> + 'static) -> Self {
        let entity = self.entity();
        let current = self.current();
        internal::bind_res(self.context(), current, entity, value, move |cx, v| {
            cx.style.row_span.insert(cx.current, *v);
            cx.needs_relayout();
        });

        self
    }
}

impl<V: View> LayoutModifiers for Handle<'_, V> {}
