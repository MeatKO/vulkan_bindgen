use crate::vulkan::{
	vk_bindgen::*,
	handle::VkHandle,
	// surface::choose_surface_format,
	extension::get_missing_extensions
};

use crate::loseit::{
	xcb_bindgen::*,
	xcb_vk_bindgen::*,
	xcb_functions::*,
	xcb_events::*,
	window_traits::*,
	window_events::WindowEvent,
};

use std::ptr::null_mut as nullptr;

const DEFAULT_WINDOW_NAME: &str = "Vulkan Window";

#[derive(Clone)]
pub struct XcbHandle
{
	xcb_conn: *mut xcb_connection_t,
	xcb_window: xcb_window_t,
	atom_wm_protocols: xcb_atom_t,
	atom_wm_delete_window: xcb_atom_t
}

impl Drop for XcbHandle
{
	fn drop(&mut self)
	{
		self.destroy()
	}
}

impl VulkanWindowHandle for XcbHandle
{
	fn new(window_title: &Option<String>, width: u32, height: u32, vk_handle: &mut VkHandle) -> Option<Self>
	{
		let mut handle = 
			XcbHandle
			{
				xcb_conn: nullptr(),
				xcb_window: 0,
				atom_wm_protocols: 0,
				atom_wm_delete_window: 0
			};

		let window_values: [u32; 1] = [
			xcb_event_mask_t::XCB_EVENT_MASK_EXPOSURE as u32 |
			xcb_event_mask_t::XCB_EVENT_MASK_STRUCTURE_NOTIFY as u32 |
			xcb_event_mask_t::XCB_EVENT_MASK_KEY_PRESS as u32 
		];
		
		unsafe
		{
			handle.xcb_conn = xcb_connect(nullptr(), nullptr());

			if xcb_connection_has_error(handle.xcb_conn) != 0
			{
				return None
			}
	
			handle.xcb_window = xcb_generate_id(handle.xcb_conn);

			let iterator = xcb_setup_roots_iterator(xcb_get_setup(handle.xcb_conn));

			xcb_create_window(
				handle.xcb_conn,
				XCB_COPY_FROM_PARENT as u8,
				handle.xcb_window,
				(*iterator.data).root,
				0,
				0,
				width as u16,
				height as u16,
				0,
				xcb_window_class_t::XCB_WINDOW_CLASS_INPUT_OUTPUT as u16,
				(*iterator.data).root_visual,
				xcb_cw_t::XCB_CW_EVENT_MASK as u32, 
				window_values.as_ptr() as _
			);

			handle.atom_wm_protocols = get_atom(handle.xcb_conn, "WM_PROTOCOLS\0");
			if handle.atom_wm_protocols == 0
			{
				panic!("xcb WM_PROTOCOLS atom was not found.")
			}

			handle.atom_wm_delete_window = get_atom(handle.xcb_conn, "WM_DELETE_WINDOW\0");
			if handle.atom_wm_protocols == 0
			{
				panic!("xcb WM_DELETE_WINDOW atom was not found.")
			}

			xcb_change_property(
				handle.xcb_conn,
				xcb_prop_mode_t::XCB_PROP_MODE_REPLACE as u8,
				handle.xcb_window,
				handle.atom_wm_protocols,
				xcb_atom_enum_t::XCB_ATOM_ATOM as u32,
				32,
				1, 
				(&mut handle.atom_wm_delete_window as *const xcb_atom_t) as _
			);

			let title = 
				match window_title
				{
					Some(title) => { title.clone() }
					None => { DEFAULT_WINDOW_NAME.to_owned() }
				};

			xcb_change_property(
				handle.xcb_conn,
				xcb_prop_mode_t::XCB_PROP_MODE_REPLACE as u8,
				handle.xcb_window,
				get_atom(handle.xcb_conn, "_NET_WM_NAME\0"),
				get_atom(handle.xcb_conn, "UTF8_STRING\0"),
				8, // sizeof(char), // also holy shit what the fuck we count the bits here ?? xd // crraawwwling in my skiiiiiiin
				title.bytes().len() as u32, 
				title.as_ptr() as _
			);

			xcb_map_window(handle.xcb_conn, handle.xcb_window);
			xcb_flush(handle.xcb_conn);
		

			let needed_extensions = vec![
				"VK_KHR_xcb_surface",
				"VK_KHR_surface"
			];

			get_missing_extensions(&needed_extensions, &vk_handle.available_extensions);

			match create_xcb_surface_function(&vk_handle.instance)
			{
				None => { panic!("This platform doesn't offer a 'vkCreateXcbSurfaceKHR' function.") }
				Some(function) => 
				{
					let surface_create_info = VkXcbSurfaceCreateInfoKHR {
						sType: VkStructureType::VK_STRUCTURE_TYPE_XCB_SURFACE_CREATE_INFO_KHR,
						connection: handle.xcb_conn,
						window: handle.xcb_window,
						flags: 0,
						pNext: nullptr()
					};

					let result = function(vk_handle.instance, &surface_create_info, nullptr(), &mut vk_handle.window_surface);
					match result
					{
						VkResult::VK_SUCCESS => {}
						res => { panic!("Vulkan is not supported on given X window. vkCreateXcbSurfaceKHR() resulted in {:?}", res) }
					}
				}
			}
		}

		Some(handle)
	}

	fn get_event(&self) -> Option<WindowEvent> 
	{
		// None



		unsafe 
		{
			if xcb_connection_has_error(self.xcb_conn) > 0
			{
				panic!("XCB connection just broke :( sad face");
			}

			// xcb_flush(self.xcb_conn);

			// return None;


			let event = xcb_poll_for_event(self.xcb_conn).as_mut()?;
			// let event = xcb_wait_for_event(self.xcb_conn).as_mut()?;

			// return None;

			println!("XCB Event ptr : {:?}", event as *mut xcb_generic_event_t);
			println!("XCB Event : {:?}", event);

			let res = xcb_connection_has_error(self.xcb_conn);
			println!(">>>>>>>>>>XCB Error : {:?}", res);

			// schedule repaint
			// let client_message = 
			// 	xcb_client_message_event_t{
			// 		response_type: 33,
			// 		format: 32,
			// 		window: self.xcb_window,
			// 		sequence: 0,
			// 		type_: 62,
			// 		data: xcb_client_message_data_t{ data8: [0u8; 20usize] }
			// 	};
			// xcb_send_event(self.xcb_conn, 0, self.xcb_window, 0, (&client_message as *const xcb_client_message_event_t) as _ );
			// xcb_flush(self.xcb_conn);

			// we clear the most significant bit of the 8 bit response_type
			// for WHATEVER reason...
			match ((*event).response_type & 0x7F) as u32
			{
				XCB_KEY_PRESS => 
				{
					// this is some major bs
					let key_code = (*((event as *mut xcb_generic_event_t) as *mut xcb_key_press_event_t)).detail;
					return Some(WindowEvent::KeyPress(convert_key_code(key_code)))
				}
				_ => { return None }
			}
		}
	}

	fn destroy(&self)
	{
		unsafe
		{
			println!("destroying XCB window.");
			xcb_destroy_window(self.xcb_conn, self.xcb_window);
		}
	}
}