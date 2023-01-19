#![allow(dead_code)]
#![allow(unused_variables)]

use crate::vulkan::vk_bindgen::*;
use crate::loseit::xcb_bindgen::*;
use crate::loseit::xcb_vk_bindgen::*;
use crate::ffi::strings::*;
use crate::ffi::cstdlib_bindgen::*;

use std::ptr::null_mut as nullptr;

pub unsafe fn get_xcb_presentation_support_function(instance: &VkInstance) -> Option<unsafe extern "C" fn(*mut VkPhysicalDevice_T, u32, *mut xcb_connection_t, u32) -> u32>
{
	std::mem::transmute::<_, PFN_vkGetPhysicalDeviceXcbPresentationSupportKHR>(vkGetInstanceProcAddr(*instance, "vkGetPhysicalDeviceXcbPresentationSupportKHR\0".as_ptr() as _))
}

pub unsafe fn create_xcb_surface_function(instance: &VkInstance) -> Option<unsafe extern "C" fn(instance: VkInstance, pCreateInfo: *const VkXcbSurfaceCreateInfoKHR, pAllocator: *const VkAllocationCallbacks, pSurface: *mut VkSurfaceKHR) -> VkResult>
{
	std::mem::transmute::<_, PFN_vkCreateXcbSurfaceKHR>(vkGetInstanceProcAddr(*instance, "vkCreateXcbSurfaceKHR\0".as_ptr() as _))
}

pub unsafe fn get_atom(conn: *mut xcb_connection_t, name: &'static str) -> xcb_atom_t
{
	let atom: xcb_atom_t;

	let cookie = xcb_intern_atom(conn, 0, name.bytes().len() as u16 - 1, to_c_string(name));
	let reply = xcb_intern_atom_reply(conn, cookie, nullptr());

	let atom = 
		match reply.as_mut()
		{
			Some(reply) => { reply.atom }
			None => { XCB_NONE }
		};

	// free xD
	free(reply as _);

	return atom;
}