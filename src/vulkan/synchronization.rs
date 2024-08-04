use crate::vulkan::vk_bindgen::*;
use crate::vulkan::handle::VkHandle;
use std::ptr::null_mut as nullptr;

pub unsafe fn create_synchronization_structures(
	// vk_handle: &mut VkHandle,
	device: &VkDevice,
	frames_in_flight_count: usize,
) -> Result<(Vec<VkSemaphore>, Vec<VkSemaphore>, Vec<VkFence>), String>
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

	let mut image_available_semaphore_vec: Vec<VkSemaphore> = vec![];
	let mut rendering_finished_semaphore_vec: Vec<VkSemaphore> = vec![];
	let mut in_flight_fence_vec: Vec<VkFence> = vec![];

	// vk_handle.image_available_semaphore_vec.resize(vk_handle.frames_in_flight, nullptr());
	// vk_handle.rendering_finished_semaphore_vec.resize(vk_handle.frames_in_flight, nullptr());
	// vk_handle.in_flight_fence_vec.resize(vk_handle.frames_in_flight, nullptr());
	image_available_semaphore_vec.resize(frames_in_flight_count, nullptr());
	rendering_finished_semaphore_vec.resize(frames_in_flight_count, nullptr());
	in_flight_fence_vec.resize(frames_in_flight_count, nullptr());

	for i in 0..frames_in_flight_count
	{
		match vkCreateSemaphore(*device, &semaphore_create_info, nullptr(), &mut image_available_semaphore_vec[i])
		{
			VkResult::VK_SUCCESS => { println!("✔️ vkCreateSemaphore()"); }
			err => { panic!("✗ vkCreateSemaphore() failed with code {:?}.", err); }
		}	
		match vkCreateSemaphore(*device, &semaphore_create_info, nullptr(), &mut rendering_finished_semaphore_vec[i])
		{
			VkResult::VK_SUCCESS => { println!("✔️ vkCreateSemaphore()"); }
			err => { panic!("✗ vkCreateSemaphore() failed with code {:?}.", err); }
		}
		match vkCreateFence(*device, &fence_create_info, nullptr(), &mut in_flight_fence_vec[i])
		{
			VkResult::VK_SUCCESS => { println!("✔️ vkCreateFence()"); }
			err => { panic!("✗ vkCreateFence() failed with code {:?}.", err); }
		}
	}

	Ok((image_available_semaphore_vec, rendering_finished_semaphore_vec, in_flight_fence_vec))
}