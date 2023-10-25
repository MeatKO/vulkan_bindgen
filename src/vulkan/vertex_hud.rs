use crate::vulkan::vk_bindgen::*;
use crate::vulkan::handle::*;
use crate::vulkan::vk_buffer::*;
use crate::cotangens::{vec2::*, vec3::*};

use crate::ffi::offsetof::offset_of;

use std::ffi::c_void;
use std::mem::size_of;
use std::ptr::null_mut as nullptr;

#[derive(Eq, Hash, PartialEq, Clone)]
pub struct VertexHud
{
	pub pos: Vec3,
	pub color: Vec3,
}

impl VertexHud
{
	pub fn get_binding_description() -> VkVertexInputBindingDescription
	{
		return VkVertexInputBindingDescription{
			binding: 0,
			stride: size_of::<VertexHud>() as u32,
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
					offset: offset_of!(VertexHud, pos) as u32
				},
				VkVertexInputAttributeDescription{
					binding: 0,
					location: 1,
					format: VkFormat::VK_FORMAT_R32G32B32_SFLOAT,
					offset: offset_of!(VertexHud, color) as u32
				},
			]	
		}
	}
}