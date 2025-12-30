use std::borrow::BorrowMut;

use decs2::component_derive::system;
use decs2::manager::dECSManager;
use parmack::handle::Handle;
use parmack::window::event::{WindowEvent, WindowActions, KeyCode, MouseCode};

use crate::detail_core::components::misc::{CameraRaycastObject, CameraRaycastObjectState, DeltaTime, GlobalVariables, WindowComponent};
use crate::vulkan::handle::VkHandle;

use super::input::InputState;

#[system]
pub fn input_processor_system()
{
	// let delta_time = decs.get_global_storage_mut_unchecked::<DeltaTime>().unwrap();
	let delta_time = &decs.get_global_storage_mut_unchecked::<GlobalVariables>().unwrap().delta_time;

	let input_state = decs.get_global_storage_mut_unchecked::<InputState>().unwrap();
	let window = decs.get_global_storage_mut_unchecked::<WindowComponent>().unwrap();
	let vk_handle = decs.get_global_storage_mut_unchecked::<VkHandle>().unwrap();
	let raycast_object = &mut decs.get_global_storage_mut_unchecked::<GlobalVariables>().unwrap().global_raycast_object;

	let process_start_time = std::time::Instant::now();
	let absolute_current_time_stamp_ms = process_start_time.duration_since(vk_handle.start_time).as_secs_f32() * 1000.0f32;

	let window = &mut window.window;

	for event in input_state.current_events_vec.iter()
	{
		match event
		{
			WindowEvent::MousePress(mouse_code, x, y) =>
			{
				match raycast_object.state.borrow_mut()
				{
					CameraRaycastObjectState::Picked(raycast_info) =>
					{
						if *mouse_code == MouseCode::ScrollDown
						{
							raycast_info.length -= 1000.0f32 * (delta_time.last_delta_time_sec / 1000.0f32);
						}
						if *mouse_code == MouseCode::ScrollUp
						{
							raycast_info.length += 1000.0f32 * (delta_time.last_delta_time_sec / 1000.0f32);
						}
					}
					_ => {}
				}
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

				input_state.current_mouse_state.set_key(*mouse_code as u8, absolute_current_time_stamp_ms);
			}
			WindowEvent::MouseRelease(mouse_code, x, y) =>
			{
				input_state.current_mouse_state.unset_key(*mouse_code as u8);
			}
			WindowEvent::KeyPress(key_code) => 
			{
				input_state.current_keyboard_state.set_key(*key_code as u8, absolute_current_time_stamp_ms);

				match key_code 
				{
					KeyCode::Escape => 
					{ 
						decs.modify_global_storage::<GlobalVariables>(
							|main_loop| 
							{
								main_loop.should_quit = true;
								Ok(())
							}
						).expect("vk_handle not found");
					}
					KeyCode::ShiftLeft => 
					{ 
						decs.modify_global_storage::<GlobalVariables>(
							|global_variables|
							{
								global_variables.focus_on_gui = !global_variables.focus_on_gui;
								Ok(())
							}
						).unwrap();
					}
					KeyCode::P => 
					{ 
						decs.modify_global_storage::<GlobalVariables>(
							|global_variables|
							{
								global_variables.should_run_physics = !global_variables.should_run_physics;
								Ok(())
							}
						).unwrap();
					}
					KeyCode::X => 
					{
						decs.modify_global_storage::<GlobalVariables>(
							|global_variables|
							{
								global_variables.render_wireframe = !global_variables.render_wireframe;
								Ok(())
							}
						).unwrap();
					}
					_ => {}
				}
			}
			WindowEvent::KeyRelease(key_code) => 
			{
				input_state.current_keyboard_state.unset_key(*key_code as u8);
			}
			WindowEvent::WindowAction(val) => 
			{
				match val
				{
					WindowActions::Close => 
					{ 
						decs.modify_global_storage::<GlobalVariables>(
							|main_loop_component|
							{
								main_loop_component.should_quit = true;
								Ok(())
							}
						).unwrap();
					}
					// WindowActions::FocusIn => 
					// { 
					// 	// window.is_focused = true; 
					// 	window.lock_pointer(); 
					// 	window.hide_cursor();
					// }
					// WindowActions::FocusOut => 
					// { 
					// 	// window.is_focused = false; 
					// 	window.unlock_pointer(); 
					// 	window.show_cursor(); 

					// 	window.show_pointer(active)
					// }

					// assuming x and y are delta from the center (will think of something better later)
					WindowActions::Motion{x, y} => 
					{
						// println!("cursor is at : {} {}", x, y);

						// vk_handle.camera.process_mouse_movement(x as f32 * delta_time_ms, y as f32 * delta_time_ms);

						let (width, height) = window.get_size();
						// let (width, height) = (800, 600);
						let mid_x = (width / 2) as i32;
						let mid_y = (height / 2) as i32;

						let delta_x = x - mid_x;
						let delta_y = -(y - mid_y);

						// if window.is_focused &&
						if
						(delta_x != 0 ||
						delta_y != 0)
						{
							window.center_pointer(true);
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