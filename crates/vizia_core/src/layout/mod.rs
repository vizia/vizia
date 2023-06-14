//! Vizia uses [morphorm](https://github.com/vizia/morphorm) for layout.
//!
//! # Stacks
//! By default, vizia will position the children of a view one after another into a vertical column called a stack.
//! The [`layout_type()`](crate::modifiers::LayoutModifiers::layout_type) modifier (or `layout-type` css property) is used to determine how a container positions its children,
//! and can be used to select between a vertical `Column`, horizontal `Row`, or `Grid`.
//!
//! The [`position_type()`](crate::modifiers::LayoutModifiers::position_type) modifier (or `position-type` css property) is used to determine whether a child view respects the
//! stack position determined by the parent (`parent-directed`), or whether to position itself relative to the top-left of its parent
//! and ignore its siblings (`self-directed`).
//!
//! # Space
//! The position of views is modified by adding space to the sides of a view. Space can be added to the `left`, `right`, `top`, and `bottom`
//! of a view, or to all sides simultaneously with the `space` modifier/ css property.
//!
//! Spacing is specified in [`Units`], which has four variants:
//! - [`Pixels`](Units::Pixels) - Specifies the space as a fixed number of logical pixels. This value is scaled with the scale factor of the window.
//! - `Percentage` - Specifies the space as a percentage of the parent size in the same axis, so parent width for `left` and `right` space
//! and parent height for `top` and `bottom` space.
//! - `Stretch` - Specifies the space as a ratio of the remaining free space of the parent. This is best understood with an example.
//! Let's say the parent is 400px wide and the child is 200px wide. The `left` and `right` space of the child are both set to `Stretch(1.0)`.
//! This means that the ratio for each is 1/2, because each has stretch 1.0 and the total stretch factor in that axis is 2.0 (1.0 + 1.0).
//! The remaining free space is the parent width minus any fixed space and size of the child, in this case the child width, so 400.0 - 200.0 = 200.0.
//! Now the computed space for the `left` and `right` sides is 1/2 of the remaining free space, so 200.0 / 2.0 = 100.0.
//! If the `left` space had been `Stretch(3.0)`, the ratio would have been 3/4 for `left` and 1/4 for `right` and the computed space would have
//! been `150.0` for `left` and `50.0` for right.
//! - `Auto` - The spacing is determined by the corresponding `child_space` of the parent. So `left` would be determined by the parent `child_left` etc.
//!
//! # Child Space
pub(crate) mod cache;
pub(crate) mod node;

pub use morphorm::{LayoutType, PositionType, Units};

pub mod bounds;

pub use bounds::*;

pub use cache::GeoChanged;
