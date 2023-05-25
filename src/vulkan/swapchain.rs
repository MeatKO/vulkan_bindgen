use crate::vulkan::vk_bindgen::*;
use crate::vulkan::handle::VkHandle;
use crate::vulkan::framebuffer::*;
use crate::vulkan::texture_view::*;

use std::ptr::null_mut as nullptr;
use std::cmp::min;

use super::depth_buffer::create_depth_buffer;

pub struct SwapchainSupportDetails 
{
	pub capabilities: VkSurfaceCapabilitiesKHR,
	pub formats: Vec<VkSurfaceFormatKHR>,
	pub present_modes: Vec<VkPresentModeKHR>
}

pub unsafe fn recreate_swapchain(vk_handle: &mut VkHandle)
{
	vkDeviceWaitIdle(vk_handle.logical_device);
	cleanup_swapchain(vk_handle);
	create_swapchain(vk_handle);
	create_swapchain_image_views(vk_handle);
	create_depth_buffer(vk_handle);
	create_framebuffer(vk_handle);
}

pub unsafe fn cleanup_swapchain(vk_handle: &VkHandle)
{
	vkDestroyImageView(vk_handle.logical_device, vk_handle.depth_image_view, nullptr());
	vkDestroyImage(vk_handle.logical_device, vk_handle.depth_image, nullptr());
	vkFreeMemory(vk_handle.logical_device, vk_handle.depth_image_memory, nullptr());

	for framebuffer in vk_handle.swapchain_framebuffers.iter()
	{
		vkDestroyFramebuffer(vk_handle.logical_device, *framebuffer, nullptr());
	}
	for image_view in vk_handle.swapchain_image_views_vec.iter()
	{
		vkDestroyImageView(vk_handle.logical_device, *image_view, nullptr());
	}
	vkDestroySwapchainKHR(vk_handle.logical_device, vk_handle.swapchain, nullptr());
}

pub unsafe fn create_swapchain(vk_handle: &mut VkHandle)
{
	vk_handle.swapchain_support_details = query_swapchain_support(vk_handle.physical_device, vk_handle.window_surface);
	
	println!("{:?}",vk_handle.swapchain_support_details.formats);
	println!("{:?}",vk_handle.swapchain_support_details.present_modes);
	// panic!();

	// Swapchain creation
	vk_handle.surface_format = choose_swap_surface_format(&vk_handle.swapchain_support_details.formats).expect("Couldn't find suitable window surface format.");
	vk_handle.present_mode = choose_swap_present_mode(&vk_handle.swapchain_support_details.present_modes);
	vk_handle.swapchain_extent = choose_swap_extent(&vk_handle.swapchain_support_details.capabilities);

	let image_count =
		min(
			vk_handle.swapchain_support_details.capabilities.minImageCount + 1, 
			vk_handle.swapchain_support_details.capabilities.maxImageCount
		);

	// queue stuff for the swapchain creation : 
	vk_handle.queue_family_indices = 
		vec![
			vk_handle.queue_handle.graphics_queue.as_ref().unwrap().family_index, 
			vk_handle.queue_handle.presentation_queue.as_ref().unwrap().family_index
		];

	let mut swapchain_create_info = VkSwapchainCreateInfoKHR{
		sType: VkStructureType::VK_STRUCTURE_TYPE_SWAPCHAIN_CREATE_INFO_KHR,
		surface: vk_handle.window_surface,
		minImageCount: image_count,
		imageFormat: vk_handle.surface_format.format,
		imageColorSpace: vk_handle.surface_format.colorSpace,
		imageExtent: vk_handle.swapchain_extent,
		imageArrayLayers: 1,
		imageSharingMode: VkSharingMode::VK_SHARING_MODE_EXCLUSIVE,
		queueFamilyIndexCount: 0,
		pQueueFamilyIndices: nullptr(),
		imageUsage: VkImageUsageFlagBits::VK_IMAGE_USAGE_COLOR_ATTACHMENT_BIT as u32,
		preTransform: vk_handle.swapchain_support_details.capabilities.currentTransform,
		compositeAlpha: VkCompositeAlphaFlagBitsKHR::VK_COMPOSITE_ALPHA_OPAQUE_BIT_KHR,
		presentMode: vk_handle.present_mode,
		clipped: VK_TRUE,
		oldSwapchain: nullptr(),
		flags: 0,
		pNext: nullptr(),
	};
	if vk_handle.graphics_queue != vk_handle.presentation_queue
	{
		swapchain_create_info.imageSharingMode = VkSharingMode::VK_SHARING_MODE_CONCURRENT;
		swapchain_create_info.queueFamilyIndexCount = 2;
		swapchain_create_info.pQueueFamilyIndices = vk_handle.queue_family_indices.as_ptr();
	}

	match vkCreateSwapchainKHR(vk_handle.logical_device, &swapchain_create_info, nullptr(), &mut vk_handle.swapchain)
	{
		VkResult::VK_SUCCESS => { println!("✔️ vkCreateSwapchainKHR()"); }
		err => { panic!("✗ vkCreateSwapchainKHR() failed with code {:?}.", err); }
	}
}

pub unsafe fn create_swapchain_image_views(vk_handle: &mut VkHandle)
{
	// Swapchain images
	let mut swapchain_images_count = 0u32;
	vkGetSwapchainImagesKHR(vk_handle.logical_device, vk_handle.swapchain, &mut swapchain_images_count, nullptr());
	let mut swapchain_images_vec = vec![ std::mem::zeroed(); swapchain_images_count as usize ];
	vkGetSwapchainImagesKHR(vk_handle.logical_device, vk_handle.swapchain, &mut swapchain_images_count, swapchain_images_vec.as_mut_ptr());

	// Swapchain image views
	vk_handle.swapchain_image_views_vec = vec![nullptr(); swapchain_images_count as usize];

	for i in 0..swapchain_images_vec.len()
	{
		vk_handle.swapchain_image_views_vec[i] = 
			create_image_view(
				vk_handle, 
				swapchain_images_vec[i],
				vk_handle.surface_format.format, 
				VkImageAspectFlagBits::VK_IMAGE_ASPECT_COLOR_BIT as u32,
			);
	}
}

// this mf-er segfaults
// at some point format_count becomes 0
// EDIT : I was destroying the window surface alongside the window handle, fixed
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

	println!("---------------------------------------------");
	println!("{:?}", details.formats);
	println!("{:?}", details.present_modes);
	// panic!();

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
	// return VkPresentModeKHR::VK_PRESENT_MODE_FIFO_KHR;
	// return VkPresentModeKHR::VK_PRESENT_MODE_MAILBOX_KHR;
	// return VkPresentModeKHR::VK_PRESENT_MODE_FIFO_RELAXED_KHR;
	// return VkPresentModeKHR::VK_PRESENT_MODE_IMMEDIATE_KHR;

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