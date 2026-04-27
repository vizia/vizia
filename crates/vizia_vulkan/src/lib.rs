//! Headless Vulkan backend for Vizia GUI framework.
//!
//! This crate provides Vulkan integration for Vizia applications, allowing
//! them to render GUI elements to Vulkan images in a headless manner
//! without requiring a windowing system.
//!
//! One `VulkanApplication` manages a single Vizia `Context` and can render to `RenderTarget`.
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
//! // Create a render target
//! let ui_target = RenderTarget::new(ui_image, swapchain_extent, swapchain_format, 1);
//!
//! // Create application with UI content
//! let mut app = VulkanApplication::new(
//!     shared,
//!     ui_target,
//!     1.0, // Scale factor (probably want to use system scale for HUDs and 1.0 for off-screen needs)
//!     |cx| {
//!         Label::new(cx, "Hello, Vulkan!");
//!     },
//! );
//!
//! app.handle_event(WindowEvent::MouseMove { x: 100.0, y: 200.0 }, Some(window));
//!
//! // Update vizia systems (layout, style, etc.)
//! if app.update() {
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
