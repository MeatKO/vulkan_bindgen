use decs::component_derive::system;
use decs::manager::dECS;

use crate::cotangens::vec3::Vec3;
use crate::detail_core::components::misc::{GlobalVariables, StringComponent};
use crate::detail_core::components::rendering::UniformBufferComponent;
use crate::detail_core::misc::rand::rand;
use crate::detail_core::model::component::VulkanModelComponent;
use crate::detail_core::phys::aabb::AABB;
use crate::vulkan::handle::VkHandle;

#[system]
pub fn game_logic_system()
{
	let vk_handle: &mut VkHandle =
		unsafe { decs.get_components_global_mut_unchecked::<VkHandle>() }.unwrap().remove(0).component;

	let global_variables: &mut GlobalVariables =
		unsafe { decs.get_components_global_mut_unchecked::<GlobalVariables>() }.unwrap().remove(0).component;

	let last_random_object_id = global_variables.global_env_map.get_mut("random_object_counter");

	let last_index = 
		match last_random_object_id
		{
			Some(id_obj) =>
			{
				*id_obj.downcast_ref::<u32>().unwrap()
			}
			None =>
			{
				global_variables.global_env_map.insert("random_object_counter".to_owned(), Box::new(0u32));
				0u32
			}
		};

	let last_random_object_time = global_variables.global_env_map.get_mut("random_object_time");

	match last_random_object_time
	{
		Some(time_obj) =>
		{
			let last_time = time_obj.downcast_ref::<std::time::Instant>().unwrap();

			if last_time.elapsed() >= std::time::Duration::from_secs_f32(0.1f32)
			{
				let random_translation = 
					Vec3 {
						x: rand(-50..50) as f32,
						y: rand(5..50) as f32,
						z: rand(-50..50) as f32,
					};

				let mut aabb = AABB::new_nonverbose(random_translation, Vec3::new(1.0f32), false);
				unsafe { aabb.process_vulkan(vk_handle) };

				let shtaiga = decs.create_entity();
				decs.add_component(shtaiga, StringComponent{ string : format!("valkyrie_{}", last_index).to_owned()}).unwrap();
				decs.add_component(shtaiga, VulkanModelComponent::new("valkyrie".to_owned())).unwrap();
				decs.add_component(shtaiga, aabb).unwrap();
				decs.add_component(shtaiga, UniformBufferComponent::new(vk_handle).unwrap()).unwrap();

				println!("added model {}", format!("valkyrie_{}", last_index).to_owned());

				global_variables.global_env_map.insert("random_object_time".to_owned(), Box::new(std::time::Instant::now()));
				global_variables.global_env_map.insert("random_object_counter".to_owned(), Box::new(last_index + 1));
			}
		}
		None => 
		{
			global_variables.global_env_map.insert("random_object_time".to_owned(), Box::new(std::time::Instant::now()));
		}
	}
}