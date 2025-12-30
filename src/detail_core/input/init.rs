use decs2::component_derive::system;
use decs2::manager::dECSManager;

use super::input::InputState;

#[system]
pub fn init_input_system()
{
	decs.add_global_storage(InputState::new(250.0f32));
}