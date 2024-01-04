use crate::{vulkan::{wrappers::vk_buffer::VulkanBuffer, vertex::Vertex, vk_bindgen::VkDescriptorSet}, detail_core::texture::texture::{VulkanTexture, Texture}};

use std::ptr::null_mut as nullptr;

pub struct MeshVulkanBuffers
{
	pub vertex: VulkanBuffer,
	pub index: VulkanBuffer,
	pub index_count: usize,
}

impl MeshVulkanBuffers
{
	pub fn new_empty() -> Self
	{
		Self
		{
			vertex: VulkanBuffer::new_empty(),
			index: VulkanBuffer::new_empty(),
			index_count: 0,
		}
	}
}

pub struct MeshBuffers
{
	pub vertex: Vec<Vertex>,
	pub index: Vec<u32>,
}

pub struct MeshAsset
{
	pub name: String,
	pub material_asset_name: String,
	pub mesh_vulkan_buffers: MeshVulkanBuffers,
	pub mesh_buffers: MeshBuffers,
}

impl MeshAsset
{
	pub fn new_empty(name: String) -> Self
	{
		Self
		{
			name,
			material_asset_name: String::new(),
			mesh_vulkan_buffers: MeshVulkanBuffers::new_empty(),
			mesh_buffers: MeshBuffers
			{
				vertex: Vec::new(),
				index: Vec::new(),
			},
		}
	}
}

pub struct MaterialAsset
{
	pub name: String,
	pub smooth_shading: bool,
	pub descriptor_set: VkDescriptorSet,
	pub albedo_map: Texture<VulkanTexture>,
	pub normal_map: Texture<VulkanTexture>,
}

impl MaterialAsset
{
	pub fn new_empty(name: String) -> Self
	{
		Self
		{
			name: name,
			smooth_shading: false,
			descriptor_set: nullptr(),
			albedo_map: Texture::new_invalid(),
			normal_map: Texture::new_invalid(),
		}
	}
}

pub struct ModelAsset
{
	pub name: String,
	pub meshes: Vec<MeshAsset>,
}

impl ModelAsset
{
	pub fn new_empty(name: String) -> Self
	{
		Self
		{
			name,
			meshes: Vec::new(),
		}
	}
}