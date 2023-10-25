use std::marker::PhantomData;

use crate::vulkan::vk_bindgen::{
	VkCommandBuffer, VkCommandBufferAllocateInfo, VkStructureType, VkCommandBufferLevel, vkAllocateCommandBuffers, VkDevice, VkResult,
};

use std::ptr::null_mut as nullptr;

use super::vk_command_pool::CommandPool;

pub struct CommandBufferAllocateInfo<'a>
{
	allocate_info: VkCommandBufferAllocateInfo,
	_lifetime_marker: PhantomData<&'a ()>
}

impl<'a> CommandBufferAllocateInfo<'a>
{
	pub fn new() -> CommandBufferAllocateInfo<'a>
	{
		CommandBufferAllocateInfo { 
			allocate_info: 
				VkCommandBufferAllocateInfo{
					sType: VkStructureType::VK_STRUCTURE_TYPE_COMMAND_BUFFER_ALLOCATE_INFO,
					commandPool: nullptr(),
					level: VkCommandBufferLevel::VK_COMMAND_BUFFER_LEVEL_PRIMARY,
					commandBufferCount: 1u32,
					pNext: nullptr(),
				},
			_lifetime_marker: PhantomData
		}
	}

	pub fn with_command_pool(mut self, in_command_pool: &CommandPool) -> CommandBufferAllocateInfo<'a>
	{
		self.allocate_info.commandPool = in_command_pool.get_command_pool_ptr();
		self
	}

	pub fn with_level(mut self, in_level: VkCommandBufferLevel) -> CommandBufferAllocateInfo<'a>
	{
		self.allocate_info.level = in_level;
		self
	}

	pub fn with_count(mut self, in_command_buffer_count: u32) -> CommandBufferAllocateInfo<'a>
	{
		self.allocate_info.commandBufferCount = in_command_buffer_count;
		self
	}

	pub fn build<'b>(self, logical_device: &VkDevice) -> Result<Vec<CommandBuffer<'b>>, String>
	{
		unsafe
		{
			if self.allocate_info.commandPool == nullptr()
			{
				return Err("CommandBufferAllocateInfo -> build() : allocate_info -> commandPool cannot be nullptr".into())
			}

			if self.allocate_info.commandBufferCount == 0u32
			{
				return Err("CommandBufferAllocateInfo -> build() : allocate_info -> commandBufferCount cannot be 0".into())
			}
	
			let mut command_buffer_ptr_vec = vec![nullptr(); self.allocate_info.commandBufferCount as usize];
			match vkAllocateCommandBuffers(*logical_device, &self.allocate_info, command_buffer_ptr_vec.as_mut_ptr())
			{
				VkResult::VK_SUCCESS => 
				{ 
					let mut out_command_buffer_vec = vec![];
					for command_buffer_ptr in command_buffer_ptr_vec.into_iter()
					{
						out_command_buffer_vec.push(
							CommandBuffer {
								command_buffer_ptr: command_buffer_ptr,
								_lifetime_marker: PhantomData,
							}
						)
					}
					Ok(out_command_buffer_vec)
				}
				err => 
				{ 
					Err(format!("CommandBufferAllocateInfo -> build() : vkAllocateCommandBuffers() failed with code {:?}", err))
				}
			}
		}
	}
}

pub struct CommandBuffer<'a>
{
	command_buffer_ptr: VkCommandBuffer,
	_lifetime_marker: PhantomData<&'a ()>
}

impl<'a> CommandBuffer<'a>
{
	pub fn get_command_buffer_ptr(&self) -> VkCommandBuffer
	{
		self.command_buffer_ptr
	}

	pub fn reset(&mut self)
	{

	}
}
