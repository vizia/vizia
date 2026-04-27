use std::ffi::CStr;

use ash::vk::Handle;

pub struct State {
    skia_context: skia_safe::gpu::DirectContext,
}

impl Clone for State {
    fn clone(&self) -> Self {
        Self { skia_context: self.skia_context.clone() }
    }
}

impl State {
    pub unsafe fn new(
        entry: ash::Entry,
        instance: ash::Instance,
        physical_device: ash::vk::PhysicalDevice,
        device: ash::Device,
        queue: ash::vk::Queue,
        queue_family_index: u32,
        api_version: u32,
    ) -> Result<Self, String> {
        let get_proc = |gpo: skia_safe::gpu::vk::GetProcOf| unsafe {
            match gpo {
                skia_safe::gpu::vk::GetProcOf::Instance(instance, name) => {
                    let ash_instance = ash::vk::Instance::from_raw(instance as _);
                    if let Some(f) = entry.get_instance_proc_addr(ash_instance, name) {
                        Some(f as *const std::ffi::c_void)
                    } else {
                        eprintln!(
                            "Failed to resolve instance fn: {}",
                            CStr::from_ptr(name).to_string_lossy()
                        );
                        None
                    }
                }
                skia_safe::gpu::vk::GetProcOf::Device(device, name) => {
                    let ash_device = ash::vk::Device::from_raw(device as _);

                    if let Some(f) = instance.get_device_proc_addr(ash_device, name) {
                        Some(f as *const std::ffi::c_void)
                    } else {
                        eprintln!(
                            "Failed to resolve device fn: {}",
                            CStr::from_ptr(name).to_string_lossy()
                        );
                        None
                    }
                }
            }
            .unwrap_or(std::ptr::null())
        };

        let mut backend_context = unsafe {
            skia_safe::gpu::vk::BackendContext::new(
                instance.handle().as_raw() as _,
                physical_device.as_raw() as _,
                device.handle().as_raw() as _,
                (queue.as_raw() as _, queue_family_index as usize),
                &get_proc,
            )
        };
        backend_context.set_max_api_version(api_version);

        let skia_context = skia_safe::gpu::direct_contexts::make_vulkan(&backend_context, None)
            .ok_or("Failed to create Skia Vulkan context")?;

        Ok(Self { skia_context })
    }

    pub fn skia_context_mut(&mut self) -> &mut skia_safe::gpu::DirectContext {
        &mut self.skia_context
    }
}
