use decs::component_derive::system;
use decs::manager::dECS;

use crate::detail_core::components::misc::StringComponent;

use super::input::InputState;

#[system]
pub fn init_input_system()
{
	let input_state = decs.create_entity();
	decs.add_component(input_state, StringComponent{ string : String::from("input_state") }).unwrap();
	decs.add_component(input_state, InputState::new(250.0f32)).unwrap();
}