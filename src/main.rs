mod ffi;
mod cotangens;
mod pixcell;
mod loseit;
mod exedra;

mod vulkan;

use cotangens::mat4x4::Mat4x4;
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
	input::input_processor::InputProcessor, 
	window::create_vulkan_surface,
};

use std::ptr::null_mut as nullptr;

use crate::{cotangens::{vec3::Vec3, mat4x4}, detail_core::model::model::{Model, VulkanModel}};
use parmack::window::event::MouseCode;

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

		create_descriptor_set_layout(&mut vk_handle);

		let mut models: Vec<Model<VulkanModel>> =
			vec![
				// Model::new("./detail/models/valkyrie/valkyrie.obj".into()).process_vk(&vk_handle),
				// Model::new("./detail/models/woag/woag.obj".into()).process_vk(&vk_handle),
				// Model::new("./detail/models/earth_2/earth_2.obj".into()).process_vk(&vk_handle),
				Model::new("./detail/models/brick_wall/brick_wall.obj".into()).process_vk(&vk_handle),
				Model::new("./detail/models/de_inferno/de_inferno.obj".into()).process_vk(&vk_handle),
			];
		
		// create vertex, index, uniform buffers for the mesh
		// create texture buffers for the material
		// for model in models.iter_mut()
		// {
		// 	model.process_meshes(&mut vk_handle);
		// }

		create_pipeline(&mut vk_handle);
		create_depth_buffer(&mut vk_handle);
		create_framebuffer(&mut vk_handle);

		create_synchronization_structures(&mut vk_handle);

		create_command_buffers(&mut vk_handle);

		let mut last_delta_time_ms : f64;
		
		// window.confine_pointer(true);
		// window.center_pointer(true);
		// window.show_pointer(false);

		let mut light_pos = Vec3::new(10.0f32);

		while !input_processor.should_quit() 
		{
			let start_time = std::time::Instant::now();
			// let absolute_current_time_stamp_ms = start_time.duration_since(vk_handle.start_time).as_secs_f32() * 1000.0f32;
			let absolute_current_time_stamp_s = start_time.duration_since(vk_handle.start_time).as_secs_f32();

			if vk_handle.mouse_input_buffer.is_pressed(MouseCode::Left as u8)
			{
				models[0].translation = &vk_handle.camera.get_position() + &(&vk_handle.camera.get_front() * &4.0f32);
			}

			if vk_handle.mouse_input_buffer.is_pressed(MouseCode::Right as u8)
			{
				light_pos = &vk_handle.camera.get_position() + &(&vk_handle.camera.get_front() * &4.0f32);
				// models[0].rotation = Vec3{ x: 0.0f32, y: vk_handle.camera.get_rotation().y - 90.0f32, z: 0.0f32};// + &(&vk_handle.camera.get_front() * &10.0f32);
			}

			models[0].rotation = Vec3{ x: 0.0f32, y: absolute_current_time_stamp_s * 10.0f32, z: 0.0f32};

			models[0].scale = Vec3::new(1.5f32);

			models[1].scale = Vec3::new(0.3f32);

			draw_frame(&mut vk_handle, &mut models, &light_pos.clone());

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

		// for model in &mut models
		// {
		// 	model.destroy(&mut vk_handle);
		// }

		vk_handle.destroy_vk_resources();
	}
}
