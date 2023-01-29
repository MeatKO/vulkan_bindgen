use crate::vulkan::vk_bindgen::*;
use crate::vulkan::handle::*;

pub unsafe fn find_memory_type(vk_handle: &VkHandle, type_filter: u32, properties: VkMemoryPropertyFlags) -> u32
{
	let mut memory_properties = std::mem::zeroed();
	vkGetPhysicalDeviceMemoryProperties(vk_handle.physical_device, &mut memory_properties);

	for i in 0..memory_properties.memoryTypeCount
	{
		if ((type_filter & (1 << i)) > 0) && (memory_properties.memoryTypes[i as usize].propertyFlags & properties) == properties
		{
			return i;
		}
	}

	panic!("failed to find suitable memory type");
}