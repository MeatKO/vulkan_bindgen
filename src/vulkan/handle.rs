use crate::calcium::{vec2::*, vec3::*};
use crate::vulkan::vk_bindgen::*;
use crate::vulkan::swapchain::*;
use crate::vulkan::queue::*;
use crate::vulkan::debugger::*;
use crate::vulkan::vertex::*;
use std::ptr::null_mut as nullptr;
use std::vec;

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

	pub debug_messenger: VkDebugUtilsMessengerEXT,

	pub vertices: Vec<Vertex>,
	pub vertex_buffer: VkBuffer,
	pub vertex_buffer_memory: VkDeviceMemory,

	pub indices: Vec<u16>,
	pub index_buffer: VkBuffer,
	pub index_buffer_memory: VkDeviceMemory,
}

impl VkHandle
{
	pub fn new_empty() -> VkHandle
	{
		return  VkHandle {
			instance: nullptr(),
			physical_device: nullptr(),
			logical_device: nullptr(),
			available_extensions: vec![],
			window_surface: nullptr(),
			window_image_format: VkFormat::VK_FORMAT_UNDEFINED,
			surface_format: VkSurfaceFormatKHR { 
				format: VkFormat::VK_FORMAT_UNDEFINED, 
				colorSpace: VkColorSpaceKHR::VK_COLOR_SPACE_SRGB_NONLINEAR_KHR 
			},
			present_mode: VkPresentModeKHR::VK_PRESENT_MODE_FIFO_KHR,
			extent: VkExtent2D { width: 0, height: 0 },
			swapchain_framebuffers: vec![],
			render_pass: nullptr(),
			queue_handle: QueueHandle::default(),
			graphics_pipeline: nullptr(),
			pipeline_layout: nullptr(),
			queue_family_indices: vec![],
			graphics_queue: nullptr(),
			presentation_queue: nullptr(),
			command_pool: nullptr(),
			command_buffer_vec: vec![],
			image_available_semaphore_vec: vec![],
			rendering_finished_semaphore_vec: vec![],
			in_flight_fence_vec: vec![],
			swapchain: nullptr(),
			swapchain_image_views_vec: vec![],
			swapchain_support_details: SwapchainSupportDetails {
				capabilities: VkSurfaceCapabilitiesKHR{ ..Default::default() },
				formats: vec![],
				present_modes: vec![]
			},
			frames_in_flight: 3,
			current_frame: 0,
			needed_device_extensions: vec![],
			layer_names: vec![],
			enable_validation_layers: true,
			vertex_shader_module: nullptr(),
			fragment_shader_module: nullptr(),
			debug_messenger: nullptr(),
			vertices: vec![
				Vertex{pos: Vec2{x: -0.5f32, y: -0.5f32}, color: Vec3{x: 1.0f32, y: 0.0f32, z: 0.0f32}},
				Vertex{pos: Vec2{x: 0.5f32, y: -0.5f32}, color: Vec3{x: 0.0f32, y: 1.0f32, z: 0.0f32}},
				Vertex{pos: Vec2{x: 0.5f32, y: 0.5f32}, color: Vec3{x: 0.0f32, y: 0.0f32, z: 1.0f32}},
				Vertex{pos: Vec2{x: -0.5f32, y: 0.5f32}, color: Vec3{x: 1.0f32, y: 1.0f32, z: 1.0f32}}
			],
			indices: vec![
				0, 1, 2, 2, 3, 0
			],
			vertex_buffer: nullptr(),
			vertex_buffer_memory: nullptr(),
			index_buffer: nullptr(),
			index_buffer_memory: nullptr(),
		};
	}

	pub unsafe fn destroy_vk_resources(&self)
	{

		vkDestroyBuffer(self.logical_device, self.vertex_buffer, nullptr());
		vkFreeMemory(self.logical_device, self.vertex_buffer_memory, nullptr());

		vkDestroyBuffer(self.logical_device, self.index_buffer, nullptr());
		vkFreeMemory(self.logical_device, self.index_buffer_memory, nullptr());

		cleanup_swapchain(self);

		for i in 0..self.frames_in_flight
		{
			vkDestroyFence(self.logical_device, self.in_flight_fence_vec[i], nullptr());
			vkDestroySemaphore(self.logical_device, self.rendering_finished_semaphore_vec[i], nullptr());
			vkDestroySemaphore(self.logical_device, self.image_available_semaphore_vec[i], nullptr());
		}
		vkDestroyCommandPool(self.logical_device, self.command_pool, nullptr());
		// for framebuffer in self.swapchain_framebuffers.iter()
		// {
		// 	vkDestroyFramebuffer(self.logical_device, *framebuffer, nullptr());
		// }
		vkDestroyPipelineLayout(self.logical_device, self.pipeline_layout, nullptr());
		vkDestroyPipeline(self.logical_device, self.graphics_pipeline, nullptr());
		vkDestroyRenderPass(self.logical_device, self.render_pass, nullptr());
		vkDestroyShaderModule(self.logical_device, self.fragment_shader_module, nullptr());
		vkDestroyShaderModule(self.logical_device, self.vertex_shader_module, nullptr());
		// for image_view in self.swapchain_image_views_vec.iter()
		// {
		// 	vkDestroyImageView(self.logical_device, *image_view, nullptr());
		// }
		// vkDestroySwapchainKHR(self.logical_device, self.swapchain, nullptr());
		vkDestroySurfaceKHR(self.instance, self.window_surface, nullptr());
		vkDestroyDevice(self.logical_device, nullptr());
		destroy_debug_utils_messenger_ext(&self.instance, &self.debug_messenger, nullptr());
		vkDestroyInstance(self.instance, nullptr());
	}
}