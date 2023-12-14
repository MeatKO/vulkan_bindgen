use decs::component_derive::system;
use decs::manager::dECS;

use crate::detail_core::components::misc::DeltaTime;
use crate::vulkan::handle::VkHandle;

#[system]
pub fn update_camera_system()
{
	let delta_time: &mut DeltaTime =
		unsafe { decs.get_components_global_mut_unchecked::<DeltaTime>() }.unwrap().remove(0).component;

	decs.modify_components_global::<VkHandle>(
		|vk_handle|
		{
			vk_handle.camera.process_movement(delta_time.last_delta_time, &vk_handle.input_buffer);
			vk_handle.camera.update_camera_vectors();
			Ok(())
		}
	).unwrap();
}