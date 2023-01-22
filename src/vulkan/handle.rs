use crate::vulkan::vk_bindgen::*;
use crate::vulkan::swapchain::*;
use crate::vulkan::queue::*;

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
	pub pipeline_layout: VkPipelineLayout,

	pub queue_handle: QueueHandle,

	pub queue_family_indices: Vec<u32>,

	pub swapchain: VkSwapchainKHR,
	pub swapchain_image_views_vec: Vec<VkImageView>,
	pub swapchain_support_details: SwapchainSupportDetails,

	pub graphics_queue: VkQueue,
	pub presentation_queue: VkQueue,

	pub command_pool: VkCommandPool,

	pub command_buffer_vec: Vec<VkCommandBuffer>,
	pub image_available_semaphore_vec: Vec<VkSemaphore>,
	pub rendering_finished_semaphore_vec: Vec<VkSemaphore>,
	pub in_flight_fence_vec: Vec<VkFence>,

	pub frames_in_flight: usize,

	pub current_frame: usize,

	pub enable_validation_layers: bool,

	pub needed_device_extensions: Vec<&'static str>,
	pub layer_names: Vec<*const i8>,

	pub fragment_shader_module: VkShaderModule,
	pub vertex_shader_module: VkShaderModule,
}