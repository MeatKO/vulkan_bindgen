use decs::component_derive::system;
use decs::manager::dECS;
use parmack::handle::Handle;

use crate::detail_core::components::misc::WindowComponent;

use super::input::InputState;

#[system]
pub fn input_polling_system()
{
	let window: &mut WindowComponent =
		unsafe { decs.get_components_global_mut_unchecked::<WindowComponent>() }.unwrap().remove(0).component;

	let input_state: &mut InputState =
		unsafe { decs.get_components_global_mut_unchecked::<InputState>() }.unwrap().remove(0).component;

	let window = &mut window.window;

	input_state.update();
	input_state.current_events_vec = window.get_events();
}