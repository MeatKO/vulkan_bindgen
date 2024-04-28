use decs::component_derive::component;
use decs::component::Component;

use super::input_buffer::InputBuffer;

#[derive(Copy, Clone, Debug)]
pub struct ButtonInfo
{
	is_pressed: bool,
	press_timestamp_ms: f32
}

#[component]
pub struct InputState
{
	pub current_keyboard_state: InputBuffer,
	pub current_mouse_state: InputBuffer,

	pub last_keyboard_state: InputBuffer,
	pub last_mouse_state: InputBuffer,
}

impl InputState
{
	pub fn new(hold_threshold_ms: f32) -> Self
	{
		return InputState{
			current_keyboard_state: InputBuffer::new(hold_threshold_ms),
			current_mouse_state: InputBuffer::new(hold_threshold_ms),
			last_keyboard_state: InputBuffer::new(hold_threshold_ms),
			last_mouse_state: InputBuffer::new(hold_threshold_ms),
		}
	}

	pub fn update(&mut self)
	{
		self.last_keyboard_state = self.current_keyboard_state.clone();
		self.last_mouse_state = self.current_mouse_state.clone();
	}
}