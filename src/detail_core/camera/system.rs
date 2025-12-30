use decs2::component_derive::system;
use decs2::manager::dECSManager;

use crate::detail_core::components::misc::{DeltaTime, GlobalVariables};
use crate::detail_core::input::input::InputState;
use crate::vulkan::handle::VkHandle;

#[system]
pub fn update_camera_system()
{
	let delta_time = &decs.get_global_storage_mut_unchecked::<GlobalVariables>().unwrap().delta_time;
	let input_state = decs.get_global_storage_mut_unchecked::<InputState>().unwrap();

	decs.modify_global_storage::<VkHandle>(
		|vk_handle|
		{
			vk_handle.camera.process_movement(delta_time.last_delta_time_sec, &input_state.current_keyboard_state);
			vk_handle.camera.update_camera_vectors();
			Ok(())
		}
	)
	.expect("could not modify the VK handle.")
}