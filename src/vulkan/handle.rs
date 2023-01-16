use crate::vulkan::vk_bindgen::*;

pub struct VkHandle
{
	pub instance: VkInstance,
	pub physical_device: VkPhysicalDevice,
	pub available_extensions: Vec<VkExtensionProperties>,
	pub window_surface: VkSurfaceKHR,
	pub window_image_format: VkFormat
}