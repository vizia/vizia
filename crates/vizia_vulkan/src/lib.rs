//! Headless Vulkan backend for Vizia GUI framework.
//!
//! This crate provides Vulkan integration for Vizia applications, allowing
//! them to render GUI elements to Vulkan image views in a headless manner
//! without requiring a windowing system.
//!
//! # Architecture
//!
//! One `VulkanApplication` manages a single Vizia `Context` (with shared
//! fonts, images, themes) and can render to multiple `RenderTarget`s
//! (ImageViews). This matches how vizia's winit backend works.
//!
//! # Example
//!
//! ```ignore
//! use vizia_vulkan::prelude::*;
//!
//! // Create shared Vulkan state (cheap to clone)
//! let shared = VulkanState::new(
//!     entry, instance, physical_device, device, queue, queue_family_index, ash::vk::API_VERSION_1_3
//! ).expect("Failed to create Vulkan state");
//!
//! // Create application with UI content
//! let mut app = VulkanApplication::new(
//!     shared,
//!     |cx| {
//!         Label::new(cx, "Hello, Vulkan!");
//!     },
//! );
//!
//! // Add a render target (ImageView provided by user)
//! let window = app.add_render_target(RenderTarget::new(image_view, extent, format));
//!
//! // Process events
//! app.handle_event(WindowEvent::MouseMove { x: 100.0, y: 200.0 }, Some(window));
//!
//! // Update vizia systems (layout, style, etc.)
//! if app.update() {
//!     // Render to all targets
//!     app.render();
//! }
//! ```

pub mod application;
pub mod prelude;
pub mod render_target;
pub mod state;

pub use application::VulkanApplication;
pub use render_target::RenderTarget;
pub use state::State;
