mod ffi;
mod cotangens;
mod pixcell;
mod loseit;
mod exedra;

mod vulkan;

use vulkan::{
	vk_bindgen::
		vkDeviceWaitIdle,
	handle::VkHandle,
	draw::draw_frame, 
	framebuffer::create_framebuffers,
	instance::create_instance,
	synchronization::create_synchronization_structures,
	depth_buffer::create_depth_buffer,
};

mod detail_core;
use detail_core::{
	input::input_processor::InputProcessor, 
	window::create_vulkan_surface,
	ui::button::UIButton,
	logic::*,
	rendering::*,
};

use crate::{cotangens::{vec3::Vec3, mat4x4}, detail_core::{model::{model::{Model, VulkanModel}, material::Material}, ui::traits::HUDElement, texture::texture::{Texture, VulkanTexture}, phys::{aabb::AABB, system::{run_physics, physics_system}}, components::misc::{StringComponent, Float32Component, Time, DeltaTime, WindowComponent}, rendering::{init::{init_rendering_objects, init_pipelines, init_window_handle}, draw::rendering_system}, logic::{game_objects::init_domatena_shtaiga_object, game_logic::game_logic}, input::system::input_system}, vulkan::{vk_bindgen::{VkFormat, VkCommandPoolCreateFlagBits, VkPolygonMode}, wrappers::{vk_command_pool::{CommandPool, CommandPoolBuilder}, vk_command_buffer::{CommandBuffer, CommandBufferBuilder}}, shader::create_shader_module, vertex::Vertex}};
use parmack::{window::event::{MouseCode, KeyCode, WindowEvent}, handle::Handle};

pub struct EventDrivenVariables
{
	pub thrown_object: bool,
	pub should_run_physics: bool,
	pub focus_on_gui: bool,
}

fn main() 
{
	unsafe
	{
		let mut decs = decs::manager::dECS::new();

		decs.add_init_system(init_window_handle);
		decs.add_init_system(init_rendering_objects);
		decs.add_init_system(init_pipelines);
		decs.add_init_system(init_domatena_shtaiga_object);
		
		decs.add_system(physics_system);
		decs.add_system(rendering_system);
		decs.add_system(input_system);
		decs.add_system(game_logic);

		let vk_handle_entity = decs.create_entity();
		decs.add_component(vk_handle_entity, StringComponent{ string : String::from("vk_handle") }).unwrap();
		decs.add_component(vk_handle_entity, VkHandle::new_empty()).unwrap();

		let main_loop_entity = decs.create_entity();
		decs.add_component(main_loop_entity, StringComponent{ string : String::from("main_loop") }).unwrap();
		decs.add_component(main_loop_entity, DeltaTime{ last_delta_time: 0.0f32, last_time_stamp: std::time::Instant::now() }).unwrap();

		match decs.get_entity_with_components_filter::<StringComponent>(|name_tag| name_tag.string == "vk_handle")
		{
			Some(entity_id) => { println!("Entity {} has nametag 'vk_handle'", entity_id) }
			None => { println!("vk_handle not found") }
		}

		let window = 
			parmack::window::WindowBuilder::new()
			.with_title("windole")
			.with_dimensions(800, 600)
			.build()
			.unwrap();

		let window_entity = decs.create_entity();
		decs.add_component(window_entity, StringComponent{ string : String::from("window") }).unwrap();
		decs.add_component(window_entity, WindowComponent{ window: window }).unwrap();

		let mut input_processor = InputProcessor::new();

		let vk_handle_entity = decs.create_entity();
		decs.add_component(vk_handle_entity, StringComponent{ string : String::from("vk_handle") }).unwrap();
		decs.add_component(vk_handle_entity, VkHandle::new_empty()).unwrap();
		
		let vk_handle = 
			match decs.get_components_global_mut::<VkHandle>()
			{
				Ok(vk_handle_vec) => 
				{
					vk_handle_vec.into_iter().next().unwrap()
				}
				Err(err) => { panic!("vk_handle not found: {}", err) }
			};

		create_instance(vk_handle);

		// vk_handle.window_surface = create_vulkan_surface(&mut window, vk_handle).unwrap();

		decs.init();

		let vk_handle = 
			match decs.get_components_global_mut::<VkHandle>()
			{
				Ok(vk_handle_vec) => 
				{
					vk_handle_vec.into_iter().next().unwrap()
				}
				Err(err) => { panic!("vk_handle not found: {}", err) }
			};

		create_depth_buffer(vk_handle);
		create_framebuffers(vk_handle);

		create_synchronization_structures(vk_handle);

		let command_buffer_count = vk_handle.frames_in_flight as u32;

		{
			let command_buffer_graphics =
			CommandBufferBuilder::new()
			.with_command_pool(&vk_handle.command_pool.as_ref().unwrap())
			.with_count(command_buffer_count)
			.build(&vk_handle.logical_device)
			.unwrap();
			vk_handle.command_buffer_vec = command_buffer_graphics;
		}
		{
			let command_buffer_hud =
			CommandBufferBuilder::new()
			.with_command_pool(&vk_handle.command_pool.as_ref().unwrap())
			.with_count(command_buffer_count)
			.build(&vk_handle.logical_device)
			.unwrap();
			vk_handle.command_buffer_hud_vec = command_buffer_hud;
		}
		{
			let command_buffer_wireframe =
			CommandBufferBuilder::new()
			.with_command_pool(&vk_handle.command_pool.as_ref().unwrap())
			.with_count(command_buffer_count)
			.build(&vk_handle.logical_device)
			.unwrap();
			vk_handle.command_buffer_wireframe_vec = command_buffer_wireframe;
		}	
		
		let mut last_delta_time_ms = 0.0f32;

		loop
		{
			let update_start = std::time::Instant::now();

			decs.update();

			let update_end = std::time::Instant::now();
			last_delta_time_ms = update_end.duration_since(update_start).as_secs_f32() * 1000.0f32;

			let main_loop_var = 
				match decs.get_components_global_mut::<DeltaTime>()
					{
						Ok(delta_time_vec) =>  { delta_time_vec.into_iter().next().unwrap() }
						Err(err) => { panic!("DeltaTime component not found: {}", err) }
					};

			main_loop_var.last_time_stamp = update_end;
			main_loop_var.last_delta_time = last_delta_time_ms;
		}

		let default_normal_map: Texture<VulkanTexture> = 
			Texture::new("./detail/textures/smiley_normal.tga".into())
			.load()
			.unwrap()
			.process_vk(
				&vk_handle, 
				VkFormat::VK_FORMAT_R8G8B8A8_UNORM
			)
			.unwrap();

		let default_albedo_map: Texture<VulkanTexture> = 
			Texture::new("./detail/textures/test.tga".into())
			.load()
			.unwrap()
			.process_vk(
				&vk_handle, 
				VkFormat::VK_FORMAT_R8G8B8A8_SRGB
			)
			.unwrap();

		let material_defaults =
			Material {
				name: "default".into(),
				albedo_path: "unused".to_string(),
				normal_path: "unused".to_string(),
				albedo_map: Some(default_albedo_map.clone()),
				normal_map: Some(default_normal_map.clone()),
			};

		let mut models: Vec<Model<VulkanModel>> =
			vec![
				// Model::new("./detail/models/brick_wall/brick_wall.obj".into()).process_meshes(&vk_handle, material_defaults.clone()),
				// Model::new("./detail/models/de_inferno/de_inferno.obj".into()).process_meshes(&vk_handle, material_defaults.clone()),

				// Model::new("./detail/models/lightbulb/lightbulb.obj".into()).process_meshes(&vk_handle, material_defaults.clone()),
				// Model::new("./detail/models/terrain/terrain.obj".into()).process_meshes(&vk_handle, material_defaults.clone()),
				
				Model::new("./detail/models/tomato_crate/tomato_crate_high_geometry.obj".into()).process_meshes(&vk_handle, material_defaults.clone()),
				
				Model::new("./detail/models/tomato_crate/tomato_crate_high_geometry.obj".into()).process_meshes(&vk_handle, material_defaults.clone()),
				Model::new("./detail/models/tomato_crate/tomato_crate_high_geometry.obj".into()).process_meshes(&vk_handle, material_defaults.clone()),
				Model::new("./detail/models/tomato_crate/tomato_crate_high_geometry.obj".into()).process_meshes(&vk_handle, material_defaults.clone()),
				Model::new("./detail/models/tomato_crate/tomato_crate_high_geometry.obj".into()).process_meshes(&vk_handle, material_defaults.clone()),
				
				Model::new("./detail/models/tomato_crate/tomato_crate_high_geometry.obj".into()).process_meshes(&vk_handle, material_defaults.clone()),
				Model::new("./detail/models/tomato_crate/tomato_crate_high_geometry.obj".into()).process_meshes(&vk_handle, material_defaults.clone()),
				Model::new("./detail/models/tomato_crate/tomato_crate_high_geometry.obj".into()).process_meshes(&vk_handle, material_defaults.clone()),
				Model::new("./detail/models/tomato_crate/tomato_crate_high_geometry.obj".into()).process_meshes(&vk_handle, material_defaults.clone()),
				Model::new("./detail/models/tomato_crate/tomato_crate_high_geometry.obj".into()).process_meshes(&vk_handle, material_defaults.clone()),
				Model::new("./detail/models/tomato_crate/tomato_crate_high_geometry.obj".into()).process_meshes(&vk_handle, material_defaults.clone()),

			];

		models[0].aabb.translation = Vec3{ x: 1.0f32, y: 1.0f32, z: 1.0f32 };
		models[0].aabb.scale = Vec3{ x: 100.0f32, y: 1.0f32, z: 100.0f32 };
		models[0].aabb.is_static = true;

		models[1].aabb.translation = Vec3{ x: 1.0f32, y: -1.0f32, z: 100.0f32 };
		models[1].aabb.scale = Vec3{ x: 100.0f32, y: 10.0f32, z: 1.0f32 };
		models[1].aabb.is_static = true;

		models[2].aabb.translation = Vec3{ x: 1.0f32, y: -1.0f32, z: -100.0f32 };
		models[2].aabb.scale = Vec3{ x: 100.0f32, y: 10.0f32, z: 1.0f32 };
		models[2].aabb.is_static = true;

		models[3].aabb.translation = Vec3{ x: 100.0f32, y: -1.0f32, z: 1.0f32 };
		models[3].aabb.scale = Vec3{ x: 1.0f32, y: 10.0f32, z: 100.0f32 };
		models[3].aabb.is_static = true;

		models[4].aabb.translation = Vec3{ x: -100.0f32, y: -1.0f32, z: 1.0f32 };
		models[4].aabb.scale = Vec3{ x: 1.0f32, y: 10.0f32, z: 100.0f32 };
		models[4].aabb.is_static = true;

		for i in 5..models.len()
		{
			let h = i - 3;
			models[i].aabb.translation = Vec3{ x: 0.0f32, y: (h * h) as f32, z: 0.0f32 };
			// models[i].aabb.scale = Vec3{ x: h as f32, y: h as f32, z: h as f32 };
			models[i].aabb.mass = (h ) as f32;
		}

		let hud_elements: Vec<Box<dyn HUDElement>> =
			vec![
				Box::new(UIButton::new(50, 50, 200, 200).process_vulkan(&vk_handle).unwrap())
			];

		for model in models.iter_mut()
		{
			match model.process_textures(&vk_handle)
			{
				Ok(()) => {},
				Err(err) => { println!("couldn't parse textures for model '{}' err : '{}'", model.name, err) }
			}
		}

		let mut light_pos = Vec3::new(10.0f32);
		let mut last_delta_time_ms : f64 = 16.6f64;

		let mut picked_object_info: Option<(usize, f32, Vec3)> = None;

		let mut event_vars = 
			EventDrivenVariables{
				thrown_object: false,
				should_run_physics: false,
				focus_on_gui: false,
			};

		while !input_processor.should_quit() 
		{
			{
				let mut aabb_references = models.iter_mut().map(|model| &mut model.aabb).collect::<Vec<&mut AABB>>();

				if event_vars.should_run_physics
				{
					run_physics(&mut aabb_references, last_delta_time_ms);
				}

				match picked_object_info
				{
					Some((index, length, obj_relative_hit)) =>
					{
						if !aabb_references[index].is_static
						{
							aabb_references[index].translation = vk_handle.camera.get_position() + (vk_handle.camera.get_front() * length) + obj_relative_hit;
							aabb_references[index].velocity = Vec3::new(0.0f32);
						}
					}
					None => {}
				}

				if !event_vars.focus_on_gui
				{
					'raycast: 
					{
						if vk_handle.mouse_input_buffer.is_pressed(MouseCode::Left as u8)
						{
							if event_vars.thrown_object
							{
								break 'raycast;
							}
	
							match picked_object_info
							{
								Some(object_info) => 
								{
									if vk_handle.input_buffer.is_pressed(KeyCode::Space as u8)
									{
										println!("throwing");
										event_vars.thrown_object = true;

										let aabb_pos = aabb_references[object_info.0].translation;

										picked_object_info = None;

										aabb_references[object_info.0].velocity += 
											(aabb_pos - vk_handle.camera.get_position()).normalize() / 1.0f32;
									}
								}
								None => 
								{
									let mut hit_points: Vec<(usize, f32)> = 
										aabb_references
										.iter_mut()
										.enumerate()
										.filter_map(
											|(index, aabb)|
											{
												match aabb.raycast_hit(vk_handle.camera.get_position(), vk_handle.camera.get_front())
												{
													Some(hit_point) =>
													{
														Some((index, hit_point.len()))
													}
													None => { None }
												}
											}
										)
										.collect();
									
									hit_points.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));
	
									for (aabb_index, camera_front_scale) in hit_points.iter().cloned()
									{
										if aabb_references[aabb_index].is_static
										{
											continue;
										}

										let hit_world_pos = vk_handle.camera.get_position() + (vk_handle.camera.get_front() * camera_front_scale);
										let aabb_center_hit_vec3 = aabb_references[aabb_index].translation - hit_world_pos;

										picked_object_info = Some((aabb_index, camera_front_scale, aabb_center_hit_vec3));
										break;
									}
								}
							}
						}
						else 
						{
							picked_object_info = None;
							event_vars.thrown_object = false;
						}
					}

					vk_handle.camera.process_movement(last_delta_time_ms as f32, &vk_handle.input_buffer);
					vk_handle.camera.update_camera_vectors();
				}
			}

			let start_time = std::time::Instant::now();
			let absolute_current_time_stamp_s = start_time.duration_since(vk_handle.start_time).as_secs_f32();

			// let pointer_pos = window.get_pointer_location();
			draw_frame(vk_handle, &mut models, &light_pos.clone(), &hud_elements);

			let end_time = std::time::Instant::now();
			last_delta_time_ms = end_time.duration_since(start_time).as_secs_f64() * 1000.0f64;
			
			input_processor.process_window_events(&mut window, vk_handle, &mut event_vars, &mut picked_object_info, last_delta_time_ms as f32);

			if !event_vars.focus_on_gui
			{
				if vk_handle.mouse_input_buffer.is_pressed(MouseCode::Right as u8)
				{
					light_pos = vk_handle.camera.get_position() + vk_handle.camera.get_front() * 4.0f32;
				}

				vk_handle.camera.process_movement(last_delta_time_ms as f32, &vk_handle.input_buffer);
				vk_handle.camera.update_camera_vectors();
			}

		}

		vkDeviceWaitIdle(vk_handle.logical_device);

		// Cleanup
		println!("Destroying vk objects...");

		vk_handle.destroy_vk_resources();
	}
}
