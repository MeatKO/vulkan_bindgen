use parmack::handle::Handle;

mod ffi;
mod cotangens;
mod pixcell;

mod vulkan;
use vulkan::{
	vk_bindgen::*, device::*,handle::VkHandle, swapchain::*,
	draw::*, framebuffer::*, command_pool::*, command_buffer::*,
	pipeline::*, instance::*, physical_device::*, synchronization::*,
	vertex::*, uniform_buffer::*, descriptor_set::*, descriptor_pool::*,
	texture::*, texture_view::*, index::*, depth_buffer::*,
};

mod loseit;

mod detail_core;
use detail_core::{
	input_processor::*, window::*,
};

use crate::cotangens::mat4x4::Mat4x4;

fn main() 
{

	// let mat = Mat4x4::new_perspective(
	// 	45.0f32.to_radians(), 
	// 	400.0f32 / 400.0f32, 
	// 	0.1f32, 
	// 	10.0f32
	// );

	// println!("mat : {:?}", mat);

	// panic!("");
	unsafe
	{
		let mut vk_handle = VkHandle::new_empty();

		create_instance(&mut vk_handle);

		let mut window = 
			parmack::window::WindowBuilder::new()
			.with_title("windole")
			.with_dimensions(800, 600)
			.build()
			.unwrap();

		vk_handle.window_surface = create_vulkan_surface(&mut window, &mut vk_handle).unwrap();

		let mut input_processor = InputProcessor::new();

		create_physical_device(&mut vk_handle);
		create_logical_device(&mut vk_handle);
		
		create_swapchain(&mut vk_handle);
		create_swapchain_image_views(&mut vk_handle);

		create_command_pool(&mut vk_handle);

		create_texture_image(&mut vk_handle);
		create_texture_image_view(&mut vk_handle);
		create_texture_sampler(&mut vk_handle);

		create_vertex_buffer(&mut vk_handle);
		create_index_buffer(&mut vk_handle);
		create_uniform_buffers(&mut vk_handle);

		create_descriptor_set_layout(&mut vk_handle);
		create_descriptor_pool(&mut vk_handle);
		create_descriptor_sets(&mut vk_handle);

		create_pipeline(&mut vk_handle);
		create_depth_buffer(&mut vk_handle);
		create_framebuffer(&mut vk_handle);

		create_synchronization_structures(&mut vk_handle);

		create_command_buffer(&mut vk_handle);

		let mut last_delta_time_ms : f64;

		// window.confine_pointer(true);
		// window.center_pointer(true);
		// window.show_pointer(false);

		while !input_processor.should_quit() 
		{
			let start_time = std::time::Instant::now();
			
			draw_frame(&mut vk_handle);

			std::thread::sleep(std::time::Duration::from_millis(15));

			let end_time = std::time::Instant::now();
			last_delta_time_ms = end_time.duration_since(start_time).as_secs_f64() * 1000.0f64;
			
			input_processor.process_window_events(last_delta_time_ms as f32, &mut window, &mut vk_handle);
			vk_handle.camera.process_movement(last_delta_time_ms as f32, &vk_handle.input_buffer);
			vk_handle.camera.update_camera_vectors();

			// println!("window size : {:?}", window.get_size());
			// println!("pointer loc : {:?}", window.get_pointer_location());
		}
		
		vkDeviceWaitIdle(vk_handle.logical_device);

		// Cleanup
		println!("Destroying vk objects...");
		
		vk_handle.destroy_vk_resources();
	}
}
