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
	input_processor::InputProcessor, 
	window::create_vulkan_surface,
};

use std::ptr::null_mut as nullptr;

use crate::{exedra::{model::Model, material::VulkanMaterialData, mesh::VulkanMeshData}, cotangens::{vec3::Vec3, mat4x4}};

fn main() 
{
	let new_identity = Mat4x4::new_identity(16.0f32);

	println!("identity mat : {:?}", new_identity);
	println!("scaled mat : {:?}", new_identity.scale(Vec3::new(0.5f32)));
	println!("translated mat : {:?}", new_identity.translate(Vec3::new(32.0f32)));
	println!("rotated_x mat : {:?}", new_identity.rotate_x(3.0f32));
	println!("rotated_y mat : {:?}", new_identity.rotate_y(3.0f32));
	println!("rotated_z mat : {:?}", new_identity.rotate_z(3.0f32));

	// let new_scale = 
	// 	{
	// 		let identity = Mat4x4::new_identity(1.0f32);

	// 		let new_zero = Mat4x4
	// 	};

	// panic!();

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
				// exedra::model::Model::load("./detail/models/viking_room/viking_room.obj").unwrap(),
				// exedra::model::Model::load("./detail/models/viking_room/viking_room.obj").unwrap(),
				// exedra::model::Model::load("./detail/models/woag/woag.obj").unwrap(),
				exedra::model::Model::load("./detail/models/valkyrie/valkyrie.obj").unwrap(),
				exedra::model::Model::load("./detail/models/de_inferno/de_inferno.obj").unwrap(),
			];

		// println!("printing model {} with {} meshes", models[0].name, models[0].meshes.len());

		// for mesh in &models[0].meshes
		// {
		// 	println!("printing mesh {}", mesh.name);
		// 	println!("Vertex Len : {}", mesh.vertices.len());
		// 	println!("Index Len : {}", mesh.indices.len());
		// 	println!("Index Count : {}", mesh.index_count);
		// 	println!("Using Materal : {}", mesh.material.name);
		// 	println!("Material with map : {}", mesh.material.diffuse_map_rel_path);

		// }

		// panic!("aaadafjnhbgyhujkmnbvgyhujkmn");

		// let texture_paths: Vec<String> = 
		// 	vec![
		// 		"./detail/models/viking_room/viking_room.tga".into(),
		// 		"./detail/textures/test.tga".into(),
		// 		"./detail/models/woag/woag.tga".into(),
		// 	];

		create_descriptor_set_layout(&mut vk_handle);

		// create vertex, index, uniform buffers for the mesh
		// create texture buffers for the material
		for model in models.iter_mut()
		{
			for mesh in model.meshes.iter_mut()
			{
				let (texture_image, texture_image_memory) = create_texture_image(&vk_handle, mesh.material.diffuse_map_rel_path.clone());
				let texture_image_view = create_texture_image_view(&vk_handle, &texture_image);
				let texture_sampler = create_texture_sampler(&vk_handle).unwrap();
				mesh.material.vulkan_data = Some(
					VulkanMaterialData{
						texture_image: texture_image,
						texture_image_memory: texture_image_memory,
						texture_image_view: texture_image_view,
						texture_sampler: texture_sampler,
					}
				);

				let (vertex_buffer, vertex_buffer_memory) =
					create_vertex_buffer(&vk_handle, &mut mesh.vertices)
					.unwrap();
		
				let (index_buffer, index_buffer_memory) =
					create_index_buffer(&vk_handle, &mut mesh.indices)
					.unwrap();

				mesh.vulkan_data = Some(
					VulkanMeshData{
						vertex_buffer: vertex_buffer,
						vertex_buffer_memory: vertex_buffer_memory,
						index_buffer: index_buffer,
						index_buffer_memory: index_buffer_memory,
						uniform_buffers: vec![],
						uniform_buffers_memory: vec![],
						uniform_buffers_mapped: vec![],
						descriptor_pool: nullptr(),
						descriptor_sets: vec![],
					}
				);

				let vulkan_mesh_data = mesh.vulkan_data.as_mut().unwrap();
				let vulkan_material_data = mesh.material.vulkan_data.as_mut().unwrap();

				create_uniform_buffers(&vk_handle, vulkan_mesh_data);

				let mut descriptor_pool = create_descriptor_pool(&vk_handle).unwrap();
				create_descriptor_sets(&vk_handle, vulkan_mesh_data, vulkan_material_data, &descriptor_pool);
				vulkan_mesh_data.descriptor_pool = create_descriptor_pool(&vk_handle).unwrap();
			}
		}

		// for (model, texture_path) in models.iter_mut().zip(texture_paths)
		// {
		// 	let (texture_image, texture_image_memory) = create_texture_image(&vk_handle, texture_path);
		// 	let texture_image_view = create_texture_image_view(&vk_handle, &texture_image);
		// 	let texture_sampler = create_texture_sampler(&vk_handle);
	
		// 	(model.vertex_buffer, model.vertex_buffer_memory) =
		// 		create_vertex_buffer(&vk_handle, &mut model.vertices)
		// 		.unwrap();
	
		// 	(model.index_buffer, model.index_buffer_memory) =
		// 		create_index_buffer(&vk_handle, &mut model.indices)
		// 		.unwrap();
	
		// 	create_uniform_buffers(&vk_handle, model);
			
		// 	// create_descriptor_pool(&vk_handle, model);
		// 	let descriptor_pool = create_descriptor_pool(&vk_handle).unwrap();
		// 	create_descriptor_sets(&vk_handle, model, &descriptor_pool);
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

		while !input_processor.should_quit() 
		{
			let start_time = std::time::Instant::now();
			// let absolute_current_time_stamp_ms = start_time.duration_since(vk_handle.start_time).as_secs_f32() * 1000.0f32;

			models[0].translation = &vk_handle.camera.get_position() + &(&vk_handle.camera.get_front() * &2.0f32);
			models[0].scale = Vec3::new(0.1f32);// + &(&vk_handle.camera.get_front() * &10.0f32);
			models[0].rotation = vk_handle.camera.get_rotation();// + &(&vk_handle.camera.get_front() * &10.0f32);

			models[1].scale = Vec3::new(0.3f32);// + &(&vk_handle.camera.get_front() * &10.0f32);
			
			println!("Cam rot : {:?}", vk_handle.camera.get_rotation());

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
