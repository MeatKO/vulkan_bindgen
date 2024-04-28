use decs::component_derive::system;
use decs::manager::dECS;

use crate::detail_core::components::misc::DeltaTime;
use crate::detail_core::input::input::InputState;
use crate::vulkan::handle::VkHandle;

#[system]
pub fn update_camera_system()
{
	let delta_time: &mut DeltaTime =
		unsafe { decs.get_components_global_mut_unchecked::<DeltaTime>() }.unwrap().remove(0).component;

	let input_state: &mut InputState =
		unsafe { decs.get_components_global_mut_unchecked::<InputState>() }.unwrap().remove(0).component;

	// let input_buffer_keyboard_index: usize = 
	// 	unsafe { decs.get_entity_with_components_filter::<StringComponent>(|string_component| {string_component.string == "input_buffer_keyboard"} ).unwrap() };
	// let input_buffer_keyboard: &mut InputBuffer = 
	// 	unsafe { decs.get_components_mut_unchecked::<InputBuffer>(input_buffer_keyboard_index) }.unwrap().remove(0).component;


	decs.modify_components_global::<VkHandle>(
		|vk_handle|
		{
			vk_handle.camera.process_movement(delta_time.last_delta_time, &input_state.current_keyboard_state);
			vk_handle.camera.update_camera_vectors();
			Ok(())
		}
	).unwrap();
}