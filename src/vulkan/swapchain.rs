use crate::vulkan::vk_bindgen::*;
use crate::vulkan::handle::VkHandle;
use std::ptr::null_mut as nullptr;

pub struct SwapchainSupportDetails 
{
	capabilities: VkSurfaceCapabilitiesKHR,
	formats: Vec<VkSurfaceFormatKHR>,
	present_modes: Vec<VkPresentModeKHR>
}

pub unsafe fn query_swapchain_support(vk_handle: &VkHandle) -> SwapchainSupportDetails
{
	let mut details = 
		SwapchainSupportDetails {
			// capabilities: std::mem::zeroed(),
			capabilities: VkSurfaceCapabilitiesKHR{},
			formats: vec![],
			present_modes: vec![]
		};

	vkGetPhysicalDeviceSurfaceCapabilitiesKHR(vk_handle.physical_device, vk_handle.window_surface, &mut details.capabilities);

	let mut format_count = 0u32;
	vkGetPhysicalDeviceSurfaceFormatsKHR(vk_handle.physical_device, vk_handle.window_surface, &mut format_count, nullptr());
	details.formats = vec![ std::mem::zeroed(); format_count as usize ];
	vkGetPhysicalDeviceSurfaceFormatsKHR(vk_handle.physical_device, vk_handle.window_surface, &mut format_count, details.formats.as_mut_ptr());
	
	let mut present_mode_count = 0u32;
	vkGetPhysicalDeviceSurfacePresentModesKHR(vk_handle.physical_device, vk_handle.window_surface, &mut present_mode_count, nullptr());
	details.present_modes = vec![ std::mem::zeroed(); format_count as usize ];
	vkGetPhysicalDeviceSurfacePresentModesKHR(vk_handle.physical_device, vk_handle.window_surface, &mut present_mode_count, details.present_modes.as_mut_ptr());

	details
}