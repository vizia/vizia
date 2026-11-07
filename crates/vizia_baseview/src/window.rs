use crate::application::ApplicationRunner;
use crate::window_settings::{FramePolicy, WindowScalePolicy};
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
use vizia_core::backend::*;
use vizia_core::prelude::*;

/// Handles a vizia_baseview application
pub(crate) struct ViziaWindow {
    application: RefCell<ApplicationRunner>,
    #[allow(clippy::type_complexity)]
    on_idle: Option<Box<dyn Fn(&mut Context) + Send>>,
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

        // Register a no-op sync waker. baseview doesn't expose a proxy-event primitive we
        // can use to wake the event loop from another thread, but it drives `on_frame` at
        // the host compositor's vsync rate anyway — signal changes from off-UI writes will
        // be observed on the next frame (at most one frame of latency).
        //
        // Registering the waker here — even as a no-op — has two benefits: (1) it matches
        // the pattern in `vizia_winit`, so embedders don't have to know about sync-runtime
        // internals; (2) if a future baseview release gains a window-wake mechanism, only
        // this one line needs to change.
        Runtime::set_sync_effect_waker(|| {});
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
        cx.add_window(WindowView {});

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

        ViziaWindow { application: RefCell::new(application), on_idle }
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
        let window_settings = WindowSettings::new()
            .with_title(win_desc.title.clone())
            .with_size(baseview::dpi::LogicalSize::new(
                // We have our own uniform non-DPI scaling factor that gets applied in addition to
                // the DPI scaling since both can change independently at runtime
                win_desc.inner_size.width as f64 * win_desc.user_scale_factor,
                win_desc.inner_size.height as f64 * win_desc.user_scale_factor,
            ))
            .with_parent(Some(parent))
            .with_fallback_scale_factor(match scale_policy {
                WindowScalePolicy::ScaleFactor(scale) => Some(scale),
                WindowScalePolicy::SystemScaleFactor => None,
            })
            .with_gl_config(GlConfig::default());

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
        let window_settings = WindowSettings::new()
            .with_title(win_desc.title.clone())
            .with_size(baseview::dpi::LogicalSize::new(
                win_desc.inner_size.width as f64 * win_desc.user_scale_factor,
                win_desc.inner_size.height as f64 * win_desc.user_scale_factor,
            ))
            .with_fallback_scale_factor(match scale_policy {
                WindowScalePolicy::ScaleFactor(scale) => Some(scale),
                WindowScalePolicy::SystemScaleFactor => None,
            })
            .with_gl_config(GlConfig::default());

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

pub struct WindowView {}

impl View for WindowView {}

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
