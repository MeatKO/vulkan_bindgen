use crate::vulkan::vk_bindgen::*;
use std::ffi::c_void;
use std::ffi::CStr;

// left in an unfinished state
pub unsafe extern "C" fn debug_callback(
	message_severity: VkDebugUtilsMessageSeverityFlagBitsEXT,
	_message_type: VkDebugUtilsMessageTypeFlagsEXT,
	p_callback_data: *const VkDebugUtilsMessengerCallbackDataEXT,
	_p_user_data: *mut c_void
) -> VkBool32
{
	// these colors are for *NIX terminals
	// they also work under windows in the vscode terminal
	match message_severity 
	{
		VkDebugUtilsMessageSeverityFlagBitsEXT::VK_DEBUG_UTILS_MESSAGE_SEVERITY_INFO_BIT_EXT => { print!("\n\x1B[36m");} // cyan
		VkDebugUtilsMessageSeverityFlagBitsEXT::VK_DEBUG_UTILS_MESSAGE_SEVERITY_WARNING_BIT_EXT => { print!("\n\x1B[33m"); } // yellow
		// VkDebugUtilsMessageSeverityFlagBitsEXT::VK_DEBUG_UTILS_MESSAGE_SEVERITY_VERBOSE_BIT_EXT => { print!("\n\x1B[37m"); } // white
		VkDebugUtilsMessageSeverityFlagBitsEXT::VK_DEBUG_UTILS_MESSAGE_SEVERITY_VERBOSE_BIT_EXT => { return VK_FALSE }
		VkDebugUtilsMessageSeverityFlagBitsEXT::VK_DEBUG_UTILS_MESSAGE_SEVERITY_ERROR_BIT_EXT => { print!("\n\x1B[31m"); } // red
		_ => {}
	}
	
	println!("Vk Validation Layer : '{}' \x1B[0m", CStr::from_ptr((*p_callback_data).pMessage).to_str().unwrap());
	// panic!("Vk Validation Layer : '{}' \x1B[0m", CStr::from_ptr((*p_callback_data).pMessage).to_str().unwrap());

	VK_FALSE
}

pub unsafe fn create_debug_utils_messenger_ext(
	instance: &VkInstance,
	p_create_info: *const VkDebugUtilsMessengerCreateInfoEXT,
	p_allocator: *const VkAllocationCallbacks,
	p_debug_messenger: *mut VkDebugUtilsMessengerEXT
) -> VkResult
{
	let function = std::mem::transmute::<_, PFN_vkCreateDebugUtilsMessengerEXT>(vkGetInstanceProcAddr(*instance, "vkCreateDebugUtilsMessengerEXT\0".as_ptr() as _));

	match function
	{
		Some(func) => 
		{
			return func(*instance, p_create_info, p_allocator, p_debug_messenger);
		}
		None => 
		{
			return VkResult::VK_ERROR_EXTENSION_NOT_PRESENT;
		}
	}
}

pub unsafe fn destroy_debug_utils_messenger_ext(
	instance: &VkInstance,
	debug_messenger: &VkDebugUtilsMessengerEXT,
	p_allocator: *const VkAllocationCallbacks
) -> VkResult
{
	let function = std::mem::transmute::<_, PFN_vkDestroyDebugUtilsMessengerEXT>(vkGetInstanceProcAddr(*instance, "vkDestroyDebugUtilsMessengerEXT\0".as_ptr() as _));

	match function
	{
		Some(func) => 
		{
			func(*instance, *debug_messenger, p_allocator);
			return VkResult::VK_SUCCESS;
		}
		None => 
		{
			return VkResult::VK_ERROR_EXTENSION_NOT_PRESENT;
		}
	}
}