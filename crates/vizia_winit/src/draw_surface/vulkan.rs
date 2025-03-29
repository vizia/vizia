use std::{
    error::Error,
    sync::{Arc, OnceLock},
};

use skia_safe::{
    gpu::{
        backend_render_targets, direct_contexts, surfaces,
        vk::{BackendContext, Format, GetProc, GetProcOf, ImageInfo, ImageLayout},
        BackendRenderTarget, DirectContext, SurfaceOrigin,
    },
    ColorSpace, ColorType, Surface, SurfaceProps,
};

use vulkano::{
    device::{
        physical::{PhysicalDevice, PhysicalDeviceType},
        Device, DeviceCreateInfo, DeviceExtensions, Queue, QueueCreateInfo, QueueFlags,
    },
    image::{Image, ImageUsage},
    instance::{Instance, InstanceCreateFlags, InstanceCreateInfo, InstanceExtensions},
    swapchain::{
        acquire_next_image, PresentMode, RectangleLayer, Surface as VulkanSurface, Swapchain,
        SwapchainAcquireFuture, SwapchainCreateInfo, SwapchainPresentInfo,
    },
    sync::{self, GpuFuture},
    Handle, LoadingError, Validated, Version, VulkanError, VulkanLibrary, VulkanObject,
};

use vizia_core::prelude::{BoundingBox, Entity};
use vizia_window::WindowDescription;

use winit::{
    dpi::PhysicalSize,
    event_loop::ActiveEventLoop,
    window::{Window, WindowAttributes},
};

use super::{DrawSurface, GraphicsBackend};

pub(super) struct WinState {
    entity: Entity,
    window: Arc<Window>,

    vsync: bool,
    is_initially_cloaked: bool,

    surfaces: Vec<(Surface, BackendRenderTarget)>,
    direct_context: DirectContext,

    queue: Arc<Queue>,
    images: Vec<Arc<Image>>,

    swapchain: Arc<Swapchain>,
    swapchain_acquire_future: Option<SwapchainAcquireFuture>,
    swapchain_needs_recreated: bool,

    previous_frame_end: Option<Box<dyn GpuFuture>>,
}

impl DrawSurface for WinState {
    fn entity(&self) -> Entity {
        self.entity
    }

    fn window(&self) -> Arc<Window> {
        self.window.clone()
    }

    fn backend(&self) -> GraphicsBackend {
        GraphicsBackend::Vulkan
    }

    fn surfaces_mut(&mut self) -> Option<(&mut Surface, &mut Surface)> {
        if self.swapchain_needs_recreated {
            self.resize(self.window.inner_size());
        }

        if self.swapchain_acquire_future.is_none() {
            match acquire_next_image(self.swapchain.clone(), None).map_err(Validated::unwrap) {
                Ok((_, suboptimal, acquire_future)) => {
                    self.swapchain_acquire_future = Some(acquire_future);
                    self.swapchain_needs_recreated |= suboptimal;
                }
                Err(VulkanError::OutOfDate) => {
                    self.swapchain_needs_recreated = true;
                    return None;
                }
                Err(e) => {
                    panic!("failed to acquire next image: {e}");
                }
            }
        }

        let acquire_future = self.swapchain_acquire_future.as_ref()?;

        let image_index = acquire_future.image_index() as usize;

        // equivalent to: get_many_mut([i, (i - 1) % len])

        let ((surface, _), (dirty_surface, _)) = {
            match self.surfaces.split_at_mut(image_index) {
                ([.., s1], [s0, ..]) => (s0, s1),
                ([], [s0, .., s1]) => (s0, s1),
                _ => unreachable!(),
            }
        };

        Some((surface, dirty_surface))
    }

    fn swap_buffers(&mut self, dirty_rect: BoundingBox) {
        let dirty_rect = self.image_bounds().intersection(&dirty_rect);
        if dirty_rect.is_empty() {
            return;
        }

        let Some(acquire_future) = self.swapchain_acquire_future.take() else {
            return;
        };
        acquire_future.wait(None).unwrap();

        let mut previous_frame_end = self.previous_frame_end.take().unwrap();
        previous_frame_end.cleanup_finished();

        // Submit

        let image_index = acquire_future.image_index();
        let (surface, _) = &mut self.surfaces[image_index as usize];

        self.direct_context.perform_deferred_cleanup(Default::default(), None);
        self.direct_context.flush_and_submit_surface(surface, None);

        // Present

        let mut swapchain_info =
            SwapchainPresentInfo::swapchain_image_index(self.swapchain.clone(), image_index);

        swapchain_info.present_mode = Some(self.current_present_mode());

        if self.supports_incremental_present() {
            let BoundingBox { x, y, w, h } = dirty_rect;
            let rect = RectangleLayer {
                offset: [x as u32, y as u32],
                extent: [w as u32, h as u32],
                layer: 0,
            };
            swapchain_info.present_regions = vec![rect];
        }

        match previous_frame_end
            .join(acquire_future)
            .then_swapchain_present(self.queue.clone(), swapchain_info)
            .then_signal_fence_and_flush()
            .map_err(Validated::unwrap)
        {
            Ok(future) => {
                self.previous_frame_end = future.boxed().into();
            }
            Err(VulkanError::OutOfDate) => {
                self.previous_frame_end = sync::now(self.queue.device().clone()).boxed().into();
                self.swapchain_needs_recreated = true;
            }
            Err(e) => {
                panic!("failed to flush future: {e}");
            }
        }
    }

    fn resize(&mut self, size: PhysicalSize<u32>) -> bool {
        let [w, h] = self.swapchain.image_extent();

        if size.width == 0 || size.height == 0 {
            return false;
        }
        if size.width == w && size.height == h {
            return false;
        }

        (self.swapchain, self.images) = self
            .swapchain
            .recreate(SwapchainCreateInfo {
                image_extent: size.into(),
                ..self.swapchain.create_info()
            })
            .expect("failed to recreate swapchain");

        self.swapchain_acquire_future = None;
        self.swapchain_needs_recreated = false;

        self.recreate_surfaces();

        true
    }

    fn is_initially_cloaked(&mut self) -> &mut bool {
        &mut self.is_initially_cloaked
    }
}

impl WinState {
    pub fn new(
        entity: Entity,
        window_attributes: &WindowAttributes,
        window_description: &WindowDescription,
        event_loop: &ActiveEventLoop,
    ) -> Result<Self, Box<dyn Error>> {
        let vsync = window_description.vsync;

        let window = event_loop.create_window(window_attributes.clone())?.into();

        let library = get_vulkan_library()?;

        let instance = get_vulkan_instance(&library, &window)?;

        let surface = create_vulkan_surface(&instance, &window)?;

        let (device_extensions, physical_device, queue_family_index) =
            select_physical_device(&instance, &surface)?;

        let (device, queue) =
            create_device(&physical_device, device_extensions, queue_family_index)?;

        let (swapchain, images) = create_swapchain(&device, &surface, window.inner_size());

        let direct_context = create_direct_context(&queue, &device);

        let previous_frame_end = sync::now(queue.device().clone()).boxed().into();

        let mut this = Self {
            entity,
            window,

            vsync,
            is_initially_cloaked: true,

            surfaces: vec![],

            direct_context,

            queue,
            images,

            swapchain,
            swapchain_acquire_future: None,
            swapchain_needs_recreated: false,

            previous_frame_end,
        };

        this.recreate_surfaces();

        Ok(this)
    }

    fn recreate_surfaces(&mut self) {
        self.surfaces.clear();
        self.surfaces.extend(self.images.iter().map(|image| {
            // The format and color type are platform dependent.

            let (format, color_type) = match image.format() {
                vulkano::format::Format::R8G8B8A8_UNORM => {
                    (Format::R8G8B8A8_UNORM, ColorType::RGBA8888)
                }
                vulkano::format::Format::B8G8R8A8_UNORM => {
                    (Format::B8G8R8A8_UNORM, ColorType::BGRA8888)
                }
                _ => unimplemented!(),
            };

            let mut image_info = ImageInfo::default();
            image_info.layout = ImageLayout::COLOR_ATTACHMENT_OPTIMAL;
            image_info.format = format;

            // SAFETY: `image` must outlive usages of `image_info`
            unsafe {
                image_info.set_image(image.handle().as_raw() as _);
            }

            let [width, height, _] = image.extent();
            let image_size = (width as i32, height as i32);

            let backend_render_target = backend_render_targets::make_vk(image_size, &image_info);

            let surface_props = SurfaceProps::new_with_text_properties(
                Default::default(),
                Default::default(),
                0.5,
                0.0,
            );

            let surface = surfaces::wrap_backend_render_target(
                &mut self.direct_context,
                &backend_render_target,
                SurfaceOrigin::TopLeft,
                color_type,
                ColorSpace::new_srgb(),
                Some(&surface_props),
            )
            .unwrap();

            (surface, backend_render_target)
        }));
    }

    #[inline]
    fn supports_incremental_present(&self) -> bool {
        self.queue //
            .device()
            .physical_device()
            .supported_extensions()
            .khr_incremental_present
    }

    #[inline]
    fn current_present_mode(&mut self) -> PresentMode {
        if self.vsync {
            PresentMode::Fifo
        } else {
            PresentMode::Immediate
        }
    }

    #[inline]
    fn image_bounds(&self) -> BoundingBox {
        let [w, h] = self.swapchain.image_extent();
        BoundingBox { x: 0.0, y: 0.0, w: w as f32, h: h as f32 }
    }
}

fn get_vulkan_library() -> Result<Arc<VulkanLibrary>, LoadingError> {
    static VULKAN_LIBRARY: OnceLock<Arc<VulkanLibrary>> = OnceLock::new();

    if let Some(library) = VULKAN_LIBRARY.get() {
        return Ok(library.clone());
    }

    let library = VulkanLibrary::new()?;

    VULKAN_LIBRARY.set(library.clone()).unwrap();

    Ok(library)
}

fn get_vulkan_instance(
    vulkan_library: &Arc<VulkanLibrary>,
    window: &Arc<Window>,
) -> Result<Arc<Instance>, Validated<VulkanError>> {
    static VULKAN_INSTANCE: OnceLock<Arc<Instance>> = OnceLock::new();

    if let Some(instance) = VULKAN_INSTANCE.get() {
        return Ok(instance.clone());
    }

    let mut enabled_extensions = InstanceExtensions {
        ext_surface_maintenance1: true, // Required for vsync support.
        ..VulkanSurface::required_extensions(window)
    };

    //
    if vulkan_library.api_version() < Version::V1_1 {
        // Required by `ext_swapchain_maintenance1` on older versions.
        enabled_extensions.khr_get_physical_device_properties2 = true;
    }

    let instance = Instance::new(
        vulkan_library.clone(),
        InstanceCreateInfo {
            flags: InstanceCreateFlags::ENUMERATE_PORTABILITY,
            enabled_extensions,
            ..Default::default()
        },
    )?;

    VULKAN_INSTANCE.set(instance.clone()).unwrap();

    Ok(instance)
}

fn create_vulkan_surface(
    instance: &Arc<Instance>,
    window: &Arc<Window>,
) -> Result<Arc<VulkanSurface>, Validated<VulkanError>> {
    VulkanSurface::from_window(instance.clone(), window.clone())
}

fn select_physical_device(
    instance: &Arc<Instance>,
    surface: &VulkanSurface,
) -> Result<(DeviceExtensions, Arc<PhysicalDevice>, u32), VulkanError> {
    let required_extensions = DeviceExtensions {
        khr_swapchain: true,
        ext_swapchain_maintenance1: true, // Required for vsync support.
        ..DeviceExtensions::empty()
    };

    let (physical_device, queue_family_index) = instance
        .enumerate_physical_devices()?
        .filter_map(|device| {
            let index = device
                .supported_extensions()
                .contains(&required_extensions)
                .then_some(device.queue_family_properties())?
                .into_iter()
                .enumerate()
                .find_map(|(i, props)| {
                    let index = i as u32;
                    let graphics = props.queue_flags.intersects(QueueFlags::GRAPHICS);
                    let surface_support = device.surface_support(index, surface).ok()?;
                    (graphics && surface_support).then_some(index)
                })?;
            Some((device, index))
        })
        .min_by_key(|(device, _)| match device.properties().device_type {
            PhysicalDeviceType::DiscreteGpu => 0,
            PhysicalDeviceType::IntegratedGpu => 1,
            PhysicalDeviceType::VirtualGpu => 2,
            PhysicalDeviceType::Cpu => 3,
            PhysicalDeviceType::Other => 4,
            _ => 5,
        })
        .expect("no suitable physical device found");

    let device_extensions = physical_device.supported_extensions().intersection(
        // Add any optional extensions we may want to use here.
        &DeviceExtensions {
            // Support incremental rendering via "dirty rects".
            khr_incremental_present: true,
            ..required_extensions
        },
    );

    Ok((device_extensions, physical_device, queue_family_index))
}

fn create_device(
    physical_device: &Arc<PhysicalDevice>,
    enabled_extensions: DeviceExtensions,
    queue_family_index: u32,
) -> Result<(Arc<Device>, Arc<Queue>), Validated<VulkanError>> {
    let (device, mut queues) = Device::new(
        physical_device.clone(),
        DeviceCreateInfo {
            queue_create_infos: vec![QueueCreateInfo { queue_family_index, ..Default::default() }],
            enabled_extensions,
            ..Default::default()
        },
    )?;

    let queue = queues.next().unwrap();

    Ok((device, queue))
}

fn create_swapchain(
    device: &Arc<Device>,
    surface: &Arc<VulkanSurface>,
    image_extent: PhysicalSize<u32>,
) -> (Arc<Swapchain>, Vec<Arc<Image>>) {
    let physical_device = device.physical_device();

    let surface_capabilities = physical_device
        .surface_capabilities(
            &surface,
            vulkano::swapchain::SurfaceInfo {
                present_mode: Some(PresentMode::Immediate),
                ..Default::default()
            },
        )
        .unwrap();

    let surface_formats = physical_device.surface_formats(&surface, Default::default()).unwrap();

    let (image_format, image_color_space) = surface_formats[0];

    let composite_alpha =
        surface_capabilities.supported_composite_alpha.into_iter().next().unwrap();

    Swapchain::new(
        device.clone(),
        surface.clone(),
        SwapchainCreateInfo {
            min_image_count: surface_capabilities.min_image_count.max(2),
            image_format,
            image_color_space,
            image_extent: image_extent.into(),
            image_usage: ImageUsage::COLOR_ATTACHMENT,
            present_mode: PresentMode::Immediate,
            present_modes: [PresentMode::Immediate, PresentMode::Fifo].into_iter().collect(),
            composite_alpha,
            ..Default::default()
        },
    )
    .expect("failed to create swapchain")
}

fn create_direct_context(queue: &Queue, device: &Device) -> DirectContext {
    let instance = device.instance();
    let physical_device = device.physical_device();

    let device_ptr = device.handle().as_raw() as _;
    let instance_ptr = instance.handle().as_raw() as _;
    let physical_device_ptr = physical_device.handle().as_raw() as _;
    let queue_ptr = (queue.handle().as_raw() as _, queue.queue_family_index() as _);

    let get_proc = make_get_proc(instance);
    let instance_extensions = convert_extension(instance.enabled_extensions().into_iter());
    let device_extensions = convert_extension(device.enabled_extensions().into_iter());

    let backend_context = unsafe {
        BackendContext::new_with_extensions(
            instance_ptr,
            physical_device_ptr,
            device_ptr,
            queue_ptr,
            &get_proc,
            &instance_extensions,
            &device_extensions,
        )
    };

    direct_contexts::make_vulkan(&backend_context, None) //
        .expect("failed to create direct context")
}

fn convert_extension<I>(extensions: I) -> Vec<&'static str>
where
    I: Iterator<Item = (&'static str, bool)>,
{
    extensions //
        .filter_map(|(feature, enabled)| enabled.then_some(feature))
        .collect()
}

fn make_get_proc(instance: &Instance) -> impl GetProc {
    let library = instance.library();

    // This function is not exposed by vulkano so we manually retrieve it from the vulkan instance.
    // https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetInstanceProcAddr.html
    let get_instance_proc_addr = {
        type GetInstanceProcAddr = unsafe extern "system" fn(
            <Instance as VulkanObject>::Handle,
            *const std::os::raw::c_char,
        )
            -> Option<unsafe extern "system" fn()>;

        let instance = instance.handle();
        let name = b"vkGetInstanceProcAddr\0".as_ptr().cast();
        unsafe {
            std::mem::transmute::<_, GetInstanceProcAddr>(
                library.get_instance_proc_addr(instance, name).unwrap(),
            )
        }
    };

    let get_device_proc_addr = instance.fns().v1_0.get_device_proc_addr;

    let get_proc = move |get_proc_of| {
        match get_proc_of {
            GetProcOf::Instance(raw_instance, name) => {
                let instance = <_>::from_raw(raw_instance as _);
                unsafe { get_instance_proc_addr(instance, name) }
            }
            GetProcOf::Device(raw_device, name) => {
                let device = <_>::from_raw(raw_device as _);
                unsafe { get_device_proc_addr(device, name) }
            }
        }
        .unwrap() as _
    };

    get_proc
}
