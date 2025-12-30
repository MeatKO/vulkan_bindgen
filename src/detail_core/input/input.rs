use decs2::component_derive::Component;
use decs2::typedef::Component;
use parmack::window::event::WindowEvent;

use super::input_buffer::InputBuffer;

#[derive(Component)]
pub struct InputState
{
	pub current_keyboard_state: InputBuffer,
	pub current_mouse_state: InputBuffer,
	pub current_events_vec: Vec<WindowEvent>,

	pub last_keyboard_state: InputBuffer,
	pub last_mouse_state: InputBuffer,
	pub last_events_vec: Vec<WindowEvent>
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
			current_events_vec: vec![],
			last_events_vec: vec![],
		}
	}

	pub fn update(&mut self)
	{
		self.last_keyboard_state = self.current_keyboard_state.clone();
		self.last_mouse_state = self.current_mouse_state.clone();

		self.last_events_vec = self.current_events_vec.clone(); 
		self.current_events_vec = vec![];
	}
}