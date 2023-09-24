use crate::vulkan::vk_bindgen::{
	vkGetPhysicalDeviceMemoryProperties,
	VkMemoryPropertyFlags,
	VkDevice,
	VkBufferCreateInfo,
	VkBuffer,
	vkCreateBuffer,
	VkResult,
	VkMemoryAllocateInfo,
	VkDeviceMemory,
	vkAllocateMemory,
	VkPhysicalDevice,
};

use std::ptr::null_mut as nullptr;

pub unsafe fn find_memory_type(
	physical_device: VkPhysicalDevice, 
	type_filter: u32, 
	properties: VkMemoryPropertyFlags
) -> Option<u32>
{
	let mut memory_properties = std::mem::zeroed();
	vkGetPhysicalDeviceMemoryProperties(physical_device, &mut memory_properties);

	for i in 0..memory_properties.memoryTypeCount
	{
		if ((type_filter & (1 << i)) > 0) && 
			((memory_properties.memoryTypes[i as usize].propertyFlags & properties) == properties)
		{
			return Some(i);
		}
	}

	None
}

// this shouldn't panic wtf, remove the .expect() below
pub fn vk_create_buffer(
	device: VkDevice,
	create_info: &VkBufferCreateInfo,
) -> Result<VkBuffer, String>
{
	unsafe 
	{
		let mut buffer: VkBuffer = nullptr();

		match vkCreateBuffer(device, create_info, nullptr(), &mut buffer)
		{
			VkResult::VK_SUCCESS => { Ok(buffer.as_mut().expect("vk_create_buffer returned nullptr")) }
			err => { Err(format!("vk_create_buffer error {:?}", err)) }
		}
	}
}

pub fn vk_allocate_memory(
	device: VkDevice,
	allocate_info: &VkMemoryAllocateInfo
) -> Result<VkDeviceMemory, String>
{
	unsafe 
	{
		let mut device_memory: VkDeviceMemory = nullptr();

		match vkAllocateMemory(device, allocate_info, nullptr(), &mut device_memory)
		{
			VkResult::VK_SUCCESS => { Ok(device_memory.as_mut().expect("vk_allocate_memory returned nullptr")) }
			err => { Err(format!("vk_allocate_memory error {:?}", err))}
		}
	}
}