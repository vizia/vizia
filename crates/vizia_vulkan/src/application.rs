//! Main VulkanApplication type - headless vizia renderer.
//!
//! A `VulkanApplication` manages a single Vizia `Context` and renders to a Vulkan `RenderTarget`.
//! This matches how vizia's winit backend works: one Context, one main window.

use ash::vk::Handle;
use skia_safe::{ColorSpace, Surface, SurfaceProps, gpu::SurfaceOrigin};
use vizia_core::backend::BackendContext;
use vizia_core::events::EventManager;
use vizia_core::prelude::*;
use vizia_window::WindowDescription;

use crate::render_target::RenderTarget;
use crate::state::State;

struct RenderTargetState {
    surface: Surface,
    dirty_surface: Surface,
    target: RenderTarget,
}
impl RenderTargetState {
    unsafe fn new(
        skia_context: &mut skia_safe::gpu::DirectContext,
        target: RenderTarget,
    ) -> Option<Self> {
        let image_info = unsafe {
            skia_safe::gpu::vk::ImageInfo::new(
                target.image.as_raw() as _,
                skia_safe::gpu::vk::Alloc::default(),
                skia_safe::gpu::vk::ImageTiling::OPTIMAL,
                skia_safe::gpu::vk::ImageLayout::UNDEFINED,
                vulkan_format_to_skia_format(target.format),
                1,
                None,
                None,
                None,
                None,
            )
        };

        let backend_render_target = skia_safe::gpu::backend_render_targets::make_vk(
            (target.extent.width as i32, target.extent.height as i32),
            &image_info,
        );

        let surface_props = SurfaceProps::new_with_text_properties(
            skia_safe::SurfacePropsFlags::default(),
            skia_safe::PixelGeometry::default(),
            0.5,
            0.0,
        );

        let mut surface = skia_safe::gpu::surfaces::wrap_backend_render_target(
            skia_context,
            &backend_render_target,
            SurfaceOrigin::TopLeft,
            vulkan_format_to_skia_color_type(target.format),
            ColorSpace::new_cicp(
                skia_safe::named_primaries::CicpId::Rec709,
                skia_safe::named_transfer_fn::CicpId::SRGB,
            ),
            Some(&surface_props),
        )?;
        let dirty_surface = surface.new_surface_with_dimensions((
            target.extent.width as i32,
            target.extent.height as i32,
        ))?;

        Some(Self { surface, dirty_surface, target })
    }

    unsafe fn replace_render_target(
        &mut self,
        skia_context: &mut skia_safe::gpu::DirectContext,
        new_target: RenderTarget,
    ) -> bool {
        let new_state = if let Some(new_state) = unsafe { Self::new(skia_context, new_target) } {
            new_state
        } else {
            return false;
        };

        self.surface = new_state.surface;
        self.dirty_surface = new_state.dirty_surface;
        self.target = new_state.target;

        true
    }
}

fn vulkan_format_to_skia_format(format: ash::vk::Format) -> skia_safe::gpu::vk::Format {
    unsafe { std::mem::transmute(format) }
}

fn vulkan_format_to_skia_color_type(format: ash::vk::Format) -> skia_safe::ColorType {
    match format {
        ash::vk::Format::A2B10G10R10_UNORM_PACK32 => skia_safe::ColorType::BGRA1010102,
        ash::vk::Format::A2R10G10B10_UNORM_PACK32 => skia_safe::ColorType::RGBA1010102,
        ash::vk::Format::B8G8R8A8_SRGB | ash::vk::Format::B8G8R8A8_UNORM => {
            skia_safe::ColorType::BGRA8888
        }
        ash::vk::Format::R8G8B8A8_SRGB | ash::vk::Format::R8G8B8A8_UNORM => {
            skia_safe::ColorType::RGBA8888
        }
        ash::vk::Format::R16G16B16A16_UNORM | ash::vk::Format::R16G16B16A16_SFLOAT => {
            skia_safe::ColorType::R16G16B16A16UNorm
        }
        ash::vk::Format::R5G6B5_UNORM_PACK16 => skia_safe::ColorType::RGB565,
        _ => unimplemented!("{format:?}"),
    }
}

pub struct VulkanApplication {
    cx: BackendContext,
    event_manager: EventManager,
    vulkan_state: State,
    render_target: Option<RenderTargetState>,
    needs_render: bool,
}
impl VulkanApplication {
    pub fn new(
        mut vulkan_state: State,
        target: RenderTarget,
        scale_factor: f64,
        content: impl FnOnce(&mut Context) + 'static,
    ) -> Option<Self> {
        let mut cx = BackendContext::new(Context::new());

        let window_description = WindowDescription {
            inner_size: vizia_window::WindowSize {
                width: target.extent.width,
                height: target.extent.height,
            },
            ..WindowDescription::new()
        };

        cx.context()
            .windows
            .insert(Entity::root(), WindowState { window_description, ..Default::default() });
        cx.context().tree.set_window(Entity::root(), true);

        cx.set_scale_factor(scale_factor);
        cx.set_window_size(Entity::root(), target.extent.width as f32, target.extent.height as f32);

        content(cx.context());

        cx.needs_refresh(Entity::root());

        let skia_context = vulkan_state.skia_context_mut();
        let render_target = unsafe { RenderTargetState::new(skia_context, target) }?;

        Some(Self {
            cx,
            event_manager: EventManager::new(),
            vulkan_state,
            render_target: Some(render_target),
            needs_render: true,
        })
    }

    pub fn replace_render_target(&mut self, new_target: RenderTarget) -> bool {
        if let Some(target_state) = self.render_target.as_mut() {
            let skia_context = self.vulkan_state.skia_context_mut();

            if unsafe { target_state.replace_render_target(skia_context, new_target) } {
                self.cx.set_window_size(
                    Entity::root(),
                    target_state.target.extent.width as f32,
                    target_state.target.extent.height as f32,
                );
                self.cx.needs_refresh(Entity::root());
                self.needs_render = true;
                return true;
            }
        }

        false
    }

    pub fn handle_event(&mut self, event: WindowEvent) {
        if let WindowEvent::MouseMove(x, y) = event {
            let mouse = &mut self.cx.context().mouse;

            mouse.previous_cursor_x = mouse.cursor_x;
            mouse.previous_cursor_y = mouse.cursor_y;
            mouse.cursor_x = x;
            mouse.cursor_y = y;
        }

        self.needs_render = true;
        self.cx.emit_window_event(Entity::root(), event);
    }

    pub fn update(&mut self) -> bool {
        self.event_manager.flush_events(self.cx.context(), |_| {});

        self.cx.process_style_updates();
        self.cx.process_visual_updates();

        self.needs_render |= self.cx.process_animations();

        self.needs_render
    }

    pub fn render(&mut self) -> bool {
        if !self.needs_render {
            return false;
        }

        if let Some(target_state) = self.render_target.as_mut() {
            self.cx.draw(
                Entity::root(),
                &mut target_state.surface,
                &mut target_state.dirty_surface,
            );
        }

        self.vulkan_state.skia_context_mut().flush_and_submit();

        self.needs_render = false;
        true
    }

    pub fn needs_render(&self) -> bool {
        self.needs_render
    }

    pub fn context(&mut self) -> &mut Context {
        self.cx.context()
    }
}
