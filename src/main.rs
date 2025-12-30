mod ffi;
mod cotangens;
mod pixcell;
mod loseit;
mod exedra;

mod vulkan;

use decs2::component_derive::Component;
use decs2::typedef::Component;

use detail_core::{
	asset_manager::manager::AssetManager, camera::system::update_camera_system, components::misc::{
		CameraRaycastObject, CameraRaycastObjectState, DeltaTime, GlobalVariables, StringComponent, WindowComponent
	}, diagnostics::system::print_delta_time_system, input::{init::init_input_system, input_processor::input_processor_system, system::input_polling_system}, logic::{
		game_logic::game_logic_system, game_objects::{
			init_domatena_shtaiga_assets_2, init_domatena_shtaiga_object, init_misc_assets, init_misc_objects
		}
	}, misc_systems::raycast_aabb_pickup::raycast_aabb_pickup_system, 
	phys::{
		new_system::physics_system_3, 
		system::physics_system_2
	}, rendering::{draw::rendering_system4, init::{
		init_buffer_objects, init_pipelines, init_rendering_assets, init_rendering_objects, init_window_handle
	}}
};

use vulkan::{
	vk_bindgen::
		vkDeviceWaitIdle,
	handle::VkHandle,
	instance::create_instance,
};

mod detail_core;

mod experimenting;
use experimenting::*;

#[derive(Component, Debug)]
struct NameComponent(pub String);

fn main() 
{
	let mut decs = decs2::manager::dECSManager::new();

	decs.add_global_storage(VkHandle::new_empty());
	decs.add_global_storage(AssetManager::new());

	decs.add_init_system(init_input_system);
	decs.add_init_system(init_window_handle);
	decs.add_init_system(init_rendering_objects);
	decs.add_init_system(init_pipelines);
	decs.add_init_system(init_buffer_objects);
	decs.add_init_system(init_rendering_assets);
	decs.add_init_system(init_misc_assets);
	decs.add_init_system(init_misc_objects);
	decs.add_init_system(init_domatena_shtaiga_assets_2);
	// decs.add_init_system(init_domatena_shtaiga_assets);
	decs.add_init_system(init_domatena_shtaiga_object);
	
	// decs.add_system(physics_system_2);
	decs.add_system(physics_system_3);
	decs.add_system(rendering_system4);
	decs.add_system(input_polling_system);
	decs.add_system(input_processor_system);
	decs.add_system(game_logic_system);
	decs.add_system(update_camera_system);
	decs.add_system(raycast_aabb_pickup_system);
	decs.add_system(print_delta_time_system);

	let window = 
		parmack::window::WindowBuilder::new()
		.with_title("windole")
		.with_dimensions(800, 600)
		.build()
		.unwrap();

	decs.add_global_storage(WindowComponent{ window: window });

	decs.modify_global_storage::<VkHandle>(
		|vk_handle| 
		{
			unsafe { create_instance(vk_handle); }
			Ok(())
		}
	)
	.expect("Could not create VK instance");

	decs.add_global_storage(GlobalVariables::new());

	decs.init_systems();

	'main_loop: 
	loop
	{
		let update_start = std::time::Instant::now();
		decs.update_systems();
		let update_end = std::time::Instant::now();

		decs.modify_global_storage::<GlobalVariables>(
			|globals| 
			{
				let delta_time_obj = &mut globals.delta_time;
				delta_time_obj.last_time_stamp = update_end;
				delta_time_obj.last_delta_time_sec = update_end.duration_since(update_start).as_secs_f32() * 1000.0f32;

				Ok(())
			}
		)
		.expect("could not get delta time");

		if decs.get_global_storage_mut_unchecked::<GlobalVariables>().unwrap().should_quit
		{
			break 'main_loop;
		}
	}

	decs.modify_global_storage::<VkHandle>(
		|vk_handle|
		{
			println!("Destroying vk objects...");
			unsafe 
			{
				vkDeviceWaitIdle(vk_handle.logical_device);
				vk_handle.destroy_vk_resources();
			}
			Ok(())
		}
	)
	.expect("could not clean up VK objects");
}
