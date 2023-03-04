use crate::loseit::{
	window::*, window_events::*,
};

use crate::detail_core::{
	camera::*,
};
use crate::vulkan::handle::VkHandle;

pub struct InputProcessor
{
	mouse_x: f32,
	mouse_y: f32,
	should_quit: bool
}

impl InputProcessor
{
	pub fn new() -> InputProcessor
	{
		return InputProcessor{
			mouse_x: 0.0f32,
			mouse_y: 0.0f32,
			should_quit: false
		};
	}

	pub fn should_quit(&self,) -> bool
	{
		return self.should_quit;
	}

	// pub fn process_window_events(&mut self, delta_time_ms: f32, window: &Window, camera: &mut Camera)
	pub fn process_window_events(&mut self, delta_time_ms: f32, window: &Window, vk_handle: &mut VkHandle)
	{
		let process_start_time = std::time::Instant::now();
		let absolute_current_time_stamp_ms = process_start_time.duration_since(vk_handle.start_time).as_secs_f32() * 1000.0f32;
		while let Some(ref mut event) = window.get_event()
		{
			match event
			{
				WindowEvent::KeyPress(key_code) => 
				{
					vk_handle.input_buffer.set_key(*key_code, absolute_current_time_stamp_ms);

					match window.convert_key_code(*key_code)
					{
						KeyValues::ESC => { self.should_quit = true; }
						_ => {}
					}
				}
				WindowEvent::KeyRelease(key_code) => 
				{
					vk_handle.input_buffer.unset_key(*key_code);
				}
				WindowEvent::WindowAction(val) => 
				{
					match val
					{
						WindowActions::CLOSE => { self.should_quit = true; }
						_ => {}
					}
				}
				_ => { }
			}
		}
	}
}