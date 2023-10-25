use crate::vulkan::{vk_bindgen::{VkCommandPoolCreateInfo, VkStructureType, VkCommandPoolCreateFlagBits, VkDevice, vkCreateCommandPool, VkResult, VkCommandPool, VkCommandBuffer, vkDestroyCommandPool}, command_buffer};

use std::ptr::null_mut as nullptr;

use super::vk_command_buffer::{CommandBuffer, CommandBufferAllocateInfo};

pub struct CommandPoolCreateInfo
{
	create_info: VkCommandPoolCreateInfo,
}

impl CommandPoolCreateInfo
{
	pub fn new() -> CommandPoolCreateInfo
	{
		CommandPoolCreateInfo { 
			create_info: 
				VkCommandPoolCreateInfo{
					sType: VkStructureType::VK_STRUCTURE_TYPE_COMMAND_POOL_CREATE_INFO,
					queueFamilyIndex: 0,
					flags: 0,	
					pNext: nullptr(),
				}
		}
	}

	pub fn with_queue_family_index(mut self, in_queue_family_index: u32) -> CommandPoolCreateInfo
	{
		self.create_info.queueFamilyIndex = in_queue_family_index;
		self
	}

	pub fn with_flag(mut self, in_flag: VkCommandPoolCreateFlagBits) -> CommandPoolCreateInfo
	{
		self.create_info.flags |= in_flag as u32;
		self
	}

	pub fn build<'a>(self, logical_device: &VkDevice) -> Result<CommandPool<'a>, String>
	{
		unsafe 
		{
			let mut command_pool_ptr = nullptr();
			match vkCreateCommandPool(*logical_device, &self.create_info, nullptr(), &mut command_pool_ptr)
			{
				VkResult::VK_SUCCESS => 
				{ 
					Ok(
						CommandPool { 
							command_pool_ptr: command_pool_ptr,
							command_buffer_vec: vec![],
						}
					)
				}
				err => 
				{ 
					Err(format!("CommandPoolCreateInfo -> build() : vkCreateCommandPool() failed with code {:?}", err))
				}
			}	
		}
	}
}

pub struct CommandPool<'a>
{
	command_pool_ptr: VkCommandPool,
	command_buffer_vec: Vec<CommandBuffer<'a>>,
}

impl<'a> CommandPool<'a>
{
	pub fn get_command_pool_ptr(&self) -> VkCommandPool
	{
		self.command_pool_ptr
	}
}