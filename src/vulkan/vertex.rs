use crate::vulkan::vk_bindgen::*;
use crate::vulkan::handle::*;
use crate::vulkan::vk_buffer::*;
use crate::cotangens::{vec2::*, vec3::*};

use crate::ffi::offsetof::offset_of;

use std::ffi::c_void;
use std::mem::size_of;
use std::ptr::null_mut as nullptr;

#[derive(Eq, Hash, PartialEq, Clone)]
pub struct Vertex
{
	pub pos: Vec3,
	pub uv: Vec2,
	pub normal: Vec3,
	pub tangent: Vec3,
	pub bitangent: Vec3,
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

	// pub fn get_attribute_descriptions() -> [VkVertexInputAttributeDescription; 3]
	pub fn get_attribute_descriptions() -> Vec<VkVertexInputAttributeDescription>
	{
		unsafe
		{
			return vec![
				VkVertexInputAttributeDescription{
					binding: 0,
					location: 0,
					format: VkFormat::VK_FORMAT_R32G32B32_SFLOAT,
					offset: offset_of!(Vertex, pos) as u32
				},
				VkVertexInputAttributeDescription{
					binding: 0,
					location: 1,
					format: VkFormat::VK_FORMAT_R32G32_SFLOAT,
					offset: offset_of!(Vertex, uv) as u32
				},
				VkVertexInputAttributeDescription{
					binding: 0,
					location: 2,
					format: VkFormat::VK_FORMAT_R32G32B32_SFLOAT,
					offset: offset_of!(Vertex, normal) as u32
				},
				VkVertexInputAttributeDescription{
					binding: 0,
					location: 3,
					format: VkFormat::VK_FORMAT_R32G32B32_SFLOAT,
					offset: offset_of!(Vertex, tangent) as u32
				},
				VkVertexInputAttributeDescription{
					binding: 0,
					location: 4,
					format: VkFormat::VK_FORMAT_R32G32B32_SFLOAT,
					offset: offset_of!(Vertex, bitangent) as u32
				},
			]	
		}
	}
}

pub unsafe fn create_vertex_buffer(
	vk_handle: &VkHandle, 
	vertices: &mut Vec<Vertex>,
) -> Result<(VkBuffer, VkDeviceMemory), String>
{
	let buffer_size = size_of::<Vertex>() * vertices.len();

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
			// Err(e) => { panic!("Couldn't create vertex staging buffer - {}", e) }
			Err(e) => { return Err(format!("Couldn't create vertex staging buffer - {}", e)) }
		};

	let mut data: *mut c_void = nullptr();
	vkMapMemory(vk_handle.logical_device, staging_buffer_memory, 0, buffer_size as u64, 0, &mut data);
	
	std::ptr::copy_nonoverlapping(vertices.as_ptr(), data as _, vertices.len());
	
	vkUnmapMemory(vk_handle.logical_device, staging_buffer_memory);

	let (buffer, buffer_memory) = 
		match create_buffer(
			vk_handle, 
			buffer_size as u64,
			VkBufferUsageFlagBits::VK_BUFFER_USAGE_TRANSFER_DST_BIT as u32 |
			VkBufferUsageFlagBits::VK_BUFFER_USAGE_VERTEX_BUFFER_BIT as u32,
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

	// vk_handle.vertex_buffer = buffer;
	// vk_handle.vertex_buffer_memory = buffer_memory;
}