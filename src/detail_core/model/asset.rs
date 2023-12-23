use crate::vulkan::{vk_bindgen::{VkBuffer, VkDeviceMemory}, wrappers::vk_buffer::VulkanBuffer, vertex::Vertex};

pub struct MeshVulkanBuffers
{
	pub vertex: VulkanBuffer,
	pub index: VulkanBuffer,
}

pub struct MeshBuffers
{
	pub vertex: Vec<Vertex>,
	pub index: Vec<u32>,
}

pub struct ModelAsset
{
	pub name: String,
	pub material_asset_name: String,
	pub mesh_vulkan_buffers: MeshVulkanBuffers,
	pub mesh_buffers: MeshBuffers,
}

impl ModelAsset
{
	pub fn new(
		name: String,
		material_asset_name: String,
		mesh_vulkan_buffers: MeshVulkanBuffers,
		mesh_buffers: MeshBuffers,
	) -> ModelAsset
	{
		ModelAsset
		{
			name,
			material_asset_name,
			mesh_vulkan_buffers,
			mesh_buffers,
		}
	}

	pub fn new_empty() -> ModelAsset
	{
		ModelAsset
		{
			name: String::new(),
			material_asset_name: String::new(),
			mesh_vulkan_buffers: MeshVulkanBuffers
			{
				vertex: VulkanBuffer::new_empty(),
				index: VulkanBuffer::new_empty(),
			},
			mesh_buffers: MeshBuffers
			{
				vertex: Vec::new(),
				index: Vec::new(),
			},
		}
	}
}