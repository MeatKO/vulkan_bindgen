use crate::vulkan::vk_bindgen::*;
// use crate::vulkan::handle::VkHandle;
use std::ptr::null_mut as nullptr;

pub struct SwapchainSupportDetails 
{
	pub capabilities: VkSurfaceCapabilitiesKHR,
	pub formats: Vec<VkSurfaceFormatKHR>,
	pub present_modes: Vec<VkPresentModeKHR>
}

pub unsafe fn query_swapchain_support(physical_device: VkPhysicalDevice, window_surface: VkSurfaceKHR) -> SwapchainSupportDetails
{
	let mut details = 
		SwapchainSupportDetails {
			capabilities: VkSurfaceCapabilitiesKHR{ ..Default::default() },
			formats: vec![],
			present_modes: vec![]
		};

	vkGetPhysicalDeviceSurfaceCapabilitiesKHR(physical_device, window_surface, &mut details.capabilities);

	let mut format_count = 0u32;
	vkGetPhysicalDeviceSurfaceFormatsKHR(physical_device, window_surface, &mut format_count, nullptr());
	details.formats = vec![ std::mem::zeroed(); format_count as usize ];
	vkGetPhysicalDeviceSurfaceFormatsKHR(physical_device, window_surface, &mut format_count, details.formats.as_mut_ptr());
	
	let mut present_mode_count = 0u32;
	vkGetPhysicalDeviceSurfacePresentModesKHR(physical_device, window_surface, &mut present_mode_count, nullptr());
	details.present_modes = vec![ std::mem::zeroed(); format_count as usize ];
	vkGetPhysicalDeviceSurfacePresentModesKHR(physical_device, window_surface, &mut present_mode_count, details.present_modes.as_mut_ptr());

	details
}

pub fn choose_swap_surface_format(available_formats: &Vec<VkSurfaceFormatKHR>) -> Option<VkSurfaceFormatKHR>
{
	if available_formats.len() == 0
	{
		return None
	}

	match 
		available_formats.iter()
		.copied()
		.filter(|format|
			format.format == VkFormat::VK_FORMAT_B8G8R8A8_SRGB && 
			format.colorSpace == VkColorSpaceKHR::VK_COLOR_SPACE_SRGB_NONLINEAR_KHR
		)
		.last()
	{
		Some(format) => { Some(format) }
		None => { Some(available_formats[0]) }
	}
}

// Panics... well not always I guess ?... Sometimes
pub fn choose_swap_present_mode(available_present_modes: &Vec<VkPresentModeKHR>) -> VkPresentModeKHR
{
	if available_present_modes.len() == 0
	{
		panic!("Empty available_present_modes vec!\nThe Vulkan specs state that VK_PRESENT_MODE_FIFO_KHR is always available.");
	}

	let preferred_mode = VkPresentModeKHR::VK_PRESENT_MODE_MAILBOX_KHR;
	if available_present_modes.contains(&preferred_mode)
	{
		return preferred_mode
	}

	VkPresentModeKHR::VK_PRESENT_MODE_FIFO_KHR
}

// this is kinda useless since the Vulkan Tutorial https://vulkan-tutorial.com/ uses GLFW and we don't...
pub fn choose_swap_extent(capabilities: &VkSurfaceCapabilitiesKHR) -> VkExtent2D
{
	capabilities.currentExtent
}