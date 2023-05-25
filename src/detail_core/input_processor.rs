use parmack::window::window_handle::WindowHandle;
use parmack::window::event::WindowEvent;
use parmack::window::event::WindowActions;
use parmack::window::event::KeyCode;
use parmack::handle::Handle;

use crate::vulkan::handle::VkHandle;

pub struct InputProcessor
{
	last_mouse_x: f32,
	last_mouse_y: f32,
	should_quit: bool
}

impl InputProcessor
{
	pub fn new() -> InputProcessor
	{
		return InputProcessor{
			last_mouse_x: 0.0f32,
			last_mouse_y: 0.0f32,
			should_quit: false
		};
	}

	pub fn should_quit(&self,) -> bool
	{
		return self.should_quit;
	}

	// pub fn process_window_events(&mut self, delta_time_ms: f32, window: &Window, camera: &mut Camera)
	pub fn process_window_events(&mut self, delta_time_ms: f32, window: &mut WindowHandle, vk_handle: &mut VkHandle)
	{
		let process_start_time = std::time::Instant::now();
		let absolute_current_time_stamp_ms = process_start_time.duration_since(vk_handle.start_time).as_secs_f32() * 1000.0f32;
		
		for event in window.get_events()
		{
			match event
			{
				WindowEvent::KeyPress(key_code) => 
				{
					vk_handle.input_buffer.set_key(key_code as u8, absolute_current_time_stamp_ms);

					match key_code 
					{
						KeyCode::ESC => { self.should_quit = true; }
						_ => {}
					}
				}
				WindowEvent::KeyRelease(key_code) => 
				{
					vk_handle.input_buffer.unset_key(key_code as u8);
				}
				WindowEvent::WindowAction(val) => 
				{
					match val
					{
						WindowActions::Close => { self.should_quit = true; }
						// WindowActions::FocusIn => 
						// { 
						// 	window.is_focused = true; 
						// 	window.lock_pointer(); 
						// 	window.hide_cursor();
						// }
						// WindowActions::FocusOut => 
						// { 
						// 	window.is_focused = false; 
						// 	window.unlock_pointer(); 
						// 	window.show_cursor(); 
						// }

						// assuming x and y are delta from the center (will think of something better later)
						WindowActions::Motion{x, y} => 
						{
							// println!("cursor is at : {} {}", x, y);

							// vk_handle.camera.process_mouse_movement(x as f32 * delta_time_ms, y as f32 * delta_time_ms);

							let (width, height) = window.get_size();
							let mid_x = (width / 2) as i32;
							let mid_y = (height / 2) as i32;

							let delta_x = x - mid_x;
							let delta_y = -(y - mid_y);

							// if window.is_focused &&
							if
							delta_x != 0 ||
							delta_y != 0
							{
								window.center_pointer(true);
							}

							// println!("delta x {} delta y {}", delta_x, delta_y);

							// vk_handle.camera.process_mouse_movement(delta_x as f32 * delta_time_ms, delta_y as f32 * delta_time_ms);
							vk_handle.camera.process_mouse_movement(delta_x as f32 * 10.0f32, delta_y as f32 * 10.0f32);
						}
						_ => {}
					}
				}
				_ => { }
			}
		}

		// let (x, y) = window.get_pointer_location();

		// println!("cursor is at : {} {}", x, y);

		// let (width, height) = window.get_size();
		// let mid_x = (width / 2) as i32;
		// let mid_y = (height / 2) as i32;

		// let delta_x = x - mid_x;
		// let delta_y = -(y - mid_y);

		// // if window.is_focused &&
		// if
		// delta_x != 0 ||
		// delta_y != 0
		// {
		// 	window.center_pointer(true);
		// }
		// println!("delta x {} delta y {}", delta_x, delta_y);
		// vk_handle.camera.process_mouse_movement(delta_x as f32 * delta_time_ms, delta_y as f32 * delta_time_ms);
	
	}
}