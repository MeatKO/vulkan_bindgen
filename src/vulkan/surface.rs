use crate::vulkan::vk_bindgen::*;
use crate::vulkan::handle::VkHandle;
use std::ptr::null_mut as nullptr;

pub fn choose_surface_format(vk_handle: &VkHandle) -> Option<VkFormat>
{
	let mut num_formats = 0u32;
	unsafe
	{
		vkGetPhysicalDeviceSurfaceFormatsKHR(vk_handle.physical_device, vk_handle.window_surface, &mut num_formats, nullptr());
	}

	if num_formats == 0
	{
		return None
	}

	let mut format_vec; 

	unsafe
	{
		format_vec = vec![std::mem::zeroed(); num_formats as usize];
		vkGetPhysicalDeviceSurfaceFormatsKHR(vk_handle.physical_device, vk_handle.window_surface, &mut num_formats, format_vec.as_mut_ptr());
	}	

	for i in 0..format_vec.len()
	{
		match format_vec[i].format
		{
			VkFormat::VK_FORMAT_R8G8B8A8_SRGB |
			VkFormat::VK_FORMAT_B8G8R8A8_SRGB |
			VkFormat::VK_FORMAT_R8G8B8A8_UNORM |
			VkFormat::VK_FORMAT_B8G8R8A8_UNORM => 
			{
				return Some(format_vec[i].format)
			}
			_ => {}
		}
	}

	None
}