#[derive(Copy, Clone, Debug)]
pub struct ButtonInfo
{
	is_pressed: bool,
	press_timestamp_ms: f32
}

#[derive(Debug)]
pub struct InputBuffer
{
	key_states: [ButtonInfo; u8::MAX as usize + 1],
	hold_threshold_ms: f32
}

impl InputBuffer
{
	pub fn new(hold_threshold_ms: f32) -> InputBuffer
	{
		return InputBuffer{
			key_states: [
				ButtonInfo{
					is_pressed: false,
					press_timestamp_ms: 0.0f32
				};
				256
			],
			hold_threshold_ms: hold_threshold_ms
		}
	}

	pub fn is_pressed(&self, key_code: u8) -> bool
	{
		return self.key_states[key_code as usize].is_pressed;
	}

	pub fn is_held(&self, key_code: u8, absolute_current_time_stamp_ms: f32) -> bool
	{
		return 
		self.key_states[key_code as usize].is_pressed &&
		(absolute_current_time_stamp_ms - self.key_states[key_code as usize].press_timestamp_ms) > self.hold_threshold_ms;
	}

	pub fn set_key(&mut self, key_code: u8, absolute_current_time_stamp_ms: f32)
	{
		if self.key_states[key_code as usize].is_pressed
		{
			return
		}

		self.key_states[key_code as usize].is_pressed = true;
		self.key_states[key_code as usize].press_timestamp_ms = absolute_current_time_stamp_ms;
	}

	pub fn unset_key(&mut self, key_code: u8)
	{
		self.key_states[key_code as usize].is_pressed = false;
		self.key_states[key_code as usize].press_timestamp_ms = 0.0f32;
	}
}