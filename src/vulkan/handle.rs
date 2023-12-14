use crate::cotangens::{
	// vec2::*, 
	vec3::*,
};

use crate::detail_core::camera::camera::Camera;
use crate::vulkan::{
	vk_bindgen::*, 
	swapchain::*, 
	queue::*,
	debugger::*,
};

use crate::detail_core::{
	camera::*,
	input::input_buffer::InputBuffer,
};

use std::ptr::null_mut as nullptr;
use std::vec;

use super::wrappers::vk_command_buffer::CommandBuffer;
use super::wrappers::vk_command_pool::CommandPool;

use decs::component_derive::component;
use decs::component::Component;

#[component]
pub struct VkHandle<'a>
{
	pub camera: Camera,
	pub input_buffer: InputBuffer,
	pub mouse_input_buffer: InputBuffer,

	pub instance: VkInstance,
	pub physical_device: VkPhysicalDevice,
	pub logical_device: VkDevice,
	pub available_extensions: Vec<VkExtensionProperties>,
	pub window_surface: VkSurfaceKHR,
	pub window_image_format: VkFormat,
	pub surface_format: VkSurfaceFormatKHR,
	pub present_mode: VkPresentModeKHR,
	pub swapchain_extent: VkExtent2D,
	pub swapchain_framebuffers: Vec<VkFramebuffer>,
	pub descriptor_set_layout: VkDescriptorSetLayout,

	pub render_pass: VkRenderPass,
	pub graphics_pipeline: VkPipeline,
	pub pipeline_layout: VkPipelineLayout,

	pub render_pass_hud: VkRenderPass,
	pub graphics_pipeline_hud: VkPipeline,
	pub pipeline_layout_hud: VkPipelineLayout,

	pub render_pass_wireframe: VkRenderPass,
	pub graphics_pipeline_wireframe: VkPipeline,
	pub pipeline_layout_wireframe: VkPipelineLayout,

	pub queue_handle: QueueHandle,

	pub queue_family_indices: Vec<u32>,

	pub swapchain: VkSwapchainKHR,
	pub swapchain_image_views_vec: Vec<VkImageView>,
	pub swapchain_support_details: SwapchainSupportDetails,

	pub graphics_queue: VkQueue,
	pub presentation_queue: VkQueue,

	pub command_pool: Option<CommandPool>,
	// pub command_pool: CommandPool<'a>,
	pub command_buffer_vec: Vec<CommandBuffer<'a>>,
	pub command_buffer_hud_vec: Vec<CommandBuffer<'a>>,
	pub command_buffer_wireframe_vec: Vec<CommandBuffer<'a>>,

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

	// pub vertices: Vec<Vertex>,
	// pub vertex_buffer: VkBuffer,
	// pub vertex_buffer_memory: VkDeviceMemory,

	// pub indices: Vec<u32>,
	// pub index_buffer: VkBuffer,
	// pub index_buffer_memory: VkDeviceMemory,

	// pub uniform_buffers: Vec<VkBuffer>,
	// pub uniform_buffers_memory: Vec<VkDeviceMemory>,
	// pub uniform_buffers_mapped: Vec<*mut UniformBufferObject>,

	// pub descriptor_pool: VkDescriptorPool,
	// pub descriptor_sets: Vec<VkDescriptorSet>,

	pub start_time: std::time::Instant,

	// pub texture_image: VkImage,
	// pub texture_image_memory: VkDeviceMemory,
	// pub texture_image_view: VkImageView,
	// pub texture_sampler: VkSampler,

	pub depth_image: VkImage,
	pub depth_image_memory: VkDeviceMemory,
	pub depth_image_view: VkImageView,
}

impl<'a> VkHandle<'a>
{
	pub fn new_empty() -> VkHandle<'a>
	{
		return  VkHandle {
			camera: Camera::new(
				Vec3{ x: 0.0f32, y: 0.0f32, z: 0.0f32 },
				Vec3{ x: 0.0f32, y: 1.0f32, z: 0.0f32 },
				// -90.0f32,
				0.0f32,
				0.0f32
			),
			input_buffer: InputBuffer::new(250.0f32),
			mouse_input_buffer: InputBuffer::new(250.0f32),
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
			swapchain_extent: VkExtent2D { width: 0, height: 0 },
			swapchain_framebuffers: vec![],
			queue_handle: QueueHandle::default(),
			descriptor_set_layout: nullptr(),

			render_pass: nullptr(),
			graphics_pipeline: nullptr(),
			pipeline_layout: nullptr(),

			render_pass_hud: nullptr(),
			graphics_pipeline_hud: nullptr(),
			pipeline_layout_hud: nullptr(),

			render_pass_wireframe: nullptr(),
			graphics_pipeline_wireframe: nullptr(),
			pipeline_layout_wireframe: nullptr(),

			queue_family_indices: vec![],
			graphics_queue: nullptr(),
			presentation_queue: nullptr(),
			// command_pool: unsafe { std::mem::MaybeUninit::zeroed },
			command_pool: None,
			// command_pool_hud: nullptr(),

			command_buffer_vec: vec![],
			command_buffer_hud_vec: vec![],
			command_buffer_wireframe_vec: vec![],

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
			// enable_validation_layers: false,
			vertex_shader_module: nullptr(),
			fragment_shader_module: nullptr(),
			debug_messenger: nullptr(),
			// vertices: vec![
			// 	Vertex{pos: Vec3{x: -0.5f32, y: -0.5f32, z: -0.5f32}, normal: Vec3{x: 0.0f32, y: 1.0f32, z: 0.0f32}, uv: Vec2 { x: 0.0f32, y: 0.0f32 }},
			// 	Vertex{pos: Vec3{x: 0.5f32, y: -0.5f32, z: -0.5f32}, normal: Vec3{x: 1.0f32, y: 0.0f32, z: 0.0f32}, uv: Vec2 { x: 1.0f32, y: 0.0f32 }},
			// 	Vertex{pos: Vec3{x: 0.5f32, y: 0.5f32, z: -0.5f32}, normal: Vec3{x: 0.0f32, y: 0.0f32, z: 1.0f32}, uv: Vec2 { x: 1.0f32, y: 1.0f32 }},
			// 	Vertex{pos: Vec3{x: -0.5f32, y: 0.5f32, z: -0.5f32}, normal: Vec3{x: 1.0f32, y: 1.0f32, z: 1.0f32}, uv: Vec2 { x: 0.0f32, y: 1.0f32 }},
				
			// 	Vertex{pos: Vec3{x: -0.5f32, y: -0.5f32, z: 0.5f32}, normal: Vec3{x: 0.0f32, y: 1.0f32, z: 0.0f32}, uv: Vec2 { x: 0.0f32, y: 0.0f32 }},
			// 	Vertex{pos: Vec3{x: 0.5f32, y: -0.5f32, z: 0.5f32}, normal: Vec3{x: 1.0f32, y: 0.0f32, z: 0.0f32}, uv: Vec2 { x: 1.0f32, y: 0.0f32 }},
			// 	Vertex{pos: Vec3{x: 0.5f32, y: 0.5f32, z: 0.5f32}, normal: Vec3{x: 0.0f32, y: 0.0f32, z: 1.0f32}, uv: Vec2 { x: 1.0f32, y: 1.0f32 }},
			// 	Vertex{pos: Vec3{x: -0.5f32, y: 0.5f32, z: 0.5f32}, normal: Vec3{x: 1.0f32, y: 1.0f32, z: 1.0f32}, uv: Vec2 { x: 0.0f32, y: 1.0f32 }},
			
			// ],
			// indices: vec![
			// 	0, 1, 2, 2, 3, 0,
			// 	4, 5, 6, 6, 7, 4,
			// 	0, 4, 5, 0, 5, 1,
			// 	2, 6, 7, 2, 7, 3, 
			// 	4, 7, 0, 0, 7, 3,
			// 	5, 1, 6, 6, 1, 2,
			// ],
			// vertex_buffer: nullptr(),
			// vertex_buffer_memory: nullptr(),
			// index_buffer: nullptr(),
			// index_buffer_memory: nullptr(),

			// uniform_buffers: vec![],
			// uniform_buffers_memory: vec![],
			// uniform_buffers_mapped: vec![],
			// descriptor_pool: nullptr(),
			// descriptor_sets: vec![],
			
			start_time: std::time::Instant::now(),

			// texture_image: nullptr(),
			// texture_image_memory: nullptr(),
			// texture_image_view: nullptr(),
			// texture_sampler: nullptr(),

			depth_image: nullptr(),
			depth_image_memory: nullptr(),
			depth_image_view: nullptr(),
		};
	}

	pub unsafe fn destroy_vk_resources(&self)
	{
		// vkDestroyBuffer(self.logical_device, self.vertex_buffer, nullptr());
		// vkFreeMemory(self.logical_device, self.vertex_buffer_memory, nullptr());

		// vkDestroyBuffer(self.logical_device, self.index_buffer, nullptr());
		// vkFreeMemory(self.logical_device, self.index_buffer_memory, nullptr());

		cleanup_swapchain(self);

		// vkDestroySampler(self.logical_device, self.texture_sampler, nullptr());
		// vkDestroyImageView(self.logical_device, self.texture_image_view, nullptr());
		
		// vkDestroyImage(self.logical_device, self.texture_image, nullptr());
    	// vkFreeMemory(self.logical_device, self.texture_image_memory, nullptr());

		// for i in 0..self.frames_in_flight
		// {
		// 	vkDestroyBuffer(self.logical_device, self.uniform_buffers[i], nullptr());
		// 	vkFreeMemory(self.logical_device, self.uniform_buffers_memory[i], nullptr());
		// }

		// vkDestroyDescriptorPool(self.logical_device, self.descriptor_pool, nullptr());
		vkDestroyDescriptorSetLayout(self.logical_device, self.descriptor_set_layout, nullptr());

		for i in 0..self.frames_in_flight
		{
			vkDestroyFence(self.logical_device, self.in_flight_fence_vec[i], nullptr());
			vkDestroySemaphore(self.logical_device, self.rendering_finished_semaphore_vec[i], nullptr());
			vkDestroySemaphore(self.logical_device, self.image_available_semaphore_vec[i], nullptr());
		}
		// vkDestroyCommandPool(self.logical_device, self.command_pool, nullptr());
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