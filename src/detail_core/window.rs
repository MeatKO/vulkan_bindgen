use parmack::window::window_handle::WindowHandle;
use parmack::handle::linux_handle::LinuxHandle;

use crate::vulkan::vk_bindgen::*;
use crate::vulkan::handle::*;

use crate::loseit::xcb_vk_bindgen::VkXcbSurfaceCreateInfoKHR;
use crate::loseit::xcb_functions::*;

use std::ptr::null_mut as nullptr;

#[cfg(target_os = "linux")]
pub fn create_vulkan_surface(window: &mut WindowHandle, vk_handle: &mut VkHandle) -> Result<*mut VkSurfaceKHR_T, String>
{
	let handle = window as &mut LinuxHandle;

	let mut surface: *mut VkSurfaceKHR_T = nullptr();

	unsafe
	{
		match create_xcb_surface_function(&vk_handle.instance)
		{
			None => { Err("couldn't find vkCreateXcbSurfaceKHR() function.".to_owned()) }
			Some(function) => 
			{
				let surface_create_info = VkXcbSurfaceCreateInfoKHR {
					sType: VkStructureType::VK_STRUCTURE_TYPE_XCB_SURFACE_CREATE_INFO_KHR,
					connection: handle.xcb_conn,
					window: handle.xcb_window,
					flags: 0,
					pNext: nullptr()
				};
	
				match function(vk_handle.instance, &surface_create_info, nullptr(), &mut surface)
				{
					VkResult::VK_SUCCESS => { Ok(surface) }
					err => { Err(format!("vulkan is not supported on given X window. vkCreateXcbSurfaceKHR() resulted in {:?}", err)) }
				}
			}
		}
	}
}

#[cfg(target_os = "windows")]
pub fn create_vulkan_surface(window: &mut WindowHandle) -> Option<VkSurfaceKHR_T>
{
	return None;
	
}