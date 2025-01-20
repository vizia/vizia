use crate::window_modifiers::WindowModifiers;
use glutin::context::GlProfile;
use vizia_core::context::TreeProps;
#[cfg(target_os = "windows")]
use winit::platform::windows::WindowExtWindows;
#[cfg(target_os = "windows")]
use winit::{platform::windows::WindowAttributesExtWindows, raw_window_handle::RawWindowHandle};

use crate::convert::cursor_icon_to_cursor_icon;
use hashbrown::HashMap;
use std::error::Error;
use std::num::NonZeroU32;
use std::{ffi::CString, sync::Arc};
use winit::raw_window_handle::HasWindowHandle;

use gl_rs as gl;
use glutin::config::Config;
use glutin_winit::DisplayBuilder;

use gl::types::*;

use glutin::{
    config::ConfigTemplateBuilder,
    context::{ContextApi, ContextAttributesBuilder},
    display::GetGlDisplay,
    prelude::*,
    surface::{SurfaceAttributesBuilder, WindowSurface},
};

use skia_safe::{
    gpu::{
        self, backend_render_targets, ganesh::context_options, gl::FramebufferInfo, ContextOptions,
        SurfaceOrigin,
    },
    ColorSpace, ColorType, PixelGeometry, Surface, SurfaceProps, SurfacePropsFlags,
};

use vizia_core::prelude::*;
use winit::event_loop::ActiveEventLoop;
use winit::window::{CursorGrabMode, CursorIcon, CustomCursor, WindowAttributes, WindowLevel};
use winit::{dpi::*, window::WindowId};

pub struct WinState {
    pub entity: Entity,
    gl_config: Config,
    gl_context: glutin::context::PossiblyCurrentContext,
    pub gl_surface: glutin::surface::Surface<glutin::surface::WindowSurface>,
    pub id: WindowId,
    pub gr_context: skia_safe::gpu::DirectContext,
    pub window: Arc<winit::window::Window>,
    pub surface: skia_safe::Surface,
    pub dirty_surface: skia_safe::Surface,
    pub should_close: bool,
    #[cfg(target_os = "windows")]
    pub is_initially_cloaked: bool,
}

impl Drop for WinState {
    fn drop(&mut self) {
        self.gl_context.make_current(&self.gl_surface).unwrap();
    }
}

impl WinState {
    pub fn new(
        event_loop: &ActiveEventLoop,
        entity: Entity,
        #[allow(unused_mut)] mut window_attributes: WindowAttributes,
        #[allow(unused_variables)] owner: Option<Arc<winit::window::Window>>,
    ) -> Result<Self, Box<dyn Error>> {
        #[cfg(target_os = "windows")]
        let (window, gl_config) = {
            if let Some(owner) = owner {
                let RawWindowHandle::Win32(handle) = owner.window_handle().unwrap().as_raw() else {
                    unreachable!();
                };
                window_attributes = window_attributes.with_owner_window(handle.hwnd.get());
            }

            // The current version of winit spawns new windows with unspecified position/size.
            // As a workaround, we'll hide the window during creation and reveal it afterward.
            let visible = window_attributes.visible;
            let window_attributes = window_attributes.with_visible(false);

            let (window, config) = build_window(event_loop, window_attributes);

            let window = window.expect("Could not create window with OpenGL context");
            // Another problem is the white background that briefly flashes on window creation.
            // To avoid this one we must wait until the first draw is complete before revealing
            // our window. The visible property won't work in this case as it prevents drawing.
            // Instead we use the "cloak" attribute, which hides the window without that issue.
            set_cloak(&window, true);
            window.set_visible(visible);

            (window, config)
        };

        #[cfg(not(target_os = "windows"))]
        let (window, gl_config) = {
            let (window, config) = build_window(event_loop, window_attributes);
            let window = window.expect("Could not create window with OpenGL context");
            (window, config)
        };

        window.set_ime_allowed(true);
        window.set_visible(true);

        let raw_window_handle = window.window_handle().unwrap().as_raw();

        let gl_display = gl_config.display();

        let context_attributes = ContextAttributesBuilder::new()
            .with_profile(GlProfile::Core)
            .with_context_api(ContextApi::OpenGl(None))
            .build(Some(raw_window_handle));

        let fallback_context_attributes = ContextAttributesBuilder::new()
            .with_profile(GlProfile::Core)
            .with_context_api(ContextApi::Gles(None))
            .build(Some(raw_window_handle));

        let not_current_gl_context = unsafe {
            gl_display.create_context(&gl_config, &context_attributes).unwrap_or_else(|_| {
                gl_display
                    .create_context(&gl_config, &fallback_context_attributes)
                    .expect("failed to create context")
            })
        };

        let (width, height): (u32, u32) = window.inner_size().into();

        let attrs = SurfaceAttributesBuilder::<WindowSurface>::new().with_srgb(Some(true)).build(
            raw_window_handle,
            NonZeroU32::new(width.max(1)).unwrap(),
            NonZeroU32::new(height.max(1)).unwrap(),
        );

        let gl_surface =
            unsafe { gl_config.display().create_window_surface(&gl_config, &attrs).unwrap() };

        let gl_context = not_current_gl_context.make_current(&gl_surface).unwrap();

        // if window_description.vsync {
        //     gl_surface
        //         .set_swap_interval(&gl_context, SwapInterval::Wait(NonZeroU32::new(1).unwrap()))
        //         .expect("Failed to set vsync");
        // }

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

        let mut gr_context = skia_safe::gpu::direct_contexts::make_gl(interface, &context_options)
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

        let mut surface =
            create_surface(&window, fb_info, &mut gr_context, num_samples, stencil_size);

        let inner_size = window.inner_size();

        let dirty_surface = surface
            .new_surface_with_dimensions((inner_size.width as i32, inner_size.height as i32))
            .unwrap();

        // Build our window
        Ok(WinState {
            entity,
            gl_config,
            gl_context,
            id: window.id(),
            gr_context,
            gl_surface,
            window: Arc::new(window),
            surface,
            dirty_surface,
            should_close: false,
            #[cfg(target_os = "windows")]
            is_initially_cloaked: true,
        })
    }

    // Returns a reference to the winit window
    pub fn window(&self) -> &winit::window::Window {
        &self.window
    }

    pub fn make_current(&mut self) {
        self.gl_context.make_current(&self.gl_surface).unwrap();
    }

    pub fn resize(&mut self, size: PhysicalSize<u32>) {
        self.gl_context.make_current(&self.gl_surface).unwrap();
        let (width, height): (u32, u32) = size.into();

        if width == 0 || height == 0 {
            return;
        }

        let fb_info = {
            let mut fboid: GLint = 0;
            unsafe { gl::GetIntegerv(gl::FRAMEBUFFER_BINDING, &mut fboid) };

            FramebufferInfo {
                fboid: fboid.try_into().unwrap(),
                format: skia_safe::gpu::gl::Format::RGBA8.into(),
                ..Default::default()
            }
        };

        self.surface = create_surface(
            &self.window,
            fb_info,
            &mut self.gr_context,
            self.gl_config.num_samples() as usize,
            self.gl_config.stencil_size() as usize,
        );

        self.dirty_surface = self
            .surface
            .new_surface_with_dimensions((width.max(1) as i32, height.max(1) as i32))
            .unwrap();

        self.gl_surface.resize(
            &self.gl_context,
            NonZeroU32::new(width.max(1)).unwrap(),
            NonZeroU32::new(height.max(1)).unwrap(),
        );
    }

    pub fn swap_buffers(&mut self) {
        self.gr_context.flush_and_submit();
        self.gl_surface.swap_buffers(&self.gl_context).expect("Failed to swap buffers");
    }
}

fn build_window(
    event_loop: &ActiveEventLoop,
    window_attributes: WindowAttributes,
) -> (Option<winit::window::Window>, Config) {
    let template = ConfigTemplateBuilder::new().with_alpha_size(8).with_transparency(true);
    let display_builder = DisplayBuilder::new().with_window_attributes(Some(window_attributes));

    display_builder
        .build(event_loop, template, |configs| {
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
        .unwrap()
}

/// Cloaks the window such that it is not visible to the user, but will still be composited.
///
/// <https://learn.microsoft.com/en-us/windows/win32/api/dwmapi/ne-dwmapi-dwmwindowattribute>
///
#[cfg(target_os = "windows")]
pub fn set_cloak(window: &winit::window::Window, state: bool) -> bool {
    use windows_sys::Win32::{
        Foundation::{BOOL, FALSE, HWND, TRUE},
        Graphics::Dwm::{DwmSetWindowAttribute, DWMWA_CLOAK},
    };

    let RawWindowHandle::Win32(handle) = window.window_handle().unwrap().as_raw() else {
        unreachable!();
    };

    let value = if state { TRUE } else { FALSE };

    let result = unsafe {
        DwmSetWindowAttribute(
            handle.hwnd.get() as HWND,
            DWMWA_CLOAK as u32,
            std::ptr::from_ref(&value).cast(),
            std::mem::size_of::<BOOL>() as u32,
        )
    };

    result == 0 // success
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

    let surface_props = SurfaceProps::new_with_text_properties(
        SurfacePropsFlags::default(),
        PixelGeometry::default(),
        0.5,
        0.0,
    );

    gpu::surfaces::wrap_backend_render_target(
        gr_context,
        &backend_render_target,
        SurfaceOrigin::BottomLeft,
        ColorType::RGBA8888,
        ColorSpace::new_srgb(),
        Some(surface_props).as_ref(),
        // None,
    )
    .expect("Could not create skia surface")
}

type WindowCallback = Option<Box<dyn Fn(&mut EventContext)>>;

pub struct Window {
    pub window: Option<Arc<winit::window::Window>>,
    pub on_close: WindowCallback,
    pub on_create: WindowCallback,
    pub should_close: bool,
    pub(crate) custom_cursors: Arc<HashMap<CursorIcon, CustomCursor>>,
}

impl Window {
    fn window(&self) -> &winit::window::Window {
        self.window.as_ref().unwrap()
    }

    pub fn new(cx: &mut Context, content: impl 'static + Fn(&mut Context)) -> Handle<Self> {
        Self {
            window: None,
            on_close: None,
            on_create: None,
            should_close: false,
            custom_cursors: Default::default(),
        }
        .build(cx, |cx| {
            cx.windows.insert(
                cx.current(),
                WindowState { content: Some(Arc::new(content)), ..Default::default() },
            );
            cx.tree.set_window(cx.current(), true);
        })
    }

    pub fn popup(
        cx: &mut Context,
        is_modal: bool,
        content: impl 'static + Fn(&mut Context),
    ) -> Handle<Self> {
        Self {
            window: None,
            on_close: None,
            on_create: None,
            should_close: false,
            custom_cursors: Default::default(),
        }
        .build(cx, |cx| {
            let parent_window = cx.parent_window();
            if is_modal {
                cx.emit_to(parent_window, WindowEvent::SetEnabled(false));
            }

            cx.windows.insert(
                cx.current(),
                WindowState {
                    owner: Some(parent_window),
                    is_modal: true,
                    content: Some(Arc::new(content)),
                    ..Default::default()
                },
            );
            cx.tree.set_window(cx.current(), true);
        })
        .lock_focus_to_within()
    }
}

impl View for Window {
    fn element(&self) -> Option<&'static str> {
        Some("window")
    }

    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|window_event, meta| match window_event {
            WindowEvent::Destroyed => {
                let parent_window = cx.parent_window().unwrap_or(Entity::root());
                cx.emit_to(parent_window, WindowEvent::SetEnabled(true));
            }

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
                let Some(icon) = cursor_icon_to_cursor_icon(*cursor) else {
                    self.window().set_cursor_visible(false);
                    return;
                };

                if let Some(custom_icon) = self.custom_cursors.get(&icon) {
                    self.window().set_cursor(custom_icon.clone());
                } else {
                    self.window().set_cursor(icon);
                }

                self.window().set_cursor_visible(true);
            }

            WindowEvent::SetTitle(title) => {
                self.window().set_title(title);
            }

            WindowEvent::SetSize(size) => {
                let _ =
                    self.window().request_inner_size(PhysicalSize::new(size.width, size.height));
            }

            WindowEvent::SetMinSize(size) => {
                self.window().set_min_inner_size(
                    size.map(|size| PhysicalSize::new(size.width, size.height)),
                );
            }

            WindowEvent::SetMaxSize(size) => {
                self.window().set_max_inner_size(
                    size.map(|size| PhysicalSize::new(size.width, size.height)),
                );
            }

            WindowEvent::SetPosition(pos) => {
                let parent_window_position = if cx.current() == Entity::root() {
                    WindowPosition::new(0, 0)
                } else {
                    cx.window_position()
                };
                self.window().set_outer_position(LogicalPosition::new(
                    parent_window_position.x + pos.x,
                    parent_window_position.y + pos.y,
                ));
                meta.consume();
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

                meta.consume();
            }

            WindowEvent::SetDecorations(flag) => {
                self.window().set_decorations(*flag);
            }

            WindowEvent::ReloadStyles => {
                cx.reload_styles().unwrap();
            }

            WindowEvent::WindowClose => {
                self.should_close = true;

                cx.close_window();

                if let Some(callback) = &self.on_close {
                    callback(cx);
                }

                meta.consume();
            }

            WindowEvent::FocusNext => {
                cx.focus_next();
            }

            WindowEvent::FocusPrev => {
                cx.focus_prev();
            }

            WindowEvent::Redraw => {
                self.window().request_redraw();
            }

            #[allow(unused_variables)]
            WindowEvent::SetEnabled(flag) => {
                #[cfg(target_os = "windows")]
                self.window().set_enable(*flag);

                self.window().focus_window();
            }

            WindowEvent::DragWindow => {
                self.window().drag_window().expect("Failed to init drag window");
                meta.consume();
            }

            WindowEvent::SetAlwaysOnTop(flag) => {
                self.window().set_window_level(if *flag {
                    WindowLevel::AlwaysOnTop
                } else {
                    WindowLevel::Normal
                });
            }

            _ => {}
        })
    }
}

impl WindowModifiers for Handle<'_, Window> {
    fn on_close(self, callback: impl Fn(&mut EventContext) + 'static) -> Self {
        self.modify(|window| window.on_close = Some(Box::new(callback)))
    }

    fn on_create(self, callback: impl Fn(&mut EventContext) + 'static) -> Self {
        self.modify(|window| window.on_create = Some(Box::new(callback)))
    }

    fn title<T: ToString>(mut self, title: impl Res<T>) -> Self {
        let entity = self.entity();
        let title = title.get(&self).to_string();
        if let Some(win_state) = self.context().windows.get_mut(&entity) {
            win_state.window_description.title = title;
        }

        self
    }

    fn inner_size<S: Into<WindowSize>>(mut self, size: impl Res<S>) -> Self {
        let entity = self.entity();
        let size = size.get(&self).into();
        if let Some(win_state) = self.context().windows.get_mut(&entity) {
            win_state.window_description.inner_size = size;
        }

        self
    }

    fn min_inner_size<S: Into<WindowSize>>(mut self, size: impl Res<Option<S>>) -> Self {
        let entity = self.entity();
        let size = size.get(&self).map(|size| size.into());
        if let Some(win_state) = self.context().windows.get_mut(&entity) {
            win_state.window_description.min_inner_size = size;
        }

        self
    }

    fn max_inner_size<S: Into<WindowSize>>(mut self, size: impl Res<Option<S>>) -> Self {
        let entity = self.entity();
        let size = size.get(&self).map(|size| size.into());
        if let Some(win_state) = self.context().windows.get_mut(&entity) {
            win_state.window_description.max_inner_size = size;
        }

        self
    }

    fn position<P: Into<vizia_window::WindowPosition>>(mut self, position: impl Res<P>) -> Self {
        let entity = self.entity();
        let pos = Some(position.get(&self).into());
        if let Some(win_state) = self.context().windows.get_mut(&entity) {
            win_state.window_description.position = pos;
        }

        self
    }

    fn resizable(mut self, flag: impl Res<bool>) -> Self {
        let entity = self.entity();
        let flag = flag.get(&self);
        if let Some(win_state) = self.context().windows.get_mut(&entity) {
            win_state.window_description.resizable = flag;
        }

        self
    }

    fn minimized(mut self, flag: impl Res<bool>) -> Self {
        let entity = self.entity();
        let flag = flag.get(&self);
        if let Some(win_state) = self.context().windows.get_mut(&entity) {
            win_state.window_description.minimized = flag;
        }

        self
    }

    fn maximized(mut self, flag: impl Res<bool>) -> Self {
        let entity = self.entity();
        let flag = flag.get(&self);
        if let Some(win_state) = self.context().windows.get_mut(&entity) {
            win_state.window_description.maximized = flag;
        }

        self
    }

    fn visible(mut self, flag: impl Res<bool>) -> Self {
        let entity = self.entity();
        let flag = flag.get(&self);
        if let Some(win_state) = self.context().windows.get_mut(&entity) {
            win_state.window_description.visible = flag
        }

        self
    }

    fn transparent(mut self, flag: bool) -> Self {
        let entity = self.entity();
        if let Some(win_state) = self.context().windows.get_mut(&entity) {
            win_state.window_description.transparent = flag
        }

        self
    }

    fn decorations(mut self, flag: bool) -> Self {
        let entity = self.entity();
        if let Some(win_state) = self.context().windows.get_mut(&entity) {
            win_state.window_description.decorations = flag
        }

        self
    }

    fn always_on_top(mut self, flag: bool) -> Self {
        let entity = self.entity();
        if let Some(win_state) = self.context().windows.get_mut(&entity) {
            win_state.window_description.always_on_top = flag
        }

        self
    }

    fn vsync(mut self, flag: bool) -> Self {
        let entity = self.entity();
        if let Some(win_state) = self.context().windows.get_mut(&entity) {
            win_state.window_description.vsync = flag
        }

        self
    }

    fn icon(mut self, width: u32, height: u32, image: Vec<u8>) -> Self {
        let entity = self.entity();
        if let Some(win_state) = self.context().windows.get_mut(&entity) {
            win_state.window_description.icon = Some(image);
            win_state.window_description.icon_width = width;
            win_state.window_description.icon_height = height;
        }

        self
    }

    fn enabled_window_buttons(mut self, window_buttons: WindowButtons) -> Self {
        let entity = self.entity();
        if let Some(win_state) = self.context().windows.get_mut(&entity) {
            win_state.window_description.enabled_window_buttons = window_buttons;
        }

        self
    }
}
