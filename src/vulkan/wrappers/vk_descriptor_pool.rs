use crate::vulkan::vk_bindgen::{VkDescriptorType, VkResult, VkDevice, VkStructureType, VkDescriptorPoolSize, VkDescriptorPoolCreateInfo, VkDescriptorPool, vkCreateDescriptorPool};
use std::ptr::null_mut as nullptr;
pub struct VkDescriptorPoolBuilder
{
	pool_size_vec: Vec<VkDescriptorPoolSize>
}

impl VkDescriptorPoolBuilder
{
	pub fn new() -> VkDescriptorPoolBuilder
	{
		VkDescriptorPoolBuilder
		{
			pool_size_vec: Vec::new()
		}
	}

	pub fn add_pool_type(mut self, pool_type: VkDescriptorType, descriptor_count: usize) -> VkDescriptorPoolBuilder
	{
		self.pool_size_vec.push(
			VkDescriptorPoolSize {
				type_: pool_type,
				descriptorCount: descriptor_count as u32
			}
		);
		self
	}

	pub fn build(self, logical_device: VkDevice, max_sets: usize) -> Result<VkDescriptorPool, String>
	{
		let pool_info = 
			VkDescriptorPoolCreateInfo {
				sType: VkStructureType::VK_STRUCTURE_TYPE_DESCRIPTOR_POOL_CREATE_INFO,
				poolSizeCount: self.pool_size_vec.len() as u32,
				pPoolSizes: self.pool_size_vec.as_ptr(),
				flags: 0,
				maxSets: max_sets as u32,
				pNext: nullptr()
			};
	
		let mut descriptor_pool: VkDescriptorPool = nullptr();
		match unsafe { vkCreateDescriptorPool(logical_device, &pool_info, nullptr(), &mut descriptor_pool) }
		{
			VkResult::VK_SUCCESS => 
			{
				Ok(descriptor_pool)
			}
			error_code => 
			{
				Err(
					format!("vkCreateDescriptorPool Failed With Code '{:?}'. logical_device_pointer:{:p} pool_info_pointer:{:p}", 
					error_code, logical_device, &pool_info).to_owned()
				)
			}
		}	
	}
}
