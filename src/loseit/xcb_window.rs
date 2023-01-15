use crate::vulkan::vk_bindgen::*;
use crate::loseit::xcb_bindgen::*;
use crate::loseit::xcb_functions::*;
use crate::loseit::window_traits::*;

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

impl VulkanWindowHandle for XcbHandle
{
	fn new(window_title: Option<String>, width: u32, height: u32) -> Option<Self>
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
			xcb_event_mask_t_XCB_EVENT_MASK_EXPOSURE |
			xcb_event_mask_t_XCB_EVENT_MASK_STRUCTURE_NOTIFY |
			xcb_event_mask_t_XCB_EVENT_MASK_KEY_PRESS
		];
		
		unsafe
		{
			handle.xcb_conn = xcb_connect(nullptr(), nullptr());

			if xcb_connection_has_error(handle.xcb_conn) != 0
			{
				// return Err("couldn't establish XCB connection.".to_owned());
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
				xcb_window_class_t_XCB_WINDOW_CLASS_INPUT_OUTPUT as u16,
				(*iterator.data).root_visual,
				xcb_cw_t_XCB_CW_EVENT_MASK, 
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
				xcb_prop_mode_t_XCB_PROP_MODE_REPLACE as u8,
				handle.xcb_window,
				handle.atom_wm_protocols,
				xcb_atom_enum_t_XCB_ATOM_ATOM,
				32,
				1, 
				(&mut handle.atom_wm_delete_window as *const xcb_atom_t) as _
			);

			let title = 
				match window_title
				{
					Some(title) => { title }
					None => { DEFAULT_WINDOW_NAME.to_owned() }
				};

			xcb_change_property(
				handle.xcb_conn,
				xcb_prop_mode_t_XCB_PROP_MODE_REPLACE as u8,
				handle.xcb_window,
				get_atom(handle.xcb_conn, "_NET_WM_NAME\0"),
				get_atom(handle.xcb_conn, "UTF8_STRING\0"),
				8, // sizeof(char), // also holy shit what the fuck we count the bits here ?? xd
				title.bytes().len() as u32, 
				title.as_ptr() as _
			);

			xcb_map_window(handle.xcb_conn, handle.xcb_window);
			xcb_flush(handle.xcb_conn);
		}

		Some(handle)
	}
}