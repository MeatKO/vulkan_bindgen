use crate::vulkan::vk_bindgen::{
	VkMemoryPropertyFlags,
	VkBufferCreateInfo,
	VkBuffer,
	VkResult,
	VkMemoryAllocateInfo,
	VkDeviceMemory,
	VkDeviceSize,
	VkBufferUsageFlags,
	VkStructureType,
	VkSharingMode,
	VkMemoryRequirements,
	vkGetBufferMemoryRequirements,
	vkBindBufferMemory,
	VkCommandBufferAllocateInfo, 
	VkCommandBufferLevel, 
	vkAllocateCommandBuffers, 
	VkCommandBufferBeginInfo, 
	VkCommandBufferUsageFlagBits, 
	vkBeginCommandBuffer, 
	VkBufferCopy, 
	vkCmdCopyBuffer, 
	VkSubmitInfo, 
	vkEndCommandBuffer, 
	vkQueueWaitIdle, 
	vkFreeCommandBuffers, 
	vkQueueSubmit,
};

use crate::vulkan::vk_memory::{
	vk_create_buffer, 
	vk_allocate_memory,
	find_memory_type,
};

use crate::vulkan::handle::VkHandle;

use std::ptr::null_mut as nullptr;

pub unsafe fn create_buffer(
	vk_handle: &VkHandle,
	byte_size: VkDeviceSize, 
	buffer_usage_flags: VkBufferUsageFlags, 
	properties: VkMemoryPropertyFlags,
) -> Result<(VkBuffer, VkDeviceMemory), String>
{
	let buffer_create_info = VkBufferCreateInfo{
		sType: VkStructureType::VK_STRUCTURE_TYPE_BUFFER_CREATE_INFO,
		size: byte_size,
		usage: buffer_usage_flags,
		sharingMode: VkSharingMode::VK_SHARING_MODE_EXCLUSIVE,
		flags: 0,
		queueFamilyIndexCount: 0,
		pQueueFamilyIndices: nullptr(),
		pNext: nullptr(),
	};

	let buffer = vk_create_buffer(vk_handle.logical_device, &buffer_create_info)?;

	// move this shit out of here, create a physical device wrapper containing these
	let mut memory_requirements: VkMemoryRequirements = std::mem::zeroed();
	vkGetBufferMemoryRequirements(vk_handle.logical_device, buffer, &mut memory_requirements);

	let memory_allocate_info = 
		VkMemoryAllocateInfo{
			sType: VkStructureType::VK_STRUCTURE_TYPE_MEMORY_ALLOCATE_INFO,
			allocationSize: memory_requirements.size,
			memoryTypeIndex: find_memory_type(
				vk_handle.physical_device, 
				memory_requirements.memoryTypeBits, 
				properties
			).unwrap(), // handle this later !
			pNext: nullptr(),
		};

	let buffer_memory = vk_allocate_memory(vk_handle.logical_device, &memory_allocate_info)?;

	vkBindBufferMemory(vk_handle.logical_device, buffer, buffer_memory, 0);

	return Ok((buffer, buffer_memory))
}

pub unsafe fn copy_buffer(
	vk_handle: &VkHandle,
	source_buffer: VkBuffer,
	destination_buffer: VkBuffer,
	size: VkDeviceSize,
) -> Result<(), String>
{
	let command_buffer_allocate_info = VkCommandBufferAllocateInfo{
		sType: VkStructureType::VK_STRUCTURE_TYPE_COMMAND_BUFFER_ALLOCATE_INFO,
		level: VkCommandBufferLevel::VK_COMMAND_BUFFER_LEVEL_PRIMARY,
		commandPool: vk_handle.command_pool.as_ref().unwrap().get_command_pool_ptr(),
		commandBufferCount: 1,
		pNext: nullptr(),
	};

	let mut command_buffer = std::mem::zeroed();
	match vkAllocateCommandBuffers(vk_handle.logical_device, &command_buffer_allocate_info, &mut command_buffer)
	{
		VkResult::VK_SUCCESS => {}
		err => { return Err(format!("vkAllocateCommandBuffers failed with code {:?}", err))}
	}

	let command_buffer_begin_info = VkCommandBufferBeginInfo{
		sType: VkStructureType::VK_STRUCTURE_TYPE_COMMAND_BUFFER_BEGIN_INFO,
		flags: VkCommandBufferUsageFlagBits::VK_COMMAND_BUFFER_USAGE_ONE_TIME_SUBMIT_BIT as u32,
		pInheritanceInfo: nullptr(),
		pNext: nullptr(),
	};

	match vkBeginCommandBuffer(command_buffer, &command_buffer_begin_info)
	{
		VkResult::VK_SUCCESS => {}
		err => { return Err(format!("vkBeginCommandBuffer failed with code {:?}", err))}
	}

	let buffer_copy_region = 
		VkBufferCopy{
			size: size,
			srcOffset: 0,
			dstOffset: 0
		};

	vkCmdCopyBuffer(command_buffer, source_buffer, destination_buffer, 1, &buffer_copy_region);
	vkEndCommandBuffer(command_buffer);

	let submit_info = VkSubmitInfo{
		sType: VkStructureType::VK_STRUCTURE_TYPE_SUBMIT_INFO,
		commandBufferCount: 1,
		pCommandBuffers: &command_buffer,
		waitSemaphoreCount: 0,
		pWaitSemaphores: nullptr(),
		signalSemaphoreCount: 0,
		pSignalSemaphores: nullptr(),
		pWaitDstStageMask: nullptr(),
		pNext: nullptr(),
	};

	vkQueueSubmit(vk_handle.graphics_queue, 1, &submit_info, nullptr());
	vkQueueWaitIdle(vk_handle.graphics_queue);

	vkFreeCommandBuffers(vk_handle.logical_device, vk_handle.command_pool.as_ref().unwrap().get_command_pool_ptr(), 1, &command_buffer);

	Ok(())
}