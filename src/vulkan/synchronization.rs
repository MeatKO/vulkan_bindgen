use crate::vulkan::vk_bindgen::*;
use crate::vulkan::handle::VkHandle;
use std::ptr::null_mut as nullptr;

pub unsafe fn create_synchronization_structures(vk_handle: &mut VkHandle)
{
	let semaphore_create_info = 
		VkSemaphoreCreateInfo{
			sType: VkStructureType::VK_STRUCTURE_TYPE_SEMAPHORE_CREATE_INFO,
			flags: 0,	
			pNext: nullptr(),
		};

	let fence_create_info = 
		VkFenceCreateInfo{
			sType: VkStructureType::VK_STRUCTURE_TYPE_FENCE_CREATE_INFO,
			flags: VkFenceCreateFlagBits::VK_FENCE_CREATE_SIGNALED_BIT as u32,	
			pNext: nullptr(),
		};

	vk_handle.image_available_semaphore_vec.resize(vk_handle.frames_in_flight, nullptr());
	vk_handle.rendering_finished_semaphore_vec.resize(vk_handle.frames_in_flight, nullptr());
	vk_handle.in_flight_fence_vec.resize(vk_handle.frames_in_flight, nullptr());

	for i in 0..vk_handle.frames_in_flight
	{
		match vkCreateSemaphore(vk_handle.logical_device, &semaphore_create_info, nullptr(), &mut vk_handle.image_available_semaphore_vec[i])
		{
			VkResult::VK_SUCCESS => { println!("✔️ vkCreateSemaphore()"); }
			err => { panic!("✗ vkCreateSemaphore() failed with code {:?}.", err); }
		}	
		match vkCreateSemaphore(vk_handle.logical_device, &semaphore_create_info, nullptr(), &mut vk_handle.rendering_finished_semaphore_vec[i])
		{
			VkResult::VK_SUCCESS => { println!("✔️ vkCreateSemaphore()"); }
			err => { panic!("✗ vkCreateSemaphore() failed with code {:?}.", err); }
		}
		match vkCreateFence(vk_handle.logical_device, &fence_create_info, nullptr(), &mut vk_handle.in_flight_fence_vec[i])
		{
			VkResult::VK_SUCCESS => { println!("✔️ vkCreateFence()"); }
			err => { panic!("✗ vkCreateFence() failed with code {:?}.", err); }
		}
	}

}