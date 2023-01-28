use crate::vulkan::vk_bindgen::*;
use crate::vulkan::handle::*;
use crate::vulkan::memory::*;
use crate::calcium::{vec2::*, vec3::*};

use crate::ffi::offsetof::offset_of;

use std::ffi::c_void;
use std::mem::size_of;
use std::ptr::null_mut as nullptr;

pub struct Vertex
{
	pub pos: Vec2,
	pub color: Vec3
}

impl Vertex
{
	pub fn get_binding_description() -> VkVertexInputBindingDescription
	{
		return VkVertexInputBindingDescription{
			binding: 0,
			stride: size_of::<Vertex>() as u32,
			inputRate: VkVertexInputRate::VK_VERTEX_INPUT_RATE_VERTEX
		}
	}

	pub fn get_attribute_descriptions() -> [VkVertexInputAttributeDescription; 2]
	{
		unsafe
		{
			return [
				VkVertexInputAttributeDescription{
					binding: 0,
					location: 0,
					format: VkFormat::VK_FORMAT_R32G32_SFLOAT,
					offset: offset_of!(Vertex, pos) as u32
				},
				VkVertexInputAttributeDescription{
					binding: 0,
					location: 1,
					format: VkFormat::VK_FORMAT_R32G32B32_SFLOAT,
					offset: offset_of!(Vertex, color) as u32
				}
			]	
		}
	}
}

pub fn create_vertex_buffer(vk_handle: &mut VkHandle)
{
	let vertex_buffer_create_info = VkBufferCreateInfo{
		sType: VkStructureType::VK_STRUCTURE_TYPE_BUFFER_CREATE_INFO,
		size: (size_of::<Vertex>() * vk_handle.vertices.len()) as u64,
		usage: VkBufferUsageFlagBits::VK_BUFFER_USAGE_VERTEX_BUFFER_BIT as u32,
		sharingMode: VkSharingMode::VK_SHARING_MODE_EXCLUSIVE,
		queueFamilyIndexCount: 0,
		pQueueFamilyIndices: nullptr(),
		flags: 0,	
		pNext: nullptr(),
	};

	unsafe 
	{
		match vkCreateBuffer(vk_handle.logical_device, &vertex_buffer_create_info, nullptr(), &mut vk_handle.vertex_buffer)
		{
			VkResult::VK_SUCCESS => { println!("✔️ vkCreateBuffer()"); }
			err => { panic!("✗ vkCreateBuffer() failed with code {:?}.", err); }
		}

		let mut memory_requirements: VkMemoryRequirements = std::mem::zeroed();
		vkGetBufferMemoryRequirements(vk_handle.logical_device, vk_handle.vertex_buffer, &mut memory_requirements);

		let memory_allocate_info = VkMemoryAllocateInfo{
			sType: VkStructureType::VK_STRUCTURE_TYPE_MEMORY_ALLOCATE_INFO,
			allocationSize: memory_requirements.size,
			memoryTypeIndex: find_memory_type(
				vk_handle, 
				memory_requirements.memoryTypeBits, 
				VkMemoryPropertyFlagBits::VK_MEMORY_PROPERTY_HOST_VISIBLE_BIT as u32 | VkMemoryPropertyFlagBits::VK_MEMORY_PROPERTY_HOST_COHERENT_BIT as u32
			),
			pNext: nullptr(),
		};

		match vkAllocateMemory(vk_handle.logical_device, &memory_allocate_info, nullptr(), &mut vk_handle.vertex_buffer_memory)
		{
			VkResult::VK_SUCCESS => { println!("✔️ vkAllocateMemory()"); }
			err => { panic!("✗ vkAllocateMemory() failed with code {:?}.", err); }
		}

        vkBindBufferMemory(vk_handle.logical_device, vk_handle.vertex_buffer, vk_handle.vertex_buffer_memory, 0);

       	let mut data: *mut c_void = nullptr();
        vkMapMemory(vk_handle.logical_device, vk_handle.vertex_buffer_memory, 0, vertex_buffer_create_info.size, 0, &mut data);
		std::ptr::copy_nonoverlapping(vk_handle.vertices.as_ptr(), data as _, vertex_buffer_create_info.size as usize);
        vkUnmapMemory(vk_handle.logical_device, vk_handle.vertex_buffer_memory);
	}
}