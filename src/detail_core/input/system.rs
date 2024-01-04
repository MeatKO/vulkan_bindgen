use std::borrow::BorrowMut;

use decs::component_derive::system;
use decs::manager::dECS;
use parmack::handle::Handle;
use parmack::window::event::{WindowEvent, WindowActions, KeyCode, MouseCode};

use crate::detail_core::components::misc::{WindowComponent, DeltaTime, MainLoopComponent, GlobalVariables, CameraRaycastObject, CameraRaycastObjectState};
use crate::vulkan::handle::VkHandle;

#[system]
pub fn input_system()
{
	let window: &mut WindowComponent =
		unsafe { decs.get_components_global_mut_unchecked::<WindowComponent>() }.unwrap().remove(0).component;

	let delta_time: &mut DeltaTime =
		unsafe { decs.get_components_global_mut_unchecked::<DeltaTime>() }.unwrap().remove(0).component;

	let vk_handle: &mut VkHandle =
		unsafe { decs.get_components_global_mut_unchecked::<VkHandle>() }.unwrap().remove(0).component;

	let raycast_object: &mut CameraRaycastObject =
		unsafe { decs.get_components_global_mut_unchecked::<CameraRaycastObject>() }.unwrap().remove(0).component;

	let process_start_time = std::time::Instant::now();
	let absolute_current_time_stamp_ms = process_start_time.duration_since(vk_handle.start_time).as_secs_f32() * 1000.0f32;

	let window = &mut window.window;

	for event in window.get_events()
	{
		match event
		{
			WindowEvent::MousePress(mouse_code, x, y) =>
			{
				match raycast_object.state.borrow_mut()
				{
					CameraRaycastObjectState::Picked(raycast_info) =>
					{
						if mouse_code == MouseCode::ScrollDown
						{
							raycast_info.length -= 1000.0f32 * (delta_time.last_delta_time / 1000.0f32);
						}
						if mouse_code == MouseCode::ScrollUp
						{
							raycast_info.length += 1000.0f32 * (delta_time.last_delta_time / 1000.0f32);
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

				vk_handle.mouse_input_buffer.set_key(mouse_code as u8, absolute_current_time_stamp_ms);
			}
			WindowEvent::MouseRelease(mouse_code, x, y) =>
			{
				vk_handle.mouse_input_buffer.unset_key(mouse_code as u8);
			}
			WindowEvent::KeyPress(key_code) => 
			{
				vk_handle.input_buffer.set_key(key_code as u8, absolute_current_time_stamp_ms);

				match key_code 
				{
					KeyCode::Escape => 
					{ 
						decs.modify_components_global::<MainLoopComponent>(
							|main_loop| 
							{
								main_loop.should_quit = true;
								Ok(())
							}
						).expect("vk_handle not found");
					}
					KeyCode::ShiftLeft => 
					{ 
						decs.modify_components_global::<GlobalVariables>(
							|global_variables|
							{
								global_variables.focus_on_gui = !global_variables.focus_on_gui;
								Ok(())
							}
						).unwrap();
					}
					KeyCode::V => 
					{ 
						decs.modify_components_global::<GlobalVariables>(
							|global_variables|
							{
								global_variables.should_run_physics = !global_variables.should_run_physics;
								Ok(())
							}
						).unwrap();
					}
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
					WindowActions::Close => 
					{ 
						decs.modify_components_global::<MainLoopComponent>(
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