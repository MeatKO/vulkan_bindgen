use crate::vulkan::descriptor_set_hud::create_descriptor_sets_hud;
use crate::vulkan::handle::VkHandle;
use crate::vulkan::uniform_buffer::UniformBufferObject;
use crate::vulkan::uniform_buffer_hud::{UniformBufferObjectHUD, create_uniform_buffers_hud};
use crate::vulkan::vk_bindgen::{
	VkBuffer, 
	VkDeviceMemory,
	VkDescriptorPool, 
	VkDescriptorSet,
};

use std::ffi::FromVecWithNulError;
use std::{ptr::null_mut as nullptr, ops::{Deref, DerefMut}, path::PathBuf};

use crate::{cotangens::{vec3::*, vec2::Vec2}, exedra::{error::ModelLoadError, model_descriptor::ModelDescriptor}, detail_core::texture::texture::{Texture, VulkanTexture}, vulkan::{vertex::{create_vertex_buffer, Vertex}, index::create_index_buffer, descriptor_set::create_descriptor_sets, descriptor_pool::create_descriptor_pool, uniform_buffer::create_uniform_buffers, vk_bindgen::VkFormat}};


use super::traits::HUDElement;

#[derive(Clone)]
pub struct VulkanButtonData
{
	pub vertex_buffer: VkBuffer,
	pub vertex_buffer_memory: VkDeviceMemory,

	pub index_buffer: VkBuffer,
	pub index_buffer_memory: VkDeviceMemory,

	pub index_count: u32,

	pub uniform_buffers: Vec<VkBuffer>,
	pub uniform_buffers_memory: Vec<VkDeviceMemory>,
	pub uniform_buffers_mapped: Vec<*mut UniformBufferObject>,

	pub descriptor_pool: VkDescriptorPool,
	pub descriptor_sets: Vec<VkDescriptorSet>,
}

pub struct UIButton
{
	pub origin_x: i32,
	pub origin_y: i32,
	pub width: i32,
	pub height: i32,
	pub vulkan_data: Option<VulkanButtonData>,
}

impl UIButton
{
	pub fn new(origin_x: i32, origin_y: i32, width: i32, height: i32) -> UIButton
	{
		UIButton { 
			origin_x: origin_x, 
			origin_y: origin_y, 
			width: width, 
			height: height, 
			vulkan_data: None
		}
	}

	pub fn get_vertices(&self) -> Vec<Vertex>
	{
		vec![
			Vertex{ 
				pos: Vec3{ x: self.origin_x as _, y: self.origin_y as _, z: 0.0f32 }, uv: Vec2{ x: 0.0f32, y: 0.0f32 }, normal: Vec3{ x: 0.0f32, y: 0.0f32, z: 0.0f32 }, tangent: Vec3{ x: 0.0f32, y: 0.0f32, z: 0.0f32 }, bitangent: Vec3{ x: 0.0f32, y: 0.0f32, z: 0.0f32 },
			},
			Vertex{ 
				pos: Vec3{ x: (self.origin_x + self.width) as _, y: self.origin_y as _, z: 0.0f32 }, uv: Vec2{ x: 0.0f32, y: 0.0f32 }, normal: Vec3{ x: 0.0f32, y: 0.0f32, z: 0.0f32 }, tangent: Vec3{ x: 0.0f32, y: 0.0f32, z: 0.0f32 }, bitangent: Vec3{ x: 0.0f32, y: 0.0f32, z: 0.0f32 },
			},
			Vertex{ 
				pos: Vec3{ x: (self.origin_x + self.width) as _, y: (self.origin_y + self.height) as _, z: 0.0f32 }, uv: Vec2{ x: 0.0f32, y: 0.0f32 }, normal: Vec3{ x: 0.0f32, y: 0.0f32, z: 0.0f32 }, tangent: Vec3{ x: 0.0f32, y: 0.0f32, z: 0.0f32 }, bitangent: Vec3{ x: 0.0f32, y: 0.0f32, z: 0.0f32 },
			},
			Vertex{ 
				pos: Vec3{ x: self.origin_x as _, y: (self.origin_y + self.height) as _, z: 0.0f32 }, uv: Vec2{ x: 0.0f32, y: 0.0f32 }, normal: Vec3{ x: 0.0f32, y: 0.0f32, z: 0.0f32 }, tangent: Vec3{ x: 0.0f32, y: 0.0f32, z: 0.0f32 }, bitangent: Vec3{ x: 0.0f32, y: 0.0f32, z: 0.0f32 },
			},
		]
	}

	// indices are intentionally ordered as two opposingly oriented triangles, problems will emerge if face culling is enabled
	pub fn get_indices(&self) -> Vec<u32>
	{
		vec![
			0, 1, 2,
			2, 3, 0,
		]
	}

	pub fn process_vulkan(mut self, vk_handle: &VkHandle, texture_map: Texture<VulkanTexture>) -> Result<UIButton, ()>
	{
		unsafe
		{
			let vertex_vec = self.get_vertices();
			let index_vec = self.get_indices();
			let (vertex_buffer, vertex_buffer_memory) =
			create_vertex_buffer(&vk_handle, &vertex_vec)
			.unwrap();

			let (index_buffer, index_buffer_memory) =
				create_index_buffer(&vk_handle, &index_vec)
				.unwrap();

			let mut vulkan_data =
				VulkanButtonData{
					vertex_buffer: vertex_buffer,
					vertex_buffer_memory: vertex_buffer_memory,
					index_buffer: index_buffer,
					index_buffer_memory: index_buffer_memory,
					index_count: index_vec.len() as _,
					uniform_buffers: vec![],
					uniform_buffers_memory: vec![],
					uniform_buffers_mapped: vec![],
					descriptor_pool: nullptr(),
					descriptor_sets: vec![],
				};

			create_uniform_buffers_hud(&vk_handle, &mut vulkan_data);

			let descriptor_pool = create_descriptor_pool(&vk_handle).unwrap();
			create_descriptor_sets_hud(&vk_handle, &mut vulkan_data, &texture_map, &texture_map, &descriptor_pool).unwrap();
			vulkan_data.descriptor_pool = descriptor_pool;
			
			self.vulkan_data = Some(vulkan_data);

			Ok(self)
		}
	}
}

impl HUDElement for UIButton
{
	fn is_inside(&self, cursor_pos_x: i32, cursor_pos_y: i32) -> bool 
	{
		if cursor_pos_x > self.origin_x && cursor_pos_x < (self.origin_x + self.width)
		{
			if cursor_pos_y > self.origin_y && cursor_pos_y < (self.origin_y + self.height)
			{
				return true
			}
		}

		return false
	}

	fn get_vulkan_data(&self) -> Option<VulkanButtonData>
	{
		self.vulkan_data.clone()
	}
}