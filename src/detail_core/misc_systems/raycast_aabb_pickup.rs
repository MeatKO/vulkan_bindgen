use decs::component_derive::system;
use decs::manager::dECS;
use parmack::window::event::{MouseCode, KeyCode};

use crate::detail_core::components::misc::{DeltaTime, GlobalVariables, RaycastObject, RaycastObjectState};
use crate::detail_core::model::model::{VulkanModel, Model};
use crate::detail_core::phys::aabb::AABB;
use crate::vulkan::handle::VkHandle;

#[system]
pub fn raycast_aabb_pickup_system()
{
	let delta_time: &mut DeltaTime =
		unsafe { decs.get_components_global_mut_unchecked::<DeltaTime>() }.unwrap().remove(0).component;

	let vk_handle: &mut VkHandle =
		unsafe { decs.get_components_global_mut_unchecked::<VkHandle>() }.unwrap().remove(0).component;

	let global_variables: &mut GlobalVariables =
		unsafe { decs.get_components_global_mut_unchecked::<GlobalVariables>() }.unwrap().remove(0).component;

	let raycast_object: &mut RaycastObject =
		unsafe { decs.get_components_global_mut_unchecked::<RaycastObject>() }.unwrap().remove(0).component;

	let mut aabb_vector = 
		unsafe {decs.get_components_global_mut_unchecked::<Model<VulkanModel>>().unwrap().into_iter().map(|model| &mut model.component.aabb).collect::<Vec<&mut AABB>>() };
	
	// rewrite this so it uses click and release events because its getting ridiculous
	if vk_handle.mouse_input_buffer.is_pressed(MouseCode::Left as u8)
	{
		match raycast_object.state
		{
			RaycastObjectState::Thrown(_) => {}
			RaycastObjectState::Picked((index, length, obj_relative_hit)) => 
			{
				if vk_handle.input_buffer.is_pressed(KeyCode::Space as u8)
				{
					println!("throwing");
					raycast_object.state = RaycastObjectState::Thrown((index, length, obj_relative_hit));

					let aabb_pos = aabb_vector[index].translation;

					aabb_vector[index].velocity += 
						(aabb_pos - vk_handle.camera.get_position()).normalize() / 1.0f32;
				}
			}
			RaycastObjectState::None => 
			{
				let mut hit_points: Vec<(usize, f32)> = 
				aabb_vector
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
					if aabb_vector[aabb_index].is_static
					{
						continue;
					}

					let hit_world_pos = vk_handle.camera.get_position() + (vk_handle.camera.get_front() * camera_front_scale);
					let aabb_center_hit_vec3 = aabb_vector[aabb_index].translation - hit_world_pos;

					raycast_object.state = RaycastObjectState::Picked((aabb_index, camera_front_scale, aabb_center_hit_vec3));
					// break;
				}
			}
		}
	}
	else 
	{
		raycast_object.state = RaycastObjectState::None;
	}
}