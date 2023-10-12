mod ffi;
mod cotangens;
mod pixcell;
mod loseit;
mod exedra;

mod vulkan;

use vulkan::{
	vk_bindgen::
		vkDeviceWaitIdle, 
	texture_view::{
		create_texture_image_view, 
		create_texture_sampler
	}, 
	swapchain::{
		create_swapchain, 
		create_swapchain_image_views
	},
	descriptor_set::{
		create_descriptor_set_layout, 
		create_descriptor_sets
	}, 
	device::create_logical_device,
	handle::VkHandle,
	draw::draw_frame, 
	framebuffer::create_framebuffer, 
	command_pool::create_command_pool, 
	command_buffer::create_command_buffers,
	pipeline::create_pipeline, 
	instance::create_instance, 
	physical_device::create_physical_device, 
	synchronization::create_synchronization_structures,
	vertex::create_vertex_buffer, 
	uniform_buffer::create_uniform_buffers,
	descriptor_pool::create_descriptor_pool,
	texture::create_texture_image,
	index::create_index_buffer, 
	depth_buffer::create_depth_buffer,
};

mod detail_core;
use detail_core::{
	input_processor::InputProcessor, 
	window::create_vulkan_surface,
};

use crate::exedra::model::Model;

fn main() 
{
	unsafe
	{
		let mut window = 
			parmack::window::WindowBuilder::new()
			.with_title("windole")
			.with_dimensions(800, 600)
			.build()
			.unwrap();

		let mut input_processor = InputProcessor::new();

		let mut vk_handle = VkHandle::new_empty();

		create_instance(&mut vk_handle);

		vk_handle.window_surface = create_vulkan_surface(&mut window, &mut vk_handle).unwrap();

		create_physical_device(&mut vk_handle);
		create_logical_device(&mut vk_handle);
		
		create_swapchain(&mut vk_handle);
		create_swapchain_image_views(&mut vk_handle);

		create_command_pool(&mut vk_handle);

		let mut models: Vec<Model> =
			vec![
				exedra::model::Model::load("./detail/models/viking_room/viking_room.obj").unwrap(),
				exedra::model::Model::load("./detail/models/viking_room/viking_room.obj").unwrap(),
				exedra::model::Model::load("./detail/models/woag/woag.obj").unwrap(),
			];

		let texture_paths: Vec<String> = 
			vec![
				"./detail/models/viking_room/viking_room.tga".into(),
				"./detail/textures/test.tga".into(),
				"./detail/models/woag/woag.tga".into(),
			];

		create_descriptor_set_layout(&mut vk_handle);

		for (model, texture_path) in models.iter_mut().zip(texture_paths)
		{
			create_texture_image(&mut vk_handle, model, texture_path);
			create_texture_image_view(&mut vk_handle, model);
			create_texture_sampler(&mut vk_handle, model);
	
			(model.vertex_buffer, model.vertex_buffer_memory) =
				create_vertex_buffer(&mut vk_handle, &mut model.vertices)
				.unwrap();
	
			(model.index_buffer, model.index_buffer_memory) =
				create_index_buffer(&mut vk_handle, &mut model.indices)
				.unwrap();
	
			create_uniform_buffers(&mut vk_handle, model);
			
			create_descriptor_pool(&vk_handle, model);
			create_descriptor_sets(&vk_handle, model);
		}

		create_pipeline(&mut vk_handle);
		create_depth_buffer(&mut vk_handle);
		create_framebuffer(&mut vk_handle);

		create_synchronization_structures(&mut vk_handle);

		create_command_buffers(&mut vk_handle);

		let mut last_delta_time_ms : f64;
		
		// window.confine_pointer(true);
		// window.center_pointer(true);
		// window.show_pointer(false);

		while !input_processor.should_quit() 
		{
			let start_time = std::time::Instant::now();
			// let absolute_current_time_stamp_ms = start_time.duration_since(vk_handle.start_time).as_secs_f32() * 1000.0f32;
			
			draw_frame(&mut vk_handle, &mut models);

			// std::thread::sleep(std::time::Duration::from_millis(15));

			let end_time = std::time::Instant::now();
			last_delta_time_ms = end_time.duration_since(start_time).as_secs_f64() * 1000.0f64;
			
			input_processor.process_window_events(last_delta_time_ms as f32, &mut window, &mut vk_handle);

			vk_handle.camera.process_movement(last_delta_time_ms as f32, &vk_handle.input_buffer);
			vk_handle.camera.update_camera_vectors();

			// println!("delta time : {:?}ms", last_delta_time_ms);

			// println!("window size : {:?}", window.get_size());
			// println!("pointer loc : {:?}", window.get_pointer_location());
			// panic!()
		}

		vkDeviceWaitIdle(vk_handle.logical_device);

		// Cleanup
		println!("Destroying vk objects...");

		for model in &mut models
		{
			model.destroy(&mut vk_handle);
		}

		vk_handle.destroy_vk_resources();
	}
}
