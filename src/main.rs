mod ffi;
mod cotangens;
mod pixcell;
mod loseit;
mod exedra;

mod vulkan;

use detail_core::{
	rendering::{
		init::{
			init_window_handle, init_rendering_objects, init_pipelines, init_buffer_objects, init_rendering_assets
		}, 
		draw::rendering_system3
	}, 
	logic::{
		game_objects::{
			init_domatena_shtaiga_object, init_domatena_shtaiga_assets
		}, 
		game_logic::game_logic_system
	}, 
	diagnostics::system::print_delta_time_system, phys::system::physics_system_2, input::system::input_system, 
	components::misc::{
		StringComponent, DeltaTime, WindowComponent, MainLoopComponent, GlobalVariables, RaycastObject, RaycastObjectState
	}, 
	camera::system::update_camera_system, misc_systems::raycast_aabb_pickup::raycast_aabb_pickup_system
};


use vulkan::{
	vk_bindgen::
		vkDeviceWaitIdle,
	handle::VkHandle,
	instance::create_instance,
};

mod detail_core;

fn main() 
{
	unsafe
	{
		let mut decs = decs::manager::dECS::new();

		decs.add_init_system(init_window_handle);
		decs.add_init_system(init_rendering_objects);
		decs.add_init_system(init_pipelines);
		decs.add_init_system(init_buffer_objects);

		decs.add_init_system(init_rendering_assets);

		decs.add_init_system(init_domatena_shtaiga_assets);
		decs.add_init_system(init_domatena_shtaiga_object);
		
		// decs.add_system(physics_system);
		decs.add_system(physics_system_2);
		decs.add_system(rendering_system3);
		decs.add_system(input_system);
		decs.add_system(game_logic_system);
		decs.add_system(update_camera_system);
		decs.add_system(raycast_aabb_pickup_system);

		decs.add_system(print_delta_time_system);

		let vk_handle_entity = decs.create_entity();
		decs.add_component(vk_handle_entity, StringComponent{ string : String::from("vk_handle") }).unwrap();
		decs.add_component(vk_handle_entity, VkHandle::new_empty()).unwrap();

		let main_loop_entity = decs.create_entity();
		decs.add_component(main_loop_entity, StringComponent{ string : String::from("main_loop") }).unwrap();
		decs.add_component(main_loop_entity, DeltaTime{ last_delta_time: 0.0f32, last_time_stamp: std::time::Instant::now() }).unwrap();
		decs.add_component(main_loop_entity, MainLoopComponent{ should_quit: false }).unwrap();

		let window = 
			parmack::window::WindowBuilder::new()
			.with_title("windole")
			.with_dimensions(800, 600)
			.build()
			.unwrap();

		let window_entity = decs.create_entity();
		decs.add_component(window_entity, StringComponent{ string : String::from("window") }).unwrap();
		decs.add_component(window_entity, WindowComponent{ window: window }).unwrap();

		decs.modify_components_global::<VkHandle>(
			|vk_handle| 
			{
				create_instance(vk_handle);
				Ok(())
			}
		).expect("vk_handle not found");

		decs.init();

		let global_vars = decs.create_entity();
		decs.add_component(global_vars, StringComponent{ string : String::from("global_vars") }).unwrap();
		decs.add_component(global_vars, GlobalVariables{ should_run_physics: false, focus_on_gui: false }).unwrap();
		decs.add_component(global_vars, RaycastObject{ state: RaycastObjectState::None }).unwrap();

		'main_loop: 
		loop
		{
			let update_start = std::time::Instant::now();

			decs.update();

			let update_end = std::time::Instant::now();

			decs.modify_components_global::<DeltaTime>(
				|delta_time_obj| 
				{
					delta_time_obj.last_time_stamp = update_end;
					delta_time_obj.last_delta_time = update_end.duration_since(update_start).as_secs_f32() * 1000.0f32;
					Ok(())
				}
			).unwrap();

			if 
			decs.get_components_global::<MainLoopComponent>().expect("missing main loop component").remove(0)
			.should_quit
			{
				break 'main_loop;
			}
		}

		decs.modify_components_global::<VkHandle>(
			|vk_handle|
			{
				println!("Destroying vk objects...");
				vkDeviceWaitIdle(vk_handle.logical_device);
				vk_handle.destroy_vk_resources();
				Ok(())
			}
		).expect("vk_handle not found");
	}
}
