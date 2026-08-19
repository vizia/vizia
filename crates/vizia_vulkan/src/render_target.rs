#[derive(Debug, Clone, Copy)]
pub struct RenderTarget {
    pub image: ash::vk::Image,
    pub extent: ash::vk::Extent2D,
    pub format: ash::vk::Format,
    pub sample_count: u32,
}
impl RenderTarget {
    pub fn new(
        image: ash::vk::Image,
        extent: ash::vk::Extent2D,
        format: ash::vk::Format,
        sample_count: u32,
    ) -> Self {
        Self { image, extent, format, sample_count }
    }
}
