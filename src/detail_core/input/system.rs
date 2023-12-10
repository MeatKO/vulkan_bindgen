use decs::component_derive::system;
use decs::manager::dECS;
use parmack::handle::Handle;
use parmack::window::event::{WindowEvent, WindowActions};

use crate::detail_core::components::misc::{WindowComponent, DeltaTime};
use crate::vulkan::handle::VkHandle;

#[system]
pub fn input_system()
{
	let window_events = 
		match decs.get_components_global_mut::<WindowComponent>()
		{
			Ok(window_handle_vec) => 
			{
				let window_handle = window_handle_vec.into_iter().next().unwrap();
				window_handle.window.get_events()
			}
			Err(err) => { panic!("vk_handle not found: {}", err) }
		};

	let delta_time: DeltaTime =
		match decs.get_components_global_mut::<DeltaTime>()
		{
			Ok(delta_time_vec) => 
			{
				delta_time_vec.into_iter().next().unwrap().clone()
			}
			Err(err) => { panic!("vk_handle not found: {}", err) }
		};

	let vk_handle = 
		match decs.get_components_global_mut::<VkHandle>()
		{
			Ok(vk_handle_vec) => 
			{
				vk_handle_vec.into_iter().next().unwrap()
			}
			Err(err) => { panic!("vk_handle not found: {}", err) }
		};

	let process_start_time = std::time::Instant::now();
	let absolute_current_time_stamp_ms = process_start_time.duration_since(vk_handle.start_time).as_secs_f32() * 1000.0f32;
	
	let delta_time_ms = delta_time.last_delta_time;

	for event in window_events
	{
		match event
		{
			WindowEvent::MousePress(mouse_code, x, y) =>
			{
				// match picked_object_info.as_mut()
				// {
				// 	Some(object_info) =>
				// 	{
				// 		if mouse_code == MouseCode::ScrollDown
				// 		{
				// 			object_info.1 -= 1000.0f32 * (delta_time_ms / 1000.0f32);
				// 		}
				// 		if mouse_code == MouseCode::ScrollUp
				// 		{
				// 			object_info.1  += 1000.0f32 * (delta_time_ms / 1000.0f32);
				// 		}
				// 	}
				// 	None => {}
				// }

				vk_handle.mouse_input_buffer.set_key(mouse_code as u8, absolute_current_time_stamp_ms);
			}
			WindowEvent::MouseRelease(mouse_code, x, y) =>
			{
				vk_handle.mouse_input_buffer.unset_key(mouse_code as u8);
			}
			WindowEvent::KeyPress(key_code) => 
			{
				vk_handle.input_buffer.set_key(key_code as u8, absolute_current_time_stamp_ms);

				// match key_code 
				// {
				// 	KeyCode::Escape => { self.should_quit = true; }
				// 	KeyCode::ShiftLeft => { event_vars.focus_on_gui = !event_vars.focus_on_gui; }
				// 	KeyCode::V => { event_vars.should_run_physics = !event_vars.should_run_physics; }
				// 	_ => {}
				// }
			}
			WindowEvent::KeyRelease(key_code) => 
			{
				vk_handle.input_buffer.unset_key(key_code as u8);
			}
			WindowEvent::WindowAction(val) => 
			{
				match val
				{
					// WindowActions::Close => { self.should_quit = true; }
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

						// let (width, height) = window.get_size();
						let (width, height) = (800, 600);
						let mid_x = (width / 2) as i32;
						let mid_y = (height / 2) as i32;

						let delta_x = x - mid_x;
						let delta_y = -(y - mid_y);

						// if window.is_focused &&
						if
						(delta_x != 0 ||
						delta_y != 0)
						{
							// window.center_pointer(true);
							vk_handle.camera.process_mouse_movement(delta_x as f32 * 10.0f32, delta_y as f32 * 10.0f32);
						}

						// println!("delta x {} delta y {}", delta_x, delta_y);

						// vk_handle.camera.process_mouse_movement(delta_x as f32 * delta_time_ms, delta_y as f32 * delta_time_ms);
						// vk_handle.camera.process_mouse_movement(delta_x as f32 * 10.0f32, delta_y as f32 * 10.0f32);
					}
					_ => {}
				}
			}
			_ => { }
		}
	}
}