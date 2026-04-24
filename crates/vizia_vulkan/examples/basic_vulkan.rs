use raw_window_handle::{HasDisplayHandle, HasWindowHandle};
use std::ffi::CStr;
use vizia_core::prelude::*;
use vizia_vulkan::prelude::*;
use winit::application::ApplicationHandler;
use winit::event::{
    ElementState, KeyEvent, MouseButton as WinitMouseButton, MouseScrollDelta, WindowEvent,
};
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::keyboard::{KeyCode, PhysicalKey};
use winit::window::Window;

unsafe extern "system" fn vulkan_debug_callback(
    severity: ash::vk::DebugUtilsMessageSeverityFlagsEXT,
    msg_type: ash::vk::DebugUtilsMessageTypeFlagsEXT,
    p_callback_data: *const ash::vk::DebugUtilsMessengerCallbackDataEXT,
    _user_data: *mut std::ffi::c_void,
) -> ash::vk::Bool32 {
    let callback_data = unsafe { *p_callback_data };
    let message_id_name = if callback_data.p_message_id_name.is_null() {
        std::borrow::Cow::from("")
    } else {
        unsafe { CStr::from_ptr(callback_data.p_message_id_name).to_string_lossy() }
    };
    let message = unsafe { CStr::from_ptr(callback_data.p_message).to_string_lossy() };

    let type_str = match msg_type {
        ash::vk::DebugUtilsMessageTypeFlagsEXT::GENERAL => "GENERAL",
        ash::vk::DebugUtilsMessageTypeFlagsEXT::VALIDATION => "VALIDATION",
        ash::vk::DebugUtilsMessageTypeFlagsEXT::PERFORMANCE => "PERFORMANCE",
        _ => "OTHER",
    };

    match severity {
        ash::vk::DebugUtilsMessageSeverityFlagsEXT::ERROR => {
            log::error!("({type_str}): {message_id_name} - {message}");
        }
        ash::vk::DebugUtilsMessageSeverityFlagsEXT::WARNING => {
            log::warn!("({type_str}): {message_id_name} - {message}");
        }
        ash::vk::DebugUtilsMessageSeverityFlagsEXT::INFO => {
            log::info!("({type_str}): {message_id_name} - {message}");
        }
        _ => {
            log::trace!("({type_str}): {message_id_name} - {message}");
        }
    }

    ash::vk::FALSE
}

#[derive(Default)]
struct EventCatcher {
    scroll: Signal<(f32, f32)>,
    mouse_over: Signal<bool>,
    clicked: Signal<bool>,
}
impl EventCatcher {
    fn new<'a>(cx: &'a mut Context) -> Handle<'a, Self> {
        let scroll_catcher = Self::default();
        let Self { scroll, mouse_over, clicked } = scroll_catcher;

        View::build(scroll_catcher, cx, |cx| {
            ZStack::new(cx, |cx| {
                Element::new(cx)
                    .width(Pixels(300.0))
                    .height(Pixels(200.0))
                    .background_color(Color::rgb(50, 50, 150))
                    .border_color(clicked.map(
                        |&clicked| {
                            if clicked { Color::gray() } else { Color::white() }
                        },
                    ))
                    .border_width(Pixels(2.0));

                VStack::new(cx, |cx| {
                    Label::new(cx, scroll.map(|scroll| format!("Scroll: {scroll:?}")))
                        .color(Color::white());

                    mouse_over.set_or_bind(cx, |cx, over| {
                        if over.get() {
                            Label::new(cx, "Mouse over").color(Color::white());
                        }
                    });
                })
                .alignment(Alignment::Center)
                .hoverable(false);
            })
            .size(Auto)
            .hoverable(false)
            .alignment(Alignment::Center);
        })
        .size(Auto)
    }
}
impl View for EventCatcher {
    fn event(&mut self, _cx: &mut EventContext, event: &mut Event) {
        use vizia_core::window::WindowEvent;
        event.map(|event: &WindowEvent, _meta| match event {
            WindowEvent::MouseScroll(x, y) => {
                self.scroll.update(|scroll| {
                    scroll.0 += *x;
                    scroll.1 += *y
                });
            }
            WindowEvent::PressDown { .. } => {
                self.clicked.update(|state| *state = true);
            }
            WindowEvent::MouseUp(_) => {
                self.clicked.update(|state| *state = false);
            }
            WindowEvent::MouseOver => self.mouse_over.update(|over| *over = true),
            WindowEvent::MouseOut => self.mouse_over.update(|over| *over = false),
            WindowEvent::MouseMove(_, _) => {}
            event => log::trace!("event: {event:?}"),
        });
    }
}
impl Model for EventCatcher {}

struct VulkanExample {
    ui_app: VulkanApplication,
    vulkan_state: State,
    window: Option<Window>,

    current_frame: usize,
    in_flight_fences: Vec<ash::vk::Fence>,
    render_finished_semaphores: Vec<ash::vk::Semaphore>,
    image_available_semaphores: Vec<ash::vk::Semaphore>,

    ui_image_memory: ash::vk::DeviceMemory,
    ui_image_view: ash::vk::ImageView,
    ui_image: ash::vk::Image,

    sampler: ash::vk::Sampler,
    descriptor_sets: Vec<ash::vk::DescriptorSet>,
    descriptor_pool: ash::vk::DescriptorPool,
    descriptor_set_layout: ash::vk::DescriptorSetLayout,
    pipeline_layout: ash::vk::PipelineLayout,
    pipeline: ash::vk::Pipeline,

    swapchain_image_views: Vec<ash::vk::ImageView>,
    swapchain_images: Vec<ash::vk::Image>,
    swapchain_extent: ash::vk::Extent2D,
    swapchain_loader: ash::khr::swapchain::Device,
    swapchain: ash::vk::SwapchainKHR,
    surface_loader: ash::khr::surface::Instance,
    surface: ash::vk::SurfaceKHR,

    command_buffers: Vec<ash::vk::CommandBuffer>,
    command_pool: ash::vk::CommandPool,
    present_queue: ash::vk::Queue,
    graphics_queue: ash::vk::Queue,
    device: ash::Device,
    physical_device: ash::vk::PhysicalDevice,
    debug_instance: Option<ash::ext::debug_utils::Instance>,
    debug_messenger: Option<ash::vk::DebugUtilsMessengerEXT>,
    instance: ash::Instance,
    entry: ash::Entry,
}

impl VulkanExample {
    fn new(event_loop: &ActiveEventLoop) -> Self {
        let window = event_loop
            .create_window(
                Window::default_attributes()
                    .with_title("Vizia Vulkan headless UI")
                    .with_inner_size(winit::dpi::PhysicalSize::new(800, 600)),
            )
            .expect("Failed to create window");

        let entry = unsafe { ash::Entry::load().expect("Failed to load Vulkan entry") };
        let (instance, debug_instance, debug_messenger) = Self::create_instance(&entry, &window);
        let surface_loader = ash::khr::surface::Instance::new(&entry, &instance);

        let display_handle =
            window.display_handle().expect("Failed to get display handle").as_raw();
        let window_handle = window.window_handle().expect("Failed to get window handle").as_raw();

        let surface = unsafe {
            ash_window::create_surface(&entry, &instance, display_handle, window_handle, None)
                .expect("Failed to create Vulkan surface")
        };

        let (physical_device, graphics_qf, present_qf) =
            Self::pick_physical_device(&instance, &surface_loader, surface);

        let (device, graphics_queue, present_queue) =
            Self::create_logical_device(&instance, physical_device, graphics_qf, present_qf);

        let swapchain_loader = ash::khr::swapchain::Device::new(&instance, &device);
        let (swapchain, swapchain_images, swapchain_format, swapchain_extent) =
            Self::create_swapchain(
                ash::vk::Extent2D {
                    width: window.inner_size().width as u32,
                    height: window.inner_size().height as u32,
                },
                &surface_loader,
                &swapchain_loader,
                physical_device,
                surface,
                graphics_qf,
                present_qf,
            );

        log::info!("Swapchain: {}x{}", swapchain_extent.width, swapchain_extent.height);

        let (ui_image, ui_image_view, ui_image_memory) = Self::create_ui_image(
            &device,
            &instance,
            physical_device,
            swapchain_format,
            swapchain_extent,
        );

        let vulkan_state = unsafe {
            State::new(
                entry.clone(),
                instance.clone(),
                physical_device,
                device.clone(),
                graphics_queue,
                graphics_qf,
                ash::vk::API_VERSION_1_3,
            )
            .expect("Failed to create VulkanState")
        };

        let ui_target = RenderTarget::new(ui_image, swapchain_extent, swapchain_format, 1);
        let ui_app =
            VulkanApplication::new(vulkan_state.clone(), ui_target, window.scale_factor(), |cx| {
                cx.add_stylesheet(include_str!(
                    "../../vizia_core/resources/themes/default_layout.css"
                ))
                .expect("failed to load style");

                VStack::new(cx, |cx| {
                    Element::new(cx)
                        .width(Units::Auto)
                        .height(Units::Auto)
                        .text_wrap(false)
                        .text("Vizia Vulkan embedding")
                        .font_size(12.0)
                        .color(Color::white())
                        .background_color(Color::darkred());

                    Element::new(cx)
                        .width(Units::Auto)
                        .height(Units::Auto)
                        .text_wrap(false)
                        .text("Move your mouse over the box below!")
                        .font_size(10.0)
                        .color(Color::yellow());

                    EventCatcher::new(cx).padding(Pixels(5.0));

                    Element::new(cx)
                        .width(Units::Auto)
                        .height(Units::Auto)
                        .text_wrap(false)
                        .text("Press ESC to exit")
                        .font_size(10.0)
                        .color(Color::gray());
                })
                .padding(Pixels(20.0))
                .background_color(Color::rgb(50, 50, 50))
                .alignment(Alignment::Center);
            })
            .expect("Failed to create VulkanApplication");

        let (
            swapchain_image_views,
            pipeline,
            pipeline_layout,
            descriptor_set_layout,
            descriptor_pool,
            descriptor_sets,
            sampler,
        ) = Self::create_resources(
            &device,
            swapchain_format,
            swapchain_extent,
            &swapchain_images,
            ui_image,
        );

        let command_pool = Self::create_command_pool(&device, graphics_qf);

        const MAX_FRAMES_IN_FLIGHT: usize = 2;
        let command_buffers =
            Self::allocate_command_buffers(&device, command_pool, MAX_FRAMES_IN_FLIGHT as u32);

        let semaphore_info = ash::vk::SemaphoreCreateInfo::default();
        let fence_info =
            ash::vk::FenceCreateInfo::default().flags(ash::vk::FenceCreateFlags::SIGNALED);

        let mut image_available_semaphores = Vec::with_capacity(MAX_FRAMES_IN_FLIGHT);
        let mut render_finished_semaphores = Vec::with_capacity(MAX_FRAMES_IN_FLIGHT);
        let mut in_flight_fences = Vec::with_capacity(MAX_FRAMES_IN_FLIGHT);

        unsafe {
            for _ in 0..MAX_FRAMES_IN_FLIGHT {
                image_available_semaphores
                    .push(device.create_semaphore(&semaphore_info, None).unwrap());
                render_finished_semaphores
                    .push(device.create_semaphore(&semaphore_info, None).unwrap());
                in_flight_fences.push(device.create_fence(&fence_info, None).unwrap());
            }
        }

        Self {
            ui_app,
            vulkan_state,
            window: Some(window),
            current_frame: 0,
            in_flight_fences,
            render_finished_semaphores,
            image_available_semaphores,
            ui_image_memory,
            ui_image_view,
            ui_image,
            sampler,
            descriptor_sets,
            descriptor_pool,
            descriptor_set_layout,
            pipeline_layout,
            pipeline,
            swapchain_image_views,
            swapchain_images,
            swapchain_extent,
            swapchain_loader,
            swapchain,
            surface_loader,
            surface,
            command_buffers,
            command_pool,
            present_queue,
            graphics_queue,
            device,
            physical_device,
            debug_instance,
            debug_messenger,
            entry,
            instance,
        }
    }

    fn create_ui_image(
        device: &ash::Device,
        instance: &ash::Instance,
        physical_device: ash::vk::PhysicalDevice,
        format: ash::vk::Format,
        extent: ash::vk::Extent2D,
    ) -> (ash::vk::Image, ash::vk::ImageView, ash::vk::DeviceMemory) {
        let image_info = ash::vk::ImageCreateInfo::default()
            .image_type(ash::vk::ImageType::TYPE_2D)
            .format(format)
            .extent(ash::vk::Extent3D { width: extent.width, height: extent.height, depth: 1 })
            .mip_levels(1)
            .array_layers(1)
            .samples(ash::vk::SampleCountFlags::TYPE_1)
            .tiling(ash::vk::ImageTiling::OPTIMAL)
            .usage(ash::vk::ImageUsageFlags::COLOR_ATTACHMENT | ash::vk::ImageUsageFlags::SAMPLED)
            .initial_layout(ash::vk::ImageLayout::UNDEFINED);

        let image = unsafe { device.create_image(&image_info, None).unwrap() };

        let mem_reqs = unsafe { device.get_image_memory_requirements(image) };
        let mem_type = Self::find_memory_type(
            &instance,
            physical_device,
            mem_reqs.memory_type_bits,
            ash::vk::MemoryPropertyFlags::DEVICE_LOCAL,
        );
        let alloc_info = ash::vk::MemoryAllocateInfo::default()
            .allocation_size(mem_reqs.size)
            .memory_type_index(mem_type);

        let memory = unsafe { device.allocate_memory(&alloc_info, None).unwrap() };
        unsafe { device.bind_image_memory(image, memory, 0).unwrap() };

        let comps = ash::vk::ComponentMapping::default();
        let view_info = ash::vk::ImageViewCreateInfo::default()
            .image(image)
            .view_type(ash::vk::ImageViewType::TYPE_2D)
            .format(format)
            .components(comps)
            .subresource_range(ash::vk::ImageSubresourceRange {
                aspect_mask: ash::vk::ImageAspectFlags::COLOR,
                base_mip_level: 0,
                level_count: 1,
                base_array_layer: 0,
                layer_count: 1,
            });

        let image_view = unsafe { device.create_image_view(&view_info, None).unwrap() };
        (image, image_view, memory)
    }

    fn create_resources(
        device: &ash::Device,
        format: ash::vk::Format,
        extent: ash::vk::Extent2D,
        swapchain_images: &[ash::vk::Image],
        ui_image: ash::vk::Image,
    ) -> (
        Vec<ash::vk::ImageView>,
        ash::vk::Pipeline,
        ash::vk::PipelineLayout,
        ash::vk::DescriptorSetLayout,
        ash::vk::DescriptorPool,
        Vec<ash::vk::DescriptorSet>,
        ash::vk::Sampler,
    ) {
        let swapchain_image_views: Vec<_> = swapchain_images
            .iter()
            .map(|&image| {
                let view_info = ash::vk::ImageViewCreateInfo::default()
                    .image(image)
                    .view_type(ash::vk::ImageViewType::TYPE_2D)
                    .format(format)
                    .components(ash::vk::ComponentMapping::default())
                    .subresource_range(ash::vk::ImageSubresourceRange {
                        aspect_mask: ash::vk::ImageAspectFlags::COLOR,
                        base_mip_level: 0,
                        level_count: 1,
                        base_array_layer: 0,
                        layer_count: 1,
                    });
                unsafe { device.create_image_view(&view_info, None).unwrap() }
            })
            .collect();

        let binding = ash::vk::DescriptorSetLayoutBinding::default()
            .binding(0)
            .descriptor_type(ash::vk::DescriptorType::COMBINED_IMAGE_SAMPLER)
            .descriptor_count(1)
            .stage_flags(ash::vk::ShaderStageFlags::FRAGMENT);
        let descriptor_set_layout = unsafe {
            device
                .create_descriptor_set_layout(
                    &ash::vk::DescriptorSetLayoutCreateInfo::default()
                        .bindings(std::slice::from_ref(&binding)),
                    None,
                )
                .unwrap()
        };

        let pipeline_layout = unsafe {
            device
                .create_pipeline_layout(
                    &ash::vk::PipelineLayoutCreateInfo::default()
                        .set_layouts(std::slice::from_ref(&descriptor_set_layout)),
                    None,
                )
                .unwrap()
        };

        let vert_module = Self::create_shader_module(device, include_bytes!("shaders/vert.spv"));
        let frag_module = Self::create_shader_module(device, include_bytes!("shaders/frag.spv"));

        let vert_stage = ash::vk::PipelineShaderStageCreateInfo::default()
            .stage(ash::vk::ShaderStageFlags::VERTEX)
            .module(vert_module)
            .name(c"main");

        let frag_stage = ash::vk::PipelineShaderStageCreateInfo::default()
            .stage(ash::vk::ShaderStageFlags::FRAGMENT)
            .module(frag_module)
            .name(c"main");

        let shader_stages = [vert_stage, frag_stage];

        let vertex_input = ash::vk::PipelineVertexInputStateCreateInfo::default();

        let input_assembly = ash::vk::PipelineInputAssemblyStateCreateInfo::default()
            .topology(ash::vk::PrimitiveTopology::TRIANGLE_LIST);

        let viewport = ash::vk::Viewport::default()
            .x(0.0)
            .y(0.0)
            .width(extent.width as f32)
            .height(extent.height as f32)
            .min_depth(0.0)
            .max_depth(1.0);
        let scissor = ash::vk::Rect2D::default().extent(extent);
        let viewport_state = ash::vk::PipelineViewportStateCreateInfo::default()
            .viewports(std::slice::from_ref(&viewport))
            .scissors(std::slice::from_ref(&scissor));

        let rasterizer = ash::vk::PipelineRasterizationStateCreateInfo::default()
            .polygon_mode(ash::vk::PolygonMode::FILL)
            .line_width(1.0)
            .cull_mode(ash::vk::CullModeFlags::NONE)
            .front_face(ash::vk::FrontFace::COUNTER_CLOCKWISE);

        let multisample = ash::vk::PipelineMultisampleStateCreateInfo::default()
            .rasterization_samples(ash::vk::SampleCountFlags::TYPE_1);

        let blend_attachment = ash::vk::PipelineColorBlendAttachmentState::default()
            .color_write_mask(
                ash::vk::ColorComponentFlags::R
                    | ash::vk::ColorComponentFlags::G
                    | ash::vk::ColorComponentFlags::B
                    | ash::vk::ColorComponentFlags::A,
            );
        let color_blend = ash::vk::PipelineColorBlendStateCreateInfo::default()
            .attachments(std::slice::from_ref(&blend_attachment));

        let mut rendering_create_info = ash::vk::PipelineRenderingCreateInfo::default()
            .color_attachment_formats(std::slice::from_ref(&format));
        let graphics_pipeline_info = ash::vk::GraphicsPipelineCreateInfo::default()
            .stages(&shader_stages)
            .vertex_input_state(&vertex_input)
            .input_assembly_state(&input_assembly)
            .viewport_state(&viewport_state)
            .rasterization_state(&rasterizer)
            .multisample_state(&multisample)
            .color_blend_state(&color_blend)
            .layout(pipeline_layout)
            .push_next(&mut rendering_create_info);

        let pipelines = unsafe {
            device.create_graphics_pipelines(
                ash::vk::PipelineCache::null(),
                std::slice::from_ref(&graphics_pipeline_info),
                None,
            )
        }
        .unwrap();

        unsafe {
            device.destroy_shader_module(vert_module, None);
            device.destroy_shader_module(frag_module, None);
        }

        let sampler_info = ash::vk::SamplerCreateInfo::default()
            .mag_filter(ash::vk::Filter::LINEAR)
            .min_filter(ash::vk::Filter::LINEAR)
            .address_mode_u(ash::vk::SamplerAddressMode::CLAMP_TO_EDGE)
            .address_mode_v(ash::vk::SamplerAddressMode::CLAMP_TO_EDGE)
            .address_mode_w(ash::vk::SamplerAddressMode::CLAMP_TO_EDGE)
            .anisotropy_enable(false)
            .max_lod(1.0);
        let sampler = unsafe { device.create_sampler(&sampler_info, None).unwrap() };

        let pool_size = ash::vk::DescriptorPoolSize::default()
            .ty(ash::vk::DescriptorType::COMBINED_IMAGE_SAMPLER)
            .descriptor_count(1);
        let pool_sizes = [pool_size];
        let descriptor_pool = unsafe {
            device
                .create_descriptor_pool(
                    &ash::vk::DescriptorPoolCreateInfo::default()
                        .max_sets(1)
                        .pool_sizes(&pool_sizes),
                    None,
                )
                .unwrap()
        };

        let descriptor_sets = unsafe {
            device
                .allocate_descriptor_sets(
                    &ash::vk::DescriptorSetAllocateInfo::default()
                        .descriptor_pool(descriptor_pool)
                        .set_layouts(&[descriptor_set_layout]),
                )
                .unwrap()
        };

        let ui_view_info = ash::vk::ImageViewCreateInfo::default()
            .image(ui_image)
            .view_type(ash::vk::ImageViewType::TYPE_2D)
            .format(format)
            .components(ash::vk::ComponentMapping::default())
            .subresource_range(ash::vk::ImageSubresourceRange {
                aspect_mask: ash::vk::ImageAspectFlags::COLOR,
                base_mip_level: 0,
                level_count: 1,
                base_array_layer: 0,
                layer_count: 1,
            });
        let ui_view = unsafe { device.create_image_view(&ui_view_info, None).unwrap() };

        let image_info = ash::vk::DescriptorImageInfo::default()
            .image_layout(ash::vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL)
            .image_view(ui_view)
            .sampler(sampler);
        let image_infos = [image_info];
        let write = ash::vk::WriteDescriptorSet::default()
            .dst_set(descriptor_sets[0])
            .dst_binding(0)
            .descriptor_type(ash::vk::DescriptorType::COMBINED_IMAGE_SAMPLER)
            .image_info(&image_infos);
        unsafe { device.update_descriptor_sets(std::slice::from_ref(&write), &[]) };

        let _ = ui_view;

        (
            swapchain_image_views,
            pipelines[0],
            pipeline_layout,
            descriptor_set_layout,
            descriptor_pool,
            descriptor_sets,
            sampler,
        )
    }

    fn create_shader_module(device: &ash::Device, spirv: &[u8]) -> ash::vk::ShaderModule {
        let spirv = spirv.to_vec();
        if spirv.is_empty() {
            panic!("Empty SPIR-V shader data");
        }
        let code_u32: &[u32] = unsafe {
            std::slice::from_raw_parts(
                spirv.as_ptr().cast(),
                spirv.len() / std::mem::size_of::<u32>(),
            )
        };
        let create_info = ash::vk::ShaderModuleCreateInfo::default().code(code_u32);
        unsafe {
            device.create_shader_module(&create_info, None).expect("Failed to create shader module")
        }
    }

    fn find_memory_type(
        instance: &ash::Instance,
        physical_device: ash::vk::PhysicalDevice,
        type_filter: u32,
        properties: ash::vk::MemoryPropertyFlags,
    ) -> u32 {
        let mem_props = unsafe { instance.get_physical_device_memory_properties(physical_device) };
        for i in 0..mem_props.memory_type_count {
            if (type_filter & (1 << i)) != 0
                && mem_props.memory_types[i as usize].property_flags & properties == properties
            {
                return i;
            }
        }
        panic!("Failed to find suitable memory type");
    }

    fn create_instance(
        entry: &ash::Entry,
        window: &winit::window::Window,
    ) -> (
        ash::Instance,
        Option<ash::ext::debug_utils::Instance>,
        Option<ash::vk::DebugUtilsMessengerEXT>,
    ) {
        let display_handle = window.display_handle().unwrap();

        let debug = true;
        let window_extensions = ash_window::enumerate_required_extensions(display_handle.as_raw())
            .unwrap()
            .iter()
            .copied();
        let extensions = [
            ash::khr::get_surface_capabilities2::NAME.as_ptr(),
            ash::ext::surface_maintenance1::NAME.as_ptr(),
        ]
        .into_iter()
        .chain(window_extensions)
        .chain(
            std::iter::once(ash::ext::debug_utils::NAME.as_ptr())
                .take(debug.then(|| 1).unwrap_or(0)),
        )
        .collect::<Vec<_>>();

        let debug_create_info = ash::vk::DebugUtilsMessengerCreateInfoEXT::default()
            .message_severity(
                ash::vk::DebugUtilsMessageSeverityFlagsEXT::WARNING
                    | ash::vk::DebugUtilsMessageSeverityFlagsEXT::ERROR
                    | ash::vk::DebugUtilsMessageSeverityFlagsEXT::VERBOSE
                    | ash::vk::DebugUtilsMessageSeverityFlagsEXT::INFO,
            )
            .message_type(
                ash::vk::DebugUtilsMessageTypeFlagsEXT::GENERAL
                    | ash::vk::DebugUtilsMessageTypeFlagsEXT::VALIDATION
                    | ash::vk::DebugUtilsMessageTypeFlagsEXT::PERFORMANCE,
            )
            .pfn_user_callback(Some(vulkan_debug_callback));

        let application_info = ash::vk::ApplicationInfo::default()
            .application_name(c"Basic")
            .application_version(ash::vk::make_api_version(0, 1, 0, 0))
            .engine_name(c"Vizia")
            .engine_version(ash::vk::make_api_version(0, 1, 0, 0))
            .api_version(ash::vk::API_VERSION_1_3);

        let enabled_layer_names = [c"VK_LAYER_KHRONOS_validation".as_ptr()];
        let instance_create_info = ash::vk::InstanceCreateInfo::default()
            .enabled_extension_names(&extensions)
            .enabled_layer_names(&enabled_layer_names)
            .application_info(&application_info);

        let instance = unsafe { entry.create_instance(&instance_create_info, None).unwrap() };

        let (debug_utils_loader, debug_messenger) = debug
            .then(|| {
                let debug_utils_loader = ash::ext::debug_utils::Instance::new(&entry, &instance);
                let debug_messenger = unsafe {
                    debug_utils_loader
                        .create_debug_utils_messenger(&debug_create_info, None)
                        .unwrap()
                };
                (debug_utils_loader, debug_messenger)
            })
            .unzip();

        (instance, debug_utils_loader, debug_messenger)
    }

    fn pick_physical_device(
        instance: &ash::Instance,
        surface_loader: &ash::khr::surface::Instance,
        surface: ash::vk::SurfaceKHR,
    ) -> (ash::vk::PhysicalDevice, u32, u32) {
        let devices = unsafe { instance.enumerate_physical_devices().unwrap() };
        for device in devices {
            let queue_families =
                unsafe { instance.get_physical_device_queue_family_properties(device) };
            let mut graphics = None;
            let mut present = None;
            for (i, family) in queue_families.iter().enumerate() {
                if family.queue_flags.contains(ash::vk::QueueFlags::GRAPHICS) {
                    graphics = Some(i as u32);
                }
                let supports = unsafe {
                    surface_loader.get_physical_device_surface_support(device, i as u32, surface)
                }
                .unwrap();
                if supports {
                    present = Some(i as u32);
                }
            }
            if let (Some(g), Some(p)) = (graphics, present) {
                return (device, g, p);
            }
        }
        panic!("No suitable physical device");
    }

    fn create_logical_device(
        instance: &ash::Instance,
        physical_device: ash::vk::PhysicalDevice,
        graphics_family: u32,
        present_family: u32,
    ) -> (ash::Device, ash::vk::Queue, ash::vk::Queue) {
        let extensions =
            [ash::khr::swapchain::NAME.as_ptr(), ash::ext::extended_dynamic_state::NAME.as_ptr()];

        let graphics_priorities = [1.0f32];
        let present_priorities = [1.0f32];

        let mut queue_create_infos = vec![
            ash::vk::DeviceQueueCreateInfo::default()
                .queue_family_index(graphics_family)
                .queue_priorities(&graphics_priorities),
        ];

        if graphics_family != present_family {
            queue_create_infos.push(
                ash::vk::DeviceQueueCreateInfo::default()
                    .queue_family_index(present_family)
                    .queue_priorities(&present_priorities),
            );
        }

        let mut dynamic_rendering =
            ash::vk::PhysicalDeviceDynamicRenderingFeatures::default().dynamic_rendering(true);

        let mut sync2 =
            ash::vk::PhysicalDeviceSynchronization2Features::default().synchronization2(true);

        let device = unsafe {
            instance
                .create_device(
                    physical_device,
                    &ash::vk::DeviceCreateInfo::default()
                        .queue_create_infos(&queue_create_infos)
                        .push_next(&mut dynamic_rendering)
                        .push_next(&mut sync2)
                        .enabled_extension_names(&extensions),
                    None,
                )
                .unwrap()
        };

        let graphics_queue = unsafe { device.get_device_queue(graphics_family, 0) };
        let present_queue = unsafe { device.get_device_queue(present_family, 0) };
        (device, graphics_queue, present_queue)
    }

    fn create_swapchain(
        extent: ash::vk::Extent2D,
        surface_loader: &ash::khr::surface::Instance,
        swapchain_loader: &ash::khr::swapchain::Device,
        physical_device: ash::vk::PhysicalDevice,
        surface: ash::vk::SurfaceKHR,
        graphics_family: u32,
        present_family: u32,
    ) -> (ash::vk::SwapchainKHR, Vec<ash::vk::Image>, ash::vk::Format, ash::vk::Extent2D) {
        let caps = unsafe {
            surface_loader
                .get_physical_device_surface_capabilities(physical_device, surface)
                .unwrap()
        };
        let formats = unsafe {
            surface_loader.get_physical_device_surface_formats(physical_device, surface).unwrap()
        };
        let present_modes = unsafe {
            surface_loader
                .get_physical_device_surface_present_modes(physical_device, surface)
                .unwrap()
        };

        let surface_format = formats
            .iter()
            .find(|f| f.format == ash::vk::Format::B8G8R8A8_UNORM)
            .unwrap_or(&formats[0]);

        let present_mode = present_modes
            .iter()
            .find(|&m| *m == ash::vk::PresentModeKHR::MAILBOX)
            .copied()
            .unwrap_or(ash::vk::PresentModeKHR::FIFO);

        let extent =
            if caps.current_extent.width != u32::MAX { caps.current_extent } else { extent };

        let mut image_count = caps.min_image_count + 1;
        if caps.max_image_count > 0 && image_count > caps.max_image_count {
            image_count = caps.max_image_count;
        }

        let queue_families = if graphics_family != present_family {
            vec![graphics_family, present_family]
        } else {
            vec![graphics_family]
        };

        let sharing_mode = if queue_families.len() > 1 {
            ash::vk::SharingMode::CONCURRENT
        } else {
            ash::vk::SharingMode::EXCLUSIVE
        };

        let create_info = ash::vk::SwapchainCreateInfoKHR::default()
            .surface(surface)
            .min_image_count(image_count)
            .image_format(surface_format.format)
            .image_color_space(surface_format.color_space)
            .image_extent(extent)
            .image_array_layers(1)
            .image_usage(
                ash::vk::ImageUsageFlags::COLOR_ATTACHMENT | ash::vk::ImageUsageFlags::TRANSFER_DST,
            )
            .image_sharing_mode(sharing_mode)
            .queue_family_indices(&queue_families)
            .pre_transform(caps.current_transform)
            .composite_alpha(ash::vk::CompositeAlphaFlagsKHR::OPAQUE)
            .present_mode(present_mode)
            .clipped(true);

        let swapchain = unsafe { swapchain_loader.create_swapchain(&create_info, None).unwrap() };
        let images = unsafe { swapchain_loader.get_swapchain_images(swapchain).unwrap() };
        (swapchain, images, surface_format.format, extent)
    }

    fn create_command_pool(device: &ash::Device, queue_family: u32) -> ash::vk::CommandPool {
        let create_info = ash::vk::CommandPoolCreateInfo::default()
            .flags(ash::vk::CommandPoolCreateFlags::RESET_COMMAND_BUFFER)
            .queue_family_index(queue_family);
        unsafe { device.create_command_pool(&create_info, None).unwrap() }
    }

    fn allocate_command_buffers(
        device: &ash::Device,
        command_pool: ash::vk::CommandPool,
        count: u32,
    ) -> Vec<ash::vk::CommandBuffer> {
        let allocate_info = ash::vk::CommandBufferAllocateInfo::default()
            .command_pool(command_pool)
            .level(ash::vk::CommandBufferLevel::PRIMARY)
            .command_buffer_count(count);
        unsafe { device.allocate_command_buffers(&allocate_info).unwrap() }
    }

    fn record_command_buffer(
        &self,
        cmd: ash::vk::CommandBuffer,
        swapchain_image: ash::vk::Image,
        swapchain_image_view: ash::vk::ImageView,
    ) {
        unsafe {
            self.device
                .begin_command_buffer(cmd, &ash::vk::CommandBufferBeginInfo::default())
                .unwrap();

            let ui_barrier = ash::vk::ImageMemoryBarrier2::default()
                .src_access_mask(ash::vk::AccessFlags2::COLOR_ATTACHMENT_WRITE)
                .dst_access_mask(ash::vk::AccessFlags2::SHADER_READ)
                .src_stage_mask(ash::vk::PipelineStageFlags2::COLOR_ATTACHMENT_OUTPUT)
                .dst_stage_mask(ash::vk::PipelineStageFlags2::ALL_GRAPHICS)
                .old_layout(
                    (self.current_frame == 0)
                        .then(|| ash::vk::ImageLayout::UNDEFINED)
                        .unwrap_or(ash::vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL),
                )
                .new_layout(ash::vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL)
                .image(self.ui_image)
                .subresource_range(ash::vk::ImageSubresourceRange {
                    aspect_mask: ash::vk::ImageAspectFlags::COLOR,
                    base_mip_level: 0,
                    level_count: 1,
                    base_array_layer: 0,
                    layer_count: 1,
                });
            let swapchain_barrier = ash::vk::ImageMemoryBarrier2::default()
                .dst_access_mask(ash::vk::AccessFlags2::COLOR_ATTACHMENT_WRITE)
                .dst_stage_mask(ash::vk::PipelineStageFlags2::ALL_GRAPHICS)
                .old_layout(ash::vk::ImageLayout::UNDEFINED)
                .new_layout(ash::vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL)
                .image(swapchain_image)
                .subresource_range(ash::vk::ImageSubresourceRange {
                    aspect_mask: ash::vk::ImageAspectFlags::COLOR,
                    base_mip_level: 0,
                    level_count: 1,
                    base_array_layer: 0,
                    layer_count: 1,
                });

            let barriers = &[swapchain_barrier, ui_barrier];
            let dependency_info =
                ash::vk::DependencyInfo::default().image_memory_barriers(barriers);
            self.device.cmd_pipeline_barrier2(cmd, &dependency_info);

            let color_attachment = ash::vk::RenderingAttachmentInfo::default()
                .image_view(swapchain_image_view)
                .image_layout(ash::vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL)
                .load_op(ash::vk::AttachmentLoadOp::CLEAR)
                .store_op(ash::vk::AttachmentStoreOp::STORE)
                .clear_value(ash::vk::ClearValue {
                    color: ash::vk::ClearColorValue { float32: [0.1, 0.1, 0.15, 1.0] },
                });

            let rendering_info = ash::vk::RenderingInfo::default()
                .render_area(ash::vk::Rect2D::default().extent(self.swapchain_extent))
                .layer_count(1)
                .color_attachments(std::slice::from_ref(&color_attachment));

            self.device.cmd_begin_rendering(cmd, &rendering_info);

            self.device.cmd_bind_pipeline(cmd, ash::vk::PipelineBindPoint::GRAPHICS, self.pipeline);
            self.device.cmd_bind_descriptor_sets(
                cmd,
                ash::vk::PipelineBindPoint::GRAPHICS,
                self.pipeline_layout,
                0,
                &[self.descriptor_sets[0]],
                &[],
            );

            self.device.cmd_draw(cmd, 3, 1, 0, 0);

            self.device.cmd_end_rendering(cmd);

            let ui_barrier = ash::vk::ImageMemoryBarrier2::default()
                .src_access_mask(ash::vk::AccessFlags2::SHADER_READ)
                .dst_access_mask(ash::vk::AccessFlags2::COLOR_ATTACHMENT_WRITE)
                .src_stage_mask(ash::vk::PipelineStageFlags2::ALL_GRAPHICS)
                .dst_stage_mask(ash::vk::PipelineStageFlags2::COLOR_ATTACHMENT_OUTPUT)
                .old_layout(ash::vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL)
                .new_layout(ash::vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL)
                .image(self.ui_image)
                .subresource_range(ash::vk::ImageSubresourceRange {
                    aspect_mask: ash::vk::ImageAspectFlags::COLOR,
                    base_mip_level: 0,
                    level_count: 1,
                    base_array_layer: 0,
                    layer_count: 1,
                });
            let swapchain_barrier = ash::vk::ImageMemoryBarrier2::default()
                .src_access_mask(ash::vk::AccessFlags2::COLOR_ATTACHMENT_WRITE)
                .src_stage_mask(ash::vk::PipelineStageFlags2::COLOR_ATTACHMENT_OUTPUT)
                .src_stage_mask(ash::vk::PipelineStageFlags2::ALL_GRAPHICS)
                .dst_stage_mask(ash::vk::PipelineStageFlags2::BOTTOM_OF_PIPE)
                .old_layout(ash::vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL)
                .new_layout(ash::vk::ImageLayout::PRESENT_SRC_KHR)
                .image(swapchain_image)
                .subresource_range(ash::vk::ImageSubresourceRange {
                    aspect_mask: ash::vk::ImageAspectFlags::COLOR,
                    base_mip_level: 0,
                    level_count: 1,
                    base_array_layer: 0,
                    layer_count: 1,
                });

            let barriers = &[swapchain_barrier, ui_barrier];
            let dependency_info =
                ash::vk::DependencyInfo::default().image_memory_barriers(barriers);
            self.device.cmd_pipeline_barrier2(cmd, &dependency_info);

            self.device.end_command_buffer(cmd).unwrap();
        }
    }

    fn draw_frame(&mut self) {
        self.ui_app.update();
        self.ui_app.render();

        unsafe {
            let frame_idx = self.current_frame % self.command_buffers.len();
            self.device
                .wait_for_fences(&[self.in_flight_fences[frame_idx]], true, u64::MAX)
                .unwrap();

            let (image_index, _) = self
                .swapchain_loader
                .acquire_next_image(
                    self.swapchain,
                    u64::MAX,
                    self.image_available_semaphores[frame_idx],
                    ash::vk::Fence::null(),
                )
                .unwrap_or((0, false));

            self.device.reset_fences(&[self.in_flight_fences[frame_idx]]).unwrap();
            self.record_command_buffer(
                self.command_buffers[frame_idx],
                self.swapchain_images[image_index as usize],
                self.swapchain_image_views[image_index as usize],
            );

            let wait_semaphores = [self.image_available_semaphores[frame_idx]];
            let wait_stages = [ash::vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT];
            let cmd_buffers = [self.command_buffers[frame_idx]];

            let submit_info = ash::vk::SubmitInfo::default()
                .wait_semaphores(&wait_semaphores)
                .wait_dst_stage_mask(&wait_stages)
                .command_buffers(&cmd_buffers);

            self.device
                .queue_submit(self.graphics_queue, &[submit_info], self.in_flight_fences[frame_idx])
                .unwrap();

            let swapchains = [self.swapchain];
            let image_indices = [image_index];
            let present_info = ash::vk::PresentInfoKHR::default()
                .swapchains(&swapchains)
                .image_indices(&image_indices);

            let _ = self.swapchain_loader.queue_present(self.present_queue, &present_info);
        }

        self.current_frame += 1;
    }
}

impl Drop for VulkanExample {
    fn drop(&mut self) {
        unsafe {
            self.device.device_wait_idle().unwrap();

            for s in &self.image_available_semaphores {
                self.device.destroy_semaphore(*s, None);
            }
            for s in &self.render_finished_semaphores {
                self.device.destroy_semaphore(*s, None);
            }
            for f in &self.in_flight_fences {
                self.device.destroy_fence(*f, None);
            }

            self.device.free_command_buffers(self.command_pool, &self.command_buffers);
            self.device.destroy_command_pool(self.command_pool, None);

            for view in &self.swapchain_image_views {
                self.device.destroy_image_view(*view, None);
            }

            self.device.destroy_sampler(self.sampler, None);
            self.device.destroy_descriptor_pool(self.descriptor_pool, None);
            self.device.destroy_descriptor_set_layout(self.descriptor_set_layout, None);
            self.device.destroy_pipeline_layout(self.pipeline_layout, None);
            self.device.destroy_pipeline(self.pipeline, None);
            self.device.destroy_image_view(self.ui_image_view, None);
            self.device.free_memory(self.ui_image_memory, None);
            self.device.destroy_image(self.ui_image, None);
            self.swapchain_loader.destroy_swapchain(self.swapchain, None);
            self.surface_loader.destroy_surface(self.surface, None);
            self.device.destroy_device(None);

            if let Some(instance) = self.debug_instance.take()
                && let Some(messenger) = self.debug_messenger.take()
            {
                instance.destroy_debug_utils_messenger(messenger, None);
            }

            self.instance.destroy_instance(None);
        }
    }
}

struct VulkanExampleRunner {
    app: Option<VulkanExample>,
}

impl VulkanExampleRunner {
    fn new() -> Self {
        Self { app: None }
    }
}

impl ApplicationHandler<()> for VulkanExampleRunner {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.app.is_none() {
            self.app = Some(VulkanExample::new(event_loop));
        }
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        if let Some(app) = self.app.as_mut() {
            match &event {
                WindowEvent::CloseRequested
                | WindowEvent::KeyboardInput {
                    event:
                        KeyEvent {
                            physical_key: PhysicalKey::Code(KeyCode::Escape),
                            state: ElementState::Pressed,
                            ..
                        },
                    ..
                } => {
                    event_loop.exit();
                }
                WindowEvent::CursorMoved { position, .. } => {
                    app.ui_app.handle_event(vizia_core::prelude::WindowEvent::MouseMove(
                        position.x as f32,
                        position.y as f32,
                    ));
                }
                WindowEvent::MouseInput { state, button, .. } => {
                    let button = match button {
                        WinitMouseButton::Left => vizia_core::prelude::MouseButton::Left,
                        WinitMouseButton::Right => vizia_core::prelude::MouseButton::Right,
                        WinitMouseButton::Middle => vizia_core::prelude::MouseButton::Middle,
                        WinitMouseButton::Other(val) => {
                            vizia_core::prelude::MouseButton::Other(*val)
                        }
                        WinitMouseButton::Back => vizia_core::prelude::MouseButton::Back,
                        WinitMouseButton::Forward => vizia_core::prelude::MouseButton::Forward,
                    };
                    let vizia_event = match state {
                        ElementState::Pressed => {
                            vizia_core::prelude::WindowEvent::MouseDown(button)
                        }
                        ElementState::Released => vizia_core::prelude::WindowEvent::MouseUp(button),
                    };
                    app.ui_app.handle_event(vizia_event);
                }
                WindowEvent::MouseWheel { device_id: _, delta, phase: _ } => {
                    let (x, y) = match delta {
                        MouseScrollDelta::LineDelta(x, y) => (*x, *y),
                        MouseScrollDelta::PixelDelta(pos) => {
                            (pos.x as f32 / 20.0, pos.y as f32 / 20.0)
                        }
                    };
                    app.ui_app.handle_event(vizia_core::prelude::WindowEvent::MouseScroll(x, y));
                }
                WindowEvent::CursorEntered { device_id: _ } => {
                    app.ui_app.handle_event(vizia_core::prelude::WindowEvent::MouseEnter);
                }
                WindowEvent::CursorLeft { device_id: _ } => {
                    app.ui_app.handle_event(vizia_core::prelude::WindowEvent::MouseLeave);
                }
                WindowEvent::RedrawRequested => {
                    app.draw_frame();
                }
                _ => {}
            }
        }
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        if let Some(app) = self.app.as_mut() {
            if let Some(window) = &app.window {
                window.request_redraw();
            }
        }
    }
}

fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    log::info!("Starting Vizia Vulkan embedded UI example...");

    let event_loop = EventLoop::new().expect("Failed to create event loop");
    let mut runner = VulkanExampleRunner::new();
    event_loop.run_app(&mut runner).expect("Failed to run event loop");
}
