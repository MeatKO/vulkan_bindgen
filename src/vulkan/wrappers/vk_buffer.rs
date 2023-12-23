use crate::vulkan::{vk_bindgen::{VkBuffer, VkDeviceMemory, VkDeviceSize, VkBufferUsageFlags, VkMemoryPropertyFlags, vkDestroyBuffer, vkFreeMemory}, handle::VkHandle, vk_buffer::create_buffer};

use std::ptr::null_mut as nullptr;

pub struct VulkanBuffer
{
	pub buffer: VkBuffer,
	pub memory: VkDeviceMemory,
}

impl VulkanBuffer
{
	pub fn new(
		vk_handle: &VkHandle,
		byte_size: VkDeviceSize, 
		buffer_usage_flags: VkBufferUsageFlags, 
		properties: VkMemoryPropertyFlags,
	) -> Result<VulkanBuffer, String>
	{
		unsafe
		{
			let (buffer, memory) = create_buffer(vk_handle, byte_size, buffer_usage_flags, properties)?;
			Ok(VulkanBuffer{buffer, memory})
		}
	}

	pub fn new_empty() -> VulkanBuffer
	{
		VulkanBuffer
		{
			buffer: 0 as VkBuffer,
			memory: 0 as VkDeviceMemory,
		}
	}

	pub unsafe fn destroy(&self, vk_handle: &VkHandle)
	{
		vkDestroyBuffer(vk_handle.logical_device, self.buffer, nullptr());
		vkFreeMemory(vk_handle.logical_device, self.memory, nullptr());
	}
}