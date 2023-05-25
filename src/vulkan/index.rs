use crate::vulkan::vk_bindgen::*;
use crate::vulkan::handle::*;
use crate::vulkan::vk_buffer::*;

use std::ffi::c_void;
use std::mem::size_of;
use std::ptr::null_mut as nullptr;

// fn size_of_elem<T>(_: &[T]) -> usize 
// {
//     std::mem::size_of::<T>()
// }

// pub unsafe fn create_index_buffer(vk_handle: &mut VkHandle)

pub unsafe fn create_index_buffer<IndexSize>(
	vk_handle: &mut VkHandle, 
	indices: &mut Vec<IndexSize>,
) -> Result<(VkBuffer, VkDeviceMemory), String>
{
	let buffer_size = size_of::<IndexSize>() * indices.len();
	
	let (staging_buffer, staging_buffer_memory) = 
		match create_buffer(
				vk_handle, 
				buffer_size as u64,
				VkBufferUsageFlagBits::VK_BUFFER_USAGE_TRANSFER_SRC_BIT as u32,
				VkMemoryPropertyFlagBits::VK_MEMORY_PROPERTY_HOST_VISIBLE_BIT as u32 |
				VkMemoryPropertyFlagBits::VK_MEMORY_PROPERTY_HOST_COHERENT_BIT as u32
			)
		{
			Ok(tuple) => { tuple }
			// Err(e) => { panic!("Couldn't create index staging buffer - {}", e) }
			Err(e) => { return Err(format!("Couldn't create index staging buffer - {}", e)) }
		};

	let mut data: *mut c_void = nullptr();
	vkMapMemory(vk_handle.logical_device, staging_buffer_memory, 0, buffer_size as u64, 0, &mut data);
	std::ptr::copy_nonoverlapping(indices.as_ptr(), data as _, indices.len());
	vkUnmapMemory(vk_handle.logical_device, staging_buffer_memory);

	let (buffer, buffer_memory) = 
		match create_buffer(
			vk_handle, 
			buffer_size as u64,
			VkBufferUsageFlagBits::VK_BUFFER_USAGE_TRANSFER_DST_BIT as u32 |
			VkBufferUsageFlagBits::VK_BUFFER_USAGE_INDEX_BUFFER_BIT as u32,
			VkMemoryPropertyFlagBits::VK_MEMORY_PROPERTY_DEVICE_LOCAL_BIT as u32
		)
		{
			Ok(tuple) => { tuple }
			// Err(e) => { panic!("Couldn't create vertex buffer - {}", e) }
			Err(e) => { return Err(format!("Couldn't create vertex buffer - {}", e)) }
		};

	match copy_buffer(vk_handle, staging_buffer, buffer, buffer_size as u64)
	{
		Ok(_) => {}
		// Err(e) => { panic!("Couldn't copy vertex buffer - {}", e) }
		Err(e) => { return Err(format!("Couldn't copy vertex buffer - {}", e)) }
	}

	vkDestroyBuffer(vk_handle.logical_device, staging_buffer, nullptr());
	vkFreeMemory(vk_handle.logical_device, staging_buffer_memory, nullptr());

	return Ok((buffer, buffer_memory))

	// vk_handle.index_buffer = buffer;
	// vk_handle.index_buffer_memory = buffer_memory;
}