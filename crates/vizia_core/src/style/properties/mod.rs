//! A list of supported style and layout properties.
//!
//! # Contents
//! #### Layout Properties
//! - [`layout-type`](#layout-type)
//! - [`position-type`](#position-type)
//! - [`size`](#size)
//!     - [`width`](#width)
//!     - [`height`](#width)
//! - [`space`](#space)
//!     - [`left`](#left)
//!     - [`right](#right)
//!     - [`top`](#top)
//!     - [`bottom`](#bottom)
//! - [`child-space`](#child-space)
//!     - [`child-left`](#child-left)
//!     - [`child-right`](#child-right)
//!     - [`child-top`](#child-top)
//!     - [`child-bottom`](#child-bottom)
//! - [`row-between`](#row-between)
//! - [`col-between`](#col-between)
//! ### Style Properties
//! - [`display`](#display)
//! - [`visibility`](#visibility)
//! - [`z-order`](#z-order)
//! - [`overflow`](#overflow)
//! - Background
//!     - [`background-color`](#background-color)
//!     - [`background-image`](#background-image)
//! - Border
//!     - [`border-width`](#border-width)
//!     - [`border-color`](#border-color)
//!     - [`border-radius`](#border-radius)
//!     - [`border-corner-shape`](#border-corner-shape)
//!     - [`outline`](#outline)
//!         -[`outline-width`](#outline-width)
//!         -[`outline-color`](#outline-color)
//!         -[`outline-offset`](#outline-offset)
//!
//!
//!
//! # Layout Properties
//!
//! ## [`layout-type`](crate::modifiers::LayoutModifiers::layout_type)
//! Controls how a parent will position any children which have `PositionType::ParentDirected`.
//!
//! - `LayoutType::Row` - Parent will stack its children horizontally.
//! - `LayoutType::Column` - (default) Parent will stack its children vertically.
//! - `LayoutType::Grid` - The position of children is determine by the grid properties.
//!
//! <details>
//!     <summary>Inline</summary>
//!
//! ```no_run
//! # use vizia_core::prelude::*;
//! # let cx = &mut Context::default();
//! # use vizia_winit::application::Application;
//! # Application::new(|cx|{
//! // Sets the width of the label to be 100 pixels.
//! HStack::new(cx, |_|{})
//!     .layout_type(LayoutType::Column);
//! # }).run();
//! ```
//! </details>
//!
//! ### CSS
//! ```css
//! hstack {
//!     layout-type: column;
//! }
//! ```
//!
//! ## [`position-type`](crate::modifiers::LayoutModifiers::position_type)
//! Determines how a child will be positioned within a parent.
//!
//! - `PositionType::ParentDirected` - The child will be positioned relative to its siblings in a stack
//! (if parent layout type is `Row` or `Column`), or relative to its grid position (if parent layout type is `Grid`).
//! - `PositionType::SelfDirected` - The child will be positioned relative to the top-left corner of its parents bounding box
//! and will ignore its siblings or grid position. This is approximately equivalent to absolute positioning.
//!
//! ### Inline
//! ```no_run
//! # use vizia_core::prelude::*;
//! # let cx = &mut Context::default();
//! # use vizia_winit::application::Application;
//! # Application::new(|cx|{
//! // Sets the width of the label to be 100 pixels.
//! Element::new(cx)
//!     .position_type(PositionType::SelfDirected);
//! # }).run();
//! ```
//! ### CSS
//! ```css
//! element {
//!     position-type: self-directed;
//! }
//! ```
//!
//! ## [`width`](crate::modifiers::LayoutModifiers::width)
//! Sets the width of the view.
//!
//! ## [`height`](crate::modifiers::LayoutModifiers::height)
//! Sets the width of the view.
