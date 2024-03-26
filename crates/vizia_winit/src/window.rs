use crate::application::UserEvent;
use std::ffi::CString;
#[cfg(not(target_arch = "wasm32"))]
use std::num::NonZeroU32;

use crate::convert::cursor_icon_to_cursor_icon;
// use femtovg::{renderer::OpenGl, Canvas, Color};

#[cfg(not(target_arch = "wasm32"))]
use gl_rs as gl;
use glutin::config::Config;
#[cfg(not(target_arch = "wasm32"))]
use glutin::surface::SwapInterval;
#[cfg(not(target_arch = "wasm32"))]
use glutin_winit::DisplayBuilder;
#[cfg(not(target_arch = "wasm32"))]
use raw_window_handle::HasRawWindowHandle;

use gl::types::*;

#[cfg(not(target_arch = "wasm32"))]
use glutin::{
    config::ConfigTemplateBuilder,
    context::{ContextApi, ContextAttributesBuilder},
    display::GetGlDisplay,
    prelude::*,
    surface::{SurfaceAttributesBuilder, WindowSurface},
};

use skia_safe::{
    gpu::{
        self, backend_render_targets, context_options, gl::FramebufferInfo, ContextOptions,
        SurfaceOrigin,
    },
    ColorType, Surface,
};
use vizia_core::backend::*;
use vizia_core::prelude::*;
use winit::event_loop::EventLoop;
use winit::window::{CursorGrabMode, WindowBuilder, WindowLevel};
use winit::{dpi::*, window::WindowId};

pub struct Window {
    gl_config: Config,
    pub gl_surface: glutin::surface::Surface<glutin::surface::WindowSurface>,
    #[cfg(not(target_arch = "wasm32"))]
    pub id: WindowId,
    #[cfg(not(target_arch = "wasm32"))]
    pub gl_context: glutin::context::PossiblyCurrentContext,

    pub gr_context: skia_safe::gpu::DirectContext,
    #[cfg(not(target_arch = "wasm32"))]
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
    ) -> (Self, Surface) {
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

        let gl_surface =
            unsafe { gl_config.display().create_window_surface(&gl_config, &attrs).unwrap() };

        let gl_context = not_current_gl_context.take().unwrap().make_current(&gl_surface).unwrap();

        if window_description.vsync {
            gl_surface
                .set_swap_interval(&gl_context, SwapInterval::Wait(NonZeroU32::new(1).unwrap()))
                .expect("Failed to set vsync");
        }

        // Build the femtovg renderer
        // let renderer = unsafe {
        //     OpenGl::new_from_function_cstr(|s| gl_display.get_proc_address(s) as *const _)
        // }
        // .expect("Cannot create renderer");

        // let mut canvas = Canvas::new(renderer).expect("Failed to create canvas");

        // let size = window.inner_size();
        // canvas.set_size(size.width, size.height, 1.0);
        // canvas.clear_rect(0, 0, size.width, size.height, Color::rgb(255, 80, 80));

        // Build skia renderer
        gl::load_with(|s| {
            gl_config.display().get_proc_address(CString::new(s).unwrap().as_c_str())
        });
        let interface = skia_safe::gpu::gl::Interface::new_load_with(|name| {
            if name == "eglGetCurrentDisplay" {
                return std::ptr::null();
            }
            gl_config.display().get_proc_address(CString::new(name).unwrap().as_c_str())
        })
        .expect("Could not create interface");

        // https://github.com/rust-skia/rust-skia/issues/476
        let mut context_options = ContextOptions::new();
        context_options.skip_gl_error_checks = context_options::Enable::Yes;

        let mut gr_context = skia_safe::gpu::DirectContext::new_gl(interface, &context_options)
            .expect("Could not create direct context");

        let fb_info = {
            let mut fboid: GLint = 0;
            unsafe { gl::GetIntegerv(gl::FRAMEBUFFER_BINDING, &mut fboid) };

            FramebufferInfo {
                fboid: fboid.try_into().unwrap(),
                format: skia_safe::gpu::gl::Format::RGBA8.into(),
                ..Default::default()
            }
        };

        let num_samples = gl_config.num_samples() as usize;
        let stencil_size = gl_config.stencil_size() as usize;

        let surface = create_surface(&window, fb_info, &mut gr_context, num_samples, stencil_size);

        // Build our window
        let win = Window {
            gl_config,
            id: window.id(),
            gl_context,
            gr_context,
            gl_surface,
            window,
            should_close: false,
        };

        (win, surface)
    }

    pub fn window(&self) -> &winit::window::Window {
        &self.window
    }

    pub fn resize(&mut self, size: PhysicalSize<u32>, surface: &mut Surface) {
        let fb_info = {
            let mut fboid: GLint = 0;
            unsafe { gl::GetIntegerv(gl::FRAMEBUFFER_BINDING, &mut fboid) };

            FramebufferInfo {
                fboid: fboid.try_into().unwrap(),
                format: skia_safe::gpu::gl::Format::RGBA8.into(),
                ..Default::default()
            }
        };

        *surface = create_surface(
            &self.window,
            fb_info,
            &mut self.gr_context,
            self.gl_config.num_samples() as usize,
            self.gl_config.stencil_size() as usize,
        );

        if size.width != 0 && size.height != 0 {
            self.gl_surface.resize(
                &self.gl_context,
                size.width.try_into().unwrap(),
                size.height.try_into().unwrap(),
            );
        }
    }

    pub fn swap_buffers(&mut self) {
        self.gr_context.flush_and_submit();
        self.gl_surface.swap_buffers(&self.gl_context).expect("Failed to swap buffers");
    }
}

pub fn create_surface(
    window: &winit::window::Window,
    fb_info: FramebufferInfo,
    gr_context: &mut skia_safe::gpu::DirectContext,
    num_samples: usize,
    stencil_size: usize,
) -> Surface {
    let size = window.inner_size();
    let size = (
        size.width.try_into().expect("Could not convert width"),
        size.height.try_into().expect("Could not convert height"),
    );
    let backend_render_target =
        backend_render_targets::make_gl(size, num_samples, stencil_size, fb_info);

    gpu::surfaces::wrap_backend_render_target(
        gr_context,
        &backend_render_target,
        SurfaceOrigin::BottomLeft,
        ColorType::RGBA8888,
        None,
        None,
    )
    .expect("Could not create skia surface")
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
                let _ = self.window().request_inner_size(LogicalSize::new(size.width, size.height));
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
