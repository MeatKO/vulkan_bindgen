use crate::vulkan::vk_bindgen::*;
use crate::vulkan::handle::*;
use std::ptr::null_mut as nullptr;

pub unsafe fn create_command_pool(vk_handle: &mut VkHandle)
{
	let command_pool_create_info = VkCommandPoolCreateInfo{
		sType: VkStructureType::VK_STRUCTURE_TYPE_COMMAND_POOL_CREATE_INFO,
		queueFamilyIndex: vk_handle.queue_family_indices[0],
		flags: VkCommandPoolCreateFlagBits::VK_COMMAND_POOL_CREATE_RESET_COMMAND_BUFFER_BIT as u32,	
		pNext: nullptr(),
	};

	match vkCreateCommandPool(vk_handle.logical_device, &command_pool_create_info, nullptr(), &mut vk_handle.command_pool)
	{
		VkResult::VK_SUCCESS => { println!("✔️ vkCreateCommandPool()"); }
		err => { panic!("✗ vkCreateCommandPool() failed with code {:?}.", err); }
	}	
}