use crate::application::ApplicationRunner;
use crate::window_settings::{FramePolicy, WindowScalePolicy};
use baseview::dpi::LogicalSize;
use baseview::gl::GlConfig;
use baseview::{
    Event, EventStatus, HandlerError, Window, WindowContext, WindowHandler, WindowSettings,
};
use gl::types::GLint;
use gl_rs as gl;
use raw_window_handle::HasWindowHandle;
use skia_safe::gpu::gl::FramebufferInfo;
use skia_safe::gpu::{
    self, ContextOptions, SurfaceOrigin, backend_render_targets, ganesh::context_options,
};
use skia_safe::{ColorSpace, ColorType, PixelGeometry, Surface, SurfaceProps, SurfacePropsFlags};
use std::cell::RefCell;

use crate::proxy::ProxyQueue;
use crate::sync_waker::{SyncWakerGuard, register_window, wake};
use vizia_core::backend::*;
use vizia_core::prelude::*;

/// Handles a vizia_baseview application
pub(crate) struct ViziaWindow {
    application: RefCell<ApplicationRunner>,
    #[allow(clippy::type_complexity)]
    on_idle: Option<Box<dyn Fn(&mut Context) + Send>>,
    // Droping this deregister the waker on drop.
    _sync_waker: Option<SyncWakerGuard>,
}

impl ViziaWindow {
    fn new(
        mut cx: BackendContext,
        win_desc: WindowDescription,
        frame_policy: FramePolicy,
        window: WindowContext,
        builder: Option<Box<dyn FnOnce(&mut Context) + Send>>,
        on_idle: Option<Box<dyn Fn(&mut Context) + Send>>,
        is_parented: bool,
    ) -> ViziaWindow {
        // Reactive runtime setup — mirrors what `vizia_winit::Application::new` does.
        //
        // Mark this thread as the UI thread so off-thread `SyncSignal` writes correctly
        // enqueue effects via `SYNC_RUNTIME` instead of the thread-local `RUNTIME` (which
        // nobody drains). `on_frame_update` later calls `Runtime::drain_pending_work` to
        // apply those effects on this thread.
        Runtime::init_on_ui_thread();

        // The runner owns the queue and drains it in `on_frame_update`.
        // proxies only hold a weak reference, so sending fails once the window is closed.
        let proxy_queue = ProxyQueue::default();
        cx.set_event_proxy(Box::new(proxy_queue.proxy(window.waker())));

        // Register the window as needing wake only if the policy is `OnDemand`.
        // `Continuous` draw runs every vsync frame anyway.
        let guard =
            (frame_policy == FramePolicy::OnDemand).then(|| register_window(window.waker()));
        Runtime::set_sync_effect_waker(wake);

        let context = window.gl_context().expect("Window was created without OpenGL support");

        unsafe { context.make_current() }.expect("Failed to make GL context current");

        // Build skia renderer
        gl::load_with(|s| context.get_proc_address_from_str(s));
        let interface = skia_safe::gpu::gl::Interface::new_load_with(|name| {
            if name == "eglGetCurrentDisplay" {
                return std::ptr::null();
            }
            context.get_proc_address_from_str(name)
        })
        .expect("Could not create interface");

        // https://github.com/rust-skia/rust-skia/issues/476
        let mut context_options = ContextOptions::new();
        context_options.skip_gl_error_checks = context_options::Enable::Yes;

        let mut gr_context = skia_safe::gpu::direct_contexts::make_gl(interface, &context_options)
            .expect("Could not create direct context");
        gr_context.set_resource_cache_limit(win_desc.skia_resource_cache_limit);

        let fb_info = {
            let mut fboid: GLint = 0;
            unsafe { gl::GetIntegerv(gl::FRAMEBUFFER_BINDING, &mut fboid) };

            FramebufferInfo {
                fboid: fboid.try_into().unwrap(),
                format: skia_safe::gpu::gl::Format::RGBA8.into(),
                ..Default::default()
            }
        };

        let initial_size = window.size();
        let initial_width = initial_size.physical.width.max(1) as i32;
        let initial_height = initial_size.physical.height.max(1) as i32;

        let mut surface = create_surface((initial_width, initial_height), fb_info, &mut gr_context);

        let dirty_surface =
            surface.new_surface_with_dimensions((initial_width, initial_height)).unwrap();

        // baseview determines the scale factor from the platform. The
        // `WindowScalePolicy::ScaleFactor(scale)`
        // value is only used as a fallback (via
        // `with_fallback_scale_factor` in `open_parented`/`open_blocking`) when the platform
        // cannot report one, so always read the actual scale factor here to keep vizia's layout consistent with the
        // real window.
        let window_scale_factor = window.scale_factor();
        let dpi_factor = window_scale_factor * win_desc.user_scale_factor;

        cx.add_main_window(Entity::root(), &win_desc, dpi_factor as f32);
        // The window can be created at a different size to `inner_size` when that's outside
        // its min and max size, so lay out at the size it actually has.
        cx.set_window_size(
            Entity::root(),
            initial_size.physical.width as f32,
            initial_size.physical.height as f32,
        );
        cx.add_window(Entity::root(), WindowView {});

        cx.0.windows.insert(
            Entity::root(),
            WindowState { window_description: win_desc.clone(), ..Default::default() },
        );

        cx.context().add_built_in_styles();
        if let Some(builder) = builder {
            (builder)(cx.context());
        }

        let application = ApplicationRunner::new(
            cx,
            gr_context,
            window_scale_factor,
            frame_policy,
            proxy_queue,
            surface,
            dirty_surface,
            win_desc,
            is_parented,
            window,
        );
        unsafe { context.make_not_current() }.expect("Failed to make GL context not current");

        ViziaWindow { application: RefCell::new(application), on_idle, _sync_waker: guard }
    }

    /// Open a new child window.
    ///
    /// * `parent` - The parent window.
    /// * `app` - The Vizia application builder.
    pub fn open_parented<P, F>(
        parent: &P,
        win_desc: WindowDescription,
        scale_policy: WindowScalePolicy,
        frame_policy: FramePolicy,
        app: F,
        on_idle: Option<Box<dyn Fn(&mut Context) + Send>>,
        ignore_default_theme: bool,
    ) -> Window
    where
        P: HasWindowHandle,
        F: Fn(&mut Context),
        F: 'static + Send,
    {
        let window_settings =
            window_settings(&win_desc, scale_policy, true).with_parent(Some(parent));

        let window = Window::create(
            window_settings,
            move |window: WindowContext| -> Result<ViziaWindow, HandlerError> {
                Runtime::init_on_ui_thread();
                let mut cx = Context::new();

                cx.ignore_default_theme = ignore_default_theme;
                cx.add_built_in_styles();

                let cx = BackendContext::new(cx);

                Ok(ViziaWindow::new(
                    cx,
                    win_desc,
                    frame_policy,
                    window,
                    Some(Box::new(app)),
                    on_idle,
                    true, // is_parented
                ))
            },
        )
        .expect("Failed to create baseview window");

        window.show().expect("Failed to show baseview window");
        window
    }

    /// Open a new window that blocks the current thread until the window is destroyed.
    ///
    /// * `app` - The Vizia application builder.
    pub fn open_blocking<F>(
        win_desc: WindowDescription,
        scale_policy: WindowScalePolicy,
        frame_policy: FramePolicy,
        app: F,
        on_idle: Option<Box<dyn Fn(&mut Context) + Send>>,
        ignore_default_theme: bool,
    ) where
        F: Fn(&mut Context),
        F: 'static + Send,
    {
        let window_settings = window_settings(&win_desc, scale_policy, false);

        let window = Window::create(
            window_settings,
            move |window: WindowContext| -> Result<ViziaWindow, HandlerError> {
                Runtime::init_on_ui_thread();
                let mut cx = Context::new();

                cx.ignore_default_theme = ignore_default_theme;
                cx.add_built_in_styles();

                let cx = BackendContext::new(cx);

                Ok(ViziaWindow::new(
                    cx,
                    win_desc,
                    frame_policy,
                    window,
                    Some(Box::new(app)),
                    on_idle,
                    false, // is_parented
                ))
            },
        )
        .expect("Failed to create baseview window");

        window.run_until_closed().expect("baseview event loop failed");
    }
}

impl WindowHandler for ViziaWindow {
    fn damage(&self, _area: baseview::DamageArea) {
        // Some hosts dispatch baseview callbacks from a different UI thread than
        // window construction; ensure this callback thread is registered.
        Runtime::init_on_ui_thread();
        if let Ok(mut application) = self.application.try_borrow_mut() {
            application.mark_damaged();
        }
    }

    fn draw(&self) -> Result<(), HandlerError> {
        // Some hosts dispatch baseview callbacks from a different UI thread than
        // window construction; ensure this callback thread is registered.
        Runtime::init_on_ui_thread();
        let Ok(mut application) = self.application.try_borrow_mut() else {
            return Ok(());
        };

        // Run the embedder's idle callback on every frame tick, not only after
        // an AppKit event. Mirrors what `vizia_winit` does in its
        // `about_to_wait` handler: the idle callback is the documented hook
        // for embedders to drain work queued from off-UI threads (e.g. a
        // plugin host delivering keys on the host thread), and invoking it
        // only from `on_event` means such work can stall indefinitely when
        // AppKit events are sparse — observed in a macOS VST3 host where
        // keystrokes queued from `IPlugView::onKeyDown` only drained when
        // the next mouse move or key event happened to arrive.
        application.handle_idle(&self.on_idle);

        // Compute needs redraw after the idle callback so
        // event queued during on idle now trigger the cb.
        let needs_redraw = application.on_frame_update();

        application.render();

        // Schedule the next frame according to the configured frame policy.
        application.schedule_next_frame(needs_redraw);

        Ok(())
    }

    fn resized(&self, new_size: baseview::WindowSize) -> Result<(), HandlerError> {
        Runtime::init_on_ui_thread();
        if let Ok(mut application) = self.application.try_borrow_mut() {
            application.handle_resized(new_size);
            application.request_redraw();
        }

        Ok(())
    }

    fn on_event(&self, event: Event) -> EventStatus {
        // Some hosts dispatch baseview callbacks from a different UI thread than
        // window construction; ensure this callback thread is registered.
        Runtime::init_on_ui_thread();
        let Ok(mut application) = self.application.try_borrow_mut() else {
            return EventStatus::Ignored;
        };

        let status = application.handle_event(event);

        application.handle_idle(&self.on_idle);

        // Wake the loop so the event is rendered on the next frame.
        application.request_redraw();

        status
    }
}

impl Drop for ViziaWindow {
    fn drop(&mut self) {
        Runtime::deinit_on_ui_thread();
    }
}

/// Builds the baseview settings for a window opened from `win_desc`.
///
/// A standalone window also gets the description's min and max size, like the winit backend,
/// and is created inside them. A parented window doesn't, since its host decides how big it can
/// be.
fn window_settings(
    win_desc: &WindowDescription,
    scale_policy: WindowScalePolicy,
    is_parented: bool,
) -> WindowSettings {
    let (min_size, max_size) = size_limits(win_desc, is_parented);

    WindowSettings::new()
        .with_title(win_desc.title.clone())
        .with_size(user_scaled_size(win_desc, is_parented))
        .with_min_size::<LogicalSize<f64>>(min_size)
        .with_max_size::<LogicalSize<f64>>(max_size)
        .with_fallback_scale_factor(match scale_policy {
            WindowScalePolicy::ScaleFactor(scale) => Some(scale),
            WindowScalePolicy::SystemScaleFactor => None,
        })
        .with_gl_config(GlConfig::default())
}

/// The min and max logical size the window is kept inside.
///
/// They're sizes of the window itself, which is what the OS enforces while the user resizes it,
/// so the user scale factor doesn't change them. baseview has no way to change them after the
/// window is open.
fn size_limits(
    win_desc: &WindowDescription,
    is_parented: bool,
) -> (Option<LogicalSize<f64>>, Option<LogicalSize<f64>>) {
    if is_parented {
        return (None, None);
    }

    // baseview 0.4.2 makes equal limits a fixed window whose resize() does nothing on macOS
    // and X11.
    if win_desc.min_inner_size == win_desc.max_inner_size {
        return (None, None);
    }

    let logical = |size: WindowSize| LogicalSize::new(size.width as f64, size.height as f64);

    (win_desc.min_inner_size.map(logical), win_desc.max_inner_size.map(logical))
}

/// The logical size the window is created at, or resized to, for the description's user scale
/// factor.
///
/// A window with min and max sizes can't be made outside them, so this clamps the size the same
/// way baseview does (max first, then min) to give the size the window actually ends up at.
pub(crate) fn user_scaled_size(
    win_desc: &WindowDescription,
    is_parented: bool,
) -> LogicalSize<f64> {
    let mut size = LogicalSize::new(
        // We have our own uniform non-DPI scaling factor that gets applied in addition to
        // the DPI scaling since both can change independently at runtime
        win_desc.inner_size.width as f64 * win_desc.user_scale_factor,
        win_desc.inner_size.height as f64 * win_desc.user_scale_factor,
    );

    let (min_size, max_size) = size_limits(win_desc, is_parented);

    if let Some(max_size) = max_size {
        size.width = size.width.min(max_size.width);
        size.height = size.height.min(max_size.height);
    }

    if let Some(min_size) = min_size {
        size.width = size.width.max(min_size.width);
        size.height = size.height.max(min_size.height);
    }

    size
}

pub struct WindowView {}

impl View for WindowView {
    fn element(&self) -> Option<&'static str> {
        Some("window")
    }
}

pub fn create_surface(
    size: (i32, i32),
    fb_info: FramebufferInfo,
    gr_context: &mut skia_safe::gpu::DirectContext,
) -> Surface {
    let backend_render_target = backend_render_targets::make_gl(size, None, 8, fb_info);

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

#[cfg(test)]
mod tests {
    use super::*;
    use baseview::dpi::Size;

    fn description() -> WindowDescription {
        WindowDescription::new()
            .with_inner_size(800, 600)
            .with_min_inner_size(400, 300)
            .with_max_inner_size(1000, 700)
    }

    fn logical(width: f64, height: f64) -> Option<Size> {
        Some(Size::Logical(LogicalSize::new(width, height)))
    }

    #[test]
    fn standalone_window_gets_the_size_limits() {
        let desc = description().with_inner_size(600, 400).with_scale_factor(1.5);
        let settings = window_settings(&desc, WindowScalePolicy::SystemScaleFactor, false);

        // The limits are the window's logical size, so the user scale doesn't change them.
        assert_eq!(settings.min_size, logical(400.0, 300.0));
        assert_eq!(settings.max_size, logical(1000.0, 700.0));
        assert_eq!(settings.size, Size::Logical(LogicalSize::new(900.0, 600.0)));
    }

    #[test]
    fn standalone_window_is_created_inside_the_size_limits() {
        let small = description().with_inner_size(200, 100);
        let large = description().with_scale_factor(1.5);
        let settings = |desc| window_settings(desc, WindowScalePolicy::SystemScaleFactor, false);

        assert_eq!(settings(&small).size, Size::Logical(LogicalSize::new(400.0, 300.0)));
        assert_eq!(settings(&large).size, Size::Logical(LogicalSize::new(1000.0, 700.0)));
    }

    #[test]
    fn equal_min_and_max_sizes_are_not_forwarded() {
        let desc = description().with_min_inner_size(500, 400).with_max_inner_size(500, 400);
        let settings = window_settings(&desc, WindowScalePolicy::SystemScaleFactor, false);

        assert_eq!(settings.min_size, None);
        assert_eq!(settings.max_size, None);
        assert_eq!(
            user_scaled_size(&desc.with_scale_factor(2.0), false),
            LogicalSize::new(1600.0, 1200.0)
        );
    }

    #[test]
    fn parented_window_leaves_its_size_to_the_host() {
        let desc = description().with_inner_size(200, 100);
        let settings = window_settings(&desc, WindowScalePolicy::SystemScaleFactor, true);

        assert_eq!(settings.min_size, None);
        assert_eq!(settings.max_size, None);
        assert_eq!(settings.size, Size::Logical(LogicalSize::new(200.0, 100.0)));
    }

    #[test]
    fn user_scale_resize_stays_inside_the_size_limits() {
        let size = |scale| user_scaled_size(&description().with_scale_factor(scale), false);

        assert_eq!(size(2.0), LogicalSize::new(1000.0, 700.0));
        assert_eq!(size(1.25), LogicalSize::new(1000.0, 700.0));
        assert_eq!(size(0.25), LogicalSize::new(400.0, 300.0));
        assert_eq!(size(1.125), LogicalSize::new(900.0, 675.0));
    }

    #[test]
    fn user_scale_resize_lets_the_min_size_win_like_baseview() {
        let desc = description().with_max_inner_size(300, 200);

        assert_eq!(user_scaled_size(&desc, false), LogicalSize::new(400.0, 300.0));
    }

    #[test]
    fn parented_user_scale_resize_is_not_clamped() {
        let desc = description().with_scale_factor(2.0);

        assert_eq!(user_scaled_size(&desc, true), LogicalSize::new(1600.0, 1200.0));
    }
}
