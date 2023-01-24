#![allow(non_upper_case_globals)]

use std::ptr::null_mut as nullptr;

mod vulkan;
use vulkan::{
	vk_bindgen::*, device::*,handle::VkHandle, queue::*, swapchain::*,
	draw::*, framebuffer::*, command_pool::*, command_buffer::*,
	pipeline::*, instance::*, physical_device::*,
};

mod loseit;
use loseit::window::*;

mod ffi;
use ffi::strings::*;

fn main() 
{
	unsafe
	{
		let mut vk_handle = VkHandle {
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
			debug_messenger: nullptr()
		};

		create_instance(&mut vk_handle);

		let _window = 
			Window::new()
			.with_title("deta:l")
			.with_dimensions(400, 400)
			.build_vulkan(&mut vk_handle);

		create_physical_device(&mut vk_handle);
	
		create_logical_device(&mut vk_handle);
		
		create_swapchain(&mut vk_handle);

		create_swapchain_image_views(&mut vk_handle);

		create_pipeline(&mut vk_handle);

		create_framebuffer(&mut vk_handle);

		create_command_pool(&mut vk_handle);

		create_command_buffer(&mut vk_handle);

		// creating semaphores & fence
		let semaphore_create_info = VkSemaphoreCreateInfo{
			sType: VkStructureType::VK_STRUCTURE_TYPE_SEMAPHORE_CREATE_INFO,
			flags: 0,	
			pNext: nullptr(),
		};

		let fence_create_info = VkFenceCreateInfo{
			sType: VkStructureType::VK_STRUCTURE_TYPE_FENCE_CREATE_INFO,
			flags: VkFenceCreateFlagBits::VK_FENCE_CREATE_SIGNALED_BIT as u32,	
			pNext: nullptr(),
		};

		vk_handle.image_available_semaphore_vec.resize(vk_handle.frames_in_flight, nullptr());
		vk_handle.rendering_finished_semaphore_vec.resize(vk_handle.frames_in_flight, nullptr());
		vk_handle.in_flight_fence_vec.resize(vk_handle.frames_in_flight, nullptr());

		for i in 0..vk_handle.frames_in_flight
		{
			match vkCreateSemaphore(vk_handle.logical_device, &semaphore_create_info, nullptr(), &mut vk_handle.image_available_semaphore_vec[i])
			{
				VkResult::VK_SUCCESS => { println!("✔️ vkCreateSemaphore()"); }
				err => { panic!("✗ vkCreateSemaphore() failed with code {:?}.", err); }
			}	
			match vkCreateSemaphore(vk_handle.logical_device, &semaphore_create_info, nullptr(), &mut vk_handle.rendering_finished_semaphore_vec[i])
			{
				VkResult::VK_SUCCESS => { println!("✔️ vkCreateSemaphore()"); }
				err => { panic!("✗ vkCreateSemaphore() failed with code {:?}.", err); }
			}
			match vkCreateFence(vk_handle.logical_device, &fence_create_info, nullptr(), &mut vk_handle.in_flight_fence_vec[i])
			{
				VkResult::VK_SUCCESS => { println!("✔️ vkCreateFence()"); }
				err => { panic!("✗ vkCreateFence() failed with code {:?}.", err); }
			}
		}

		loop 
		{
			draw_frame(&mut vk_handle);
		}
		
		vkDeviceWaitIdle(vk_handle.logical_device);

		std::thread::sleep(std::time::Duration::from_secs(2));

		// Cleanup
		println!("Destroying vk objects...");
		
		vk_handle.destroy_vk_resources();
	}
}
