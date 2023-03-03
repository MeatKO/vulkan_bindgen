use crate::vulkan::vk_bindgen::*;
use crate::vulkan::handle::*;
use std::ptr::null_mut as nullptr;

pub unsafe fn create_descriptor_pool(vk_handle: &mut VkHandle)
{
	let pool_size = 
		VkDescriptorPoolSize {
			type_: VkDescriptorType::VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER,
			descriptorCount: vk_handle.frames_in_flight as u32
		};

	let pool_info = 
		VkDescriptorPoolCreateInfo {
			sType: VkStructureType::VK_STRUCTURE_TYPE_DESCRIPTOR_POOL_CREATE_INFO,
			poolSizeCount: 1,
			pPoolSizes: &pool_size,
			flags: 0,
			maxSets: vk_handle.frames_in_flight as u32,
			pNext: nullptr()
		};
	
	match vkCreateDescriptorPool(vk_handle.logical_device, &pool_info, nullptr(), &mut vk_handle.descriptor_pool)
	{
		VkResult::VK_SUCCESS => { println!("✔️ vkCreateDescriptorPool()"); }
		err => { panic!("✗ vkCreateDescriptorPool() failed with code {:?}.", err); }
	}	
}