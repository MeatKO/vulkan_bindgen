use crate::vulkan::vk_bindgen::*;
use crate::vulkan::handle::*;
use std::ptr::null_mut as nullptr;

pub unsafe fn create_descriptor_pool(
	vk_handle: &VkHandle
) -> Result<VkDescriptorPool, String>
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
	
	let mut descriptor_pool: VkDescriptorPool = nullptr();
	match vkCreateDescriptorPool(vk_handle.logical_device, &pool_info, nullptr(), &mut descriptor_pool)
	{
		VkResult::VK_SUCCESS => 
		{
			Ok(descriptor_pool)
		}
		error_code => 
		{
			Err(
				format!("vkCreateDescriptorPool Failed With Code '{:?}'. logical_device_pointer:{:p} pool_info_pointer:{:p}", 
				error_code, vk_handle.logical_device, &pool_info).to_owned()
			)
		}
	}	
}