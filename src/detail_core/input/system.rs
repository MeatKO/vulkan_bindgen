use decs2::component_derive::system;
use decs2::manager::dECSManager;
use parmack::handle::Handle;

use crate::detail_core::components::misc::WindowComponent;

use super::input::InputState;

#[system]
pub fn input_polling_system()
{
	// let window: &mut WindowComponent =
	// 	unsafe { decs.get_components_global_mut_unchecked::<WindowComponent>() }.unwrap().remove(0).component;

	// let input_state: &mut InputState =
	// 	unsafe { decs.get_components_global_mut_unchecked::<InputState>() }.unwrap().remove(0).component;

	let input_state = decs.get_global_storage_mut_unchecked::<InputState>().unwrap();
	let window = decs.get_global_storage_mut_unchecked::<WindowComponent>().unwrap();

	let window = &mut window.window;

	input_state.update();
	input_state.current_events_vec = window.get_events();
}