use crate::application::UserEvent;
#[cfg(not(target_arch = "wasm32"))]
use std::num::NonZeroU32;

use crate::convert::cursor_icon_to_cursor_icon;
use femtovg::{renderer::OpenGl, Canvas, Color};

#[cfg(not(target_arch = "wasm32"))]
use glutin::surface::SwapInterval;
#[cfg(not(target_arch = "wasm32"))]
use glutin_winit::DisplayBuilder;
#[cfg(not(target_arch = "wasm32"))]
use raw_window_handle::HasRawWindowHandle;

#[cfg(not(target_arch = "wasm32"))]
use glutin::{
    config::ConfigTemplateBuilder,
    context::{ContextApi, ContextAttributesBuilder},
    display::GetGlDisplay,
    prelude::*,
    surface::{SurfaceAttributesBuilder, WindowSurface},
};

use vizia_core::backend::*;
use vizia_core::prelude::*;
use winit::event_loop::EventLoop;
use winit::window::{CursorGrabMode, WindowBuilder, WindowLevel};
use winit::{dpi::*, window::WindowId};

pub struct Window {
    pub id: WindowId,
    #[cfg(not(target_arch = "wasm32"))]
    context: glutin::context::PossiblyCurrentContext,
    #[cfg(not(target_arch = "wasm32"))]
    surface: glutin::surface::Surface<glutin::surface::WindowSurface>,
    window: winit::window::Window,
    pub should_close: bool,
}

#[cfg(target_os = "windows")]
impl Window {
    /// Cloaks the window such that it is not visible to the user, but will still be composited.
    /// We use this to work around the "blank window flash" startup bug on windows.
    ///
    /// <https://learn.microsoft.com/en-us/windows/win32/api/dwmapi/ne-dwmapi-dwmwindowattribute>
    ///
    pub(crate) fn set_cloak(&self, state: bool) -> bool {
        use raw_window_handle::RawWindowHandle::Win32;
        use winapi::shared::minwindef::{BOOL, FALSE, TRUE};
        use winapi::um::dwmapi::{DwmSetWindowAttribute, DWMWA_CLOAK};

        let Win32(handle) = self.window.raw_window_handle() else {
            unreachable!();
        };

        let value = if state { TRUE } else { FALSE };
        let result = unsafe {
            DwmSetWindowAttribute(
                handle.hwnd as _,
                DWMWA_CLOAK,
                &value as *const BOOL as *const _,
                std::mem::size_of::<BOOL>() as u32,
            )
        };

        result == 0 // success
    }
}

#[cfg(target_arch = "wasm32")]
impl Window {
    pub fn new(
        events_loop: &EventLoop<UserEvent>,
        window_description: &WindowDescription,
    ) -> (Self, Canvas<OpenGl>) {
        let window_builder = WindowBuilder::new();

        let canvas_element = {
            use wasm_bindgen::JsCast;
            let document = web_sys::window().unwrap().document().unwrap();
            if let Some(canvas_id) = &window_description.target_canvas {
                document.get_element_by_id(canvas_id).unwrap()
            } else {
                let element = document.create_element("canvas").unwrap();
                document.body().unwrap().insert_adjacent_element("afterbegin", &element).unwrap();
                element
            }
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .unwrap()
        };

        let renderer =
            OpenGl::new_from_html_canvas(&canvas_element).expect("Cannot create renderer");

        let mut canvas = Canvas::new(renderer).expect("Failed to create canvas");

        // tell winit about the above canvas
        let window_builder = {
            use winit::platform::web::WindowBuilderExtWebSys;
            window_builder.with_canvas(Some(canvas_element))
        };

        // Apply generic WindowBuilder properties
        let window_builder = apply_window_description(window_builder, &window_description);

        // Get the window handle. this is a winit::window::Window
        let handle = window_builder.build(&events_loop).unwrap();

        // Build our window
        let window = Window { id: handle.id(), window: handle, should_close: false };

        let size = window.window().inner_size();
        canvas.set_size(size.width as u32, size.height as u32, 1.0);
        canvas.clear_rect(0, 0, size.width as u32, size.height as u32, Color::rgb(255, 80, 80));

        (window, canvas)
    }

    pub fn window(&self) -> &winit::window::Window {
        &self.window
    }

    pub fn resize(&self, _size: PhysicalSize<u32>) {
        // TODO?
    }

    pub fn swap_buffers(&self) {
        // Intentional no-op
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl Window {
    pub fn new(
        events_loop: &EventLoop<UserEvent>,
        window_description: &WindowDescription,
    ) -> (Self, Canvas<OpenGl>) {
        let window_builder = WindowBuilder::new();

        //Windows COM doesn't play nicely with winit's drag and drop right now
        #[cfg(target_os = "windows")]
        let window_builder = {
            use winit::platform::windows::WindowBuilderExtWindows;
            window_builder.with_drag_and_drop(false)
        };

        // Apply generic WindowBuilder properties
        let window_builder = apply_window_description(window_builder, window_description);

        let template =
            ConfigTemplateBuilder::new().with_alpha_size(8).with_transparency(cfg!(cgl_backend));
        let display_builder = DisplayBuilder::new().with_window_builder(Some(window_builder));

        let (window, gl_config) = display_builder
            .build(events_loop, template, |configs| {
                // Find the config with the maximum number of samples, so our triangle will
                // be smooth.
                configs
                    .reduce(|accum, config| {
                        let transparency_check = config.supports_transparency().unwrap_or(false)
                            & !accum.supports_transparency().unwrap_or(false);

                        if transparency_check || config.num_samples() < accum.num_samples() {
                            config
                        } else {
                            accum
                        }
                    })
                    .unwrap()
            })
            .unwrap();

        let window = window.unwrap();

        let raw_window_handle = Some(window.raw_window_handle());

        let gl_display = gl_config.display();

        let context_attributes = ContextAttributesBuilder::new().build(raw_window_handle);
        let fallback_context_attributes = ContextAttributesBuilder::new()
            .with_context_api(ContextApi::Gles(None))
            .build(raw_window_handle);
        let mut not_current_gl_context = Some(unsafe {
            gl_display.create_context(&gl_config, &context_attributes).unwrap_or_else(|_| {
                gl_display
                    .create_context(&gl_config, &fallback_context_attributes)
                    .expect("failed to create context")
            })
        });

        let (width, height): (u32, u32) = window.inner_size().into();
        let raw_window_handle = window.raw_window_handle();
        let attrs = SurfaceAttributesBuilder::<WindowSurface>::new().build(
            raw_window_handle,
            NonZeroU32::new(width.max(1)).unwrap(),
            NonZeroU32::new(height.max(1)).unwrap(),
        );

        let surface =
            unsafe { gl_config.display().create_window_surface(&gl_config, &attrs).unwrap() };

        let gl_context = not_current_gl_context.take().unwrap().make_current(&surface).unwrap();

        // Build the femtovg renderer
        let renderer = unsafe {
            OpenGl::new_from_function_cstr(|s| gl_display.get_proc_address(s) as *const _)
        }
        .expect("Cannot create renderer");

        if window_description.vsync {
            surface
                .set_swap_interval(&gl_context, SwapInterval::Wait(NonZeroU32::new(1).unwrap()))
                .expect("Failed to set vsync");
        }

        let mut canvas = Canvas::new(renderer).expect("Failed to create canvas");

        let size = window.inner_size();
        canvas.set_size(size.width, size.height, 1.0);
        canvas.clear_rect(0, 0, size.width, size.height, Color::rgb(255, 80, 80));

        // Build our window
        let win =
            Window { id: window.id(), context: gl_context, surface, window, should_close: false };

        (win, canvas)
    }

    pub fn window(&self) -> &winit::window::Window {
        &self.window
    }

    pub fn resize(&self, size: PhysicalSize<u32>) {
        if size.width != 0 && size.height != 0 {
            self.surface.resize(
                &self.context,
                size.width.try_into().unwrap(),
                size.height.try_into().unwrap(),
            );
        }
    }

    pub fn swap_buffers(&self) {
        self.surface.swap_buffers(&self.context).expect("Failed to swap buffers");
    }
}

impl View for Window {
    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|window_event, _| match window_event {
            WindowEvent::GrabCursor(flag) => {
                let grab_mode = if *flag { CursorGrabMode::Locked } else { CursorGrabMode::None };
                self.window().set_cursor_grab(grab_mode).expect("Failed to set cursor grab");
            }

            WindowEvent::SetCursorPosition(x, y) => {
                self.window()
                    .set_cursor_position(winit::dpi::Position::Physical(PhysicalPosition::new(
                        *x as i32, *y as i32,
                    )))
                    .expect("Failed to set cursor position");
            }

            WindowEvent::SetCursor(cursor) => {
                if let Some(icon) = cursor_icon_to_cursor_icon(*cursor) {
                    self.window().set_cursor_visible(true);
                    self.window().set_cursor_icon(icon);
                } else {
                    self.window().set_cursor_visible(false);
                }
            }

            WindowEvent::SetTitle(title) => {
                self.window().set_title(title);
            }

            WindowEvent::SetSize(size) => {
                self.window().set_inner_size(LogicalSize::new(size.width, size.height));
            }

            WindowEvent::SetMinSize(size) => {
                self.window()
                    .set_min_inner_size(size.map(|size| LogicalSize::new(size.width, size.height)));
            }

            WindowEvent::SetMaxSize(size) => {
                self.window()
                    .set_max_inner_size(size.map(|size| LogicalSize::new(size.width, size.height)));
            }

            WindowEvent::SetPosition(pos) => {
                self.window().set_outer_position(LogicalPosition::new(pos.x, pos.y));
            }

            WindowEvent::SetResizable(flag) => {
                self.window().set_resizable(*flag);
            }

            WindowEvent::SetMinimized(flag) => {
                self.window().set_minimized(*flag);
            }

            WindowEvent::SetMaximized(flag) => {
                self.window().set_maximized(*flag);
            }

            WindowEvent::SetVisible(flag) => {
                self.window().set_visible(*flag);
            }

            WindowEvent::SetDecorations(flag) => {
                self.window().set_decorations(*flag);
            }

            WindowEvent::ReloadStyles => {
                cx.reload_styles().unwrap();
            }

            WindowEvent::WindowClose => {
                self.should_close = true;
            }

            WindowEvent::FocusNext => {
                cx.focus_next();
            }

            WindowEvent::FocusPrev => {
                cx.focus_prev();
            }

            _ => {}
        })
    }
}

fn apply_window_description(
    mut builder: WindowBuilder,
    description: &WindowDescription,
) -> WindowBuilder {
    builder = builder.with_title(&description.title).with_inner_size(LogicalSize::new(
        description.inner_size.width,
        description.inner_size.height,
    ));

    if let Some(min_inner_size) = description.min_inner_size {
        builder = builder
            .with_min_inner_size(LogicalSize::new(min_inner_size.width, min_inner_size.height))
    }

    if let Some(max_inner_size) = description.max_inner_size {
        builder = builder
            .with_max_inner_size(LogicalSize::new(max_inner_size.width, max_inner_size.height));
    }

    if let Some(position) = description.position {
        builder = builder.with_position(LogicalPosition::new(position.x, position.y));
    }

    builder
        .with_resizable(description.resizable)
        .with_maximized(description.maximized)
        // Accesskit requires that the window start invisible until accesskit is initialized.
        .with_visible(false)
        .with_window_level(if description.always_on_top {
            WindowLevel::AlwaysOnTop
        } else {
            WindowLevel::Normal
        })
        .with_transparent(description.transparent)
        .with_decorations(description.decorations)
        .with_window_icon(description.icon.as_ref().map(|icon| {
            winit::window::Icon::from_rgba(
                icon.clone(),
                description.icon_width,
                description.icon_height,
            )
            .unwrap()
        }))
}
