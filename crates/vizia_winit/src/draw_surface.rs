use std::{error::Error, fmt::Display, sync::Arc};

use skia_safe::{SamplingOptions, Surface};

use winit::{
    dpi::PhysicalSize,
    event_loop::ActiveEventLoop,
    window::{Window, WindowAttributes},
};

#[cfg(target_os = "windows")]
use winit::raw_window_handle::{HasWindowHandle, RawWindowHandle};

use vizia_core::{
    prelude::{BoundingBox, Entity},
    style::Color,
};
use vizia_window::{GraphicsBackend, WindowDescription};

#[cfg(feature = "gl")]
mod gl;

#[cfg(feature = "dx12")]
mod dx12;

#[cfg(feature = "metal")]
mod metal;

#[cfg(feature = "vulkan")]
mod vulkan;

/// A trait for graphics backends that can be used to draw to a window.
///
pub trait DrawSurface {
    /// The entity associated with this graphics backend.
    fn entity(&self) -> Entity;

    /// The window associated with this graphics backend.
    fn window(&self) -> Arc<Window>;

    /// The active graphics backend.
    fn backend(&self) -> GraphicsBackend;

    /// Mutable references to the surface and dirty surface.
    ///
    fn surfaces_mut(&mut self) -> Option<(&mut Surface, &mut Surface)>;

    /// Present the rendered frame to the window.
    ///
    /// The implementation of this method should be resilient to being called
    /// at a high frequency with potentially redundant or invalid dirty rects.
    ///
    fn swap_buffers(&mut self, dirty_rect: BoundingBox);

    /// Resize the window to the given size.
    ///
    /// The implementation of this method should be resilient to being called
    /// at a high frequency with potentially redundant or invalid size values.
    ///
    fn resize(&mut self, size: PhysicalSize<u32>) -> bool;

    // Provided methods

    fn make_current(&mut self) {}

    fn draw_surface(&mut self, draw: &mut dyn FnMut(&mut Surface) -> BoundingBox) {
        let Some((surface, dirty_surface)) = self.surfaces_mut() else {
            return;
        };

        let dirty_rect = draw(dirty_surface);

        let canvas = surface.canvas();
        canvas.clear(Color::transparent());
        dirty_surface.draw(canvas, (0, 0), SamplingOptions::default(), None);

        self.make_current();
        self.swap_buffers(dirty_rect);
    }

    fn is_initially_cloaked(&mut self) -> &mut bool;

    #[cfg(target_os = "windows")]
    fn raw_window_handle(&self) -> winit::raw_window_handle::Win32WindowHandle {
        match self.window().window_handle().unwrap().as_raw() {
            RawWindowHandle::Win32(handle) => handle,
            _ => unreachable!(),
        }
    }

    /// Cloaks the window such that it is not visible to the user, but will still be composited.
    ///
    /// <https://learn.microsoft.com/en-us/windows/win32/api/dwmapi/ne-dwmapi-dwmwindowattribute>
    ///
    #[cfg(target_os = "windows")]
    fn set_cloak(&mut self, state: bool) -> bool {
        use windows::Win32::{
            Foundation::{BOOL, FALSE, HWND, TRUE},
            Graphics::Dwm::{DwmSetWindowAttribute, DWMWA_CLOAK},
        };

        let handle = self.raw_window_handle();

        let value = if state { TRUE } else { FALSE };

        let result = unsafe {
            DwmSetWindowAttribute(
                HWND(handle.hwnd.get() as _),
                DWMWA_CLOAK,
                std::ptr::from_ref(&value).cast(),
                std::mem::size_of::<BOOL>() as u32,
            )
        };

        result.is_ok()
    }
}

/// An error reported when creation failed for all possible graphics backends.
///
/// As we have multiple backend options, this error type is used to group each
/// error that occursed with the associated backend that caused it.
///
#[derive(Debug)]
pub struct BackendCreationError {
    errors: Vec<(GraphicsBackend, Box<dyn Error>)>,
}

impl Display for BackendCreationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "No supported graphics backend found. Errors:")?;
        for (backend, error) in &self.errors {
            writeln!(f, "Backend: {:?}, Error: {}", backend, error)?;
        }
        Ok(())
    }
}

impl Error for BackendCreationError {}

/// Create a graphics backend for the given window.
///
pub(super) fn create_draw_surface(
    preferred_backend: Option<GraphicsBackend>,
    entity: Entity,
    window_attributes: WindowAttributes,
    window_description: &WindowDescription,
    event_loop: &ActiveEventLoop,
) -> Result<Box<dyn DrawSurface>, BackendCreationError> {
    let mut errors = Vec::with_capacity(4);

    let mut backends = [
        #[cfg(feature = "dx12")]
        GraphicsBackend::Dx12,
        #[cfg(feature = "metal")]
        GraphicsBackend::Metal,
        #[cfg(feature = "vulkan")]
        GraphicsBackend::Vulkan,
        #[cfg(feature = "gl")]
        GraphicsBackend::Gl,
    ];

    if let Some(pref) = preferred_backend {
        backends.sort_by_key(|backend| *backend != pref);
    }

    for backend in backends {
        match create(&backend, entity, &window_attributes, window_description, event_loop) {
            Ok(ok) => {
                return Ok(ok);
            }
            Err(e) => {
                errors.push((backend, e));
            }
        }
    }

    Err(BackendCreationError { errors }.into())
}

fn create(
    backend: &GraphicsBackend,
    entity: Entity,
    window_attributes: &WindowAttributes,
    window_description: &WindowDescription,
    event_loop: &ActiveEventLoop,
) -> Result<Box<dyn DrawSurface>, Box<dyn Error>> {
    match backend {
        #[cfg(feature = "gl")]
        GraphicsBackend::Gl => {
            let ws = gl::WinState::new(entity, window_attributes, window_description, event_loop)?;
            Ok(Box::new(ws) as _)
        }
        #[cfg(feature = "dx12")]
        GraphicsBackend::Dx12 => {
            let ws =
                dx12::WinState::new(entity, window_attributes, window_description, event_loop)?;
            Ok(Box::new(ws) as _)
        }
        #[cfg(feature = "metal")]
        GraphicsBackend::Metal => {
            let ws =
                metal::WinState::new(entity, window_attributes, window_description, event_loop)?;
            Ok(Box::new(ws) as _)
        }
        #[cfg(feature = "vulkan")]
        GraphicsBackend::Vulkan => {
            let ws =
                vulkan::WinState::new(entity, window_attributes, window_description, event_loop)?;
            Ok(Box::new(ws) as _)
        }
        _ => unreachable!(),
    }
}
