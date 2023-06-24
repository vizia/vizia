use super::internal;
use crate::prelude::*;
use crate::style::SystemFlags;

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
        /// - `PositionType::ParentDirected` - The child will be positioned relative to its siblings in a stack
        /// (if parent layout type is `Row` or `Column`), or relative to its grid position (if parent layout type is `Grid`).
        /// - `PositionType::SelfDirected` - The child will be positioned relative to the top-left corner of its parents bounding box
        /// and will ignore its siblings or grid position. This is approximately equivalent to absolute positioning.
        ///
        /// # Example
        /// ```
        /// # use vizia_core::prelude::*;
        /// # let cx = &mut Context::default();
        /// Element::new(cx).position_type(PositionType::SelfDirected);
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
        /// - `Units::Auto` - The left space will be determined by the parent `child-left`, see [`child_left`](crate::prelude::LayoutModifiers::left).
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
        /// - `Units::Auto` - The right space will be determined by the parent `child-left`, see [`child_left`](crate::prelude::LayoutModifiers::left).
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
        /// - `Units::Auto` - The top space will be determined by the parent `child-left`, see [`child_left`](crate::prelude::LayoutModifiers::left).
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
        /// - `Units::Auto` - The bottom space will be determined by the parent `child-left`, see [`child_left`](crate::prelude::LayoutModifiers::left).
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
        value.set_or_bind(self.context(), entity, |cx, v| {
            let value = v.into();
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
    fn size<U: Into<Units>>(mut self, value: impl Res<U>) -> Self {
        let entity = self.entity();
        value.set_or_bind(self.context(), entity, |cx, v| {
            let value = v.into();
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
        child_left,
        Units,
        SystemFlags::RELAYOUT
    );

    modifier!(
        /// Sets the space between the right side of the view and the right side of its children.
        ///
        /// Applies only to child views which have a `right` property set to `Auto`.
        child_right,
        Units,
        SystemFlags::RELAYOUT
    );

    modifier!(
        /// Sets the space between the top side of the view and the top side of its children.
        ///
        /// Applies only to child views which have a `top` property set to `Auto`.
        child_top,
        Units,
        SystemFlags::RELAYOUT
    );

    modifier!(
        /// Sets the space between the bottom side of the view and the bottom side of its children.
        ///
        /// Applies only to child views which have a `bottom` property set to `Auto`.
        child_bottom,
        Units,
        SystemFlags::RELAYOUT
    );

    /// Sets the space between the vew and its children.
    ///
    /// The child_space works by overriding the `Auto` space properties of its children.
    fn child_space<U: Into<Units>>(mut self, value: impl Res<U>) -> Self {
        let entity = self.entity();
        value.set_or_bind(self.context(), entity, |cx, v| {
            let value = v.into();
            cx.style.child_left.insert(cx.current, value);
            cx.style.child_right.insert(cx.current, value);
            cx.style.child_top.insert(cx.current, value);
            cx.style.child_bottom.insert(cx.current, value);

            cx.style.needs_relayout();
        });

        self
    }

    modifier!(
        /// Sets the space between the views children in a vertical stack.
        row_between,
        Units,
        SystemFlags::RELAYOUT
    );

    modifier!(
        /// Sets the space between the views children in a horizontal stack.
        col_between,
        Units,
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
        value.set_or_bind(self.context(), entity, |cx, v| {
            let value = v.into();
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
    fn max_size<U: Into<Units>>(mut self, value: impl Res<U>) -> Self {
        let entity = self.entity();
        value.set_or_bind(self.context(), entity, |cx, v| {
            let value = v.into();
            cx.style.max_width.insert(cx.current, value);
            cx.style.max_height.insert(cx.current, value);

            cx.needs_relayout();
        });

        self
    }

    modifier!(
        /// Sets the minimum left space of the view.
        min_left,
        Units,
        SystemFlags::RELAYOUT
    );

    modifier!(
        /// Sets the minimum right space of the view.
        min_right,
        Units,
        SystemFlags::RELAYOUT
    );

    modifier!(
        /// Sets the minimum top space of the view.
        min_top,
        Units,
        SystemFlags::RELAYOUT
    );

    modifier!(
        /// Sets the minimum bottom space of the view.
        min_bottom,
        Units,
        SystemFlags::RELAYOUT
    );

    /// Sets the minimum space for all sides of the view.
    fn min_space<U: Into<Units>>(mut self, value: impl Res<U>) -> Self {
        let entity = self.entity();
        value.set_or_bind(self.context(), entity, |cx, v| {
            let value = v.into();
            cx.style.min_left.insert(cx.current, value);
            cx.style.min_right.insert(cx.current, value);
            cx.style.min_top.insert(cx.current, value);
            cx.style.min_bottom.insert(cx.current, value);

            cx.style.needs_relayout();
        });

        self
    }

    modifier!(
        /// Sets the maximum left space of the view.
        max_left,
        Units,
        SystemFlags::RELAYOUT
    );

    modifier!(
        /// Sets the maximum right space of the view.
        max_right,
        Units,
        SystemFlags::RELAYOUT
    );

    modifier!(
        /// Sets the maximum top space of the view.
        max_top,
        Units,
        SystemFlags::RELAYOUT
    );

    modifier!(
        /// Sets the maximum bottom space of the view.
        max_bottom,
        Units,
        SystemFlags::RELAYOUT
    );

    /// Sets the maximum space for all sides of the view.
    fn max_space<U: Into<Units>>(mut self, value: impl Res<U>) -> Self {
        let entity = self.entity();
        value.set_or_bind(self.context(), entity, |cx, v| {
            let value = v.into();
            cx.style.max_left.insert(cx.current, value);
            cx.style.max_right.insert(cx.current, value);
            cx.style.max_top.insert(cx.current, value);
            cx.style.max_bottom.insert(cx.current, value);

            cx.style.needs_relayout();
        });

        self
    }
}

impl<'a, V: View> LayoutModifiers for Handle<'a, V> {}
