mod ffi;
mod cotangens;

mod loseit;
use loseit::{
	window::*, window_events::*,
};

mod vulkan;
use vulkan::{
	vk_bindgen::*, device::*,handle::VkHandle, swapchain::*,
	draw::*, framebuffer::*, command_pool::*, command_buffer::*,
	pipeline::*, instance::*, physical_device::*, synchronization::*,
	vertex::*, uniform_buffer::*, descriptor_set::*, descriptor_pool::*,
};

mod detail_core;
use detail_core::{
	camera::*, input_processor::*,
};

use crate::{vulkan::index::create_index_buffer, cotangens::vec3::Vec3};

fn main() 
{
	unsafe
	{
		let mut vk_handle = VkHandle::new_empty();

		create_instance(&mut vk_handle);

		let window = 
			Window::new()
			.with_title("deta:l")
			.with_dimensions(400, 400)
			.build_vulkan(&mut vk_handle);

		let mut input_processor = InputProcessor::new();

		create_physical_device(&mut vk_handle);
	
		create_logical_device(&mut vk_handle);
		
		create_swapchain(&mut vk_handle);

		create_swapchain_image_views(&mut vk_handle);

		create_command_pool(&mut vk_handle);

		create_vertex_buffer(&mut vk_handle);

		create_index_buffer(&mut vk_handle);

		create_uniform_buffers(&mut vk_handle);

		create_descriptor_set_layout(&mut vk_handle);

		create_descriptor_pool(&mut vk_handle);

		create_descriptor_sets(&mut vk_handle);

		create_pipeline(&mut vk_handle);

		create_framebuffer(&mut vk_handle);

		create_synchronization_structures(&mut vk_handle);

		create_command_buffer(&mut vk_handle);

		let mut last_delta_time_ms = 0.0f32;

		while !input_processor.should_quit() 
		{
			input_processor.process_window_events(last_delta_time_ms, &window, &mut vk_handle);

			vk_handle.camera.process_movement(last_delta_time_ms, &vk_handle.input_buffer);
			vk_handle.camera.update_camera_vectors();

			let start_time = std::time::Instant::now();
			
			draw_frame(&mut vk_handle);
			std::thread::sleep(std::time::Duration::from_millis(16));

			let end_time = std::time::Instant::now();

			last_delta_time_ms = end_time.duration_since(start_time).as_secs_f32() * 1000.0f32;

			// println!("pos : {:?}", vk_handle.camera.position);
			// println!("frame time {}ms", last_delta_time_ms);
		}
		
		vkDeviceWaitIdle(vk_handle.logical_device);

		// Cleanup
		println!("Destroying vk objects...");
		
		vk_handle.destroy_vk_resources();
	}
}
