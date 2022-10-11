use super::internal;
use crate::prelude::*;

/// Modifiers for changing the layout properties of a view.
pub trait LayoutModifiers: internal::Modifiable {
    modifier!(
        /// Sets the layout type of the view.
        ///
        /// The layout type controls how a parent will position any children which have `PositionType::ParentDirected`.
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
        /// # let cx = &mut Context::new();
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
        LayoutType
    );

    modifier!(
        /// Sets the position type of the view.
        ///
        /// The position type determines how a child will be positioned within a parent.
        ///
        /// - `PositionType::ParentDirected` - The child will be positioned relative to its siblings in a stack
        /// (if parent layout type is `Row` or `Column`), or relative to its grid position (if parent layout type is `Grid`).
        /// - `PositionType::SelfDirected` - The child will be positioned relative to the top-left corner of its parents bounding box
        /// and will ignore its siblings or grid position. This is approximately equivalent to absolute positioning.
        ///
        /// # Example
        /// ```
        /// # use vizia_core::prelude::*;
        /// # let cx = &mut Context::new();
        /// Element::new(cx).position_type(PositionType::SelfDirected);
        /// ```
        position_type,
        PositionType
    );

    modifier!(
        /// Sets the space on the left side of the view.
        ///
        /// The left space, along with the right space, determines the horizontal position of a view.
        ///
        /// - `Units::Pixels(...)` - The left space will be a fixed number of points. This will scale with the DPI of the target display.
        /// - `Units::Percentage(...)` - The left space will be a proportion of the parent width.
        /// - `Units::Stretch(...)` - The left space will be a ratio of the remaining free space, see [`Units`](crate::prelude::Units).
        /// - `Units::Auto` - The left space will be determined by the parent `child-left`, see [`child_left`](crate::prelude::LayoutModifiers::left).
        ///
        /// # Example
        /// ```
        /// # use vizia_core::prelude::*;
        /// # let cx = &mut Context::new();
        /// Element::new(cx).left(Units::Pixels(100.0));
        /// ```
        left,
        Units
    );

    modifier!(
        /// Sets the space on the right side of the view.
        ///
        /// The right space, along with the left space, determines the horizontal position of a view.
        ///
        /// - `Units::Pixels(...)` - The right space will be a fixed number of points. This will scale with the DPI of the target display.
        /// - `Units::Percentage(...)` - The right space will be a proportion of the parent width.
        /// - `Units::Stretch(...)` - The right space will be a ratio of the remaining free space, see [`Units`](crate::prelude::Units).
        /// - `Units::Auto` - The right space will be determined by the parent `child-left`, see [`child_left`](crate::prelude::LayoutModifiers::left).
        ///
        /// # Example
        /// ```
        /// # use vizia_core::prelude::*;
        /// # let cx = &mut Context::new();
        /// Element::new(cx).right(Units::Pixels(100.0));
        /// ```
        right,
        Units
    );

    modifier!(top, Units);

    modifier!(bottom, Units);

    fn space<U: Into<Units>>(mut self, value: impl Res<U>) -> Self {
        let entity = self.entity();
        value.set_or_bind(self.context(), entity, |cx, entity, v| {
            let value = v.into();
            cx.style.left.insert(entity, value);
            cx.style.right.insert(entity, value);
            cx.style.top.insert(entity, value);
            cx.style.bottom.insert(entity, value);

            cx.need_relayout();
            cx.need_redraw();
        });

        self
    }

    modifier!(width, Units);

    modifier!(height, Units);

    fn size<U: Into<Units>>(mut self, value: impl Res<U>) -> Self {
        let entity = self.entity();
        value.set_or_bind(self.context(), entity, |cx, entity, v| {
            let value = v.into();
            cx.style.width.insert(entity, value);
            cx.style.height.insert(entity, value);

            cx.need_relayout();
            cx.need_redraw();
        });

        self
    }

    modifier!(child_left, Units);

    modifier!(child_right, Units);

    modifier!(child_top, Units);

    modifier!(child_bottom, Units);

    fn child_space<U: Into<Units>>(mut self, value: impl Res<U>) -> Self {
        let entity = self.entity();
        value.set_or_bind(self.context(), entity, |cx, entity, v| {
            let value = v.into();
            cx.style.child_left.insert(entity, value);
            cx.style.child_right.insert(entity, value);
            cx.style.child_top.insert(entity, value);
            cx.style.child_bottom.insert(entity, value);

            cx.need_relayout();
            cx.need_redraw();
        });

        self
    }

    modifier!(row_between, Units);

    modifier!(col_between, Units);

    modifier!(min_width, Units);

    modifier!(min_height, Units);

    fn min_size<U: Into<Units>>(mut self, value: impl Res<U>) -> Self {
        let entity = self.entity();
        value.set_or_bind(self.context(), entity, |cx, entity, v| {
            let value = v.into();
            cx.style.min_width.insert(entity, value);
            cx.style.min_height.insert(entity, value);

            cx.need_relayout();
            cx.need_redraw();
        });

        self
    }

    modifier!(max_width, Units);

    modifier!(max_height, Units);

    fn max_size<U: Into<Units>>(mut self, value: impl Res<U>) -> Self {
        let entity = self.entity();
        value.set_or_bind(self.context(), entity, |cx, entity, v| {
            let value = v.into();
            cx.style.max_width.insert(entity, value);
            cx.style.max_height.insert(entity, value);

            cx.need_relayout();
            cx.need_redraw();
        });

        self
    }

    modifier!(min_left, Units);

    modifier!(min_right, Units);

    modifier!(min_top, Units);

    modifier!(min_bottom, Units);

    fn min_space<U: Into<Units>>(mut self, value: impl Res<U>) -> Self {
        let entity = self.entity();
        value.set_or_bind(self.context(), entity, |cx, entity, v| {
            let value = v.into();
            cx.style.min_left.insert(entity, value);
            cx.style.min_right.insert(entity, value);
            cx.style.min_top.insert(entity, value);
            cx.style.min_bottom.insert(entity, value);

            cx.need_relayout();
            cx.need_redraw();
        });

        self
    }

    modifier!(max_left, Units);

    modifier!(max_right, Units);

    modifier!(max_top, Units);

    modifier!(max_bottom, Units);

    fn max_space<U: Into<Units>>(mut self, value: impl Res<U>) -> Self {
        let entity = self.entity();
        value.set_or_bind(self.context(), entity, |cx, entity, v| {
            let value = v.into();
            cx.style.max_left.insert(entity, value);
            cx.style.max_right.insert(entity, value);
            cx.style.max_top.insert(entity, value);
            cx.style.max_bottom.insert(entity, value);

            cx.need_relayout();
            cx.need_redraw();
        });

        self
    }

    fn grid_rows(mut self, rows: Vec<Units>) -> Self {
        let entity = self.entity();
        self.context().style.grid_rows.insert(entity, rows);
        self.context().need_relayout();
        self
    }

    fn grid_cols(mut self, cols: Vec<Units>) -> Self {
        let entity = self.entity();
        self.context().style.grid_cols.insert(entity, cols);
        self.context().need_relayout();
        self
    }

    modifier!(row_index, usize);
    modifier!(row_span, usize);
    modifier!(col_index, usize);
    modifier!(col_span, usize);
}

impl<'a, V: View> LayoutModifiers for Handle<'a, V> {}
