use crate::exedra::model::Model;
use crate::vulkan::vk_bindgen::*;
use crate::vulkan::handle::*;
use std::ptr::null_mut as nullptr;

pub unsafe fn create_descriptor_pool(
	vk_handle: &mut VkHandle,
	model: &mut Model
)
{
	let pool_sizes = 
		vec![
			VkDescriptorPoolSize {
				type_: VkDescriptorType::VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER,
				descriptorCount: vk_handle.frames_in_flight as u32
			},
			VkDescriptorPoolSize {
				type_: VkDescriptorType::VK_DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER,
				descriptorCount: vk_handle.frames_in_flight as u32
			},
		];
		
	let pool_info = 
		VkDescriptorPoolCreateInfo {
			sType: VkStructureType::VK_STRUCTURE_TYPE_DESCRIPTOR_POOL_CREATE_INFO,
			poolSizeCount: pool_sizes.len() as u32,
			pPoolSizes: pool_sizes.as_ptr(),
			flags: 0,
			maxSets: vk_handle.frames_in_flight as u32,
			pNext: nullptr()
		};
	
	match vkCreateDescriptorPool(vk_handle.logical_device, &pool_info, nullptr(), &mut model.descriptor_pool)
	{
		VkResult::VK_SUCCESS => { println!("✔️ vkCreateDescriptorPool()"); }
		err => { panic!("✗ vkCreateDescriptorPool() failed with code {:?}.", err); }
	}	
}