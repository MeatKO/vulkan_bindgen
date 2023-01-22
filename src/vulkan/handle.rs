use crate::vulkan::vk_bindgen::*;

pub struct VkHandle
{
	pub instance: VkInstance,
	pub physical_device: VkPhysicalDevice,
	pub logical_device: VkDevice,
	pub available_extensions: Vec<VkExtensionProperties>,
	pub window_surface: VkSurfaceKHR,
	pub window_image_format: VkFormat,
	pub surface_format: VkSurfaceFormatKHR,
	pub present_mode: VkPresentModeKHR,
	pub extent: VkExtent2D,
	pub swapchain_framebuffers: Vec<VkFramebuffer>,
	pub render_pass: VkRenderPass,
	pub graphics_pipeline: VkPipeline,
	pub swapchain: VkSwapchainKHR,

	pub graphics_queue: VkQueue,
	pub presentation_queue: VkQueue,

	pub command_buffer: VkCommandBuffer,

	pub image_available_semaphore: VkSemaphore,
	pub rendering_finished_semaphore: VkSemaphore,
	pub in_flight_fence: VkFence
}